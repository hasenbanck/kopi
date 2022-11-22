use super::{Seal, String, Unseal, Value, ValueScope};

/// A function.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Function<'scope>(pub(crate) v8::Local<'scope, v8::Function>);

impl<'scope> Seal<Function<'scope>> for v8::Local<'scope, v8::Function> {
    #[inline(always)]
    fn seal(self) -> Function<'scope> {
        Function(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Function>> for Function<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Function> {
        self.0
    }
}

impl<'scope> From<Function<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Function<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Function<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Function>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Function<'scope> {
    /// Returns the name of the function.
    #[inline(always)]
    pub fn name(&self, scope: &mut ValueScope<'scope>) -> String<'scope> {
        self.0.get_name(scope.unseal()).seal()
    }

    /// Sets the name of the function.
    #[inline(always)]
    pub fn set_name(&self, name: String<'scope>) {
        self.0.set_name(name.unseal())
    }

    /// Returns the (zero-indexed) column number of the functions definition, if available.
    #[inline(always)]
    pub fn script_column_number(&self) -> Option<u32> {
        self.0.get_script_column_number()
    }

    /// Returns the (zero-indexed) line number of the functions definition, if available.
    #[inline(always)]
    pub fn script_line_number(&self) -> Option<u32> {
        self.0.get_script_line_number()
    }
}
