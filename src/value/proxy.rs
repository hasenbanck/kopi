use super::{Object, Seal, Unseal, Value, ValueScope};

/// A proxy object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Proxy<'scope>(pub(crate) v8::Local<'scope, v8::Proxy>);

impl<'scope> Seal<Proxy<'scope>> for v8::Local<'scope, v8::Proxy> {
    #[inline(always)]
    fn seal(self) -> Proxy<'scope> {
        Proxy(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Proxy>> for Proxy<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Proxy> {
        self.0
    }
}

impl<'scope> From<Proxy<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Proxy<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Proxy<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Proxy>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Proxy<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: Proxy<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> Proxy<'scope> {
    /// TODO what is the error case for this?
    /// Returns a new [`Proxy`].
    ///
    /// # Parameters
    /// * `target`:  The original object which you want to proxy.
    /// * `handler`: An object that defines which operations will be
    ///              intercepted and how to redefine intercepted operations.
    #[inline(always)]
    pub fn new(
        scope: &mut ValueScope<'scope>,
        target: Object<'scope>,
        handler: Object<'scope>,
    ) -> Proxy<'scope> {
        v8::Proxy::new(scope.unseal(), target.unseal(), handler.unseal())
            .expect("TODO")
            .seal()
    }

    /// Returns the handler of the proxy.
    #[inline(always)]
    pub fn handler(&self, scope: &mut ValueScope<'scope>) -> Value<'scope> {
        self.0.get_handler(scope.unseal()).seal()
    }

    /// Returns the target of the proxy.
    #[inline(always)]
    pub fn target(&self, scope: &mut ValueScope<'scope>) -> Value<'scope> {
        self.0.get_target(scope.unseal()).seal()
    }

    /// Returns `true` if the proxy is switched off.
    #[inline(always)]
    pub fn is_revoked(&self) -> bool {
        self.0.is_revoked()
    }

    /// Switches the proxy off.
    #[inline(always)]
    pub fn revoke(&self) {
        self.0.revoke()
    }
}
