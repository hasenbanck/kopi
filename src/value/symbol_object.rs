use super::{Object, Seal, Unseal, Value};

/// A symbol object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SymbolObject<'scope>(pub(crate) v8::Local<'scope, v8::SymbolObject>);

impl<'scope> Seal<SymbolObject<'scope>> for v8::Local<'scope, v8::SymbolObject> {
    #[inline(always)]
    fn seal(self) -> SymbolObject<'scope> {
        SymbolObject(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::SymbolObject>> for SymbolObject<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::SymbolObject> {
        self.0
    }
}

impl<'scope> From<SymbolObject<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: SymbolObject<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for SymbolObject<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::SymbolObject>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<SymbolObject<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: SymbolObject<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> SymbolObject<'scope> {
    // TODO rusty_v8 doesn't export the SymbolObject operations of V8.
}
