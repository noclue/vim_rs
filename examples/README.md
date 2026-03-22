# vim_rs usage examples

This workspace contains runnable examples for the `vim_rs` crate. The **`snippets`** package holds many small binaries; **`vtui`** is a separate terminal UI demo (unchanged layout).

Build everything from this directory:

```bash
cargo build
```

Run a specific example:

```bash
cargo run -p snippets --bin <example_name>
```

For `vtui`:

```bash
cargo run -p vtui
```

## Environment variables

Most snippets expect:

| Variable | Purpose |
|----------|---------|
| `VIM_SERVER` | vCenter or ESXi hostname or URL (passed to `ClientBuilder`) |
| `VIM_USERNAME` | Login user |
| `VIM_PASSWORD` | Login password |
| `RUST_LOG` | Log level (`info`, `debug`, `trace`, …). Use `trace` for full wire-level `vim_rs` logs. |
| `VIM_PROTOCOL` | Communication protocol. Valid values are 'auto', 'json', or 'soap'. Default is 'auto'. |

Optional / example-specific:

| Variable | Used by |
|----------|---------|
| `COMPUTE_RESOURCE` | `env_browser` — display name of a cluster or standalone host (`ComputeResource`) |
| `DATASTORE` | `retrieve_ds_hosts` — datastore id (e.g. `datastore-107001`) |
| `VM_INVENTORY_PATH` | `vm_ip`, `vm_rename`, `vm_toggle_wol` — inventory path to a VM |
| `NEW_VM_NAME` | `vm_rename` — new display name |
| `MO_REF` | `inventory_path`, `vm_disabled_method_len` — managed object as `Type:id` (only the first `:` separates type from id; e.g. `VirtualMachine:vm-42`) |

A `.env` file in the working directory is loaded automatically by the shared `snippets::connect` helper when present.

## Snippets (`cargo run -p snippets --bin …`)

| Binary | Summary |
|--------|---------|
| `dynamic_property_fetch` | `Client::fetch_property` for arbitrary properties (here: root folder `permission` as JSON). |
| `env_browser` | `EnvironmentBrowser` / config descriptors for a named `ComputeResource`. |
| `eventster` | `EventManager` + `EventHistoryCollector`: recent events with filtering. |
| `inventory_path` | Resolve inventory path from `MO_REF`. Uses **One** `RetrievePropertiesEx` (govmomi `mo.Ancestors` shape). Prints path to stdout. |
| `vm_disabled_method_len` | Generated `VirtualMachine::disabled_method` for `MO_REF`: single-property fetch without PropertyCollector; **JSON** or **SOAP** (`VIM_PROTOCOL`). |
| `mac_monitor` | `WaitForUpdates`-style cache: track VM NIC MACs and guest IPs (~30s demo). |
| `perf_metrics` | `PerformanceManager`: counter map, container view of VMs, per-VM stats sample. |
| `print_vm_addresses` | All VMs: guest net vs hardware NICs (`vim_retrievable!`). |
| `property_collector` | Manual `PropertyCollector` + `ContainerView` for VM names; optional `AlarmManager` sample. |
| `retrieve_ds_hosts` | Hosts mounted on a datastore (`vim_retrievable!`). |
| `retrieve_host_info` | All hosts under root: health, version, CPU/memory/uptime (`vim_retrievable!`). |
| `retrieve_recent_task` | Custom traversal from `TaskManager` to `recentTask` (`vim_retrievable!`). |
| `root_objects` | `RootObjects`: VIM + EAM/PBM/VSLM/SMS “about” style probes. |
| `vm_events` | `vim_updatable!` + property cache: VM create/update/remove for 60s. |
| `vm_ip` | `SearchIndex` + `vim_retrievable!` for `guest.ip_address` on one VM path. |
| `vm_rename` | Rename VM display name via `rename_task` + `TaskTracker`. |
| `vm_toggle_wol` | Toggle Wake-on-LAN on all vNICs: devices, reconfig task, `TaskTracker`. |

Source files live under `snippets/src/`; each file’s module docs describe APIs and env vars in more detail.

## vtui

Interactive Ratatui browser for inventory and properties. See `vtui/README.md`. Build with `cargo run -p vtui` from this workspace.
