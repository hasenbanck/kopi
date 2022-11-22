use super::{Object, Seal, Unseal, Value};

/// A WASM module object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct WasmModuleObject<'scope>(pub(crate) v8::Local<'scope, v8::WasmModuleObject>);

impl<'scope> Seal<WasmModuleObject<'scope>> for v8::Local<'scope, v8::WasmModuleObject> {
    #[inline(always)]
    fn seal(self) -> WasmModuleObject<'scope> {
        WasmModuleObject(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::WasmModuleObject>> for WasmModuleObject<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::WasmModuleObject> {
        self.0
    }
}

impl<'scope> From<WasmModuleObject<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: WasmModuleObject<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for WasmModuleObject<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::WasmModuleObject>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<WasmModuleObject<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: WasmModuleObject<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> WasmModuleObject<'scope> {
    // TODO it's not clear yet how WASM integration should look like. So we don't expose an API for now.
}
