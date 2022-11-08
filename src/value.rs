///! Implements the utility functionality for handling V8 values.
use crate::{create_string, create_string_from_static, error::TypeError};

pub(crate) mod fastcall_impl;
pub(crate) mod from_value_impl;
pub(crate) mod into_value_impl;

/// A V8 value.
pub struct Value<'value, 'scope> {
    pub(crate) scope: &'value mut v8::HandleScope<'scope>,
    pub(crate) value: v8::Local<'scope, v8::Value>,
}

// TODO port the missing functionality for primitives, arrays, byte data and structs.
impl<'borrow, 'scope> Value<'borrow, 'scope> {
    /// Creates a new [`Value`] from the given local that is valid inside the given scope.
    #[inline(always)]
    pub(crate) fn new(
        scope: &'borrow mut v8::HandleScope<'scope>,
        value: v8::Local<'scope, v8::Value>,
    ) -> Value<'borrow, 'scope> {
        Self { scope, value }
    }

    /// Returns the reference to the [`v8::HandleScope<'scope>`] and [`v8::Local`] of the value.
    pub fn take(
        self,
    ) -> (
        &'borrow mut v8::HandleScope<'scope>,
        v8::Local<'scope, v8::Value>,
    ) {
        let Self { scope, value } = self;
        (scope, value)
    }

    /// Returns `true` if this value is a boolean value.
    #[inline(always)]
    pub fn is_boolean(&self) -> bool {
        self.value.is_boolean()
    }

    /// Returns `true` if this value is a string value.
    #[inline(always)]
    pub fn is_string(&self) -> bool {
        self.value.is_string()
    }

    /// Returns `true` if this value is a 32-bit unsigned integer.
    #[inline(always)]
    pub fn is_uint32(&self) -> bool {
        self.value.is_uint32()
    }

    /// Returns `true` if this value is a 32-bit signed integer.
    #[inline(always)]
    pub fn is_int32(&self) -> bool {
        self.value.is_int32()
    }

    /// Returns `true` if this value is a big integer.
    #[inline(always)]
    pub fn is_bigint(&self) -> bool {
        self.value.is_big_int()
    }

    /// Returns `true` if this value is a number.
    #[inline(always)]
    pub fn is_number(&self) -> bool {
        self.value.is_number()
    }

    /// Returns thr string of the value.
    #[allow(clippy::inherent_to_string)]
    #[inline(always)]
    pub fn to_string(&mut self) -> String {
        self.value.to_rust_string_lossy(self.scope)
    }

    /// Returns the boolean value.
    #[inline(always)]
    pub fn to_boolean(&mut self) -> bool {
        self.value.to_boolean(self.scope).boolean_value(self.scope)
    }

    /// Returns the boolean value if the value is a boolean.
    #[inline(always)]
    pub fn try_boolean(&self) -> Option<bool> {
        v8::Local::<v8::Boolean>::try_from(self.value)
            .ok()
            .map(|v| v.is_true())
    }

    /// Returns the string value if the value is a string.
    #[inline(always)]
    pub fn try_string(&mut self) -> Option<String> {
        v8::Local::<v8::String>::try_from(self.value)
            .ok()
            .map(|v| v.to_rust_string_lossy(self.scope))
    }

    /// Returns the 32-bit unsigned integer.
    #[inline(always)]
    pub fn try_uint32(&self) -> Option<u32> {
        v8::Local::<v8::Uint32>::try_from(self.value)
            .ok()
            .map(|v| v.value())
    }

    /// Returns the 32-bit signed integer.
    #[inline(always)]
    pub fn try_int32(&self) -> Option<i32> {
        v8::Local::<v8::Int32>::try_from(self.value)
            .ok()
            .map(|v| v.value())
    }

    /// Returns the 64-bit signed integer.
    #[inline(always)]
    pub fn try_integer(&self) -> Option<i64> {
        v8::Local::<v8::Integer>::try_from(self.value)
            .ok()
            .map(|v| v.value())
    }

    /// Returns the big integer value as i64. Also indicates if the value was lossless.
    #[inline(always)]
    pub fn try_bigint_as_i64(&self) -> Option<(i64, bool)> {
        v8::Local::<v8::BigInt>::try_from(self.value)
            .ok()
            .map(|v| v.i64_value())
    }

    /// Returns the big integer value as u64. Also indicates if the value was lossless.
    #[inline(always)]
    pub fn try_bigint_as_u64(&self) -> Option<(u64, bool)> {
        v8::Local::<v8::BigInt>::try_from(self.value)
            .ok()
            .map(|v| v.u64_value())
    }

