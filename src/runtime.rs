//! Implements the ECMAScript runtime.

use std::{any::Any, cell::RefCell, ffi::c_void, rc::Rc, sync::Arc};

// Needs to be public for the `static_function` macro.
/// Slot inside the runtime in which we save a `Rc<RefCell<S>>` to the state `S`.
#[doc(hidden)]
pub const STATE_DATA_SLOT: u32 = 0;

use crate::{
    error::{create_error_from_exception, Error},
    extension::FunctionDeclaration,
    traits::DeserializeOwned,
    value::{new_string, NewStringType, Seal},
    Extension, HeapStatistics, V8_INITIALIZATION,
};

/// Configures a ECMAScript runtime.
pub struct RuntimeOptions<STATE> {
    /// Sets the initial size of the heap.
    pub initial_heap_size: usize,
    /// Sets the maximum size of the heap.
    pub max_heap_size: usize,
    /// Enables the capturing of the current stack trace when an uncaught exception
    /// occurs and report it to the message listeners. Sets the limit of how many
    /// frames are captures.
    pub capture_stack_trace_for_uncaught_exceptions: Option<i32>,
    /// Extensions add build-in functionality to a runtime.
    pub extensions: Vec<Extension<STATE>>,
}

impl<STATE> Default for RuntimeOptions<STATE> {
    fn default() -> Self {
        Self {
            initial_heap_size: 512 * 1024,    // 512 KiB
            max_heap_size: 512 * 1024 * 1024, // 512 MiB
            capture_stack_trace_for_uncaught_exceptions: None,
            extensions: vec![],
        }
    }
}

/// The runtime that runs ECMAScript code inside the V8 engine.
pub struct Runtime<STATE> {
    isolate: v8::OwnedIsolate,
    main_context: v8::Global<v8::Context>,
    _closures: Box<[Arc<dyn Any>]>,
    _state: Rc<RefCell<STATE>>,
}

impl<STATE> Drop for Runtime<STATE> {
    fn drop(&mut self) {
        // We want to make sure that nothing will run inside the isolate, since
        // the pointer to the state inside the isolate and closures would be invalid
        // after the drop (stored in slot STATE_DATA_SLOT).
        self.isolate.terminate_execution();
    }
}

