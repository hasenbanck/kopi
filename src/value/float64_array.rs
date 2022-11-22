use std::{ffi::c_void, mem::ManuallyDrop, ptr::null_mut};

use super::{ArrayBufferView, Object, Seal, TypedArray, Unseal, Value, ValueScope};

/// A Float64Array backed by a array buffer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Float64Array<'scope>(pub(crate) v8::Local<'scope, v8::Float64Array>);

impl<'scope> Seal<Float64Array<'scope>> for v8::Local<'scope, v8::Float64Array> {
    #[inline(always)]
    fn seal(self) -> Float64Array<'scope> {
        Float64Array(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Float64Array>> for Float64Array<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Float64Array> {
        self.0
    }
}

impl<'scope> From<Float64Array<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Float64Array<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Float64Array<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Float64Array>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Float64Array<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: Float64Array<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> From<Float64Array<'scope>> for ArrayBufferView<'scope> {
    #[inline(always)]
    fn from(value: Float64Array<'scope>) -> Self {
        ArrayBufferView(value.0.into())
    }
}

impl<'scope> From<Float64Array<'scope>> for TypedArray<'scope> {
    #[inline(always)]
    fn from(value: Float64Array<'scope>) -> Self {
        TypedArray(value.0.into())
    }
}

pub unsafe extern "C" fn boxed_slice_deleter_callback(
    data: *mut c_void,
    length: usize,
    _deleter_data: *mut c_void,
) {
    let slice_ptr = std::ptr::slice_from_raw_parts_mut(data as *mut f64, length);
    drop(Box::from_raw(slice_ptr));
}

pub unsafe extern "C" fn vec_deleter_callback(
    data: *mut c_void,
    length: usize,
    deleter_data: *mut c_void,
) {
    let capacity = deleter_data as usize;
    drop(Vec::from_raw_parts(data as *mut f64, length, capacity));
}

impl<'scope> Float64Array<'scope> {
    /// Creates a new [`Float64Array`].
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, length: usize) -> Float64Array<'scope> {
        let data = vec![0f64; length].into_boxed_slice();
        Self::new_from_boxed_slice(scope, data)
    }

    /// Creates a new [`Float64Array`] from a boxed slice.
    #[inline(always)]
    pub fn new_from_boxed_slice(
        scope: &mut ValueScope<'scope>,
        data: Box<[f64]>,
    ) -> Float64Array<'scope> {
        let mut data = ManuallyDrop::new(data);

        let length = data.len();
        let byte_length = length * std::mem::size_of::<f64>();
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
        v8::Float64Array::new(scope.unseal(), buffer, 0, length)
            .expect("Float64Array could not be created")
            .seal()
    }

    /// Creates a new [`Float64Array`] from a vec.
    #[inline(always)]
    pub fn new_from_vec(scope: &mut ValueScope<'scope>, data: Vec<f64>) -> Float64Array<'scope> {
        let mut data = ManuallyDrop::new(data);

        let length = data.len();
        let capacity = data.capacity();
        let byte_length = length * std::mem::size_of::<f64>();
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
        v8::Float64Array::new(scope.unseal(), buffer, 0, length)
            .expect("Float64Array could not be created")
            .seal()
    }

    /// Returns the number of elements inside the float64 array.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.byte_length() / std::mem::size_of::<f64>()
    }

    /// Returns `true` if the [`Float64Array`] is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        (self.0.byte_length() / std::mem::size_of::<f64>()) == 0
    }

    /// Returns a slice into the data.
    #[inline(always)]
    pub fn as_ref(&self, scope: &mut ValueScope<'scope>) -> &[f64] {
        let byte_length = self.0.byte_length();
        let length = byte_length / std::mem::size_of::<f64>();

        let data_ptr = self
            .0
            .buffer(scope.unseal())
            .expect("Float64Array has no backing array buffer")
            .data()
            .wrapping_add(self.0.byte_offset()) as *const f64;
        assert_eq!(data_ptr as usize % std::mem::align_of::<f64>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts(data_ptr, length) }
    }

    /// Returns a mutable slice into the data.
    #[inline(always)]
    pub fn as_mut(&mut self, scope: &mut ValueScope<'scope>) -> &mut [f64] {
        let byte_length = self.0.byte_length();
        let length = byte_length / std::mem::size_of::<f64>();

        let data_ptr = self
            .0
            .buffer(scope.unseal())
            .expect("Float64Array has no backing array buffer")
            .data()
            .wrapping_add(self.0.byte_offset()) as *mut f64;
        assert_eq!(data_ptr as usize % std::mem::align_of::<f64>(), 0);

        // SAFETY: The API only allows to create array buffer with initialized data.
        unsafe { std::slice::from_raw_parts_mut(data_ptr, length) }
    }

    /// Copy the contents of the [`Float64Array`] without the overhead of getting the underlying
    /// array buffer.
    ///
    /// Returns the number of **bytes** actually written.
    #[inline(always)]
    pub fn copy(&self, dest: &mut [f64]) -> usize {
        let byte_length = dest.len() * std::mem::size_of::<f64>();

        // SAFETY: We made sure that the align are compatible and the new size is correct.
        let byte_slice =
            unsafe { std::slice::from_raw_parts_mut(dest.as_mut_ptr() as *mut u8, byte_length) };

        self.0.copy_contents(byte_slice)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn u8_f64_compatibility() {
        assert!(std::mem::align_of::<f64>() > std::mem::align_of::<u8>());
        assert_eq!(std::mem::align_of::<f64>() % std::mem::align_of::<u8>(), 0);
    }
}
