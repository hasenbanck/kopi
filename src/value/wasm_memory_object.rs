use super::{Object, Seal, Unseal, Value};

/// A WASM memory object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct WasmMemoryObject<'scope>(pub(crate) v8::Local<'scope, v8::WasmMemoryObject>);

impl<'scope> Seal<WasmMemoryObject<'scope>> for v8::Local<'scope, v8::WasmMemoryObject> {
    #[inline(always)]
    fn seal(self) -> WasmMemoryObject<'scope> {
        WasmMemoryObject(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::WasmMemoryObject>> for WasmMemoryObject<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::WasmMemoryObject> {
        self.0
    }
}

impl<'scope> From<WasmMemoryObject<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: WasmMemoryObject<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for WasmMemoryObject<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::WasmMemoryObject>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<WasmMemoryObject<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: WasmMemoryObject<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> WasmMemoryObject<'scope> {
    // TODO rusty_v8 doesn't expose the buffer accessor for the WasmMemoryObject.
}
