# Kopi

Kopi is a small abstraction to easily and safely embed an ECMAScript execution
engine inside a Rust based application. It uses the V8 execution engine to
provide JITed ECMAScript execution and aims to use very few dependencies.

## Status ##

This crate is still in the design phase and in large parts unfinished.
Once it reaches a certain stage of maturity it will be published to
cargo.io.

## Features

 * `getrandom` - V8 expects the implementation to provide a good, strong 
                 entropy source, or else it might use a weak entropy
                 sources. This features uses the `getrandom` crate to
                 provide a strong entropy source provided by the operating
                 system.
 * (1) `serde` - Implements the `FromValue` and `IntoValue` traits for every
                 type that implements the `serde::Serialize` and
                 `serde::Deserialize` traits.

(1) Not implemented yet

## Example

```rust
use kopi::*;

fastcall_function! {
    fn mul(x: f64, y: f64) -> f64 {
        x * y
    }
}

initialize_v8(InitializationOptions::default());

let mut extension = Extension::new(None);
extension.add_function("madd", move |(a, b, c): (f32, f32, f32)| a + (b * c));
extension.add_fastcall_function("mul", mul);

let mut runtime = Runtime::new(
    RuntimeOptions {
        extensions: vec![extension],
        ..Default::default()
    },
    (),
)
.expect("Can't create runtime");

let val: i32 = runtime
    .execute("madd(10, 5, 6)")
    .expect("Can't execute code");

assert_eq!(val, 40);

let val: i32 = runtime.execute("mul(10, 20)").expect("Can't execute code");

assert_eq!(val, 200);
```

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