    /// Returns the 64-bit float.
    #[inline(always)]
    pub fn try_float(&self) -> Option<f64> {
        v8::Local::<v8::Number>::try_from(self.value)
            .ok()
            .map(|v| v.value())
    }
}

/// A builder to build a [`Value`].
pub struct ValueBuilder<'value, 'scope> {
    scope: &'value mut v8::HandleScope<'scope>,
}

impl<'borrow, 'scope> ValueBuilder<'borrow, 'scope> {
    /// Creates a new [`ValueBuilder`].
    #[inline(always)]
    pub(crate) fn new(
        scope: &'borrow mut v8::HandleScope<'scope>,
    ) -> ValueBuilder<'borrow, 'scope> {
        Self { scope }
    }

    /// The undefined value.
    #[inline(always)]
    pub fn undefined(&'borrow mut self) -> Value<'borrow, 'scope> {
        let value = v8::undefined(self.scope).into();
        Value::new(self.scope, value)
    }

    /// The boolean value.
    #[inline(always)]
    pub fn boolean(&'borrow mut self, boolean: bool) -> Value<'borrow, 'scope> {
        let value = v8::Boolean::new(self.scope, boolean).into();
        Value::new(self.scope, value)
    }

    /// Creates a float value.
    #[inline(always)]
    pub fn float(&'borrow mut self, number: f64) -> Value<'borrow, 'scope> {
        let value = v8::Number::new(self.scope, number).into();
        Value::new(self.scope, value)
    }

    /// Creates an integer value.
    #[inline(always)]
    pub fn integer(&'borrow mut self, number: i32) -> Value<'borrow, 'scope> {
        let value = v8::Integer::new(self.scope, number).into();
        Value::new(self.scope, value)
    }

    /// Creates a bigint value from a given i64.
    #[inline(always)]
    pub fn bigint_from_i64(&'borrow mut self, number: i64) -> Value<'borrow, 'scope> {
        let value = v8::BigInt::new_from_i64(self.scope, number).into();
        Value::new(self.scope, value)
    }

    /// Creates a bigint value from a given u64.
    #[inline(always)]
    pub fn bigint_from_u64(&'borrow mut self, number: u64) -> Value<'borrow, 'scope> {
        let value = v8::BigInt::new_from_u64(self.scope, number).into();
        Value::new(self.scope, value)
    }

    /// Creates a string value.
    #[inline(always)]
    pub fn string<S: AsRef<str>>(&'borrow mut self, string: S) -> Value<'borrow, 'scope> {
        let value = create_string(self.scope, string).into();
        Value::new(self.scope, value)
    }

    /// Creates a string value from a static string.
    #[inline(always)]
    pub fn string_from_static(&'borrow mut self, string: &'static str) -> Value<'borrow, 'scope> {
        let value = create_string_from_static(self.scope, string).into();
        Value::new(self.scope, value)
    }
}

/// Trait to convert a Rust value into a [`Value`] using a [`ValueBuilder`].
pub trait IntoValue {
    /// Needs to convert the given type to a [`Value`].
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError>;

    /// Defines if the type is generally `undefined`. Most useful for the `()` type, where we don't
    /// want to set any return value at all. Standard implementation should be fine for most
    /// custom implementation.
    #[inline(always)]
    fn is_undefined() -> bool {
        false
    }
}

/// Trait to convert a [`Value`] into a Rust value.
pub trait FromValue {
    /// The type of the target value.
    type Value;

    /// Needs to convert the given [`Value`] into the expected type.
    fn from_v8(value: Value) -> Result<Self::Value, TypeError>;
}

/// Trait for types that are supported to be used as arguments for fastcall functions.
/// Sealed trait, since there is only a limited amount of types supported by V8.
pub trait FastcallArgument: private::Sealed {
    #[doc(hidden)]
    type Value;

    /// The V8 Type that maps to the implementor.
    #[doc(hidden)]
    fn v8_type() -> v8::fast_api::Type;
}

/// Trait for types that are supported to be used as return value for fastcall functions.
/// Sealed trait, since there is only a limited amount of types supported by V8.
pub trait FastcallReturnValue: private::Sealed {
    #[doc(hidden)]
    type Value;

    /// The CType that maps to the implementor.
    #[doc(hidden)]
    fn c_type() -> v8::fast_api::CType;
}

pub(crate) mod private {
    pub trait Sealed {}
}
