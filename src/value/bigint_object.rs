use super::{Object, Seal, Unseal, Value};

/// A BigInt object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BigIntObject<'scope>(pub(crate) v8::Local<'scope, v8::BigIntObject>);

impl<'scope> Seal<BigIntObject<'scope>> for v8::Local<'scope, v8::BigIntObject> {
    #[inline(always)]
    fn seal(self) -> BigIntObject<'scope> {
        BigIntObject(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::BigIntObject>> for BigIntObject<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::BigIntObject> {
        self.0
    }
}

impl<'scope> From<BigIntObject<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: BigIntObject<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for BigIntObject<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::BigIntObject>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<BigIntObject<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: BigIntObject<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> BigIntObject<'scope> {
    // TODO rusty_v8 doesn't export the BigIntObject operations of V8.
}
