pub use v8::PromiseState;

use super::{Object, Seal, Unseal, Value, ValueScope};

/// A Promise object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Promise<'scope>(pub(crate) v8::Local<'scope, v8::Promise>);

impl<'scope> Seal<Promise<'scope>> for v8::Local<'scope, v8::Promise> {
    #[inline(always)]
    fn seal(self) -> Promise<'scope> {
        Promise(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Promise>> for Promise<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Promise> {
        self.0
    }
}

impl<'scope> From<Promise<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Promise<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Promise<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Promise>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<Promise<'scope>> for Object<'scope> {
    #[inline(always)]
    fn from(value: Promise<'scope>) -> Self {
        Object(value.0.into())
    }
}

impl<'scope> Promise<'scope> {
    // TODO rework the promise API once the async story is well defined.

    /// Returns the current state of the promise.
    #[inline(always)]
    pub fn state(&self) -> PromiseState {
        self.0.state()
    }

    /// Returns `true` if the promise has at least one derived promise, and
    /// therefore resolve/reject handlers (including default handler).
    #[inline(always)]
    pub fn has_handler(&self) -> bool {
        self.0.has_handler()
    }

    /// Returns the result of the promise if not pending.
    #[inline(always)]
    pub fn result(&self, scope: &mut ValueScope<'scope>) -> Value<'scope> {
        self.0.result(scope.unseal()).seal()
    }
}
