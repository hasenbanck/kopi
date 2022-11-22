use super::{Name, Primitive, Seal, String, Unseal, Value, ValueScope};

/// A symbol value.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Symbol<'scope>(pub(crate) v8::Local<'scope, v8::Symbol>);

impl<'scope> Seal<Symbol<'scope>> for v8::Local<'scope, v8::Symbol> {
    #[inline(always)]
    fn seal(self) -> Symbol<'scope> {
        Symbol(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Symbol>> for Symbol<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Symbol> {
        self.0
    }
}

impl<'scope> From<Symbol<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Symbol<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Symbol<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Symbol>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Symbol<'scope>> for Primitive<'scope> {
    #[inline(always)]
    fn from(value: Symbol<'scope>) -> Self {
        Primitive(value.0.into())
    }
}

impl<'scope> From<Symbol<'scope>> for Name<'scope> {
    #[inline(always)]
    fn from(value: Symbol<'scope>) -> Self {
        Name(value.0.into())
    }
}

impl<'scope> Symbol<'scope> {
    /// Creates a new symbol using the optional description.
    pub fn new<S>(scope: &mut ValueScope<'scope>, description: Option<String>) -> Symbol<'scope> {
        v8::Symbol::new(scope.unseal(), description.map(|d| d.unseal())).seal()
    }

    /// Returns the description of the symbol. Returns `undefined` if the description is not set.
    #[inline(always)]
    pub fn description(&self, scope: &mut ValueScope<'scope>) -> Value {
        self.0.description(scope.unseal()).seal()
    }
}
