use super::{Seal, Unseal, Value, ValueScope};

/// The superclass of primitive values.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Primitive<'scope>(v8::Local<'scope, v8::Primitive>);

impl<'scope> Seal<Primitive<'scope>> for v8::Local<'scope, v8::Primitive> {
    #[inline(always)]
    fn seal(self) -> Primitive<'scope> {
        Primitive(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Primitive>> for Primitive<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Primitive> {
        self.0
    }
}

impl<'scope> From<Primitive<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Primitive<'scope>) -> Self {
        Value::new(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Primitive<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Primitive>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Primitive<'scope> {
    /// Creates a null value.
    #[inline(always)]
    pub fn new_null(scope: &mut ValueScope<'scope>) -> Primitive<'scope> {
        v8::null(scope.unseal()).seal()
    }

    /// Creates a undefined value.
    #[inline(always)]
    pub fn new_undefined(scope: &mut ValueScope<'scope>) -> Primitive<'scope> {
        v8::undefined(scope.unseal()).seal()
    }
}
