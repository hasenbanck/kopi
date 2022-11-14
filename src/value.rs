//! Values that the engine uses.
//!
//! The API is optimized to be used in the [`Serialize`] and [`Deserialize`] traits.

mod array;
mod bigint;
mod boolean;
mod error;
mod int32;
mod integer;
mod map;
mod name;
mod number;
mod object;
mod primitive;
mod set;
mod stack_trace;
mod string;
mod uint32;

pub(crate) use string::new_string;
// TODO wrap all V8 exports.
pub use v8::{
    ArrayBuffer, ArrayBufferView, BigInt64Array, BigIntObject, BigUint64Array, BooleanObject, Data,
    DataView, Date, External, FixedArray, Float32Array, Float64Array, Function, Int16Array,
    Int32Array, Int8Array, Message, NumberObject, PrimitiveArray, Promise, PromiseResolver, Proxy,
    RegExp, SharedArrayBuffer, StringObject, Symbol, SymbolObject, TypedArray, Uint16Array,
    Uint32Array, Uint8Array, Uint8ClampedArray, WasmMemoryObject, WasmModuleObject,
};

pub use self::{
    array::Array,
    bigint::BigInt,
    boolean::Boolean,
    error::Error,
    int32::Int32,
    integer::Integer,
    map::Map,
    name::Name,
    number::Number,
    object::Object,
    primitive::Primitive,
    set::Set,
    stack_trace::{StackFrame, StackTrace},
    string::{NewStringType, String},
    uint32::Uint32,
};

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
mod test {
    use super::ValueScope;
    use crate::value::Value;

    #[test]
    fn transparent_representation_value_scope_() {
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
