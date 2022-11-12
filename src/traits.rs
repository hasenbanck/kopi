///! Traits to abstract common operations on ECMAScript values.
use crate::{
    error::TypeError,
    value::{Value, ValueScope},
};

/// Trait to convert a Rust value into a [`Value`] using a [`ValueBuilder`].
pub trait IntoValue {
    /// Needs to convert the given type to a [`Value`].
    fn into_v8<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError>;

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
    fn from_v8<'scope>(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self::Value, TypeError>;
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

macro_rules! fastcall_argument {
    ($value_type:ty, $v8_type:ident) => {
        impl super::FastcallArgument for $value_type {
            type Value = $value_type;

            #[inline(always)]
            fn v8_type() -> v8::fast_api::Type {
                v8::fast_api::Type::$v8_type
            }
        }
    };
}

// TODO V8 also supports:
//      * pointer to an embedder type
//      * JavaScript array of primitive types

fastcall_argument!(bool, Bool);
fastcall_argument!(i32, Int32);
fastcall_argument!(u32, Uint32);
fastcall_argument!(f32, Float32);
fastcall_argument!(f64, Float64);

macro_rules! fastcall_return_value {
    ($value_type:ty, $c_type:ident) => {
        impl super::FastcallReturnValue for $value_type {
            type Value = $value_type;

            #[inline(always)]
            fn c_type() -> v8::fast_api::CType {
                v8::fast_api::CType::$c_type
            }
        }
    };
}

fastcall_return_value!(bool, Bool);
fastcall_return_value!(i32, Int32);
fastcall_return_value!(u32, Uint32);
fastcall_return_value!(f32, Float32);
fastcall_return_value!(f64, Float64);

macro_rules! fastcall_sealed {
    ($value_type:ty) => {
        impl private::Sealed for $value_type {}
    };
}

fastcall_sealed!(());
fastcall_sealed!(bool);
fastcall_sealed!(i32);
fastcall_sealed!(u32);
fastcall_sealed!(f32);
fastcall_sealed!(f64);

pub(crate) mod private {
    pub trait Sealed {}
}
