use serde::{Deserialize, Serialize};

use crate::{
    error::TypeError,
    value::{Local, Value, ValueScope},
    FromValue, IntoValue,
};

impl<T> FromValue for T
where
    T: Serialize,
{
    type Value = T;

    // TODO Implement serde deserialization, since serde_v8 does not cut it.
    fn from_v8(_scope: &mut ValueScope, _value: Local<Value>) -> Result<Self::Value, TypeError> {
        todo!()
    }
}

impl<'de, T> IntoValue for T
where
    T: Deserialize<'de>,
{
    // TODO Implement serde serialization, since serde_v8 does not cut it.
    fn into_v8<'borrow, 'scope>(
        self,
        _scope: &mut ValueScope<'borrow, 'scope>,
    ) -> Result<Local<'scope, Value>, TypeError> {
        todo!()
    }
}
