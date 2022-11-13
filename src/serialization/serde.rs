use serde::{Deserialize, Serialize};

use crate::{
    error::TypeError,
    value::{Value, ValueScope},
};

/// Converts a ECMAScript value to a deserializable type.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub fn deserialize_value<'scope, T>(
    _scope: &mut ValueScope<'scope>,
    _value: Value<'scope>,
) -> Result<T, TypeError>
where
    T: Deserialize<'scope>,
{
    todo!()
}

/// Converts a serializable type to a ECMAScript value.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub fn serialize_value<'scope, T>(
    _scope: &mut ValueScope<'scope>,
    _value: T,
) -> Result<Value<'scope>, TypeError>
where
    T: Serialize,
{
    todo!()
}
