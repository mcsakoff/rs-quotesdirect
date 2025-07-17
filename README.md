# CQG Quotes Direct (FIX FAST) in Rust
[![Crates.io](https://img.shields.io/crates/v/quotesdirectlib?style=flat-square)](https://crates.io/crates/quotesdirectlib)
[![Build Status](https://img.shields.io/github/actions/workflow/status/mcsakoff/rs-quotesdirect/rust.yml?branch=main&style=flat-square)](https://github.com/mcsakoff/rs-quotesdirect/actions/workflows/rust.yml?query=branch%3Amain)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

**Quotes Direct API** provides fast and reliable market data feeds using the industry-standard FIX formats.

The repository contains:

- `rs-quotesdirectlib` - Quotes Direct API library.
- `sds-client` - Example application of Security Definition Server client.
- `ffs-client` - Example application of data feed client.

## Quotes Direct API library

This library provides structures, functions and methods for:

- reading TCP and UDP packets
- parsing incoming FAST messages
- generating outgoing FIX messages

## Quotes Direct SDS Client Example

Example implementation of the CQG [Quotes Direct](https://help.cqg.com/apihelp/#!Documents/quotesdirectfixfast.htm) Security Definition Service (SDS) client.

### How to run

Obtain your username/password and FeedIDs from [sales@cqg.com](mailto:sales@cqg.com).

Edit the configuration file `examples/sds-client.yaml` and run the following command:

```shell
$ cd examples
$ cargo run --bin sds-client
```

## Quotes Direct Data Feed Client Example

### How to run

Get multicast address and port from Security Definition Service.

Edit the configuration file `examples/ffs-client.yaml` and run the following command:

```shell
$ cd examples
$ cargo run --bin ffs-client
```

## License

This project is licensed under the [MIT license](LICENSE).
