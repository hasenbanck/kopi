//! Values that the engine uses.
//!
//! The API is optimized to be used in the [`crate::Serialize`] and [`crate::Deserialize`] traits.
//!
//! Extension functions are not supposed to work on the values that the engine uses.
//!
//! If functionality should be provided on engine types, these should be implemented using
//! ECMAScript modules.

mod array;
mod array_buffer;
mod array_buffer_view;
mod bigint;
mod bigint64_array;
mod bigint_object;
mod biguint64_array;
mod boolean;
mod boolean_object;
mod data_view;
mod date;
mod error;
mod external;
mod float32_array;
mod float64_array;
mod function;
mod int16_array;
mod int32;
mod int32_array;
mod int8_array;
mod integer;
mod map;
mod message;
mod name;
mod number;
mod number_object;
mod object;
mod primitive;
mod promise;
mod promise_resolver;
mod proxy;
mod regexp;
mod set;
mod stack_trace;
mod string;
mod string_object;
mod symbol;
mod symbol_object;
mod typed_array;
mod uint16_array;
mod uint32;
mod uint32_array;
mod uint8_array;
mod uint8_clamped_array;
mod wasm_memory_object;
mod wasm_module_object;

pub(crate) use string::new_string;

pub use self::{
    array::Array,
    array_buffer::ArrayBuffer,
    array_buffer_view::ArrayBufferView,
    bigint::BigInt,
    bigint64_array::BigInt64Array,
    bigint_object::BigIntObject,
    biguint64_array::BigUint64Array,
    boolean::Boolean,
    boolean_object::BooleanObject,
    data_view::DataView,
    date::Date,
    error::Error,
    external::External,
    float32_array::Float32Array,
    float64_array::Float64Array,
    function::Function,
    int16_array::Int16Array,
    int32::Int32,
    int32_array::Int32Array,
    int8_array::Int8Array,
    integer::Integer,
    map::Map,
    message::Message,
    name::Name,
    number::Number,
    number_object::NumberObject,
    object::Object,
    primitive::Primitive,
    promise::{Promise, PromiseState},
    promise_resolver::PromiseResolver,
    proxy::Proxy,
    regexp::RegExp,
    set::Set,
    stack_trace::{StackFrame, StackTrace},
    string::{NewStringType, String},
    string_object::StringObject,
    symbol::Symbol,
    symbol_object::SymbolObject,
    typed_array::TypedArray,
    uint16_array::Uint16Array,
    uint32::Uint32,
    uint32_array::Uint32Array,
    uint8_array::Uint8Array,
    uint8_clamped_array::Uint8ClampedArray,
    wasm_memory_object::WasmMemoryObject,
    wasm_module_object::WasmModuleObject,
};

// TODO test the methods if they function as expected.

/// Trait for sealing private types. `T` is the public type into which the private type is sealed.
pub(crate) trait Seal<T> {
    fn seal(self) -> T;
}

/// Trait for unsealing public types. `T` is the private type into which the public type is unsealed.
pub(crate) trait Unseal<T> {
    fn unseal(self) -> T;
}

/// Scopes the lifetime of values.
#[repr(transparent)]
pub struct ValueScope<'scope>(
    // We created the ValueScope to shield the public API from the functionality that
    // v8::HandleScope exposes.
    v8::HandleScope<'scope>,
);

impl<'borrow, 'scope> Seal<&'borrow mut ValueScope<'scope>>
    for &'borrow mut v8::HandleScope<'scope>
{
    fn seal(self) -> &'borrow mut ValueScope<'scope> {
        // SAFETY: Safe because ValueScope is a transparent representation of v8::HandleScope.
        //         https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
        unsafe { &mut *(self as *mut v8::HandleScope<'scope> as *mut ValueScope<'scope>) }
    }
}

