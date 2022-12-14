pub use v8::NewStringType;

use super::{Name, Primitive, Seal, Unseal, Value, ValueScope};

/// Maximal string length.
/// As declared in "include/v8-primitive.h".
#[cfg(target_pointer_width = "32")]
static MAX_STRING_SIZE: usize = (1 << 28) - 16;

/// Maximal string length.
/// As declared in "include/v8-primitive.h".
#[cfg(target_pointer_width = "64")]
static MAX_STRING_LENGTH: usize = (1 << 29) - 24;

/// A string value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct String<'scope>(pub(crate) v8::Local<'scope, v8::String>);

impl<'scope> Seal<String<'scope>> for v8::Local<'scope, v8::String> {
    #[inline(always)]
    fn seal(self) -> String<'scope> {
        String(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::String>> for String<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::String> {
        self.0
    }
}

impl<'scope> From<String<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: String<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for String<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::String>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<String<'scope>> for Primitive<'scope> {
    #[inline(always)]
    fn from(value: String<'scope>) -> Self {
        Primitive(value.0.into())
    }
}

impl<'scope> From<String<'scope>> for Name<'scope> {
    #[inline(always)]
    fn from(value: String<'scope>) -> Self {
        Name(value.0.into())
    }
}

impl<'scope> String<'scope> {
    /// Creates a new string. Will truncate string if they are too long.
    pub fn new<S>(
        scope: &mut ValueScope<'scope>,
        string: S,
        string_type: NewStringType,
    ) -> String<'scope>
    where
        S: AsRef<str>,
    {
        let data = string.as_ref().as_bytes();
        let max_length = usize::min(MAX_STRING_LENGTH, data.len());

        v8::String::new_from_utf8(scope.unseal(), &data[..max_length], string_type)
            .expect("String is too large for V8")
            .seal()
    }

    /// Creates a new string from a static string. Will truncate string if they are too long.
    pub fn new_from_static(scope: &mut ValueScope<'scope>, string: &'static str) -> String<'scope> {
        let data = string.as_bytes();
        let max_length = usize::min(MAX_STRING_LENGTH, data.len());

        v8::String::new_external_onebyte_static(scope.unseal(), &data[..max_length])
            .expect("String is too large for V8")
            .seal()
    }

    /// Returns the value of the string.
    #[inline(always)]
    pub fn value(&self, scope: &mut ValueScope<'scope>) -> std::string::String {
        self.0.to_rust_string_lossy(scope.unseal())
    }

    // TODO export safe variants of the write_* functions.
}

/// Utility function to create a new V8 string. Will truncate string if they are too long.
pub(crate) fn new_string<'scope, S>(
    scope: &mut v8::HandleScope<'scope, ()>,
    string: S,
    string_type: NewStringType,
) -> v8::Local<'scope, v8::String>
where
    S: AsRef<str>,
{
    let data = string.as_ref().as_bytes();
    let max_length = usize::min(MAX_STRING_LENGTH, data.len());
    v8::String::new_from_utf8(scope, &data[..max_length], string_type)
        .expect("String is too large for V8")
}

#[cfg(test)]
mod test {
    use crate::value::string::MAX_STRING_LENGTH;

    #[test]
    fn verify_max_string_length() {
        assert_eq!(MAX_STRING_LENGTH, v8::String::max_length());
    }
}
