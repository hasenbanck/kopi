use super::{Seal, StackTrace, String, Unseal, Value, ValueScope};

/// A error message.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Message<'scope>(pub(crate) v8::Local<'scope, v8::Message>);

impl<'scope> Seal<Message<'scope>> for v8::Local<'scope, v8::Message> {
    #[inline(always)]
    fn seal(self) -> Message<'scope> {
        Message(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Message>> for Message<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Message> {
        self.0
    }
}

impl<'scope> Message<'scope> {
    /// Returns the string of the message.
    #[inline(always)]
    pub fn get(&self, scope: &mut ValueScope<'scope>) -> String<'scope> {
        self.0.get(scope.unseal()).seal()
    }

    /// Returns the exception stack trace.
    ///
    /// By default stack traces are not captured for uncaught exceptions.
    /// Can be activated by setting `capture_stack_trace_for_uncaught_exceptions` of
    /// the [`crate::RuntimeOptions`] to some value.
    #[inline(always)]
    pub fn stack_trace(&self, scope: &mut ValueScope<'scope>) -> Option<StackTrace<'scope>> {
        self.0.get_stack_trace(scope.unseal()).map(|s| s.seal())
    }

    /// Returns the source line content where the error originates.
    #[inline(always)]
    pub fn source_line(&self, scope: &mut ValueScope<'scope>) -> Option<String<'scope>> {
        self.0.get_source_line(scope.unseal()).map(|s| s.seal())
    }

    /// Returns the resource name for the script from where the function causing
    /// the error originates.
    #[inline(always)]
    pub fn script_resource_name(&self, scope: &mut ValueScope<'scope>) -> Option<Value<'scope>> {
        self.0
            .get_script_resource_name(scope.unseal())
            .map(|s| s.seal())
    }

    /// Returns the line number, 1-based, of the line where the error occurred.
    #[inline(always)]
    pub fn line_number(&self, scope: &mut ValueScope<'scope>) -> Option<usize> {
        self.0.get_line_number(scope.unseal())
    }

    /// Returns the index within the script of the first character where the error occurred.
    #[inline(always)]
    pub fn start_position(&self) -> i32 {
        self.0.get_start_position()
    }

    /// Returns the index within the script of the last character where the error occurred.
    #[inline(always)]
    pub fn end_position(&self) -> i32 {
        self.0.get_end_position()
    }

    /// Returns the WASM function index where the error occurred. Returns `-1` if
    /// message is not from a WASM script.
    #[inline(always)]
    pub fn wasm_function_index(&self) -> i32 {
        self.0.get_wasm_function_index()
    }

    /// Returns the error level of the message.
    #[inline(always)]
    pub fn error_level(&self) -> i32 {
        self.0.error_level()
    }

    /// Returns the index within the line of the first character where the error occurred.
    #[inline(always)]
    pub fn start_column(&self) -> usize {
        self.0.get_start_column()
    }

    /// Returns the index within the line of the last character where the error occurred.
    #[inline(always)]
    pub fn end_column(&self) -> usize {
        self.0.get_end_column()
    }

    /// Returns `true` if the message is shared across different origins.
    #[inline(always)]
    pub fn is_shared_cross_origin(&self) -> bool {
        self.0.is_shared_cross_origin()
    }

    /// Returns `true` if the message is opaque.
    #[inline(always)]
    pub fn is_opaque(&self) -> bool {
        self.0.is_opaque()
    }
}
