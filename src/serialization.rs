//! Implements the serialization / deserialization of ECMAScript values.

mod from_value_impl;
mod into_value_impl;
pub use from_value_impl::*;
pub use into_value_impl::*;

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use self::serde::*;
