use super::{Object, Seal, Unseal, Value};

/// A hash set.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Set<'scope>(pub(crate) v8::Local<'scope, v8::Set>);

impl<'scope> Seal<Set<'scope>> for v8::Local<'scope, v8::Set> {
    #[inline(always)]
    fn seal(self) -> Set<'scope> {
        Set(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Set>> for Set<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Set> {
        self.0
    }
}

impl<'scope> From<Set<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Set<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Set<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Set>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Set<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: Set<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> Set<'scope> {
    // TODO rusty_v8 doesn't export the set operations of V8.
}
