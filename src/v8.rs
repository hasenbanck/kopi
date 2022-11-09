//! Structures to work with V8.

pub(crate) use v8::{
    fast_api, icu, new_default_platform, new_single_threaded_default_platform, Context,
    ContextScope, CreateParams, External, Function, FunctionCallback, FunctionCallbackArguments,
    FunctionTemplate, Global, Isolate, MapFnTo, ObjectTemplate, OwnedIsolate, ReturnValue, Script,
    TryCatch, V8,
};
pub use v8::{
    null, undefined, Array, ArrayBuffer, ArrayBufferView, BigInt, BigInt64Array, BigIntObject,
    BigUint64Array, Boolean, Data, DataView, Date, Exception, Float32Array, Float64Array,
    HandleScope, Int16Array, Int32, Int32Array, Int8Array, Integer, Local, NewStringType, Number,
    Object, String, Uint16Array, Uint32, Uint32Array, Uint8Array, Value,
};

static MAX_STRING_SIZE: once_cell::sync::Lazy<usize> =
    once_cell::sync::Lazy::new(v8::String::max_length);

/// Utility function to safely create string. Will truncate string if they are too long.
pub fn create_string<'scope, S: AsRef<str>>(
    scope: &mut HandleScope<'scope, ()>,
    string: S,
) -> Local<'scope, String> {
    let data = string.as_ref().as_bytes();
    let max_length = usize::min(*MAX_STRING_SIZE, data.len());
    String::new_from_utf8(scope, &data[..max_length], v8::NewStringType::Normal)
        .expect("String is too large for V8")
}

/// Utility function to safely create a string from static string data. Will truncate string if they are too long.
pub fn create_string_from_static<'scope>(
    scope: &mut HandleScope<'scope>,
    string: &'static str,
) -> Local<'scope, String> {
    let data = string.as_bytes();
    let max_length = usize::min(*MAX_STRING_SIZE, data.len());
    String::new_external_onebyte_static(scope, &data[..max_length])
        .expect("String is too large for V8")
}
