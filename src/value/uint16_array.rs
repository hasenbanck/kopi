use super::{ArrayBuffer, Seal, Unseal, Value, ValueScope};

/// A uint16 array backed by a array buffer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Uint16Array<'scope>(pub(crate) v8::Local<'scope, v8::Uint16Array>);

impl<'scope> Seal<Uint16Array<'scope>> for v8::Local<'scope, v8::Uint16Array> {
    #[inline(always)]
    fn seal(self) -> Uint16Array<'scope> {
        Uint16Array(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Uint16Array>> for Uint16Array<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Uint16Array> {
        self.0
    }
}

impl<'scope> From<Uint16Array<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Uint16Array<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Uint16Array<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Uint16Array>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Uint16Array<'scope> {
    /// Creates a new uint16 array.
    ///
    /// Returns `None` if the array could not be created.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, length: usize) -> Uint16Array<'scope> {
        let boxed = vec![0u16; length].into_boxed_slice();
        let store = v8::ArrayBuffer::new_backing_store_from_boxed_slice(scope.unseal(), boxed);
        let buffer = v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into());
        v8::Uint16Array::new(scope.unseal(), buffer, 0, length)
            .expect("Uint16Array could not be created")
            .seal()
    }

    /// Creates a new uint8 array from a boxed slice.
    #[inline(always)]
    pub fn new_from_boxed_slice(
        scope: &mut ValueScope<'scope>,
        data: Box<[u16]>,
    ) -> Uint16Array<'scope> {
        let length = data.len();
        let store = v8::ArrayBuffer::new_backing_store_from_boxed_slice(data);
        let buffer = v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into());
        v8::Uint16Array::new(scope.unseal(), buffer, 0, length)
            .expect("Uint16Array could not be created")
            .seal()
    }

    /// Creates a new uint16 array from a vec.
    #[inline(always)]
    pub fn new_from_vec(scope: &mut ValueScope<'scope>, data: Vec<u16>) -> Uint16Array<'scope> {
        let length = data.len();
        let store = v8::ArrayBuffer::new_backing_store_from_vec(data);
        let buffer = v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into());
        v8::Uint16Array::new(scope.unseal(), buffer, 0, length)
            .expect("Uint16Array could not be created")
            .seal()
    }

    /// Returns the number of elements inside the uint16 array.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.byte_length() / std::mem::size_of::<u16>()
    }

    /// Returns `true` if the uint8 array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        (self.0.byte_length() / std::mem::size_of::<u16>()) == 0
    }

    /// Returns a slice into the data.
    #[inline(always)]
    pub fn as_ref(&self, scope: &mut ValueScope<'scope>) -> &[u16] {
        let byte_length = self.0.byte_length();
        assert_eq!(byte_length % std::mem::size_of::<u16>(), 0);
        let length = byte_length / std::mem::size_of::<u16>();

        // SAFETY: We must trust that the byte_offset provided by V8 is valid.
        let data_ptr = unsafe {
            self.0
                .buffer(scope.unseal())
                .expect("Uint8Array has no backing array buffer")
                .data()
                .wrapping_add(self.0.byte_offset()) as *const u16
        };
        assert_eq!(data_ptr as usize % std::mem::align_of::<u16>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts(data_ptr, length) }
    }

    /// Returns a mutable slice into the data.
    #[inline(always)]
    pub fn as_mut(&mut self, scope: &mut ValueScope<'scope>) -> &mut [u16] {
        let byte_length = self.0.byte_length();
        assert_eq!(byte_length % std::mem::size_of::<u16>(), 0);
        let length = byte_length / std::mem::size_of::<u16>();

        // SAFETY: We must trust that the byte_offset provided by V8 is valid.
        let data_ptr = unsafe {
            self.0
                .buffer(scope.unseal())
                .expect("Uint8Array has no backing array buffer")
                .data()
                .wrapping_add(self.0.byte_offset()) as *mut u16
        };
        assert_eq!(data_ptr as usize % std::mem::align_of::<u16>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts_mut(data_ptr, length) }
    }

    /// Copy the contents of the uint16 array without the overhead of getting the underlying
    /// array buffer.
    ///
    /// Returns the number of bytes actually written.
    #[inline(always)]
    pub fn copy(&self, dest: &mut [u16]) -> usize {
        let byte_length = dest.len() * std::mem::size_of::<u16>();

        // SAFETY: We made sure that the align are compatible and the new size is correct.
        let byte_slice =
            unsafe { std::slice::from_raw_parts_mut(dest.as_mut_ptr() as *mut u8, byte_length) };

        self.0.copy_contents(byte_slice)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn u8_u16_compatibility() {
        assert!(std::mem::align_of::<u16>() > std::mem::align_of::<u8>());
        assert_eq!(std::mem::align_of::<u16>() % std::mem::align_of::<u8>(), 0);
    }
}
