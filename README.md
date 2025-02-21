# yaws - an environment-neutral Web server capability

[![CI](https://github.com/pinkforest/yaws/actions/workflows/CI.yml/badge.svg)](https://github.com/yaws-rs/yaws/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/yaws.svg)](https://crates.io/crates/yaws)
[![Docs](https://docs.rs/yaws/badge.svg)](https://docs.rs/yaws)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.60.0-blue)

HTTP is everywhere, from embedded to big iron. YAWS is a harmonized environment-neutral open source HTTP server capability able to be provided through modern WebAssembly, io_uring, microkernel, RISC-V, embedded runtimes often without POSIX, standard library or operating system support. YAWS democratizes HTTP by providing HTTP server capability in these domains, allowing everyone to integrate modern HTTP interface safely and securely into where ever and whatever they build that requires a HTTP server capability.

## yaws Binary

Running yaws requires a runtime, we provide the below examples:

Lunatic Flavor:

$ `RUSTFLAGS="--cfg yaws_flavor=\"lunatic\"" cargo run --bin yaws --target wasm32-wasi`

io_uring Flavor:

$ `RUSTFLAGS="--cfg yaws_flavor=\"io_uring\"" cargo run --bin yaws`

## yaws Library

yaws has two flavors available controlled via `cfg(yaws_flawor)`

- [ ] `cfg(yaws_flavor = "io_uring")` - `yaws-run-uring` - TODO
- [ ] `cfg(yaws_flavor = "lunatic")` - `yaws-run-lunatic` - TODO

### yaws Flavor: io_uring

This will use io_uring flavor of Yaws Host side

Goal: Fast I/O

### yaws Flavor: lunatic

This will use the Lunatic flavor of Yaws inside Lunatic VM

Goal: Run inside Lunatic VM

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

