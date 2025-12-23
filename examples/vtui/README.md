# vTUI: VMware VM visualization for the terminal

vTUI is a tool that allows you to browse the vCenter inventory in the terminal. It is a simple tool
that uses the VMware API to monitor vCenter inventory and render it in a terminal window.

vTUI's main purpose is to demonstrate how to use the vim_rs library to interact with the VMware API
in a Text User Interface (TUI) application.

vTUI uses the `PropertyCollector` API to retrieve inventory object from the vCenter server. It then
displays the VMs in a terminal window using the Ratatui library.

## Features

- Visualize vCenter inventory directly in your terminal.
- Real-time inventory updates using the PropertyCollector API.
- Search for specific objects using "/"
- Navigate to related objects using shortcuts (v)m, (n)etwork, (h)ost, (d)atastore
- Dive into the details of a VM, Host etc.
- Save an object details to a file
- Go back to the previous view using Backspace
- Browse the Cluster, Host, VM, Network and Datastore inventory
- TUI built with the Ratatui library for a smooth user experience.
- Clean and minimalistic design suitable for server environments.
- Logging support for debugging and monitoring.

## Installation

Ensure you have Rust 1.85 installed.

Then configure vTUI via environment variables **or** a local `.env` file.

### Option A: `.env` file (recommended)

Create `examples/vtui/.env`:

```bash
VIM_SERVER=https://your-vcenter.sdk
VIM_USERNAME=administrator@vsphere.local
VIM_PASSWORD=your-password
VIM_INSECURE=true
LOG_LEVEL=info
```

### Option B: environment variables

Set the following environment variables:
- `VIM_SERVER` - FQDN of a vCenter server (version 8.0.2 or later).
- `VIM_USERNAME` - Username for vCenter authentication.
- `VIM_PASSWORD` - Password for vCenter authentication.
- `VIM_INSECURE` - Set to `true` to ignore SSL certificate validation (not recommended for production).
- `LOG_LEVEL` - Set to `debug` or `trace` for verbose logging (optional).

## Usage

To run vTUI, run the following command:

```bash
cargo run --bin vtui
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your
improvements.
