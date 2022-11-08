//! Safe API to embed an ECMAScript engine.
//!
//! Uses the V8 runtime to let the user embed ECMAScript in their application.
//!
//! # Example
//!
//! ```rust
//! use kopi::*;
//!
//! initialize_v8(InitializationOptions::default());
//!
//! let mut extension = Extension::new(None);
//! extension.add_function("madd", move |(a, b, c): (f32, f32, f32)| a + (b * c));
//!
//! let mut runtime = Runtime::new(
//!     RuntimeOptions {
//!         extensions: vec![extension],
//!         ..Default::default()
//!     },
//!     (),
//! )
//! .expect("Can't create runtime");
//!
//! let val: i32 = runtime
//!     .execute("madd(10, 5, 6)")
//!     .expect("Can't execute code");
//!
//! assert_eq!(val, 40);
//! ```
#![deny(missing_docs)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::unwrap_used)]

mod macros;

#[doc(hidden)]
pub mod _macros {
    //! These exports are needed by the `static_function` and `fastcall_function` macro.
    //! They are not supposed to be used by the user.

    pub use once_cell::sync::Lazy;
    pub use v8::{
        fast_api::{CType, FastApiCallbackOptions, FastFunction, Type},
        External, FunctionCallback, FunctionCallbackArguments, HandleScope, Local, MapFnTo, Object,
        ReturnValue,
    };

    pub use crate::{
        extension::{get_argument, set_result},
        runtime::STATE_DATA_SLOT,
    };
}

pub mod error;
mod extension;
mod runtime;
mod value;

#[cfg(target_pointer_width = "16")]
compile_error!("16 bit systems are not supported");

use std::{
    fmt::{Display, Formatter},
    num::NonZeroU32,
};

pub use extension::{
    Extension, FastcallFunction, FunctionArguments, FunctionWithStateArguments, StaticFunction,
};
pub use runtime::{Runtime, RuntimeOptions};
pub use value::{
    from_value_impl::*, into_value_impl::*, FastcallArgument, FastcallReturnValue, FromValue,
    IntoValue, Value, ValueBuilder,
};

const DEFAULT_V8_FLAGS: &str = "--turbo_fast_api_calls";

#[cfg(target_endian = "little")]
const ICU_FILE_NAME: &str = "icudt71l.dat";

#[cfg(target_endian = "big")]
const ICU_FILE_NAME: &'static str = "icudt71b.dat";

static V8_INITIALIZATION: std::sync::Once = std::sync::Once::new();

static MAX_STRING_SIZE: once_cell::sync::Lazy<usize> =
    once_cell::sync::Lazy::new(v8::String::max_length);

/// Represents the version number of the V8 engine.
#[derive(Copy, Clone)]
pub struct Version {
    /// The major milestone version (based on the chromium release).
    pub milestone_major: u32,
    /// The minor milestone version (based on the chromium release).
    pub milestone_minor: u32,
    /// Number that is increased for each `LKGR`.
    pub revision: u32,
    /// Number that is increased for each included backport patch.
    pub patch: u32,
}

impl TryFrom<&str> for Version {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut version_numbers = value.split('.');

        let Some(Ok(milestone_major)) = version_numbers.next().map(|s| s.parse::<u32>()) else {
            return Err(());
        };
        let Some(Ok(milestone_minor)) = version_numbers.next().map(|s| s.parse::<u32>()) else {
            return Err(());
        };
        let Some(Ok(revision)) = version_numbers.next().map(|s| s.parse::<u32>()) else {
            return Err(());
        };
        let Some(Ok(patch)) = version_numbers.next().map(|s| s.parse::<u32>()) else {
            return Err(());
        };

        Ok(Self {
            milestone_major,
            milestone_minor,
            revision,
            patch,
        })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.milestone_major, self.milestone_minor, self.revision, self.patch
        )
    }
}

/// Returns the version of the V8 engine.
pub fn version_v8() -> Version {
    v8::V8::get_version()
        .try_into()
        .expect("V8 version string is not of the expected format")
}

/// Configures the initialization of the V8 engine.
pub struct InitializationOptions {
    /// Configures if the V8 engine should run single threaded or multi threaded mode.
    pub execution_model: ExecutionModel,
    /// Optional ICU data used for internationalization (icudt71*.dat).
    /// Use [`prepare_icu_data`] to properly align the data.
    ///
    /// If no data is given, we try to load the file from the work folder.
    pub icu_data: Option<&'static [Aligned16]>,
    /// The default locale used for internationalization.
    ///
    /// Must be a valid locale based on ECMA402.
    pub default_locale: String,
}

impl Default for InitializationOptions {
    fn default() -> Self {
        Self {
            execution_model: ExecutionModel::MultiThreaded(None),
            icu_data: None,
            default_locale: "en-US".to_string(),
        }
    }
}

/// Configures if the V8 engine should run in single threaded mode or with a thread pool for
/// background tasks. Background tasks include internal tasks like the GC and code compilation.
#[derive(Clone, Copy)]
pub enum ExecutionModel {
    /// The engine will be single threaded.
    SingleThreaded,
    /// The engine will be multi threaded.
    ///
    /// The option defines the number of worker threads to allocate for background tasks.
    /// If set to `None`, V8 will decide the number itself based on the CPU core count.
    MultiThreaded(Option<NonZeroU32>),
}

