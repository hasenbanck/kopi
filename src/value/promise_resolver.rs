use super::{Object, Promise, Seal, Unseal, Value, ValueScope};

/// A PromiseResolver object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PromiseResolver<'scope>(pub(crate) v8::Local<'scope, v8::PromiseResolver>);

impl<'scope> Seal<PromiseResolver<'scope>> for v8::Local<'scope, v8::PromiseResolver> {
    #[inline(always)]
    fn seal(self) -> PromiseResolver<'scope> {
        PromiseResolver(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::PromiseResolver>> for PromiseResolver<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::PromiseResolver> {
        self.0
    }
}

impl<'scope> From<PromiseResolver<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: PromiseResolver<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> From<PromiseResolver<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: PromiseResolver<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> PromiseResolver<'scope> {
    // TODO return an error.
    /// Create a new [`PromiseResolver`], along with an associated promise in pending state.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>) -> PromiseResolver<'scope> {
        v8::PromiseResolver::new(scope.unseal())
            .expect("TODO")
            .seal()
    }

    /// Returns the associated promise.
    #[inline(always)]
    pub fn promise(&self, scope: &mut ValueScope<'scope>) -> Promise<'scope> {
        self.0.get_promise(scope.unseal()).seal()
    }

    // TODO what does the return value even mean?
    /// Resolve the associated promise with a given value.
    ///
    /// Ignored if the promise is no longer pending.
    #[inline(always)]
    pub fn resolve(&self, scope: &mut ValueScope<'scope>, value: Value<'scope>) -> Option<bool> {
        self.0.resolve(scope.unseal(), value.unseal())
    }

    // TODO what does the return value even mean?
    /// Reject the associated promise with a given value.
    /// Ignored if the promise is no longer pending.
    #[inline(always)]
    pub fn reject(&self, scope: &mut ValueScope<'scope>, value: Value<'scope>) -> Option<bool> {
        self.0.reject(scope.unseal(), value.unseal())
    }
}
