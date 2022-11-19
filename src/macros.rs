//! Implements public visible macros.

/// Helper macro to count arguments.
///
/// Can be replaced once the feature "macro_metavar_expr" is stable:
/// https://github.com/rust-lang/rust/issues/83527
#[doc(hidden)]
#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

/// Macro to implement the [`crate::StaticFunction`] trait. Static functions can be attached to runtimes
/// to provide build-in functionality.
///
/// When given as the first argument, the function can also mutate the runtime state.
/// 
/// Function arguments need to implement the [`crate::Deserialize`] trait.
/// Return parameter need to implement the [`crate::Serialize`] trait.
///
/// # Example
/// 
/// ```rust
/// use kopi::static_function;
/// 
/// struct State;
///
/// static_function! {
///     fn static_function_0(state: &mut State, x: i32, y: i32) -> i32 { 1 }
/// }
///
/// static_function! {
///     fn static_function_1(state: &mut State, x: i32, y: i32) {}
/// }
///
/// static_function! {
///     fn static_function_2(state: &mut State) -> i32 { 1 }
/// }
///
/// static_function! {
///     fn static_function_3(state: &mut State) {}
/// }
/// 
/// static_function! {
///     fn static_function_4(x: i32, y: i32) -> i32 { 1 }
/// }
///
/// static_function! {
///     fn static_function_5(x: i32, y: i32) {}
/// }
///
/// static_function! {
///     fn static_function_6() -> i32 { 1 }
/// }
///
/// static_function! {
///     fn static_function_7() {}
/// }
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! static_function {
    (fn $function_name:ident() $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call() { $function_block }
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                _scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                _args: $crate::_macros::FunctionCallbackArguments<'scope>,
                _rv: $crate::_macros::ReturnValue,
            ) {
                Self::call();
            }
        }
    );
    (fn $function_name:ident() -> $return_type:ty $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call() -> $return_type $function_block
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                _args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                let result = Self::call();
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty) $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call($state_name : &mut $state_type) $function_block
            
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                _args: $crate::_macros::FunctionCallbackArguments<'scope>,
                _rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let $state_name = unsafe { &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>) };
                let mut borrow = $state_name.borrow_mut();
                
                Self::call(&mut borrow);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty) -> $return_type:ty $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call($state_name : &mut $state_type) -> $return_type $function_block
            
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let $state_name = unsafe { &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>) };
                let mut borrow = $state_name.borrow_mut();
                
                let result = Self::call(&mut borrow);
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty $(,$arg_name:ident : $arg_type:ty)*) $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call($state_name : &mut $state_type $(,$arg_name : $arg_type)*) $function_block
            
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let $state_name = unsafe { &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>) };
                let mut borrow = $state_name.borrow_mut();

                let counter_value = -1; 
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                Self::call(&mut borrow $(,$arg_name)*);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty $(,$arg_name:ident : $arg_type:ty)*) -> $return_type:ty $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call($state_name : &mut $state_type $(,$arg_name : $arg_type)*) -> $return_type $function_block
            
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let $state_name = unsafe { &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>) };
                let mut borrow = $state_name.borrow_mut();

                let counter_value = -1; 
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                let result = Self::call(&mut borrow $(,$arg_name)*);
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
    (fn $function_name:ident($first_arg_name:ident : $first_arg_type:ty $(,$arg_name:ident : $arg_type:ty)*) $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call($first_arg_name : $first_arg_type $(,$arg_name : $arg_type)*) $function_block
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                let counter_value = 0; 
                let Some($first_arg_name) = $crate::_macros::get_argument::<$first_arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                Self::call($first_arg_name $(,$arg_name)*);
            }
        }
    );
    (fn $function_name:ident($first_arg_name:ident : $first_arg_type:ty $(,$arg_name:ident : $arg_type:ty)*) -> $return_type:ty $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;

        impl $crate::StaticFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $function_name {
            #[inline(always)]
            fn call($first_arg_name : $first_arg_type $(,$arg_name : $arg_type)*) -> $return_type $function_block
            
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                let counter_value = 0; 
                let Some($first_arg_name) = $crate::_macros::get_argument::<$first_arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                let result = Self::call($first_arg_name $(,$arg_name)*);
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
}

/// Macro to implement the [`crate::FastcallFunction`] trait. Fastcall functions can be
/// attached to runtimes to provide build-in functionality and can be called very efficiently
/// by V8.
/// 
/// They can't directly mutate the runtime context and can't throw exceptions.
///
/// When given as the first argument, the function can also mutate the runtime state.
///
/// Function arguments need to implement the [`crate::FastcallArgument`] trait.
/// Currently supported are: bool, i32, u32, f32, f64. 
/// 
/// Return parameter need to implement the [`crate::FastcallReturnValue`] trait.
/// Currently supported are: bool, i32, u32, f32, f64.
///
/// Those traits can't be implemented by the user, since V8 only supports a very
/// limited set of primitives for fast calls.
/// 
/// u64 and i64 are supported by V8, but their values get truncated and they are not
/// converted to bigints. They will get supported, once V8 implements the bigint
/// conversion in the `fastapi` API.
/// 
/// # Example
/// 
/// ```rust
/// use kopi::fastcall_function;
///
/// struct State;
///
/// fastcall_function! {
///     fn static_function_0(state: &mut State, x: i32, y: i32) -> i32 { 1 }
/// }
///
/// fastcall_function! {
///     fn static_function_1(state: &mut State, x: i32, y: i32) {}
/// }
///
/// fastcall_function! {
///     fn static_function_2(state: &mut State) -> i32 { 1 }
/// }
///
/// fastcall_function! {
///     fn static_function_3(state: &mut State) {}
/// }
///
/// fastcall_function! {
///     fn static_function_4(x: i32, y: i32) -> i32 { 1 }
/// }
///
/// fastcall_function! {
///     fn static_function_5(x: i32, y: i32) {}
/// }
///
/// fastcall_function! {
///     fn static_function_6() -> i32 { 1 }
/// }
///
/// fastcall_function! {
///     fn static_function_7() {}
/// }
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! fastcall_function {
    (fn $function_name:ident() $function_block:block) => (
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                &[$crate::_macros::Type::V8Value]
            }
            
            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(_recv: $crate::_macros::Local<$crate::_macros::Object>) {
                Self::call()
            }

            #[inline(always)]
            fn call() $function_block
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                _scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                _args: $crate::_macros::FunctionCallbackArguments<'scope>,
                _rv: $crate::_macros::ReturnValue,
            ) {
                Self::call();
            }
        }
    );
    (fn $function_name:ident() -> $return_type:ty $function_block:block ) => (
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                &[$crate::_macros::Type::V8Value]
            }

            fn return_type(&self) -> $crate::_macros::CType {
                use $crate::FastcallReturnValue;
                <$return_type>::C_TYPE
            }

            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(_recv: $crate::_macros::Local<$crate::_macros::Object>) -> $return_type {
                Self::call()
            }

            #[inline(always)]
            fn call() -> $return_type $function_block
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                _args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                let result = Self::call();
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty) $function_block:block ) => (
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                &[
                    $crate::_macros::Type::V8Value,
                    $crate::_macros::Type::CallbackOptions,
                ]
            }
            
            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(
                _recv: $crate::_macros::Local<$crate::_macros::Object>,
                fast_api_callback_options: *mut $crate::_macros::FastApiCallbackOptions,
            ) {
                // SAFETY: We know that the pointer point to these structs as defined by rusty_v8.
                let opts: &mut $crate::_macros::FastApiCallbackOptions =
                    unsafe { &mut *fast_api_callback_options };
        
                // SAFETY: When registering the function, we made sure that the data contains the
                //         external reference to the state data.
                let $state_name = unsafe {
                    &*($crate::_macros::Local::<$crate::_macros::External>::cast(opts.data.data)
                        .value() as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = $state_name.borrow_mut();
                
                Self::call(&mut borrow)
            }
        
            #[inline(always)]
            fn call($state_name : &mut $state_type) $function_block

            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                _args: $crate::_macros::FunctionCallbackArguments<'scope>,
                _rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let state = unsafe {
                    &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = state.borrow_mut();
                
                Self::call(&mut borrow);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty) -> $return_type:ty $function_block:block ) => (
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                &[
                    $crate::_macros::Type::V8Value,
                    $crate::_macros::Type::CallbackOptions,
                ]
            }
        
            fn return_type(&self) -> $crate::_macros::CType {
                use $crate::FastcallReturnValue;
                <$return_type>::C_TYPE
            }
        
            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(
                _recv: $crate::_macros::Local<$crate::_macros::Object>,
                fast_api_callback_options: *mut $crate::_macros::FastApiCallbackOptions,
            ) -> $return_type {
                // SAFETY: We know that the pointer point to these structs as defined by rusty_v8.
                let opts: &mut $crate::_macros::FastApiCallbackOptions =
                    unsafe { &mut *fast_api_callback_options };
        
                // SAFETY: When registering the function, we made sure that the data contains the
                //         external reference to the state data.
                let $state_name = unsafe {
                    &*($crate::_macros::Local::<$crate::_macros::External>::cast(opts.data.data)
                        .value() as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = $state_name.borrow_mut();
                
                Self::call(&mut borrow)
            }
        
            #[inline(always)]
            fn call($state_name : &mut $state_type) -> $return_type $function_block

            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let state = unsafe {
                    &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = state.borrow_mut();
                
                let result = Self::call(&mut borrow);
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty $(,$arg_name:ident : $arg_type:ty)*) $function_block:block ) => (
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                use $crate::{count, FastcallArgument};
                
                static ARGS : [$crate::_macros::Type; 2 + $crate::count!($($arg_type)*)] = [
                    $crate::_macros::Type::V8Value,
                    $(<$arg_type>::V8_TYPE,)*
                    $crate::_macros::Type::CallbackOptions,
                ];
                
                &ARGS
            }

            fn return_type(&self) -> $crate::_macros::CType {
                $crate::_macros::CType::Void
            }
            
            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(
                _recv: $crate::_macros::Local<$crate::_macros::Object>,
                $($arg_name: $arg_type,)*
                fast_api_callback_options: *mut $crate::_macros::FastApiCallbackOptions,
            ) {
                // SAFETY: We know that the pointer point to these structs as defined by rusty_v8.
                let opts: &mut $crate::_macros::FastApiCallbackOptions =
                    unsafe { &mut *fast_api_callback_options };
        
                // SAFETY: When registering the function, we made sure that the data contains the
                //         external reference to the state data.
                let $state_name = unsafe {
                    &*($crate::_macros::Local::<$crate::_macros::External>::cast(opts.data.data)
                        .value() as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = $state_name.borrow_mut();
                
                Self::call(&mut borrow $(,$arg_name)*);
            }
        
            #[inline(always)]
            fn call($state_name : &mut $state_type $(,$arg_name : $arg_type)*) $function_block
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let state = unsafe {
                    &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = state.borrow_mut();
        
                let counter_value = -1; 
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                Self::call(&mut borrow $(,$arg_name)*);
            }
        }
    );
    (fn $function_name:ident($state_name:ident : &mut $state_type:ty $(,$arg_name:ident : $arg_type:ty)*) -> $return_type:ty $function_block:block ) => (         
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                use $crate::{count, FastcallArgument};
            
                static ARGS : [$crate::_macros::Type; 2 + $crate::count!($($arg_type)*)] = [
                    $crate::_macros::Type::V8Value,
                    $(<$arg_type>::V8_TYPE,)*
                    $crate::_macros::Type::CallbackOptions,
                ];
                
                &ARGS
            }
        
            fn return_type(&self) -> $crate::_macros::CType {
                use $crate::FastcallReturnValue;
                <$return_type>::C_TYPE
            }
        
            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(
                _recv: $crate::_macros::Local<$crate::_macros::Object>,
                $($arg_name: $arg_type,)*
                fast_api_callback_options: *mut $crate::_macros::FastApiCallbackOptions,
            ) -> $return_type {
                // SAFETY: We know that the pointer point to these structs as defined by rusty_v8.
                let opts: &mut $crate::_macros::FastApiCallbackOptions =
                    unsafe { &mut *fast_api_callback_options };
        
                // SAFETY: When registering the function, we made sure that the data contains the
                //         external reference to the state data.
                let $state_name = unsafe {
                    &*($crate::_macros::Local::<$crate::_macros::External>::cast(opts.data.data)
                        .value() as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = $state_name.borrow_mut();
                
                Self::call(&mut borrow $(,$arg_name)*)
            }
        
            #[inline(always)]
            fn call($state_name : &mut $state_type $(,$arg_name : $arg_type)*) -> $return_type $function_block

            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                // SAFETY: This is safe since we know that the state is stored in that slot
                //         and the data is bound to the lifetime of this runtime.
                let state = unsafe {
                    &*(scope.get_data($crate::_macros::STATE_DATA_SLOT) as *const std::cell::RefCell<$state_type>)
                };
                let mut borrow = state.borrow_mut();
        
                let counter_value = -1; 
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                let result = Self::call(&mut borrow $(,$arg_name)*);
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
    (fn $function_name:ident($first_arg_name:ident : $first_arg_type:ty $(,$arg_name:ident : $arg_type:ty)*) $function_block:block ) => (
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                use $crate::{count, FastcallArgument};
                
                static ARGS : [$crate::_macros::Type; 2 + $crate::count!($($arg_type)*)] = [
                    $crate::_macros::Type::V8Value,
                    <$first_arg_type>::V8_TYPE,
                    $(<$arg_type>::V8_TYPE,)*
                ];
                
                &ARGS
            }
            
            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(
                _recv: $crate::_macros::Local<$crate::_macros::Object>,
                $first_arg_name: $first_arg_type,
                $($arg_name: $arg_type,)*
            ) {
                Self::call($first_arg_name $(,$arg_name)*)
            }

            #[inline(always)]
            fn call($first_arg_name : $first_arg_type $(,$arg_name : $arg_type)*) $function_block
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                let counter_value = 0; 
                let Some($first_arg_name) = $crate::_macros::get_argument::<$first_arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                Self::call($first_arg_name $(,$arg_name)*);
            }
        }
    );
    (fn $function_name:ident($first_arg_name:ident : $first_arg_type:ty $(,$arg_name:ident : $arg_type:ty)*) -> $return_type:ty $function_block:block ) => (
        #[allow(non_camel_case_types)]
        struct $function_name;
        
        unsafe impl $crate::FastcallFunction for $function_name {
            fn callback() -> $crate::_macros::FunctionCallback {
                use $crate::_macros::MapFnTo;
                Self::v8_func.map_fn_to()
            }
        }
        
        impl $crate::_macros::FastFunction for $function_name {
            fn args(&self) -> &'static [$crate::_macros::Type] {
                use $crate::{count, FastcallArgument};
                
                static ARGS : [$crate::_macros::Type; 2 + $crate::count!($($arg_type)*)] = [
                    $crate::_macros::Type::V8Value,
                    <$first_arg_type>::V8_TYPE,
                    $(<$arg_type>::V8_TYPE,)*
                ];
                
                &ARGS
            }
        
            fn return_type(&self) -> $crate::_macros::CType {
                use $crate::FastcallReturnValue;
                <$return_type>::C_TYPE
            }
        
            fn function(&self) -> *const std::ffi::c_void {
                Self::fast_call as *const std::ffi::c_void
            }
        }
        
        impl $function_name {
            fn fast_call(
                _recv: $crate::_macros::Local<$crate::_macros::Object>,
                $first_arg_name: $first_arg_type,
                $($arg_name: $arg_type,)*
            ) -> $return_type {
                Self::call($first_arg_name $(,$arg_name)*)
            }

            #[inline(always)]
            fn call($first_arg_name : $first_arg_type $(,$arg_name : $arg_type)*) -> $return_type $function_block
        
            #[inline(always)]
            fn v8_func<'borrow, 'scope>(
                scope: &'borrow mut $crate::_macros::HandleScope<'scope>,
                args: $crate::_macros::FunctionCallbackArguments<'scope>,
                mut rv: $crate::_macros::ReturnValue,
            ) {
                let counter_value = 0; 
                let Some($first_arg_name) = $crate::_macros::get_argument::<$first_arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                $(
                let counter_value = counter_value + 1;
                let Some($arg_name) = $crate::_macros::get_argument::<$arg_type>(scope, &args, &mut rv, counter_value) else {
                    return;
                };
                )*
                
                let result = Self::call($first_arg_name $(,$arg_name)*);
                $crate::_macros::set_result::<$return_type>(scope, rv, result);
            }
        }
    );
}
