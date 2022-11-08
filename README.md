# Kopi

Kopi is a small abstraction to easily and safely embed an ECMAScript execution engine inside a Rust based application.
It uses the V8 execution engine to provide JITed ECMAScript execution and aims to use very few dependencies.

## Features

 * `getrandom` - V8 expects the implementation to provide a good, strong 
                 entropy source, or else it might use weak entropy
                 sources. This features uses the `getrandom` crate to
                 provide a strong entropy source provided by the operating
                 system.

## Testing

Some tests need a ICU data file placed inside the project root directory
(`icudt71l.dat` for little endian or `icudt71b.dat` for big endian systems).

You need to download a ICU version 71 data file from the ICU project [release page](https://github.com/unicode-org/icu/releases).

## Roadmap

Sooner than later following functionality will be added:

 * Module loading
 * Embedded types
 * Fallible fastcall functions
 * Optional serde support

We're not yet totally sure how the API for async should look. 

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
