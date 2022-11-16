use super::{Seal, Unseal, Value, ValueScope};

/// A uint8 clamped array backed by a array buffer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Uint8ClampedArray<'scope>(pub(crate) v8::Local<'scope, v8::Uint8ClampedArray>);

impl<'scope> Seal<Uint8ClampedArray<'scope>> for v8::Local<'scope, v8::Uint8ClampedArray> {
    #[inline(always)]
    fn seal(self) -> Uint8ClampedArray<'scope> {
        Uint8ClampedArray(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Uint8ClampedArray>> for Uint8ClampedArray<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Uint8ClampedArray> {
        self.0
    }
}

impl<'scope> From<Uint8ClampedArray<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Uint8ClampedArray<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Uint8ClampedArray<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Uint8ClampedArray>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Uint8ClampedArray<'scope> {
    /// Creates a new uint8 clamped array.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, length: usize) -> Uint8ClampedArray<'scope> {
        let data = vec![0u8; length].into_boxed_slice();
        Self::new_from_boxed_slice(scope, data)
    }

    /// Creates a new uint8 clamped array from a boxed slice.
    #[inline(always)]
    pub fn new_from_boxed_slice(
        scope: &mut ValueScope<'scope>,
        data: Box<[u8]>,
    ) -> Uint8ClampedArray<'scope> {
        let length = data.len();
        let store = v8::ArrayBuffer::new_backing_store_from_boxed_slice(data);
        let buffer = v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into());
        v8::Uint8ClampedArray::new(scope.unseal(), buffer, 0, length)
            .expect("Uint8ClampedArray could not be created")
            .seal()
    }

    /// Creates a new uint8 clamped array from a vec.
    #[inline(always)]
    pub fn new_from_vec(
        scope: &mut ValueScope<'scope>,
        data: Vec<u8>,
    ) -> Uint8ClampedArray<'scope> {
        let length = data.len();
        let store = v8::ArrayBuffer::new_backing_store_from_vec(data);
        let buffer = v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into());
        v8::Uint8ClampedArray::new(scope.unseal(), buffer, 0, length)
            .expect("Uint8ClampedArray could not be created")
            .seal()
    }

    /// Returns the number of elements inside the uint8 clamped array.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.byte_length()
    }

    /// Returns `true` if the uint8 clamped array is empty.
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
            .expect("Uint8ClampedArray has no backing array buffer")
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
            .expect("Uint8ClampedArray has no backing array buffer")
            .data();

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts_mut(data as *mut u8, self.0.byte_length()) }
    }
}
