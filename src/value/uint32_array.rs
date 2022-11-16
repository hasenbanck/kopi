use std::{ffi::c_void, mem::ManuallyDrop, ptr::null_mut};

use super::{Seal, Unseal, Value, ValueScope};

/// A uint32 array backed by a array buffer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Uint32Array<'scope>(pub(crate) v8::Local<'scope, v8::Uint32Array>);

impl<'scope> Seal<Uint32Array<'scope>> for v8::Local<'scope, v8::Uint32Array> {
    #[inline(always)]
    fn seal(self) -> Uint32Array<'scope> {
        Uint32Array(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Uint32Array>> for Uint32Array<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Uint32Array> {
        self.0
    }
}

impl<'scope> From<Uint32Array<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Uint32Array<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Uint32Array<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Uint32Array>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

pub unsafe extern "C" fn boxed_slice_deleter_callback(
    data: *mut c_void,
    length: usize,
    _deleter_data: *mut c_void,
) {
    let slice_ptr = std::ptr::slice_from_raw_parts_mut(data as *mut u32, length);
    drop(Box::from_raw(slice_ptr));
}

pub unsafe extern "C" fn vec_deleter_callback(
    data: *mut c_void,
    length: usize,
    deleter_data: *mut c_void,
) {
    let capacity = deleter_data as usize;
    drop(Vec::from_raw_parts(data as *mut u32, length, capacity));
}

impl<'scope> Uint32Array<'scope> {
    /// Creates a new uint32 array.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, length: usize) -> Uint32Array<'scope> {
        let data = vec![0u32; length].into_boxed_slice();
        Self::new_from_boxed_slice(scope, data)
    }

    /// Creates a new uint32 array from a boxed slice.
    #[inline(always)]
    pub fn new_from_boxed_slice(
        scope: &mut ValueScope<'scope>,
        data: Box<[u32]>,
    ) -> Uint32Array<'scope> {
        let mut data = ManuallyDrop::new(data);

        let length = data.len();
        let byte_length = length * std::mem::size_of::<u32>();
        let data_ptr = data.as_mut_ptr();

        // SAFETY: The data is properly aligned and initialized and the deleter will safely delete it.
        let store = unsafe {
            v8::ArrayBuffer::new_backing_store_from_ptr(
                data_ptr as *mut c_void,
                byte_length,
                boxed_slice_deleter_callback,
                null_mut(),
            )
        };

        let buffer = v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into());
        v8::Uint32Array::new(scope.unseal(), buffer, 0, length)
            .expect("Uint32Array could not be created")
            .seal()
    }

    /// Creates a new uint32 array from a vec.
    #[inline(always)]
    pub fn new_from_vec(scope: &mut ValueScope<'scope>, data: Vec<u32>) -> Uint32Array<'scope> {
        let mut data = ManuallyDrop::new(data);

        let length = data.len();
        let capacity = data.capacity();
        let byte_length = length * std::mem::size_of::<u32>();
        let data_ptr = data.as_mut_ptr();

        // SAFETY: The data is properly aligned and initialized and the deleter will safely delete it.
        let store = unsafe {
            v8::ArrayBuffer::new_backing_store_from_ptr(
                data_ptr as *mut c_void,
                byte_length,
                vec_deleter_callback,
                capacity as *mut c_void,
            )
        };

        let buffer = v8::ArrayBuffer::with_backing_store(scope.unseal(), &store.into());
        v8::Uint32Array::new(scope.unseal(), buffer, 0, length)
            .expect("Uint32Array could not be created")
            .seal()
    }

    /// Returns the number of elements inside the uint32 array.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.byte_length() / std::mem::size_of::<u32>()
    }

    /// Returns `true` if the uint32 array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        (self.0.byte_length() / std::mem::size_of::<u32>()) == 0
    }

    /// Returns a slice into the data.
    #[inline(always)]
    pub fn as_ref(&self, scope: &mut ValueScope<'scope>) -> &[u32] {
        let byte_length = self.0.byte_length();
        let length = byte_length / std::mem::size_of::<u32>();

        let data_ptr = self
            .0
            .buffer(scope.unseal())
            .expect("Uint8Array has no backing array buffer")
            .data()
            .wrapping_add(self.0.byte_offset()) as *const u32;
        assert_eq!(data_ptr as usize % std::mem::align_of::<u32>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts(data_ptr, length) }
    }

    /// Returns a mutable slice into the data.
    #[inline(always)]
    pub fn as_mut(&mut self, scope: &mut ValueScope<'scope>) -> &mut [u32] {
        let byte_length = self.0.byte_length();
        let length = byte_length / std::mem::size_of::<u32>();

        let data_ptr = self
            .0
            .buffer(scope.unseal())
            .expect("Uint8Array has no backing array buffer")
            .data()
            .wrapping_add(self.0.byte_offset()) as *mut u32;
        assert_eq!(data_ptr as usize % std::mem::align_of::<u32>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts_mut(data_ptr, length) }
    }

    /// Copy the contents of the uint32 array without the overhead of getting the underlying
    /// array buffer.
    ///
    /// Returns the number of **bytes** actually written.
    #[inline(always)]
    pub fn copy(&self, dest: &mut [u32]) -> usize {
        let byte_length = dest.len() * std::mem::size_of::<u32>();

        // SAFETY: We made sure that the align are compatible and the new size is correct.
        let byte_slice =
            unsafe { std::slice::from_raw_parts_mut(dest.as_mut_ptr() as *mut u8, byte_length) };

        self.0.copy_contents(byte_slice)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn u8_u32_compatibility() {
        assert!(std::mem::align_of::<u32>() > std::mem::align_of::<u8>());
        assert_eq!(std::mem::align_of::<u32>() % std::mem::align_of::<u8>(), 0);
    }
}