impl<STATE> Runtime<STATE> {
    /// Creates a new [`Runtime`] with the given state.
    ///
    /// [`crate::initialize()`] must be called before instantiating a [`Runtime`].
    pub fn new(mut options: RuntimeOptions<STATE>, state: STATE) -> Result<Self, Error> {
        if !V8_INITIALIZATION.is_completed() {
            return Err(Error::V8NotInitialized);
        }

        let mut config = v8::CreateParams::default();
        config = config.heap_limits(options.initial_heap_size, options.max_heap_size);

        let mut runtime_closures = Vec::default();
        let state = Rc::new(RefCell::new(state));
        let state_ptr = Rc::as_ptr(&state) as *const RefCell<STATE> as *mut c_void;

        let mut isolate = v8::Isolate::new(config);

        if let Some(frame_limit) = options.capture_stack_trace_for_uncaught_exceptions {
            isolate.set_capture_stack_trace_for_uncaught_exceptions(true, frame_limit.max(0))
        }

        // TODO Test how namespaces are overwritten. Also support "nested" namespaces like "a.b.c".
        let main_context = {
            let isolate_scope = &mut v8::HandleScope::new(&mut isolate);
            isolate_scope.set_data(STATE_DATA_SLOT, state_ptr);

            let global_template = v8::ObjectTemplate::new(isolate_scope);

            // Set the global functions.
            for Extension {
                declarations,
                closures,
                ..
            } in options
                .extensions
                .iter_mut()
                .filter(|e| e.namespace.is_none())
            {
                for (function_name, function_declaration) in declarations.drain() {
                    let function_name =
                        new_string(isolate_scope, function_name, NewStringType::Normal);

                    let function = match function_declaration {
                        FunctionDeclaration::Closure {
                            cb_data,
                            function_callback,
                        } => {
                            let external = v8::External::new(isolate_scope, cb_data);
                            v8::FunctionTemplate::builder_raw(function_callback)
                                .data(external.into())
                                .build(isolate_scope)
                        }
                        FunctionDeclaration::Static(function_callback) => {
                            v8::FunctionTemplate::builder_raw(function_callback)
                                .build(isolate_scope)
                        }
                        FunctionDeclaration::Fastcall {
                            fastcall,
                            function_callback,
                        } => {
                            let external = v8::External::new(isolate_scope, state_ptr);
                            v8::FunctionTemplate::builder_raw(function_callback)
                                .data(external.into())
                                .build_fast(isolate_scope, &*fastcall, None)
                        }
                    };

                    global_template.set(function_name.into(), function.into());
                }

                runtime_closures.append(closures);
            }

            let global_context = v8::Context::new_from_template(isolate_scope, global_template);
            let global_context_scope = &mut v8::ContextScope::new(isolate_scope, global_context);

            // Set the global functions that are inside a namespace object.
            for Extension {
                namespace,
                declarations,
                closures,
                ..
            } in options
                .extensions
                .iter_mut()
                .filter(|e| e.namespace.is_some())
            {
                if let Some(namespace) = namespace {
                    let namespace_name =
                        new_string(global_context_scope, namespace, NewStringType::Normal);
                    let namespace_object = v8::Object::new(global_context_scope);

                    for (function_name, function_declaration) in declarations.drain() {
                        let function_name =
                            new_string(global_context_scope, function_name, NewStringType::Normal);

                        let function = match function_declaration {
                            FunctionDeclaration::Closure {
                                cb_data,
                                function_callback,
                            } => {
                                let external = v8::External::new(global_context_scope, cb_data);
                                v8::Function::builder_raw(function_callback)
                                    .data(external.into())
                                    .build(global_context_scope)
                                    .ok_or_else(|| {
                                        Error::Internal("Can't build function".to_string())
                                    })?
                            }
                            FunctionDeclaration::Static(function_callback) => {
                                v8::Function::builder_raw(function_callback)
                                    .build(global_context_scope)
                                    .ok_or_else(|| {
                                        Error::Internal("Can't build function".to_string())
                                    })?
                            }
                            FunctionDeclaration::Fastcall {
                                fastcall,
                                function_callback,
                            } => {
                                let external = v8::External::new(global_context_scope, state_ptr);
                                v8::FunctionTemplate::builder_raw(function_callback)
                                    .data(external.into())
                                    .build_fast(global_context_scope, &*fastcall, None)
                                    .get_function(global_context_scope)
                                    .ok_or_else(|| {
                                        Error::Internal("Can't build function".to_string())
                                    })?
                            }
                        };

                        namespace_object.set(
                            global_context_scope,
                            function_name.into(),
                            function.into(),
                        );
                    }

                    global_context.global(global_context_scope).set(
                        global_context_scope,
                        namespace_name.into(),
                        namespace_object.into(),
                    );
                }

                runtime_closures.append(closures);
            }

            v8::Global::new(global_context_scope, global_context)
        };

        let runtime = Self {
            isolate,
            main_context,
            _closures: runtime_closures.into_boxed_slice(),
            _state: state,
        };

        Ok(runtime)
    }

    // TODO add support for compiling modules.
    // TODO add support for creating a new runtime from a snapshot

    /// Executes the ECMAScript as a classic script inside the runtime and returns the evaluated value.
    pub fn execute<T, SOURCE>(&mut self, source: SOURCE) -> Result<T, Error>
    where
        T: DeserializeOwned,
        SOURCE: AsRef<str>,
    {
        let source = source.as_ref();

        let scope = &mut v8::HandleScope::with_context(&mut self.isolate, &self.main_context);
        let source = new_string(scope, source, NewStringType::Normal);

        let try_catch_scope = &mut v8::TryCatch::new(scope);

        let Some(script) = v8::Script::compile(try_catch_scope, source, None) else {
            let exception = try_catch_scope.exception();
            return Err(create_error_from_exception(try_catch_scope, exception));
        };

        let Some(v8_value) = script.run(try_catch_scope) else {
            let exception = try_catch_scope.exception();
            return Err(create_error_from_exception(try_catch_scope, exception));
        };

        T::deserialize(try_catch_scope.seal(), v8_value.seal()).map_err(Error::Type)
    }

    /// Returns a collection of information about the heap of the engine.
    pub fn heap_statistics(&mut self) -> HeapStatistics {
        HeapStatistics::new(&mut self.isolate)
    }
}

#[cfg(test)]
mod test {
    use std::{
        cell::RefCell,
        rc::Rc,
        sync::{
            atomic::{AtomicI32, Ordering},
            Arc,
        },
        thread::JoinHandle,
    };

