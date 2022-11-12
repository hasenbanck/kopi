use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    ffi::{c_int, c_void},
    marker::PhantomData,
    sync::Arc,
};

use v8::NewStringType;

use crate::{
    runtime::STATE_DATA_SLOT,
    traits::{FromValue, IntoValue},
    value::{self, Seal, Unseal},
};

/// Traits for static functions, which can be called faster than closures.
///
/// This trait should be implemented by using the [`crate::static_function`] macro.
pub trait StaticFunction {
    #[doc(hidden)]
    fn callback() -> v8::FunctionCallback;
}

/// Traits for fastcall functions, which are the fastest to call.
///
/// # Safety
///
/// Implementer of this trait must make sure that the function does not:
///  * Call any ECMAScript functionality inside V8.
///  * Allocate on the V8 heap.
///
/// This trait should be implemented by using the [`crate::fastcall_function`] macro.
pub unsafe trait FastcallFunction: v8::fast_api::FastFunction {
    #[doc(hidden)]
    fn callback() -> v8::FunctionCallback;
}

/// Trait for the arguments of extension functions.
///
/// This is a sealed trait that is not supposed to be implemented outside the crate.
pub trait FunctionArguments<F, R>: private::Sealed {
    #[doc(hidden)]
    fn call<'scope>(
        scope: &mut v8::HandleScope<'scope>,
        args: v8::FunctionCallbackArguments<'scope>,
        rv: v8::ReturnValue,
        cb_data: &F,
    );
}

/// Trait for the arguments of extension functions that can mutate the runtime state.
///
/// This is a sealed trait that is not supposed to be implemented outside the crate.
pub trait FunctionWithStateArguments<F, R, S>: private::Sealed {
    #[doc(hidden)]
    fn call<'scope>(
        scope: &mut v8::HandleScope<'scope>,
        args: v8::FunctionCallbackArguments<'scope>,
        rv: v8::ReturnValue,
        cb_data: &F,
        state: &mut S,
    );
}

mod private {
    /// Seal for the [`super::FunctionArguments`] trait.
    pub trait Sealed {}
}

// Must be public because of the `static_function` macro.
#[doc(hidden)]
#[inline(always)]
pub fn set_result<'scope, R>(
    scope: &mut v8::HandleScope<'scope>,
    mut rv: v8::ReturnValue,
    result: R,
) where
    R: 'static + IntoValue,
{
    let scope = scope.seal();

    // Some types can skip the serialization, like for example `()`.
    if !R::is_undefined() {
        let value = match result.into_v8(scope) {
            Ok(value) => value,
            Err(err) => {
                let msg = value::String::new(scope, String::from(err), NewStringType::Normal);
                value::Error::new_type_error(scope, msg)
            }
        };
        rv.set(value.unseal());
    }
}

// Must be public because of the `static_function` macro.
#[doc(hidden)]
#[inline(always)]
pub fn get_argument<'scope, A>(
    scope: &mut v8::HandleScope<'scope>,
    args: &v8::FunctionCallbackArguments<'scope>,
    rv: &mut v8::ReturnValue,
    pos: c_int,
) -> Option<A>
where
    A: FromValue<Value = A>,
{
    let scope = scope.seal();

    let local_value = args.get(pos);
    return match A::from_v8(scope, local_value.seal()) {
        Ok(arg) => Some(arg),
        Err(err) => {
            let msg = value::String::new(scope, &String::from(err), NewStringType::Normal);
            let error = value::Error::new_type_error(scope, msg);
            rv.set(error.unseal());
            None
        }
    };
}

