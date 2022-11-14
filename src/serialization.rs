//! Implements the serialization / deserialization of ECMAScript values.

mod deserialize_impl;
mod serialize_impl;
pub use deserialize_impl::*;
pub use serialize_impl::*;

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use self::serde::*;
