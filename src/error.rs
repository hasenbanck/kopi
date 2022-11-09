//! Implements the errors that the crate can throw.

use std::fmt::Debug;

use crate::{
    value,
    value::{Exception, Local, Value},
};

/// Errors that the crate can throw.
#[derive(Debug)]
pub enum Error {
    /// A general type error (e.g. when type conversion failed or an unexpected tape in in argument
    /// or return value was encountered).
    Type(TypeError),
    /// The V8 engine was expected to be initialized before calling this functionality.
    V8NotInitialized,
    /// An EcmaScript error.
    EcmaScript(String),
    /// An implementation specific error occurred.
    Internal(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Type(err) => write!(f, "Type error: {}", err),
            Error::V8NotInitialized => write!(f, "V8 engine is not initialized"),
            Error::EcmaScript(msg) => write!(f, "ECMAScript error: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// A general type error (e.g. when type conversion failed or an unexpected tape in in argument
/// or return value was encountered).
#[derive(Debug)]
pub struct TypeError {
    /// The message of the type error.
    pub msg: &'static str,
    /// The string representation of the source value.
    pub source: String,
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.msg, self.source)
    }
}

impl From<TypeError> for String {
    fn from(te: TypeError) -> Self {
        format!("{}, {}", te.msg, &te.source)
    }
}

/// Shortcut to create a type error.
pub fn create_type_error(
    msg: &'static str,
    scope: &mut value::HandleScope,
    value: &Value,
) -> TypeError {
    let source = value.to_rust_string_lossy(scope);
    TypeError { msg, source }
}

/// Creates an error from an exception.
pub(crate) fn create_error_from_exception<'scope, T>(
    scope: &mut value::HandleScope<'scope>,
    exception: Option<Local<Value>>,
) -> Result<T, Error> {
    let Some(exception) = exception else {
        return Err(Error::Internal("Exception was not set".to_string()));
    };

    let msg = Exception::create_message(scope, exception);

    // TODO create a proper EcmaScript error from the Local<Message> (lines etc.).
    let message_string = msg.get(scope).to_rust_string_lossy(scope);

    let line_number = msg.get_line_number(scope).unwrap_or(0);

    let formatted = format!("'{}' in line: {}", message_string, line_number);

    Err(Error::EcmaScript(formatted))
}
