use std::{ffi::c_void, mem::ManuallyDrop, ptr::null_mut};

use super::{Seal, Unseal, Value, ValueScope};

/// A [`Int8Array`] backed by a array buffer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Int8Array<'scope>(pub(crate) v8::Local<'scope, v8::Int8Array>);

impl<'scope> Seal<Int8Array<'scope>> for v8::Local<'scope, v8::Int8Array> {
    #[inline(always)]
    fn seal(self) -> Int8Array<'scope> {
        Int8Array(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Int8Array>> for Int8Array<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Int8Array> {
        self.0
    }
}

impl<'scope> From<Int8Array<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Int8Array<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Int8Array<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Int8Array>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

pub unsafe extern "C" fn boxed_slice_deleter_callback(
    data: *mut c_void,
    length: usize,
    _deleter_data: *mut c_void,
) {
    let slice_ptr = std::ptr::slice_from_raw_parts_mut(data as *mut i8, length);
    drop(Box::from_raw(slice_ptr));
}

pub unsafe extern "C" fn vec_deleter_callback(
    data: *mut c_void,
    length: usize,
    deleter_data: *mut c_void,
) {
    let capacity = deleter_data as usize;
    drop(Vec::from_raw_parts(data as *mut i8, length, capacity));
}

impl<'scope> Int8Array<'scope> {
    /// Creates a new [`Int8Array`].
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, length: usize) -> Int8Array<'scope> {
        let data = vec![0i8; length].into_boxed_slice();
        Self::new_from_boxed_slice(scope, data)
    }

    /// Creates a new [`Int8Array`] from a boxed slice.
    #[inline(always)]
    pub fn new_from_boxed_slice(
        scope: &mut ValueScope<'scope>,
        data: Box<[i8]>,
    ) -> Int8Array<'scope> {
        let mut data = ManuallyDrop::new(data);

        let length = data.len();
        let byte_length = length * std::mem::size_of::<i8>();
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
        v8::Int8Array::new(scope.unseal(), buffer, 0, length)
            .expect("Int8Array could not be created")
            .seal()
    }

    /// Creates a new [`Int8Array`] from a vec.
    #[inline(always)]
    pub fn new_from_vec(scope: &mut ValueScope<'scope>, data: Vec<i8>) -> Int8Array<'scope> {
        let mut data = ManuallyDrop::new(data);

        let length = data.len();
        let capacity = data.capacity();
        let byte_length = length * std::mem::size_of::<i8>();
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
        v8::Int8Array::new(scope.unseal(), buffer, 0, length)
            .expect("Int8Array could not be created")
            .seal()
    }

    /// Returns the number of elements inside the int8 array.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.byte_length() / std::mem::size_of::<i8>()
    }

    /// Returns `true` if the [`Int8Array`] is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        (self.0.byte_length() / std::mem::size_of::<i8>()) == 0
    }

    /// Returns a slice into the data.
    #[inline(always)]
    pub fn as_ref(&self, scope: &mut ValueScope<'scope>) -> &[i8] {
        let byte_length = self.0.byte_length();
        let length = byte_length / std::mem::size_of::<i8>();

        let data_ptr = self
            .0
            .buffer(scope.unseal())
            .expect("Int8Array has no backing array buffer")
            .data()
            .wrapping_add(self.0.byte_offset()) as *const i8;
        assert_eq!(data_ptr as usize % std::mem::align_of::<i8>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts(data_ptr, length) }
    }

    /// Returns a mutable slice into the data.
    #[inline(always)]
    pub fn as_mut(&mut self, scope: &mut ValueScope<'scope>) -> &mut [i8] {
        let byte_length = self.0.byte_length();
        let length = byte_length / std::mem::size_of::<i8>();

        let data_ptr = self
            .0
            .buffer(scope.unseal())
            .expect("Int8Array has no backing array buffer")
            .data()
            .wrapping_add(self.0.byte_offset()) as *mut i8;
        assert_eq!(data_ptr as usize % std::mem::align_of::<i8>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts_mut(data_ptr, length) }
    }

    /// Copy the contents of the [`Int8Array`] without the overhead of getting the underlying
    /// array buffer.
    ///
    /// Returns the number of **bytes** actually written.
    #[inline(always)]
    pub fn copy(&self, dest: &mut [i8]) -> usize {
        let byte_length = dest.len() * std::mem::size_of::<i8>();

        // SAFETY: We made sure that the align are compatible and the new size is correct.
        let byte_slice =
            unsafe { std::slice::from_raw_parts_mut(dest.as_mut_ptr() as *mut u8, byte_length) };

        self.0.copy_contents(byte_slice)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn u8_i8_compatibility() {
        assert_eq!(std::mem::align_of::<i8>(), std::mem::align_of::<u8>());
    }
}