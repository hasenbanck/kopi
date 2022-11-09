//! Implements the serialization / deserialization of ECMAScript values.

#[cfg(not(feature = "serde"))]
mod from_value_impl;
#[cfg(not(feature = "serde"))]
mod into_value_impl;
#[cfg(not(feature = "serde"))]
pub use from_value_impl::*;
#[cfg(not(feature = "serde"))]
pub use into_value_impl::*;

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use self::serde::*;
