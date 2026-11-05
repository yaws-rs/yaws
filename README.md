# yaws - an environment-neutral Web server capability

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.60.0-blue)

HTTP is everywhere, from embedded to big iron.

Yaws is a harmonized environment-neutral open source HTTP server capability.

Yaws through it's different flabors is able to be provided through modern WebAssembly, io_uring, microkernel, RISC-V, embedded runtimes often without POSIX, standard library or operating system support.

Yaws democratizes HTTP by providing HTTP server capability in these domains, allowing everyone to integrate modern HTTP interface safely and securely into where ever and whatever they build that requires a HTTP server capability.

See [Yaws Book](https://yaws-rs.github.io/book) for more.

## Blueprint

[![CI](https://github.com/yaws-rs/blueprint/actions/workflows/CI.yml/badge.svg)](https://github.com/yaws-rs/blueprint/actions/workflows/CI.yml) [yaws/blueprint](https://github.com/yaws-rs/blueprint)

| Repository | Crate | Docs | Description |
| :---       | :---  | :--- | :---        |
| [yaws-rs/blueprint](https://github.com/yaws-rs/blueprint) | [![Crates.io](https://img.shields.io/crates/v/blueprint.svg)](https://crates.io/crates/blueprint) | [![Docs](https://docs.rs/blueprint/badge.svg)](https://docs.rs/blueprint) | BluePrint & Orbit definitions |

## TLS (Transport Layer Security)

[![CI](https://github.com/yaws-rs/tls/actions/workflows/CI.yml/badge.svg)](https://github.com/yaws-rs/tls/actions/workflows/CI.yml) [yaws/tls](https://github.com/yaws-rs/tls)

| Repository | Crate | Docs | Description |
| :---       | :---  | :--- | :---        |
| [yaws-rs/tls/ytls-blueprint](https://github.com/yaws-rs/tls/tree/main/ytls-blueprint) | [![Crates.io](https://img.shields.io/crates/v/blueprint-ytls.svg)](https://crates.io/crates/blueprint-ytls) | [![Docs](https://docs.rs/blueprint-ytls/badge.svg)](https://docs.rs/blueprint-ytls) | yTLS ([github](github.com/yolotls/yolotls), [book](https://yolotls.github.io/book/)) BluePrint & Orbit implementations |
| [yaws-rs/tls/rustls-blueprint](https://github.com/yaws-rs/tls/tree/main/rustls-blueprint) | [![Crates.io](https://img.shields.io/crates/v/blueprint-rustls.svg)](https://crates.io/crates/blueprint-rustls) | [![Docs](https://docs.rs/blueprint-rustls/badge.svg)](https://docs.rs/blueprint-rustls) | RusTLS ([github](https://github.com/rustls/rustls)) BluePrint & Orbit implementations |

## H11Spec (HTTP/1.1)

[![CI](https://github.com/yaws-rs/h11spec/actions/workflows/CI.yml/badge.svg)](https://github.com/yaws-rs/h11spec/actions/workflows/CI.yml) [yaws/h11spec](https://github.com/yaws-rs/h11spec)

| Repository | Crate | Docs | Description |
| :---       | :---  | :--- | :---        |
| [yaws-rs/h11spec/h11types](https://github.com/yaws-rs/h11spec/tree/main/h11types) | [![Crates.io](https://img.shields.io/crates/v/h11types.svg)](https://crates.io/crates/h11types) | [![Docs](https://docs.rs/h11types/badge.svg)](https://docs.rs/h11types) | H11 Base types |
| [yaws-rs/h11spec/blueprint](https://github.com/yaws-rs/h11spec/tree/main/blueprint) | [![Crates.io](https://img.shields.io/crates/v/blueprint-h11spec.svg)](https://crates.io/crates/blueprint-h11spec) | [![Docs](https://docs.rs/blueprint-h11spec/badge.svg)](https://docs.rs/blueprint-h11spec) | H11 BluePrint & Orbit implementations |

## H2Spec (HTTP/2)

[![CI](https://github.com/yaws-rs/h2spec/actions/workflows/CI.yml/badge.svg)](https://github.com/yaws-rs/h2spec/actions/workflows/CI.yml) [yaws/h2spec](https://github.com/yaws-rs/h2spec)

| Repository | Crate | Docs | Description |
| :---       | :---  | :--- | :---        |
| [yaws-rs/h2spec/h2types](https://github.com/yaws-rs/h2spec/tree/main/h2types) | [![Crates.io](https://img.shields.io/crates/v/h2types.svg)](https://crates.io/crates/h2types) | [![Docs](https://docs.rs/h2types/badge.svg)](https://docs.rs/h2types) | H2 Base types |
| [yaws-rs/h2spec/blueprint](https://github.com/yaws-rs/h2spec/tree/main/blueprint) | [![Crates.io](https://img.shields.io/crates/v/blueprint-h2spec.svg)](https://crates.io/crates/blueprint-h2spec) | [![Docs](https://docs.rs/blueprint-h2spec/badge.svg)](https://docs.rs/blueprint-h2spec) | H2 BluePrint & Orbit implementations |

## Applications

| Repository | Crate | Docs | Description |
| :---       | :---  | :--- | :---        |
| [yaws-rs/h11spec/apps/rick_roll](https://github.com/yaws-rs/h11spec/tree/main/apps/rick_roll) | [![Crates.io](https://img.shields.io/crates/v/yaws-app-rick-roll.svg)](https://crates.io/crates/yaws-app-rick-roll) | [![Docs](https://docs.rs/yaws-app-rick-roll/badge.svg)](https://docs.rs/yaws-app-rick-roll) | Rick Roll as a Service |
| [yaws-rs/h11spec/apps/mirror](https://github.com/yaws-rs/h11spec/tree/main/apps/mirror) | [![Crates.io](https://img.shields.io/crates/v/yaws-app-mirror.svg)](https://crates.io/crates/yaws-app-mirror) | [![Docs](https://docs.rs/yaws-app-mirror/badge.svg)](https://docs.rs/yaws-app-mirror) | Bounces back your HTTP Post content |

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

