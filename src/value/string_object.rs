use super::{Object, Seal, Unseal, Value};

/// A string object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct StringObject<'scope>(pub(crate) v8::Local<'scope, v8::StringObject>);

impl<'scope> Seal<StringObject<'scope>> for v8::Local<'scope, v8::StringObject> {
    #[inline(always)]
    fn seal(self) -> StringObject<'scope> {
        StringObject(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::StringObject>> for StringObject<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::StringObject> {
        self.0
    }
}

impl<'scope> From<StringObject<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: StringObject<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for StringObject<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::StringObject>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<StringObject<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: StringObject<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> StringObject<'scope> {
    // TODO rusty_v8 doesn't export the StringObject operations of V8.
}
