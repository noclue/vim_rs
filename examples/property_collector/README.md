This project contains basic tests and demonstrations how to use the VIM bindings.

Before running the api_tests you need a vCenter endpoint configured with the following environment
variables:

* VIM_SERVER : vCenter server FQDN or IP
* VIM_USERNAME : vCenter server user name e.g. "administrator@vsphere.local"
* VIM_PASSWORD : vCenter Server password
* RUST_LOG : log level. For debug please use "trace"