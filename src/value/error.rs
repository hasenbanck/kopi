use super::{String, ValueScope};
use crate::value::{Seal, Unseal, Value};

/// Holds the constructors for error values.
pub struct Error;

impl Error {
    /// Creates a new error.
    #[inline(always)]
    pub fn new_error<'scope>(scope: &mut ValueScope<'scope>, message: String) -> Value<'scope> {
        v8::Exception::error(scope.unseal(), message.unseal()).seal()
    }

    /// Creates a new range error.
    #[inline(always)]
    pub fn new_range_error<'scope>(
        scope: &mut ValueScope<'scope>,
        message: String,
    ) -> Value<'scope> {
        v8::Exception::range_error(scope.unseal(), message.unseal()).seal()
    }

    /// Creates a new reference error.
    #[inline(always)]
    pub fn new_reference_error<'scope>(
        scope: &mut ValueScope<'scope>,
        message: String,
    ) -> Value<'scope> {
        v8::Exception::reference_error(scope.unseal(), message.unseal()).seal()
    }

    /// Creates a new syntax error.
    #[inline(always)]
    pub fn new_syntax_error<'scope>(
        scope: &mut ValueScope<'scope>,
        message: String,
    ) -> Value<'scope> {
        v8::Exception::syntax_error(scope.unseal(), message.unseal()).seal()
    }

    /// Creates a new type error.
    #[inline(always)]
    pub fn new_type_error<'scope>(
        scope: &mut ValueScope<'scope>,
        message: String,
    ) -> Value<'scope> {
        v8::Exception::type_error(scope.unseal(), message.unseal()).seal()
    }

    // TODO use value::Message
    /// Creates an error message for the given exception.
    #[inline(always)]
    pub fn new_message<'scope>(
        scope: &mut ValueScope<'scope>,
        exception: Value<'scope>,
    ) -> v8::Local<'scope, v8::Message> {
        v8::Exception::create_message(scope.unseal(), exception.unseal())
    }
}
