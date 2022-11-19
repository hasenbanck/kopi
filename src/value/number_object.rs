use super::{Seal, Unseal, Value};

/// A number object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct NumberObject<'scope>(pub(crate) v8::Local<'scope, v8::NumberObject>);

impl<'scope> Seal<NumberObject<'scope>> for v8::Local<'scope, v8::NumberObject> {
    #[inline(always)]
    fn seal(self) -> NumberObject<'scope> {
        NumberObject(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::NumberObject>> for NumberObject<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::NumberObject> {
        self.0
    }
}

impl<'scope> From<NumberObject<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: NumberObject<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for NumberObject<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::NumberObject>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> NumberObject<'scope> {
    // TODO rusty_v8 doesn't export the NumberObject operations of V8.
}