impl<'borrow, 'scope> Unseal<&'borrow mut v8::HandleScope<'scope>>
    for &'borrow mut ValueScope<'scope>
{
    fn unseal(self) -> &'borrow mut v8::HandleScope<'scope> {
        // SAFETY: Safe because ValueScope is a transparent representation of v8::HandleScope.
        //         https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
        unsafe { &mut *(self as *mut ValueScope<'scope> as *mut v8::HandleScope<'scope>) }
    }
}

/// The superclass of all types.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Value<'scope>(v8::Local<'scope, v8::Value>);

impl<'scope> Seal<Value<'scope>> for v8::Local<'scope, v8::Value> {
    #[inline(always)]
    fn seal(self) -> Value<'scope> {
        Value(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Value>> for Value<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Value> {
        self.0
    }
}

impl<'scope> Value<'scope> {
    /// Returns `true` if the value is null.
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Returns `true` if the value is undefined.
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        self.0.is_undefined()
    }

    /// Returns `true` if the value is null or undefined.
    #[inline(always)]
    pub fn is_null_or_undefined(&self) -> bool {
        self.0.is_null_or_undefined()
    }

    /// Returns the string representation of the value.
    #[inline(always)]
    pub fn to_string_representation(&self, scope: &mut ValueScope<'scope>) -> std::string::String {
        self.0.to_rust_string_lossy(scope.unseal())
    }

    /// Returns the bool representation of the value.
    #[inline(always)]
    pub fn to_boolean_representation(&self, scope: &mut ValueScope<'scope>) -> bool {
        self.0.boolean_value(scope.unseal())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{new_string, NewStringType, Seal, Value, ValueScope};
    use crate::{error::create_error_from_exception, initialize_with_defaults};

    pub(crate) fn test_value<F>(source: &str, test: F)
    where
        F: for<'scope> FnOnce(Value<'scope>),
    {
        initialize_with_defaults();

        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        let isolate_scope = &mut v8::HandleScope::new(isolate);
        let global_template = v8::ObjectTemplate::new(isolate_scope);
        let global_context = v8::Context::new_from_template(isolate_scope, global_template);
        let global_context_scope = &mut v8::ContextScope::new(isolate_scope, global_context);

        let source = new_string(global_context_scope, source, NewStringType::Normal);

        let try_catch_scope = &mut v8::TryCatch::new(global_context_scope);

        let Some(script) = v8::Script::compile(try_catch_scope, source, None) else {
            let exception = try_catch_scope.exception();
            let err = create_error_from_exception(try_catch_scope, exception);
            panic!("Can't compile script: {}", err);
        };

        let Some(v8_value) = script.run(try_catch_scope) else {
            let exception = try_catch_scope.exception();
            let err = create_error_from_exception(try_catch_scope, exception);
            panic!("Can't run script: {}", err);
        };

        test(v8_value.seal())
    }

    #[macro_export]
    macro_rules! test_value {
        ($source:literal, | $ident:ident : $value_type:ty | $block:block) => {
            $crate::value::test::test_value($source, |v| {
                let $ident = <$value_type>::try_from(v).expect("Not the expected type");
                $block
            })
        };
    }

    #[test]
    fn transparent_representation_value_scope() {
        // Make sure that both types are of the same size and alignment.
        assert_eq!(
            std::mem::size_of::<ValueScope>(),
            std::mem::size_of::<v8::HandleScope>()
        );
        assert_eq!(
            std::mem::align_of::<ValueScope>(),
            std::mem::align_of::<v8::HandleScope>()
        );
        // Make sure that we don't have a ZST.
        assert_ne!(std::mem::size_of::<ValueScope>(), 0);
    }

    #[test]
    fn transparent_representation_value() {
        // Make sure that both types are of the same size and alignment.
        assert_eq!(
            std::mem::size_of::<Value>(),
            std::mem::size_of::<v8::Local<'_, v8::Value>>()
        );
        assert_eq!(
            std::mem::align_of::<Value>(),
            std::mem::align_of::<v8::Local<'_, v8::Value>>()
        );
        // Make sure that we don't have a ZST.
        assert_ne!(std::mem::size_of::<Value>(), 0);
    }
}
