//! Values used inside the ECMAScript engine.

mod bigint;
mod boolean;
mod error;
mod int32;
mod integer;
mod number;
mod primitive;
mod string;
mod uint32;

pub use bigint::BigInt;
pub use boolean::Boolean;
pub use error::Error;
pub use int32::Int32;
pub use integer::Integer;
pub use number::Number;
pub use primitive::Primitive;
pub(crate) use string::new_string;
pub use string::{NewStringType, String};
pub use uint32::Uint32;
// TODO wrap all exports.
pub use v8::{
    Array, ArrayBuffer, ArrayBufferView, BigInt64Array, BigIntObject, BigUint64Array,
    BooleanObject, Data, DataView, Date, FixedArray, Float32Array, Float64Array, Function,
    Int16Array, Int32Array, Int8Array, Local, Map, Message, Name, Object, PrimitiveArray, Promise,
    Proxy, RegExp, Set, SharedArrayBuffer, StackFrame, StackTrace, StringObject, Symbol,
    SymbolObject, TypedArray, Uint16Array, Uint32Array, Uint8Array, Uint8ClampedArray,
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

// TODO remove all constructors from this struct. The `ValueScope` should only act as a lifetime marker.
impl<'scope> ValueScope<'scope> {
    /// Returns the original stack trace that was captured at the creation time of
    /// a given exception if available.
    #[inline(always)]
    pub fn exception_stack_trace(&mut self, exception: Value) -> Option<Local<'scope, StackTrace>> {
        v8::Exception::get_stack_trace(&mut self.0, exception.unseal())
    }

    /// Returns the current execution stack trace.
    #[inline(always)]
    pub fn current_stack_trace(&mut self, frame_limit: usize) -> Option<Local<'scope, StackTrace>> {
        StackTrace::current_stack_trace(&mut self.0, frame_limit)
    }

    /// Returns a particular stack frame of a stack trace at the particular index.
    #[inline(always)]
    pub fn get_stack_frame(
        &mut self,
        stack_trace: &mut Local<StackTrace>,
        index: usize,
    ) -> Option<Local<'scope, StackFrame>> {
        StackTrace::get_frame(stack_trace, &mut self.0, index)
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
    fn unseal(self) -> Local<'scope, v8::Value> {
        self.0
    }
}

impl<'scope> Value<'scope> {
    /// Creates a new value.
    #[inline(always)]
    pub fn new(value: v8::Local<'scope, v8::Value>) -> Value<'scope> {
        Value(value)
    }

    /// Returns true if the value is null.
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Returns true if the value is undefined.
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        self.0.is_undefined()
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

    #[test]
    fn scope_transmute_safety() {
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
}
