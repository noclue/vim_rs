# VMware vSphere API client for Rust

`vim_rs` is an async, type-safe client for the VMware vSphere VI JSON API. SOAP/XML is an optional experimental transport for direct ESXi connectivity. Types are generated from the vSphere 9.1 OpenAPI spec, and the original VIM documentation is rendered as rustdoc.

This page is the user guide. The project overview and the EuroRust 2025 talk are in the [repository README](https://github.com/noclue/vim_rs/blob/main/README.md). Why the types look the way they do is in [vim_rs/docs](https://github.com/noclue/vim_rs/tree/main/vim_rs/docs). Release notes are in [CHANGELOG.md](https://github.com/noclue/vim_rs/blob/main/CHANGELOG.md).

```toml
vim_rs = "0.6"
```

## Connect

```rust
use vim_rs::core::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new("vcenter.example.com")
        .basic_authn("administrator@vsphere.local", "password")
        .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .insecure(true) // self-signed certificates
        .build()
        .await?;

    // client: Arc<Client> — clone it to share the session
    client.close().await?;
    Ok(())
}
```

`.app_details` names your application so sessions can be identified in the vCenter UI and through SessionManager.

`build()` returns `Arc<Client>` with a live session and service content already loaded. Clone the `Arc` to pass it around. The `vim_rs` handle is cheap to clone. The underlying `reqwest` client is not — reuse one HTTP client across builders when you open more than one session (see below).

### Session lifetime

Call `client.close().await` when the session should end. `close` is non-consuming and idempotent: other `Arc` clones may still exist, and a second `close` returns `Ok` without sending another Logout.

Destructor logout is a **0.6.x compatibility fallback**. It runs only on a multi-thread Tokio runtime, when `close` was never called, as the last inner handle drops. On a `current_thread` runtime it logs a warning. It will be **removed in 0.7.0**. Stop PropertyCollector waiters and destroy caches before `close`. Details: [CLIENT.md](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/CLIENT.md).

## TLS and your own HTTP client

By default the **`default-client`** feature is on. It enables `reqwest/default`, which on reqwest 0.13 means **rustls** (not OpenSSL), plus `charset`, `http2`, and `system-proxy`.

To pick a TLS stack yourself — native-tls / OpenSSL, or rustls with the **ring** provider — disable default features and pass a `reqwest::Client` into `ClientBuilder::new`.

With **`default-features = false`**, `ClientBuilder::new` takes `(server, reqwest::Client)`. `insecure()` and `http_client()` are not on the builder; configure TLS and proxies on the `reqwest` client. For self-signed certificates, set `danger_accept_invalid_certs` and `danger_accept_invalid_hostnames` on the reqwest builder (the same effect as `.insecure(true)`).

**VI/JSON only** (vCenter; the session is the `vmware-api-session-id` header, no HTTP cookies):

```toml
vim_rs = { version = "0.6", default-features = false }
reqwest = { version = "0.13", default-features = false, features = ["rustls"] }
# Often useful, and present when default-client is on: "charset", "http2", "system-proxy"
```

```rust
use vim_rs::core::ClientBuilder;

let http = reqwest::Client::builder().build()?;
let client = ClientBuilder::new("vcenter.example.com", http)
    .basic_authn("administrator@vsphere.local", "password")
    .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    .build()
    .await?;
```

**SOAP/XML** needs the `vim_rs` **`xml`** feature, reqwest **`cookies`**, and `.cookie_store(true)`:

```toml
vim_rs = { version = "0.6", default-features = false, features = ["xml"] }
reqwest = { version = "0.13", default-features = false, features = ["rustls", "cookies", "charset", "http2", "system-proxy"] }
```

```rust
use vim_rs::core::ClientBuilder;
use vim_rs::core::client::TransportMode;

let http = reqwest::Client::builder().cookie_store(true).build()?;
let client = ClientBuilder::new("esxi.example.com", http)
    .basic_authn("root", "password")
    .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    .transport(TransportMode::Soap)
    .build()
    .await?;
```

[`tls_rustls_only`](https://github.com/noclue/vim_rs/tree/main/tls_rustls_only) is an OpenSSL-free graph: `rustls-no-provider` plus a separate `rustls` crate with the **ring** provider, instead of `features = ["rustls"]` alone.

To confirm OpenSSL is absent, run `cargo tree -i openssl-sys` from your crate. Cargo then prints `error: package ID specification 'openssl-sys' did not match any packages` and exits 101. That message means the inverse query found nothing. If `openssl-sys` were present, the command would print a tree rooted at that package.

### ESXi login

VI/JSON against vCenter does not use HTTP cookies. SOAP/XML against ESXi does. Enable the `xml` feature, enable reqwest's `cookies` feature, and call `.cookie_store(true)` on `reqwest::Client::builder()` before `build()`. If you opted out of `default-client`, pass that configured client to `ClientBuilder::new(server, http_client)`.

## Wire logging

Wire logging is off by default. It uses its own log targets so transport lines can be filtered separately from the rest of the application.

```rust
use vim_rs::core::ClientBuilder;
use vim_rs::WireLoggingMode;

let client = ClientBuilder::new("vcenter.example.com")
    .basic_authn("administrator@vsphere.local", "password")
    .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    .wire_logging(WireLoggingMode::Summary)
    .build()
    .await?;
```

- **`Off`** — no dedicated wire lines (default).
- **`Summary`** — method, path, size, HTTP status, and duration at `log::Level::Debug`. Start here when a call fails.
- **`Detailed`** — may include bodies at `log::Level::Trace` for non-session traffic. **`SessionManager`** calls (login, logout, session APIs) stay summary-only. Credentials are never logged.

Targets are **`vim_rs::wire::json`** and **`vim_rs::wire::soap`**. Prefer a target filter over a global `trace`:

- `RUST_LOG=vim_rs::wire::json=debug`
- `RUST_LOG=vim_rs::wire::soap=debug`
- `RUST_LOG=vim_rs::wire::json=trace` for JSON bodies, where they are allowed

Detailed logs get large. Use them for a short debugging session.

## Managed objects

VIM is a remote object API. A stub needs the session, the managed-object type, and an id. Root objects live on service content. `PropertyCollector` is always present; other references are optional.

```rust
let content = client.service_content();
let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);
```

Stubs in `vim_rs::mo` store `Arc<dyn VimClient>`. `Client` implements that trait, so `client.clone()` is the usual argument. The facade itself is described in [CLIENT.md](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/CLIENT.md).

Properties and methods are both remote calls. Properties take no parameters.

```rust
let events = collector.read_next_events(10).await?;
let vms = view.view().await?;
```

## Property retrieval

`vim_retrievable!` and `vim_updatable!` are exported from `vim_rs`. You do not need a direct `vim_macros` dependency.

### One-shot retrieval

`vim_retrievable!` declares a struct whose fields are property paths, then loads every matching object in one call:

```rust
use vim_rs::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        power_state = "runtime.power_state",
        connected = "runtime.connection_state",
        cpu_usage = "summary.quick_stats.overall_cpu_usage",
        memory_usage = "summary.quick_stats.overall_memory_usage",
        uptime = "summary.quick_stats.uptime",
    }
);

async fn print_hosts(client: &Client) -> Result<()> {
    let retriever = ObjectRetriever::new(client.clone())?;

    let hosts: Vec<Host> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;

    for host in hosts {
        println!("Host {} is {:?}", host.name, host.power_state);
    }
    Ok(())
}
```

[`retrieve_object`](https://docs.rs/vim_rs/latest/vim_rs/core/pc_retrieve/struct.ObjectRetriever.html#method.retrieve_object) loads one managed object by reference and does not create a `ContainerView` or `ListView`:

```rust
let host_moref: ManagedObjectReference = /* ... */;
let one: Option<Host> = retriever.retrieve_object(&host_moref).await?;
```

`retrieve_objects_from_list` loads an explicit list of ids.

A trailing `?` on a path forces `Option<T>` even when the spec marks the property required, for example `effective_cpu = "summary.effective_cpu"?`. Simulators, and occasionally real hosts, omit fields the spec calls mandatory. Without `?`, `TryFrom` fails and the whole row is skipped. More on that under [vcsim](#govc-vcsim).

### Continuous monitoring

`vim_updatable!` plus `CacheManager` keeps a local replica through `PropertyCollector::wait_for_updates_ex`:

```rust
vim_updatable!(
    struct VmDetails: VirtualMachine {
        name = "name",
        power_state = "runtime.power_state",
    }
);

impl Display for VmDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VM ({}): {} with power state: {:?}", self.id.value, self.name, self.power_state
        )
    }
}

struct ChangeListener {}

impl ObjectCacheListener<VmDetails> for ChangeListener {
    fn on_new(&mut self, obj: &VmDetails) -> CacheAction {
        info!("New VM: {}", obj);
        CacheAction::Keep
    }

    fn on_update(&mut self, obj: &VmDetails) -> CacheAction {
        info!("VM updated: {}", obj);
        CacheAction::Keep
    }

    fn on_remove(&mut self, obj: VmDetails) {
        info!("VM removed: {}", obj);
    }
}

async fn monitor_vms(client: &Arc<Client>) -> Result<(), Error> {
    let cache = Box::new(ObjectCache::new_with_listener(Box::new(ChangeListener {})));
    let mut manager = CacheManager::new(client.clone())?;
    let mut monitor = manager.create_monitor()?;

    manager.add_container_cache(cache, &client.service_content().root_folder).await?;

    let start = Instant::now();
    loop {
        let updates = monitor.wait_updates(10).await?;
        if let Some(updates) = updates {
            manager.apply_updates(updates)?;
        }
        if start.elapsed().as_secs() > 60 {
            break;
        }
    }

    manager.destroy().await?;
    Ok(())
}
```

`CacheManager::add_list_cache` watches a fixed set of objects. For govc vcsim, call `manager.set_cancel_wait_on_filter_change(true)` before the loop when caches are added or removed during a long poll. See [vcsim](#govc-vcsim).

Destroy the cache manager before `client.close()`.

## Tasks

Methods named `*_Task` return a `Task` managed object. `TaskTracker` waits for them through PropertyCollector. Create one tracker per client and reuse it.

```rust
use vim_rs::core::tasks::TaskTracker;
use vim_rs::mo::VirtualMachine;

async fn rename_vm(client: Arc<Client>, vm_ref: ManagedObjectReference, new_name: &str) -> Result<()> {
    let task_tracker = TaskTracker::new(client.clone());

    let vm = VirtualMachine::new(client.clone(), &vm_ref.value);
    let task_ref = vm.rename_task(new_name).await?;

    task_tracker.wait::<()>(task_ref).await?;
    Ok(())
}
```

**`wait::<T>()`** deserializes the task result with the active transport (`miniserde` for JSON, SOAP when XML is selected):

```rust
task_tracker.wait::<()>(task_ref).await?;

let vm_ref: ManagedObjectReference = task_tracker.wait(task_ref).await?;
```

**`wait_any()`** returns the raw `VimAny` and skips a typed conversion:

```rust
let result: Option<VimAny> = task_tracker.wait_any(task_ref).await?;
match result {
    None => println!("Task completed with no return value"),
    Some(VimAny::Value(v)) => println!("Primitive result: {:?}", v),
    Some(VimAny::Object(o)) => println!("Object result: {:?}", o.data_type()),
}
```

- The monitoring loop starts on the first `wait_any()` (and `wait` uses that path) and stops when every tracked task has finished.
- Tasks are removed from the shared `ListView` when they reach a terminal state.
- A failed task is `Error::TaskFailed` and carries the `MethodFault`.
- Shut the tracker down before `client.close()`.

A full program: [`examples/snippets/src/vm_rename.rs`](https://github.com/noclue/vim_rs/blob/main/examples/snippets/src/vm_rename.rs).

## Optional XML transport

The default transport is VI JSON. XML is off unless you enable it and select it on the builder.

```toml
vim_rs = { version = "0.6", features = ["xml"] }
```

`TransportMode::Auto` probes and works for either vCenter or ESXi:

```rust
use vim_rs::core::ClientBuilder;
use vim_rs::core::client::TransportMode;

let client = ClientBuilder::new("vc-or-esxi.example.com")
    .basic_authn("root", "password")
    .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    .insecure(true)
    .transport(TransportMode::Auto)
    .build()
    .await?;
```

`TransportMode::Soap` forces SOAP/XML, which is what standalone ESXi requires:

```rust
use vim_rs::core::ClientBuilder;
use vim_rs::core::client::TransportMode;

let client = ClientBuilder::new("esxi.example.com")
    .basic_authn("root", "password")
    .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    .insecure(true)
    .transport(TransportMode::Soap)
    .build()
    .await?;
```

Caveats:

- XML currently serves the core VIM API. VSAN, SPBM/PBM, SMS, VSLM, and EAM return errors over XML.
- The feature is experimental. When a call fails, turn on [wire logging](#wire-logging) and filter `vim_rs::wire::soap` or `vim_rs::wire::json`.
- Enabling `xml` increases release binary size by about 500 KB and debug build time by about 30–40%.
- Leaving `xml` off keeps VI/JSON build time and binary size.

XML does not use the Hello System API to negotiate the release. It starts from the release `vim_rs` was built against.

- An older server is used at that server's newest release.
- A newer server falls back to the `vim_rs` release.
- `client.api_release()` is therefore not a capability probe on XML. Use `client.service_content().about.api_version`.

One JSON caveat appears when `xml` is enabled, including for JSON payloads: if you deserialize polymorphic JSON yourself, the `_typeName` key must appear before subtype-specific fields.

Managed-object property reads over SOAP use `RetrievePropertiesEx`. The response is parsed once into `VimAny`, and generated getters (for example `VirtualMachine::disabled_method`) narrow to the concrete Rust type without a second XML round trip.

When you hold a `VimAny` or `ValueElements` yourself, recover `T` with `VimAny::into_any()` or `ValueElements::into_any()` and `downcast`.

The SOAP client and the schema-guided deserializer are described in [CLIENT.md](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/CLIENT.md) and [METADATA_DRIVEN_XML_DE.md](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/METADATA_DRIVEN_XML_DE.md).

## govc vcsim

[govc vcsim](https://github.com/vmware/govmomi) is useful for local tests. Its SOAP output differs from vCenter and ESXi. The following only matter when the target is vcsim over XML.

1. **`vcsim_compat`** (requires `xml`) drops malformed or incomplete elements during client SOAP unmarshalling instead of failing the whole response — for example `HostConfigInfo.optionDef` without `optionType`. For a single parse, `vim_rs::xml::de::from_xml_with` and the helpers next to it do the same without the feature.

   ```toml
   vim_rs = { version = "0.6", features = ["xml", "vcsim_compat"] }
   ```

2. **Path suffix `?`** on `vim_retrievable!` / `vim_updatable!` forces `Option<T>` for a property the spec marks required. See [one-shot retrieval](#one-shot-retrieval).

3. **`CacheManager::set_cancel_wait_on_filter_change(true)`**. vcsim often does not push a newly registered filter into an in-flight `WaitForUpdatesEx` poll, so `add_cache` / `remove_cache` looks stuck until the wait times out. With this set, a filter change sends `CancelWaitForUpdates` and the next `Monitor::wait_updates` observes the new filters. The default is `false`. Production vCenter and ESXi generally do not need it.

## `defaults`

```toml
vim_rs = { version = "0.6", features = ["defaults"] }
```

This generates `Default` for structs, enums, and trait objects, which makes update specs writable with struct-update syntax:

```rust
let config_spec = VirtualMachineConfigSpec {
    device_change: Some(device_changes),
    ..Default::default()
};
```

The feature adds a large amount of generated code and compile time. A full example is [`examples/snippets/src/vm_toggle_wol.rs`](https://github.com/noclue/vim_rs/blob/main/examples/snippets/src/vm_toggle_wol.rs). Why `Default` is opt-in is in [TYPE_SYSTEM.md](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/TYPE_SYSTEM.md#defaults).

## Working with polymorphic values

VIM inheritance is modeled with traits where the tree is deep, and with enums where the alternatives are a flat list. The rationale, the `Deref` parent-field layout, and the pruned `MethodFault` / `Event` trees are [TYPE_SYSTEM.md](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/TYPE_SYSTEM.md). The operations below are the ones application code actually performs.

**Downcast a trait object to a struct** (`AsAny`):

```rust
let e1000 = vd[0].as_any_ref().downcast_ref::<VirtualE1000>().unwrap();
```

**Cast one trait to another** (`CastInto`, import `vim_rs::types::convert::CastInto`):

```rust
let eth: &dyn VirtualEthernetCardTrait = vd.as_ref().into_ref().unwrap();
let eth: Box<dyn VirtualEthernetCardTrait> = vd.into_box().unwrap();
```

Inherited fields on a concrete struct are reached through `Deref`. A `VirtualE1000` derefs to `VirtualEthernetCard`, which derefs to `VirtualDevice`, so `device.key` and `eth_card.mac_address` compile as field access.

**Faults and events** do not have generated subtypes. The base struct carries `type_: Option<StructType>` and `extra_fields_`, whose keys are the API's camelCase names:

```rust
fn get_event_type_id(event: &Event) -> String {
    let Some(type_) = event.type_ else {
        return "Event".to_string();
    };
    if type_.child_of(StructType::EventEx) || type_.child_of(StructType::ExtendedEvent) {
        if let Some(event_type_id) = event.extra_fields_["eventTypeId"].as_str() {
            return event_type_id.to_string();
        }
    }
    type_.as_str().to_string()
}
```

Turn one dynamic field into a generated type with `from_value`:

```rust
use vim_rs::types::mini_helpers::from_value;

let value = event.extra_fields_["managedObject"].clone();
let managed_object: ManagedObjectReference = from_value(value)?;
```

`ValueElements::as_str()` returns the VIM type name (`"string"`, `"ArrayOfManagedObjectReference"`) when you need to log or branch on a dynamic property-collector value.

## Examples

Runnable programs, including the environment variables they expect, are listed in [examples/README.md](https://github.com/noclue/vim_rs/blob/main/examples/README.md). From `examples/`:

```bash
cargo run -p snippets --bin retrieve_host_info
cargo run -p vtui
```

## Further reading

- [Type system](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/TYPE_SYSTEM.md) — traits, enums, compositional inheritance, pruned faults and events
- [Client architecture](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/CLIENT.md) — facade, transports, `close`
- [Repository and code generation](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/REPOSITORY.md) — regenerating bindings from the OpenAPI spec
- [EuroRust 2025 talk](https://www.youtube.com/watch?v=C87GIl1n8v4) — *How to Fit an Elephant in a Rusty Refrigerator*
