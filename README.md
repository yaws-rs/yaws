# yaws

[![CI](https://github.com/pinkforest/yaws/actions/workflows/CI.yml/badge.svg)](https://github.com/yaws-rs/yaws/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/yaws.svg)](https://crates.io/crates/yaws)
[![Docs](https://docs.rs/yaws/badge.svg)](https://docs.rs/yaws)
[![Deps](https://deps.rs/repo/github/pinkforest/yaws/status.svg)](https://deps.rs/repo/github/yaws-rs/yaws)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.60.0-blue)

Hi I'm Yaws and I swim where the hyper doesn't

For now I am just trying out some tricks..

## Run

Lunatic Flavor:

$ `RUSTFLAGS="--cfg yaws_flavor=\"lunatic\"" cargo run yaws`

io_uring Flavor:

$ `RUSTFLAGS="--cfg yaws_flavor=\"io_uring\"" cargo run yaws -- io_uring`

## Tricks

* Lunatic guest impl can co-exist with Host io_uring / I/O Rings impl.
* TCP_NOTSENT_LOWAT - https://lwn.net/Articles/560082/
* h1/2spec sans-io - https://github.com/brettcannon/sans-io/blob/main/how-to-sans-io.rst
* gRPC + no-serialization hand-off

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

## yaws Spec Compliance

yaws aims to be h1 and h2 spec compliant

- [ ] `yaws-spec-h1` - TODO
- [ ] `yaws-spec-h2` - TODO

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

