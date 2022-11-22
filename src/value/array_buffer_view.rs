use super::{Object, Seal, Unseal, Value};

/// A super class for "views" on top of array buffers.
///
/// Can either be a data view or a typed array.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ArrayBufferView<'scope>(pub(crate) v8::Local<'scope, v8::ArrayBufferView>);

impl<'scope> Seal<ArrayBufferView<'scope>> for v8::Local<'scope, v8::ArrayBufferView> {
    #[inline(always)]
    fn seal(self) -> ArrayBufferView<'scope> {
        ArrayBufferView(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::ArrayBufferView>> for ArrayBufferView<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::ArrayBufferView> {
        self.0
    }
}

impl<'scope> From<ArrayBufferView<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: ArrayBufferView<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for ArrayBufferView<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::ArrayBufferView>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<ArrayBufferView<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: ArrayBufferView<'scope>) -> Self {
        Object(value.0.into())
    }
}
