use super::{Seal, Unseal, Value, ValueScope};

/// A integer value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Integer<'scope>(v8::Local<'scope, v8::Integer>);

impl<'scope> Seal<Integer<'scope>> for v8::Local<'scope, v8::Integer> {
    #[inline(always)]
    fn seal(self) -> Integer<'scope> {
        Integer(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Integer>> for Integer<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Integer> {
        self.0
    }
}

impl<'scope> From<Integer<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Integer<'scope>) -> Self {
        Value::new(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Integer<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Integer>::try_from(value.0)?;
        Ok(Self(inner.into()))
    }
}

impl<'scope> Integer<'scope> {
    /// Creates a new integer from the given i32 value.
    #[inline(always)]
    pub fn new_from_i32(scope: &mut ValueScope<'scope>, value: i32) -> Integer<'scope> {
        v8::Integer::new(scope.unseal(), value).seal()
    }

    /// Creates a new integer from the given u32 value.
    #[inline(always)]
    pub fn new_from_u32(scope: &mut ValueScope<'scope>, value: u32) -> Integer<'scope> {
        v8::Integer::new_from_unsigned(scope.unseal(), value).seal()
    }

    /// Returns the value of the integer.
    #[inline(always)]
    pub fn value(&self) -> i64 {
        self.0.value()
    }
}