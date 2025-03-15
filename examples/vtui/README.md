vTUI: VMware VM visualization for the terminal

vTUI is a tool that allows you to visualize VMware VMs in the terminal. It is a simple tool that
uses the VMware API to get the VMs and their status and then displays them in a terminal window.

vTUIs main purpose is to demonstrate how to use the vim_rs library to interact with the VMware API
in a Text User Interface (TUI) application.

vTUI uses the `PropertyCollector` API to get the VMs and their status from the vCenter server. It
then displays the VMs in a terminal window using the tui-rs library. The basic steps of using the
API are as follows:

1. Firstly after login a `View` is created for all VMs in the rootfolder using `ViewManager`
2. A filter is created in the `PropertyCollector` for the VMs using the `CreateFilter` method.
3. The `WaitForUpdates` method is used to get continuous updates about VMs and their status from the vCenter server.
4. The VMs are displayed in a terminal window using the tui-rs library.
5. Upon exit, the view, filter and session are destroyed to release server resources.

## Usage

You need to set up the following environment variables to use vTUI:

- `VIM_SERVER` - FQDN of a vCenter server version 8.0.2 or later.
- `VIM_USERNAME` - Username to authenticate with the vCenter server.
- `VIM_PASSWORD` - Password to authenticate with the vCenter server.

To run vTUI, run the following command:

```bash
cargo run --bin vtui
```
