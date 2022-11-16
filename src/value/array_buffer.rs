use super::{Seal, Unseal, Value, ValueScope};

/// A array buffer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ArrayBuffer<'scope>(pub(crate) v8::Local<'scope, v8::ArrayBuffer>);

impl<'scope> Seal<ArrayBuffer<'scope>> for v8::Local<'scope, v8::ArrayBuffer> {
    #[inline(always)]
    fn seal(self) -> ArrayBuffer<'scope> {
        ArrayBuffer(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::ArrayBuffer>> for ArrayBuffer<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::ArrayBuffer> {
        self.0
    }
}

impl<'scope> From<ArrayBuffer<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: ArrayBuffer<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for ArrayBuffer<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::ArrayBuffer>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

// TODO test all buffer.

impl<'scope> ArrayBuffer<'scope> {
    /// Creates a new array buffer from the given boxed slice.
    #[inline(always)]
    pub fn new_from_boxed_slice(
        scope: &mut ValueScope<'scope>,
        data: Box<[u8]>,
    ) -> ArrayBuffer<'scope> {
        let store = v8::ArrayBuffer::new_backing_store_from_boxed_slice(data);
        v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into()).seal()
    }

    /// Creates a new array buffer from the given Vec.
    #[inline(always)]
    pub fn new_from_vec(scope: &mut ValueScope<'scope>, data: Vec<u8>) -> ArrayBuffer<'scope> {
        let store = v8::ArrayBuffer::new_backing_store_from_vec(data);
        v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into()).seal()
    }

    /// Returns length of the array in bytes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.byte_length()
    }

    /// Returns `true` if the array buffer is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.byte_length() == 0
    }
}

impl<'scope> AsRef<[u8]> for ArrayBuffer<'scope> {
    fn as_ref(&self) -> &[u8] {
        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts(self.0.data() as *const u8, self.0.byte_length()) }
    }
}

impl<'scope> AsMut<[u8]> for ArrayBuffer<'scope> {
    fn as_mut(&mut self) -> &mut [u8] {
        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts_mut(self.0.data() as *mut u8, self.0.byte_length()) }
    }
}
