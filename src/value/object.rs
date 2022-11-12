use super::{Seal, Unseal, Value, ValueScope};

/// A object value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Object<'scope>(v8::Local<'scope, v8::Object>);

impl<'scope> Seal<Object<'scope>> for v8::Local<'scope, v8::Object> {
    #[inline(always)]
    fn seal(self) -> Object<'scope> {
        Object(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Object>> for Object<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Object> {
        self.0
    }
}

impl<'scope> From<Object<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Object<'scope>) -> Self {
        Value::new(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Object<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Object>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Object<'scope> {
    /// Creates a new object.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>) -> Object<'scope> {
        v8::Object::new(scope.unseal()).seal()
    }

    // TODO wrap the functions.
}
