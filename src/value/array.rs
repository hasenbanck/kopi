use super::{Seal, Unseal, Value, ValueScope};

/// A array value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Array<'scope>(v8::Local<'scope, v8::Array>);

impl<'scope> Seal<Array<'scope>> for v8::Local<'scope, v8::Array> {
    #[inline(always)]
    fn seal(self) -> Array<'scope> {
        Array(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Array>> for Array<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Array> {
        self.0
    }
}

impl<'scope> From<Array<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Array<'scope>) -> Self {
        Value::new(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Array<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Array>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Array<'scope> {
    /// Creates a new array. Can have at most [`i32::MAX`] elements.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>, size: usize) -> Array<'scope> {
        let size = size.min(i32::MAX as usize);
        v8::Array::new(scope.unseal(), size as i32).seal()
    }

    /// Creates a new array from the given slice of values.
    #[inline(always)]
    pub fn new_with_elements<E: AsRef<[Value<'scope>]>>(
        scope: &mut ValueScope<'scope>,
        elements: E,
    ) -> Array<'scope> {
        let elements = elements.as_ref();

        // SAFETY: This is safe, since Value wraps a v8::Local<v8::Value> transparently.
        let elements = unsafe {
            std::slice::from_raw_parts(
                elements.as_ptr() as *const v8::Local<'scope, v8::Value>,
                elements.len(),
            )
        };

        v8::Array::new_with_elements(scope.unseal(), elements).seal()
    }

    /// Returns the element at the given array position.
    #[inline(always)]
    pub fn get(&self, scope: &mut ValueScope<'scope>, pos: u32) -> Option<Value<'scope>> {
        self.0.get_index(scope.unseal(), pos).map(|v| v.seal())
    }

    /// Sets the value at the given array position. Returns `true` if the value could be written.
    #[inline(always)]
    pub fn set(&self, scope: &mut ValueScope<'scope>, pos: u32, value: Value<'scope>) -> bool {
        self.0
            .set_index(scope.unseal(), pos, value.unseal())
            .unwrap_or(false)
    }

    /// Returns the length of the array.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.0.length()
    }

    /// Returns `true` if the array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.length() == 0
    }
}
