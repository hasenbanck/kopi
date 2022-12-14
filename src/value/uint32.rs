use super::{Integer, Number, Primitive, Seal, Unseal, Value};

/// A 32-bit unsigned integer value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Uint32<'scope>(pub(crate) v8::Local<'scope, v8::Uint32>);

impl<'scope> Seal<Uint32<'scope>> for v8::Local<'scope, v8::Uint32> {
    #[inline(always)]
    fn seal(self) -> Uint32<'scope> {
        Uint32(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Uint32>> for Uint32<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Uint32> {
        self.0
    }
}

impl<'scope> From<Uint32<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Uint32<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Uint32<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Uint32>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Uint32<'scope>> for Primitive<'scope> {
    #[inline(always)]
    fn from(value: Uint32<'scope>) -> Self {
        Primitive(value.0.into())
    }
}

impl<'scope> From<Uint32<'scope>> for Number<'scope> {
    #[inline(always)]
    fn from(value: Uint32<'scope>) -> Self {
        Number(value.0.into())
    }
}

impl<'scope> From<Uint32<'scope>> for Integer<'scope> {
    #[inline(always)]
    fn from(value: Uint32<'scope>) -> Self {
        Integer(value.0.into())
    }
}

impl<'scope> Uint32<'scope> {
    /// Returns the value of the uint32.
    #[inline(always)]
    pub fn value(&self) -> u32 {
        self.0.value()
    }
}
