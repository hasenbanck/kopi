use super::{Seal, Unseal, Value};

/// A value that wraps an external data pointer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct External<'scope>(pub(crate) v8::Local<'scope, v8::External>);

impl<'scope> Seal<External<'scope>> for v8::Local<'scope, v8::External> {
    #[inline(always)]
    fn seal(self) -> External<'scope> {
        External(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::External>> for External<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::External> {
        self.0
    }
}

impl<'scope> From<External<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: External<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for External<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::External>::try_from(value.0)?;
        Ok(Self(inner))
    }
}
