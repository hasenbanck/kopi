use super::{Primitive, Seal, Unseal, Value};

/// The name superclass. Can either be a string or symbol.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Name<'scope>(pub(crate) v8::Local<'scope, v8::Name>);

impl<'scope> Seal<Name<'scope>> for v8::Local<'scope, v8::Name> {
    #[inline(always)]
    fn seal(self) -> Name<'scope> {
        Name(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Name>> for Name<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Name> {
        self.0
    }
}

impl<'scope> From<Name<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Name<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Name<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Name>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Name<'scope>> for Primitive<'scope> {
    #[inline(always)]
    fn from(value: Name<'scope>) -> Self {
        Primitive(value.0.into())
    }
}
