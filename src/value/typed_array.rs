use super::{ArrayBufferView, Object, Seal, Unseal, Value};

/// A super class for "views" into array buffers of a specific typed value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct TypedArray<'scope>(pub(crate) v8::Local<'scope, v8::TypedArray>);

impl<'scope> Seal<TypedArray<'scope>> for v8::Local<'scope, v8::TypedArray> {
    #[inline(always)]
    fn seal(self) -> TypedArray<'scope> {
        TypedArray(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::TypedArray>> for TypedArray<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::TypedArray> {
        self.0
    }
}

impl<'scope> From<TypedArray<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: TypedArray<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for TypedArray<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::TypedArray>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<TypedArray<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: TypedArray<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> From<TypedArray<'scope>> for ArrayBufferView<'scope> {
    #[inline(always)]
    fn from(value: TypedArray<'scope>) -> Self {
        ArrayBufferView(value.0.into())
    }
}
