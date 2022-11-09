///! Traits to abstract common operations on ECMAScript values.
use crate::{
    error::TypeError,
    value::{Local, Value, ValueScope},
};

pub(crate) mod fastcall_impl;
pub(crate) mod from_value_impl;
pub(crate) mod into_value_impl;

/// Trait to convert a Rust value into a [`Value`] using a [`ValueBuilder`].
pub trait IntoValue {
    /// Needs to convert the given type to a [`Value`].
    fn into_v8<'borrow, 'scope>(
        self,
        scope: &mut ValueScope<'borrow, 'scope>,
    ) -> Result<Local<'scope, Value>, TypeError>;

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

    /// Needs to convert the given [`Local<Value>`] into the expected type.
    fn from_v8(scope: &mut ValueScope, value: Local<Value>) -> Result<Self::Value, TypeError>;
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