#[rustfmt::skip]
macro_rules! impl_function_arguments {
    () => (
        impl<FN, RE> FunctionArguments<FN, RE> for ()
        where
            FN: 'static + Send + Sync + Fn(()) -> RE,
            RE: 'static + IntoValue,
        {
            #[inline(always)]
            fn call<'scope>(
                scope: &mut v8::HandleScope<'scope>,
                _args: v8::FunctionCallbackArguments<'scope>,
                rv: v8::ReturnValue,
                op: &FN,
            ) {
                let result = op(());
                set_result(scope, rv, result);
            }
        }
        
        impl<FN, RE, STATE> FunctionWithStateArguments<FN, RE, STATE> for ()
        where
            FN: 'static + Send + Sync + Fn(&mut STATE, ()) -> RE,
            RE: 'static + IntoValue,
        {
            #[inline(always)]
            fn call<'scope>(
                scope: &mut v8::HandleScope<'scope>,
                _args: v8::FunctionCallbackArguments<'scope>,
                rv: v8::ReturnValue,
                op: &FN,
                state: &mut STATE
            ) {
                let result = op(state, ());
                set_result(scope, rv, result);
            }
        }
        
        impl private::Sealed for () {}
    );
    ($($generic:ident)*; $($arg:ident)*; $($count:literal)*) => {
        impl<FN, RE, $($generic,)*> FunctionArguments<FN, RE> for ($($generic,)*)
        where
            FN: 'static + Send + Sync + Fn(($($generic,)*)) -> RE,
            RE: 'static + IntoValue,
            $($generic: FromValue<Value = $generic>,)*
        {
            #[inline(always)]
            fn call<'scope>(
                scope: &mut v8::HandleScope<'scope>,
                args: v8::FunctionCallbackArguments<'scope>,
                mut rv: v8::ReturnValue,
                op: &FN,
            ) {
                $(
                let Some($arg) = get_argument(scope, &args, &mut rv, $count) else {
                    return;
                };
                )*
                let result = op(($($arg,)*));
                set_result(scope, rv, result);
            }
        }

        impl<FN, RE, STATE, $($generic,)*> FunctionWithStateArguments<FN, RE, STATE> for ($($generic,)*)
        where
            FN: 'static + Send + Sync + Fn(&mut STATE, ($($generic,)*)) -> RE,
            RE: 'static + IntoValue,
            $($generic: FromValue<Value = $generic>,)*
        {
            #[inline(always)]
            fn call<'scope>(
                scope: &mut v8::HandleScope<'scope>,
                args: v8::FunctionCallbackArguments<'scope>,
                mut rv: v8::ReturnValue,
                op: &FN,
                state: &mut STATE
            ) {
                $(
                let Some($arg) = get_argument(scope, &args, &mut rv, $count) else {
                    return;
                };
                )*
                let result = op(state, ($($arg,)*));
                set_result(scope, rv, result);
            }
        }

        impl<$($generic,)*> private::Sealed for ($($generic,)*) {}
    };
}

impl_function_arguments!();
impl_function_arguments!(
    A;
    a;
    0
);
impl_function_arguments!(
    A B;
    a b;
    0 1
);
impl_function_arguments!(
    A B C;
    a b c;
    0 1 2
);
impl_function_arguments!(
    A B C D;
    a b c d;
    0 1 2 3
);
impl_function_arguments!(
    A B C D E;
    a b c d e;
    0 1 2 3 4
);
impl_function_arguments!(
    A B C D E F;
    a b c d e f;
    0 1 2 3 4 5
);

impl_function_arguments!(
    A B C D E F G;
    a b c d e f g;
    0 1 2 3 4 5 6
);
impl_function_arguments!(
    A B C D E F G H;
    a b c d e f g h;
    0 1 2 3 4 5 6 7
);
impl_function_arguments!(
    A B C D E F G H I;
    a b c d e f g h i;
    0 1 2 3 4 5 6 7 8
);
impl_function_arguments!(
    A B C D E F G H I J;
    a b c d e f g h i j;
    0 1 2 3 4 5 6 7 8 9
);
impl_function_arguments!(
    A B C D E F G H I J K;
    a b c d e f g h i j k;
    0 1 2 3 4 5 6 7 8 9 10
);
impl_function_arguments!(
    A B C D E F G H I J K L;
    a b c d e f g h i j k l;
    0 1 2 3 4 5 6 7 8 9 10 11
);
impl_function_arguments!(
    A B C D E F G H I J K L M;
    a b c d e f g h i j k l m;
    0 1 2 3 4 5 6 7 8 9 10 11 12
);
impl_function_arguments!(
    A B C D E F G H I J K L M N;
    a b c d e f g h i j k l m n;
    0 1 2 3 4 5 6 7 8 9 10 11 12 13
);
impl_function_arguments!(
    A B C D E F G H I J K L M N O;
    a b c d e f g h i j k l m n o;
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
);
impl_function_arguments!(
    A B C D E F G H I J K L M N O P;
    a b c d e f g h i j k l m n o p;
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
);

pub(crate) enum FunctionDeclaration {
    Closure {
        cb_data: *mut c_void,
        function_callback: v8::FunctionCallback,
    },
    Static(v8::FunctionCallback),
    Fastcall {
        fastcall: Box<dyn v8::fast_api::FastFunction>,
        function_callback: v8::FunctionCallback,
    },
}

/// Creates a extension, which provide the functionality to call native Rust code from within scripts.
pub struct Extension<STATE> {
    pub(crate) namespace: Option<String>,
    pub(crate) declarations: HashMap<String, FunctionDeclaration>,
    pub(crate) closures: Vec<Arc<dyn Any>>,
    _state_marker: PhantomData<STATE>,
}

impl<STATE> Extension<STATE> {
    /// Creates a new [`Extension`]. If no namespace is given, then the functions will be created
    /// in the global namespace.
    pub fn new(namespace: Option<&str>) -> Self {
        let namespace = namespace.map(|n| n.into());
        Self {
            namespace,
            declarations: HashMap::default(),
            closures: Vec::default(),
            _state_marker: PhantomData::default(),
        }
    }

