use super::{Seal, Unseal, Value, ValueScope};

/// A number value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Number<'scope>(v8::Local<'scope, v8::Number>);

impl<'scope> Seal<Number<'scope>> for v8::Local<'scope, v8::Number> {
    #[inline(always)]
    fn seal(self) -> Number<'scope> {
        Number(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Number>> for Number<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Number> {
        self.0
    }
}

impl<'scope> From<Number<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Number<'scope>) -> Self {
        Value::new(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Number<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Number>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Number<'scope> {
    /// Creates a new number from the given f64 value.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, value: f64) -> Number<'scope> {
        v8::Number::new(scope.unseal(), value).seal()
    }

    /// Returns the value of the number.
    #[inline(always)]
    pub fn value(&self) -> f64 {
        self.0.value()
    }
}
