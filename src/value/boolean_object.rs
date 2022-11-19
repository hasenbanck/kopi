use super::{Seal, Unseal, Value};

/// A boolean object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BooleanObject<'scope>(pub(crate) v8::Local<'scope, v8::BooleanObject>);

impl<'scope> Seal<BooleanObject<'scope>> for v8::Local<'scope, v8::BooleanObject> {
    #[inline(always)]
    fn seal(self) -> BooleanObject<'scope> {
        BooleanObject(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::BooleanObject>> for BooleanObject<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::BooleanObject> {
        self.0
    }
}

impl<'scope> From<BooleanObject<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: BooleanObject<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for BooleanObject<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::BooleanObject>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> BooleanObject<'scope> {
    // TODO rusty_v8 doesn't export the BooleanObject operations of V8.
}
