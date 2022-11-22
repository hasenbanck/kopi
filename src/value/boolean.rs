use super::{Primitive, Seal, Unseal, Value, ValueScope};

/// A boolean value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Boolean<'scope>(pub(crate) v8::Local<'scope, v8::Boolean>);

impl<'scope> Seal<Boolean<'scope>> for v8::Local<'scope, v8::Boolean> {
    #[inline(always)]
    fn seal(self) -> Boolean<'scope> {
        Boolean(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Boolean>> for Boolean<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Boolean> {
        self.0
    }
}

impl<'scope> From<Boolean<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Boolean<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Boolean<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Boolean>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Boolean<'scope>> for Primitive<'scope> {
    #[inline(always)]
    fn from(value: Boolean<'scope>) -> Self {
        Primitive(value.0.into())
    }
}

impl<'scope> Boolean<'scope> {
    /// Creates a new boolean.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, value: bool) -> Boolean<'scope> {
        v8::Boolean::new(scope.unseal(), value).seal()
    }

    /// Returns the value of the boolean.
    #[inline(always)]
    pub fn value(&self) -> bool {
        self.0.is_true()
    }
}

#[cfg(test)]
mod test {
    use crate::value::{test::test_value, Boolean};

    #[test]
    fn value() {
        test_value("true", |v| {
            let b = Boolean::try_from(v).expect("Not a boolean");
            assert!(b.value());
        });
        test_value("false", |v| {
            let b = Boolean::try_from(v).expect("Not a boolean");
            assert!(!b.value());
        });
    }
}
