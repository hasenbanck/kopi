use super::{Seal, Unseal, Value};

/// A regular expression.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RegExp<'scope>(pub(crate) v8::Local<'scope, v8::RegExp>);

impl<'scope> Seal<RegExp<'scope>> for v8::Local<'scope, v8::RegExp> {
    #[inline(always)]
    fn seal(self) -> RegExp<'scope> {
        RegExp(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::RegExp>> for RegExp<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::RegExp> {
        self.0
    }
}

impl<'scope> From<RegExp<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: RegExp<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for RegExp<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::RegExp>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> RegExp<'scope> {
    // TODO rusty_v8 doesn't export the RegExp operations of V8.
}
