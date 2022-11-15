use super::{ArrayBuffer, Seal, Unseal, Value, ValueScope};

/// A data view into an array buffer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct DataView<'scope>(pub(crate) v8::Local<'scope, v8::DataView>);

impl<'scope> Seal<DataView<'scope>> for v8::Local<'scope, v8::DataView> {
    #[inline(always)]
    fn seal(self) -> DataView<'scope> {
        DataView(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::DataView>> for DataView<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::DataView> {
        self.0
    }
}

impl<'scope> From<DataView<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: DataView<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for DataView<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::DataView>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> DataView<'scope> {
    // TODO rust_v8 doesn't expose the data view constructors.

    /// Returns the number of elements inside the data view.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.byte_length()
    }

    /// Returns `true` if the data view is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.byte_length() == 0
    }

    /// Returns a slice into the data.
    #[inline(always)]
    pub fn as_ref(&self, scope: &mut ValueScope<'scope>) -> &[u8] {
        let data = self
            .0
            .buffer(scope.unseal())
            .expect("DataView has no backing array buffer")
            .data();

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts(data as *const u8, self.0.byte_length()) }
    }

    /// Returns a mutable slice into the data.
    #[inline(always)]
    pub fn as_mut(&mut self, scope: &mut ValueScope<'scope>) -> &mut [u8] {
        let data = self
            .0
            .buffer(scope.unseal())
            .expect("DataView has no backing array buffer")
            .data();

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts_mut(data as *mut u8, self.0.byte_length()) }
    }
}
