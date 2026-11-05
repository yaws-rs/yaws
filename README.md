# yaws - an environment-neutral Web server capability

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.60.0-blue)

HTTP is everywhere, from embedded to big iron.

Yaws is a harmonized environment-neutral open source HTTP server capability.

Yaws through it's different flabors is able to be provided through modern WebAssembly, io_uring, microkernel, RISC-V, embedded runtimes often without POSIX, standard library or operating system support.

Yaws democratizes HTTP by providing HTTP server capability in these domains, allowing everyone to integrate modern HTTP interface safely and securely into where ever and whatever they build that requires a HTTP server capability.

See [Yaws Book](https://yaws-rs.github.io/book) for more.

## Core

## H11Spec

Repository: [yaws/h11spec](https://github.com/yaws-rs/h11spec)
[![CI](https://github.com/yaws/h11spec/actions/workflows/CI.yml/badge.svg)](https://github.com/yaws-rs/h11spec/actions/workflows/CI.yml)

| Repository | Crate | Docs |
| :---       | :---  | :--- |
| [yaws-rs/h11spec/h11types](https://github.com/yaws-rs/h11spec/tree/main/h11types) | [![Crates.io](https://img.shields.io/crates/v/h11types.svg)](https://crates.io/crates/h11types) | [![Docs](https://docs.rs/h11types/badge.svg)](https://docs.rs/h11types) |
| [yaws-rs/h11spec/blueprint](https://github.com/yaws-rs/h11spec/tree/main/blueprint) | [![Crates.io](https://img.shields.io/crates/v/blueprint-h11spec.svg)](https://crates.io/crates/blueprint-h11spec) | [![Docs](https://docs.rs/blueprint-h11spec/badge.svg)](https://docs.rs/blueprint-h11spec) |

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

