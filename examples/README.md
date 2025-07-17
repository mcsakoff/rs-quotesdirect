# Quotes Direct API library examples
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

## Quotes Direct SDS Client Example

Example implementation of the [CQG Quotes Direct](https://help.cqg.com/apihelp/#!Documents/quotesdirectfixfast.htm) Security Definition Service (SDS) client.

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