    use crate::{error::Error, *};

    #[test]
    fn runtime_creation() {
        initialize_with_defaults();

        // Multiple runtimes can be created.
        let runtime0 = Runtime::new(RuntimeOptions::default(), ());
        assert!(runtime0.is_ok());

        let runtime1 = Runtime::new(RuntimeOptions::default(), ());
        assert!(runtime1.is_ok());
    }

    #[test]
    fn runtime_creation_multiple_thread() {
        initialize_with_defaults();

        let handle0: JoinHandle<Result<(), Error>> = std::thread::spawn(|| {
            let mut runtime0 = Runtime::new(RuntimeOptions::default(), ())?;
            let val: i32 = runtime0.execute("var x = 30; x")?;
            assert_eq!(val, 30);
            Ok(())
        });

        let handle1: JoinHandle<Result<(), Error>> = std::thread::spawn(|| {
            let mut runtime1 = Runtime::new(RuntimeOptions::default(), ())?;
            let val: i32 = runtime1.execute("var x = 20; x")?;
            assert_eq!(val, 20);
            Ok(())
        });

        handle0.join().expect("thread 0 died").expect("error found");
        handle1.join().expect("thread 1 died").expect("error found");
    }

    #[test]
    fn heap_statistics() {
        const MAX_HEAP_SIZE: usize = 5 * 1024 * 1024;

        initialize_with_defaults();

        let mut runtime = Runtime::new(
            RuntimeOptions {
                max_heap_size: MAX_HEAP_SIZE,
                ..Default::default()
            },
            (),
        )
        .expect("Can't not create runtime");

        let heap_statistics = runtime.heap_statistics();

        // This only tests if the values make some sense.
        assert!(heap_statistics.heap_size_limit() >= MAX_HEAP_SIZE);
        assert!(heap_statistics.total_heap_size() >= 64 * 1024);
        assert!(heap_statistics.used_heap_size() >= 64 * 1024);
        assert!(heap_statistics.total_physical_size() >= 64 * 1024);
    }

    #[test]
    fn execute_code() {
        initialize_with_defaults();
        let mut runtime =
            Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        let val: i32 = runtime.execute("42 + 3").expect("Can't execute code");

        assert_eq!(val, 45);
    }

    #[test]
    fn execute_code_is_stateful() {
        initialize_with_defaults();
        let mut runtime =
            Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        let val: i32 = runtime.execute("var x = 1; x").expect("Can't execute code");
        assert_eq!(val, 1);

        let val: i32 = runtime.execute("x += 2; x").expect("Can't execute code");
        assert_eq!(val, 3);
    }

    #[test]
    fn execute_code_compile_error() {
        initialize_with_defaults();
        let mut runtime =
            Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        let ret: Result<(), Error> = runtime.execute("var = let");
        let err = ret.expect_err("Expected an Script error");
        assert!(matches!(err, Error::Script { .. }))
    }

    #[test]
    fn execute_code_execution_error() {
        initialize_with_defaults();
        let mut runtime =
            Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        let ret: Result<(), Error> = runtime.execute("unknown_function()");
        let err = ret.expect_err("Expected an Script error");
        assert!(matches!(err, Error::Script { .. }))
    }

    #[test]
    fn execute_code_simple_functions() {
        initialize_with_defaults();

        let counter = Arc::new(AtomicI32::new(42));
        let thread_counter1 = counter.clone();
        let thread_counter2 = counter.clone();

        let mut test_extension = Extension::new(Some("test"));
        test_extension.add_function("counter", move |()| {
            thread_counter1.fetch_add(10, Ordering::SeqCst)
        });

        let mut global_extension = Extension::new(None);
        global_extension.add_function("counter", move |()| {
            thread_counter2.fetch_add(20, Ordering::SeqCst)
        });

        let mut runtime = Runtime::new(
            RuntimeOptions {
                extensions: vec![test_extension, global_extension],
                ..Default::default()
            },
            (),
        )
        .expect("Can't create runtime");

        let val: i32 = runtime
            .execute("test.counter()")
            .expect("Can't execute code");

        assert_eq!(val, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 52);

        let val: i32 = runtime.execute("counter()").expect("Can't execute code");

        assert_eq!(val, 52);
        assert_eq!(counter.load(Ordering::SeqCst), 72);
    }

