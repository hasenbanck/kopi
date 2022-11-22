use super::{Object, Seal, Unseal, Value, ValueScope};

/// A date value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Date<'scope>(pub(crate) v8::Local<'scope, v8::Date>);

impl<'scope> Seal<Date<'scope>> for v8::Local<'scope, v8::Date> {
    #[inline(always)]
    fn seal(self) -> Date<'scope> {
        Date(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Date>> for Date<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Date> {
        self.0
    }
}

impl<'scope> From<Date<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Date<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Date<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Date>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Date<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: Date<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> Date<'scope> {
    /// Creates a new date from the given number of milliseconds elapsed since
    /// January 1, 1970 00:00:00 UTC.
    pub fn new<S>(scope: &mut ValueScope<'scope>, value: f64) -> Option<Date<'scope>> {
        v8::Date::new(scope.unseal(), value).map(|d| d.seal())
    }

    /// Returns the value of the date (milliseconds elapsed since January 1, 1970 00:00:00 UTC).
    #[inline(always)]
    pub fn value(&self) -> f64 {
        self.0.value_of()
    }
}

#[cfg(test)]
mod test {
    use crate::value::{test::test_value, Date};

    // TODO write a test fixture to test constructors.

    #[test]
    fn value() {
        test_value("new Date(0)", |v| {
            let v = Date::try_from(v).expect("Not a Date");
            assert_eq!(v.value(), 0.0);
        });
        test_value("new Date('1995-12-17T03:24:00')", |v| {
            let v = Date::try_from(v).expect("Not a Date");
            assert_eq!(v.value(), 819167040000.0);
        });
        test_value("new Date('2020-05-12T23:50:21.817Z')", |v| {
            let v = Date::try_from(v).expect("Not a Date");
            assert_eq!(v.value(), 1589327421817.0);
        });
    }
}