    #[inline(always)]
    fn v8_func<'borrow, 'scope, F, A, R>(
        scope: &'borrow mut v8::HandleScope<'scope>,
        args: v8::FunctionCallbackArguments<'scope>,
        rv: v8::ReturnValue,
    ) where
        F: 'static + Send + Sync + Fn(A) -> R,
        A: FunctionArguments<F, R>,
        R: IntoValue,
    {
        // SAFETY: This is safe since we made sure to leak the boxed callback (static lifetime)
        //         and the implementation makes sure, that the data contains the pointer of the
        //         expected closure callback for this function callback.
        let cb_data = unsafe {
            &*(v8::Local::<v8::External>::cast(args.data()).value() as *const c_void as *const F)
        };

        A::call(scope, args, rv, cb_data);
    }

    #[inline(always)]
    fn v8_func_with_state<'borrow, 'scope, F, A, R>(
        scope: &'borrow mut v8::HandleScope<'scope>,
        args: v8::FunctionCallbackArguments<'scope>,
        rv: v8::ReturnValue,
    ) where
        F: 'static + Send + Sync + Fn(&mut STATE, A) -> R,
        A: FunctionWithStateArguments<F, R, STATE>,
        R: IntoValue,
    {
        // SAFETY: This is safe since we made sure to leak the boxed callback (static lifetime)
        //         and the implementation makes sure, that the data contains the pointer of the
        //         expected closure callback for this function callback.
        let cb_data = unsafe {
            &*(v8::Local::<v8::External>::cast(args.data()).value() as *const c_void as *const F)
        };

        // SAFETY: This is safe since we know that the state is stored in that slot
        //         and the data is bound to the lifetime of this runtime.
        let state = unsafe { &*(scope.get_data(STATE_DATA_SLOT) as *const RefCell<STATE>) };
        let mut borrow = state.borrow_mut();

        A::call(scope, args, rv, cb_data, &mut borrow);
    }

    /// Add a function to the extension with the given name as function name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kopi::Extension;
    ///
    /// let mut extension = Extension::<()>::new(None);
    /// extension.add_function("madd", move |(a, b, c): (f32, f32, f32)| a + (b * c));
    /// ```
    pub fn add_function<F, A, R>(&mut self, name: &str, function: F)
    where
        F: 'static + Send + Sync + Fn(A) -> R,
        A: FunctionArguments<F, R>,
        R: IntoValue,
    {
        use v8::MapFnTo;

        let name = name.into();

        // We wrap the function in an Arc, so that it's lifetime can be tracked on runtimes and
        // snapshots.
        let closure = Arc::new(function);

        let cb_data = Arc::as_ptr(&closure) as *mut F as *mut c_void;
        let function_callback = Self::v8_func::<F, A, R>.map_fn_to();

        self.declarations.insert(
            name,
            FunctionDeclaration::Closure {
                cb_data,
                function_callback,
            },
        );

        self.closures.push(closure);
    }

    /// Add a function to the extension with the given name as function name and the state of the
    /// runtime.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kopi::Extension;
    ///
    /// let mut extension = Extension::<i32>::new(None);
    /// extension.add_function_with_state("adder", move |state, (x,): (i32,)| {
    ///     *state + x;
    /// });
    /// ```
    pub fn add_function_with_state<F, A, R>(&mut self, name: &str, function: F)
    where
        F: 'static + Send + Sync + Fn(&mut STATE, A) -> R,
        A: FunctionWithStateArguments<F, R, STATE>,
        R: IntoValue,
    {
        use v8::MapFnTo;

        let name = name.into();

        // We leak the callback to give it a static lifetime, so that V8 can call it safely.
        let cb_data = Box::leak(Box::new(function)) as *mut F as *mut c_void;
        let function_callback = Self::v8_func_with_state::<F, A, R>.map_fn_to();

        self.declarations.insert(
            name,
            FunctionDeclaration::Closure {
                cb_data,
                function_callback,
            },
        );
    }

    /// Add a static function to the extension with the given name as function name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kopi::{static_function, Extension};
    ///
    /// static_function! {
    ///     fn mul(x: f64, y: f64) -> f64 {
    ///         x * y
    ///     }
    /// }
    ///
    /// let mut extension = Extension::<()>::new(None);
    /// extension.add_static_function("mul", mul);
    /// ```
    #[allow(unused_variables)]
    pub fn add_static_function<F>(&mut self, name: &str, function: F)
    where
        F: 'static + StaticFunction,
    {
        let name = name.into();

        let function_callback = F::callback();

        self.declarations
            .insert(name, FunctionDeclaration::Static(function_callback));
    }

    /// Add a fastcall function to the extension with the given name as function name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kopi::{fastcall_function, Extension};
    ///
    /// fastcall_function! {
    ///     fn mul(x: f64, y: f64) -> f64 {
    ///         x * y
    ///     }
    /// }
    ///
    /// let mut extension = Extension::<()>::new(None);
    /// extension.add_fastcall_function("mul", mul);
    /// ```
    #[allow(unused_variables)]
    pub fn add_fastcall_function<F>(&mut self, name: &str, function: F)
    where
        F: 'static + FastcallFunction,
    {
        let name = name.into();

        let function_callback = F::callback();

        self.declarations.insert(
            name,
            FunctionDeclaration::Fastcall {
                fastcall: Box::new(function),
                function_callback,
            },
        );
    }
}