    #[test]
    fn global_functions_are_global() {
        initialize_with_defaults();

        let counter = Arc::new(AtomicI32::new(10));
        let thread_counter1 = counter.clone();

        let mut global_extension = Extension::new(None);
        global_extension.add_function("counter", move |()| {
            thread_counter1.fetch_add(35, Ordering::SeqCst)
        });

        let mut runtime = Runtime::new(
            RuntimeOptions {
                extensions: vec![global_extension],
                ..Default::default()
            },
            (),
        )
        .expect("Can't create runtime");

        let _: () = runtime
            .execute("let js_counter = function() { return counter(); };")
            .expect("Can't execute code");

        let val: i32 = runtime.execute("js_counter()").expect("Can't execute code");

        assert_eq!(val, 10);
        assert_eq!(counter.load(Ordering::SeqCst), 45);
    }

    #[test]
    fn execute_code_simple_function_with_state() {
        initialize_with_defaults();

        struct State(i32);
        let state = State(55);

        let mut test_extension = Extension::new(Some("test"));
        test_extension.add_function_with_state("counter", move |state: &mut State, ()| {
            state.0 += 5;
            state.0
        });

        let mut runtime = Runtime::new(
            RuntimeOptions {
                extensions: vec![test_extension],
                ..Default::default()
            },
            state,
        )
        .expect("Can't create runtime");

        let val: i32 = runtime
            .execute("test.counter()")
            .expect("Can't execute code");

        assert_eq!(val, 60);
    }

    static_function! {
        fn sub(x: i32, y: i32) -> i32 {
            x - y
        }
    }

    #[test]
    fn execute_code_static() {
        initialize_with_defaults();

        let mut test_extension = Extension::new(Some("test"));
        test_extension.add_static_function("sub", sub);

        let mut runtime = Runtime::new(
            RuntimeOptions {
                extensions: vec![test_extension],
                ..Default::default()
            },
            (),
        )
        .expect("Can't create runtime");

        let val: i32 = runtime
            .execute("test.sub(10, 3)")
            .expect("Can't execute code");

        assert_eq!(val, 7);
    }

    static_function! {
        fn sub_from_state(state: &mut Rc<RefCell<i32>>, x: i32) {
            let mut y = state.borrow_mut();
            *y -= x;
        }
    }

    #[test]
    fn execute_code_static_with_state() {
        initialize_with_defaults();

        let state = Rc::new(RefCell::new(50i32));
        let runtime_state = state.clone();

        let mut test_extension = Extension::new(Some("test"));
        test_extension.add_static_function("sub_from_state", sub_from_state);

        let mut runtime = Runtime::new(
            RuntimeOptions {
                extensions: vec![test_extension],
                ..Default::default()
            },
            runtime_state,
        )
        .expect("Can't create runtime");

        let _: () = runtime
            .execute("test.sub_from_state(5)")
            .expect("Can't execute code");

        assert_eq!(*state.borrow(), 45);
    }

    fastcall_function! {
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }
    }

    #[test]
    fn execute_code_fastcall() {
        initialize_with_defaults();

        let mut test_extension = Extension::new(Some("test"));
        test_extension.add_fastcall_function("add", add);

        let mut runtime = Runtime::new(
            RuntimeOptions {
                extensions: vec![test_extension],
                ..Default::default()
            },
            (),
        )
        .expect("Can't create runtime");

        let val: i32 = runtime
            .execute("test.add(15, 70)")
            .expect("Can't execute code");

        assert_eq!(val, 85);
    }

    fastcall_function! {
        fn add_to_state(state: &mut Rc<RefCell<i32>>, x: i32) {
            let mut y = state.borrow_mut();
            *y += x;
        }
    }

    #[test]
    fn execute_code_fastcall_with_state() {
        initialize_with_defaults();

        let state = Rc::new(RefCell::new(99i32));
        let runtime_state = state.clone();

        let mut test_extension = Extension::new(Some("test"));
        test_extension.add_fastcall_function("add_to_state", add_to_state);

        let mut runtime = Runtime::new(
            RuntimeOptions {
                extensions: vec![test_extension],
                ..Default::default()
            },
            runtime_state,
        )
        .expect("Can't create runtime");

        let _: () = runtime
            .execute("test.add_to_state(2)")
            .expect("Can't execute code");

        assert_eq!(*state.borrow(), 101);
    }
}
