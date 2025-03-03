# vim_rs usage examples
This folder contains examples of how to use the vim_rs library.

The examples are arrange in a Rust workspace to facilitate faster build time and thus faster development and testing.

To build the examples, run the following command:
```bash
cargo build
```
To run the examples you need to set the following environment variables:
- `VIM_SERVER` - FQDN of a vCenter server version 8.0.2 or later.
- `VIM_USERNAME` - Username to authenticate with the vCenter server.
- `VIM_PASSWORD` - Password to authenticate with the vCenter server.
- `RUST_LOG` - Log level for the examples. Set to `debug` to see all example logs. Set to `trace` to see all communication logs.

To run the examples, run the following command:
```bash
cargo run --bin <example_name>
```
Where `<example_name>` is the name of the example you want to run.

The examples are:
- `eventster` - retrieves events from the vCenter server.
- `mac_monitor` - monitors the MAC and IP addresses of a VM over time using PropertyCollector::WaitForUpdates` API.
- `perf_metrics` - retrieves Virtual Machine performance metrics from the vCenter server.
- `property_collector` - retrieves Virtual Machine properties from the vCenter server using container view to select all virtual machines in the root folder.