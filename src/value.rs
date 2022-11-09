//! Values to work with V8.

pub use v8::{
    Array, ArrayBuffer, ArrayBufferView, BigInt, BigInt64Array, BigIntObject, BigUint64Array,
    Boolean, Data, DataView, Date, Float32Array, Float64Array, Int16Array, Int32, Int32Array,
    Int8Array, Integer, Local, NewStringType, Number, Object, Primitive, String, Uint16Array,
    Uint32, Uint32Array, Uint8Array, Value,
};

static MAX_STRING_SIZE: once_cell::sync::Lazy<usize> =
    once_cell::sync::Lazy::new(String::max_length);

/// A scope in which the values can live. Can be used to create new values.
#[repr(transparent)]
pub struct ValueScope<'borrow, 'scope>(pub(crate) &'borrow mut v8::HandleScope<'scope>);

// TODO create the missing value constructors
impl<'borrow, 'scope> ValueScope<'borrow, 'scope> {
    /// Creates a string representation from the given value.
    #[inline(always)]
    pub fn value_to_string(&mut self, value: &Value) -> std::string::String {
        value.to_rust_string_lossy(self.0)
    }

    /// Creates a new string. Will truncate string if they are too long.
    pub fn new_string<S: AsRef<str>>(&mut self, string: S) -> Local<'scope, String> {
        let data = string.as_ref().as_bytes();
        let max_length = usize::min(*MAX_STRING_SIZE, data.len());
        String::new_from_utf8(self.0, &data[..max_length], v8::NewStringType::Normal)
            .expect("String is too large for V8")
    }

    ///Creates a new string from a static string. Will truncate string if they are too long.
    pub fn new_string_from_static(&mut self, string: &'static str) -> Local<'scope, String> {
        let data = string.as_bytes();
        let max_length = usize::min(*MAX_STRING_SIZE, data.len());
        String::new_external_onebyte_static(self.0, &data[..max_length])
            .expect("String is too large for V8")
    }

    /// Creates a new undefined value.
    #[inline(always)]
    pub fn new_undefined(&mut self) -> Local<'scope, Primitive> {
        v8::undefined(self.0)
    }

    /// Creates a new null value.
    #[inline(always)]
    pub fn new_null(&mut self) -> Local<'scope, Primitive> {
        v8::null(self.0)
    }

    /// Creates a new integer value.
    #[inline(always)]
    pub fn new_integer(&mut self, value: i32) -> Local<'scope, Integer> {
        Integer::new(self.0, value)
    }

    /// Creates a new number value.
    #[inline(always)]
    pub fn new_number(&mut self, value: f64) -> Local<'scope, Number> {
        Number::new(self.0, value)
    }

    /// Creates a new boolean value.
    #[inline(always)]
    pub fn new_boolean(&mut self, value: bool) -> Local<'scope, Boolean> {
        Boolean::new(self.0, value)
    }

    /// Creates a new big integer value for a u64.
    #[inline(always)]
    pub fn new_bigint_from_u64(&mut self, value: u64) -> Local<'scope, BigInt> {
        BigInt::new_from_u64(self.0, value)
    }

    /// Creates a new big integer value for a i64.
    #[inline(always)]
    pub fn new_bigint_from_i64(&mut self, value: i64) -> Local<'scope, BigInt> {
        BigInt::new_from_i64(self.0, value)
    }
}

/// Utility function to safely create string. Will truncate string if they are too long.
pub(crate) fn create_string<'scope, S: AsRef<str>>(
    scope: &mut v8::HandleScope<'scope, ()>,
    string: S,
) -> Local<'scope, String> {
    let data = string.as_ref().as_bytes();
    let max_length = usize::min(*MAX_STRING_SIZE, data.len());
    String::new_from_utf8(scope, &data[..max_length], v8::NewStringType::Normal)
        .expect("String is too large for V8")
}
