Project demonstrating the consumption of event data from vCenter using Rust.

Before running the project you need a vCenter endpoint configured with the following environment
variables:

* VC_SERVER : vCenter server FQDN or IP
* VC_USERNAME : vCenter server user name e.g. "administrator@vsphere.local"
* VC_PASSWORD : vCenter Server password
* RUST_LOG : log level. For debug please use "trace"