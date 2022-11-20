use super::{Seal, String, Unseal, Value, ValueScope};

/// A stack trace.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct StackTrace<'scope>(v8::Local<'scope, v8::StackTrace>);

impl<'scope> Seal<StackTrace<'scope>> for v8::Local<'scope, v8::StackTrace> {
    #[inline(always)]
    fn seal(self) -> StackTrace<'scope> {
        StackTrace(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::StackTrace>> for StackTrace<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::StackTrace> {
        self.0
    }
}

impl<'scope> StackTrace<'scope> {
    /// Returns the original stack trace that was captured at the creation time of the given
    /// exception if available.
    #[inline(always)]
    pub fn exception_stack_trace(
        scope: &mut ValueScope<'scope>,
        exception: Value,
    ) -> Option<StackTrace<'scope>> {
        v8::Exception::get_stack_trace(scope.unseal(), exception.unseal()).map(|e| e.seal())
    }

    /// Returns the current execution stack trace with the given frame depth limit.
    #[inline(always)]
    pub fn current_stack_trace(
        scope: &mut ValueScope<'scope>,
        frame_limit: usize,
    ) -> Option<StackTrace<'scope>> {
        v8::StackTrace::current_stack_trace(scope.unseal(), frame_limit).map(|e| e.seal())
    }

    /// Returns the number of frames inside the strack trace.
    #[inline(always)]
    pub fn get_frame_count(&self) -> usize {
        self.0.get_frame_count()
    }

    /// Returns the stack frame of a stack trace at the particular index.
    #[inline(always)]
    pub fn get_stack_frame(
        &mut self,
        scope: &mut ValueScope<'scope>,
        index: usize,
    ) -> Option<StackFrame<'scope>> {
        self.0.get_frame(scope.unseal(), index).map(|sf| sf.seal())
    }
}

/// A stack frame.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct StackFrame<'scope>(v8::Local<'scope, v8::StackFrame>);

impl<'scope> Seal<StackFrame<'scope>> for v8::Local<'scope, v8::StackFrame> {
    #[inline(always)]
    fn seal(self) -> StackFrame<'scope> {
        StackFrame(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::StackFrame>> for StackFrame<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::StackFrame> {
        self.0
    }
}

impl<'scope> StackFrame<'scope> {
    /// Returns the 1-based number of the line for the associated function call.
    #[inline(always)]
    pub fn line_number(&self) -> usize {
        self.0.get_line_number()
    }

    /// Returns the 1-based column offset on the line for the associated function call.
    #[inline(always)]
    pub fn column(&self) -> usize {
        self.0.get_column()
    }

    /// Returns the id of the script for the function for this stack frame.
    #[inline(always)]
    pub fn script_id(&self) -> usize {
        self.0.get_script_id()
    }

    /// Returns the name of the resource that contains the script for the function
    /// for this stack frame.
    #[inline(always)]
    pub fn script_name(&self, scope: &mut ValueScope<'scope>) -> Option<String<'scope>> {
        self.0.get_script_name(scope.unseal()).map(|s| s.seal())
    }

    /// Returns the name or url of the resource that contains the script for the function for
    /// this stack frame.
    #[inline(always)]
    pub fn script_name_or_source_url(
        &self,
        scope: &mut ValueScope<'scope>,
    ) -> Option<String<'scope>> {
        self.0
            .get_script_name_or_source_url(scope.unseal())
            .map(|s| s.seal())
    }

    /// Returns the name of the function associated with this stack frame.
    #[inline(always)]
    pub fn function_name(&self, scope: &mut ValueScope<'scope>) -> Option<String<'scope>> {
        self.0.get_function_name(scope.unseal()).map(|s| s.seal())
    }

    /// Returns `true` when the associated function was compiled via a call to `eval()`.
    #[inline(always)]
    pub fn is_eval(&self) -> bool {
        self.0.is_eval()
    }

    /// Returns `true` when the associated function was called as a constructor via `new`.
    #[inline(always)]
    pub fn is_constructor(&self) -> bool {
        self.0.is_constructor()
    }

    /// Returns `true` when the associated functions was defined in wasm.
    #[inline(always)]
    pub fn is_wasm(&self) -> bool {
        self.0.is_wasm()
    }

    /// Returns `true` when the associated function was defined by the user.
    #[inline(always)]
    pub fn is_user_javascript(&self) -> bool {
        self.0.is_user_javascript()
    }
}
