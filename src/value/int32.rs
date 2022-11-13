use super::{Integer, Seal, Unseal, Value};

/// A int32 value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Int32<'scope>(pub(crate) v8::Local<'scope, v8::Int32>);

impl<'scope> Seal<Int32<'scope>> for v8::Local<'scope, v8::Int32> {
    #[inline(always)]
    fn seal(self) -> Int32<'scope> {
        Int32(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Int32>> for Int32<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Int32> {
        self.0
    }
}

impl<'scope> From<Int32<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Int32<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Int32<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Int32>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Int32<'scope>> for Integer<'scope> {
    #[inline(always)]
    fn from(value: Int32<'scope>) -> Self {
        Integer(value.0.into())
    }
}

impl<'scope> Int32<'scope> {
    /// Returns the value of the int32.
    #[inline(always)]
    pub fn value(&self) -> i32 {
        self.0.value()
    }
}