/// Initialized the V8 engine. Needs to be called once before creating a runtime.
///
/// Subsequent calls will result in a NOP.
///
/// # Panics
///
/// Panics if the V8 engine could not be initialized.
pub fn initialize_v8(options: InitializationOptions) {
    V8_INITIALIZATION.call_once(|| {
        let (flags, platform) = match options.execution_model {
            ExecutionModel::SingleThreaded => {
                let flags = format!("{} {}", DEFAULT_V8_FLAGS, "--single-threaded");
                let platform = v8::new_single_threaded_default_platform(false);

                (flags, platform)
            }
            ExecutionModel::MultiThreaded(thread_pool_size) => {
                let flags = String::from(DEFAULT_V8_FLAGS);
                let thread_pool_size = thread_pool_size.map(|t| t.get()).unwrap_or(0);
                let platform = v8::new_default_platform(thread_pool_size, false);

                (flags, platform)
            }
        };

        load_icu(&options);

        v8::icu::set_default_locale(options.default_locale.as_ref());

        #[cfg(feature = "getrandom")]
        {
            #[inline]
            fn get_entropy(data: &mut [u8]) -> bool {
                getrandom::getrandom(data).is_ok()
            }

            #[cfg(feature = "getrandom")]
            v8::V8::set_entropy_source(get_entropy);
        }

        v8::V8::set_flags_from_string(flags.as_ref());

        v8::V8::initialize_platform(platform.make_shared());
        v8::V8::initialize();
    });
}

fn load_icu(options: &InitializationOptions) {
    // Either use the provided ICU file, or try to load a local ICU file.
    let icu_data = match options.icu_data {
        Some(icu_data) => Some(icu_data),
        None => match std::fs::read(ICU_FILE_NAME) {
            Ok(icu_data) => {
                let icu_data = prepare_icu_data(&icu_data).expect("Invalid ICU data");
                Some(icu_data)
            }
            Err(_) => None,
        },
    };

    if let Some(icu_data) = icu_data {
        assert_eq!(
            icu_data.as_ptr() as usize % 16,
            0,
            "ICU data is not aligned to 16 bytes"
        );

        // SAFETY: We know that `Aligned16` is a multiple of byte alignment.
        let byte_data = unsafe {
            std::slice::from_raw_parts(
                icu_data.as_ptr() as *const u8,
                std::mem::size_of_val(icu_data),
            )
        };

        if let Err(err_code) = v8::icu::set_common_data_71(byte_data) {
            panic!("ICU could not be initialized: {}", err_code)
        }
    }
}

/// Data aligned to 16 byte.
#[repr(C, align(16))]
pub struct Aligned16([u8; 16]);

/// Helper function to properly align the ICU data to 16 bytes.
///
/// It will leak the data to give it the expected static lifetime.
pub fn prepare_icu_data<D: AsRef<[u8]>>(data: D) -> Option<&'static [Aligned16]> {
    let data = data.as_ref();
    if data.len() % 16 != 0 {
        None
    } else {
        let aligned: Vec<Aligned16> = data
            .chunks_exact(16)
            .map(|chunk: &[u8]| -> Aligned16 {
                let chunk: [u8; 16] = chunk.try_into().expect("chunk size is not 16 bytes");
                Aligned16(chunk)
            })
            .collect();
        Some(aligned.leak())
    }
}

/// Utility function to safely create string. Will truncate string if they are too long.
pub(crate) fn create_string<'scope, S: AsRef<str>>(
    scope: &mut v8::HandleScope<'scope, ()>,
    string: S,
) -> v8::Local<'scope, v8::String> {
    let data = string.as_ref().as_bytes();
    let max_length = usize::min(*MAX_STRING_SIZE, data.len());
    v8::String::new_from_utf8(scope, &data[..max_length], v8::NewStringType::Normal)
        .expect("String is too large for V8")
}

/// Utility function to safely create a string from static string data. Will truncate string if they are too long.
pub(crate) fn create_string_from_static<'scope>(
    scope: &mut v8::HandleScope<'scope>,
    string: &'static str,
) -> v8::Local<'scope, v8::String> {
    let data = string.as_bytes();
    let max_length = usize::min(*MAX_STRING_SIZE, data.len());
    v8::String::new_external_onebyte_static(scope, &data[..max_length])
        .expect("String is too large for V8")
}

#[cfg(test)]
mod test {
    use crate::{initialize_v8, version_v8, InitializationOptions, Runtime, RuntimeOptions};

    #[test]
    fn test_version_v8() {
        let version = version_v8();
        assert!(version.milestone_major >= 10);
    }

    // For this test to run we need an ICU file in the root folder.
    #[test]
    fn test_icu() {
        initialize_v8(InitializationOptions::default());

        let mut runtime =
            Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        let formatted_value: String = runtime
            .execute(
                r#"
const value = 200;
const formattedValue = new Intl.NumberFormat("de-DE", {
  style: "currency",
  currency: "EUR"
}).format(value);
// We replace the non breaking space with a normal space.
formattedValue.replace(/\s/g,' ')
            "#,
            )
            .expect("Can't execute script");

        assert_eq!(formatted_value, "200,00 €");
    }
}
