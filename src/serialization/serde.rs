mod deserializer;
mod serializer;

use deserializer::ValueDeserializer;
use serde::{Deserialize, Serialize};

use crate::{
    error::TypeError,
    serialization::serde::serializer::ValueSerializer,
    value::{Value, ValueScope},
};

/// Converts a engine value to a deserializable type.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub fn from_value<'scope, T>(
    scope: &mut ValueScope<'scope>,
    value: Value<'scope>,
) -> Result<T, TypeError>
where
    T: Deserialize<'scope>,
{
    let deserializer = &mut ValueDeserializer::from_value(scope, value);
    let t = T::deserialize(deserializer)?;
    Ok(t)
}

/// Converts a serializable type to a engine value.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub fn to_value<'scope, T>(
    scope: &mut ValueScope<'scope>,
    value: T,
) -> Result<Value<'scope>, TypeError>
where
    T: Serialize,
{
    let mut serializer = ValueSerializer { scope };
    let value = value.serialize(&mut serializer)?;
    Ok(value)
}
