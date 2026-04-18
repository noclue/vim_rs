# vim_rs Workflow Guide for LLMs

**How to use this file:** Skim **Steps 1–4** first (connection → calls → retrieval → tasks). Treat **Step 3.5+** (polymorphism, Deref), **Navigation paths**, **MCP tools**, and **Anti-patterns** as lookup sections—do not load them all unless the task needs them.

## Overview

vim_rs is a Rust SDK for the vSphere API: type-safe MO bindings, `vim_retrievable!` / `ObjectRetriever` for bulk properties, async `ClientBuilder`, default VI JSON plus optional SOAP/XML (`xml` feature).

## Two-Tier Knowledge System

When building vim_rs applications, use these resources in order:

1. **API Reference** (search with filter='managed_objects', 'methods', 'structures', 'enums', 'traits')
   - WHAT exists: managed objects, methods, types, enums
   - Use to find the correct Rust types and method names

2. **Code Examples** (search with filter='examples', then get by ID)
   - HOW to code it: working patterns for common tasks
   - Use to understand proper usage patterns

## Standard Workflow for vim_rs Applications

### Step 1: Client Connection (ALWAYS USE THIS PATTERN)

Every vim_rs application starts by connecting with `ClientBuilder`. By default this uses the VI JSON API, which is the preferred transport for vCenter. **Always use this exact baseline pattern unless XML is explicitly needed:**

```rust
use anyhow::{Context, Result};
use std::env;
use std::sync::Arc;
use vim_rs::core::{Client, ClientBuilder};

pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").context("VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").context("VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").context("VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
        // Optional: `.wire_logging(WireLoggingMode::Summary)` — see Step 1.0
        .build()
        .await?;

    log::debug!("Connected to {}", client.service_content().about.full_name);
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    // Your application logic here

    Ok(())
}
```

**Key Points:**
- Use `ClientBuilder::new()` - NOT manual client construction
- Use `.insecure(true)` for self-signed certs (typical in vSphere)
- Use `.basic_authn()` - NOT manual session management
- Store credentials in environment variables (VIM_SERVER, VIM_USERNAME, VIM_PASSWORD)
- Client is thread-safe (`Arc<Client>`)
- Managed-object stubs store an `Arc<dyn VimClient>` internally; `Client` implements `VimClient`.
- Default transport is JSON unless you set `TransportMode` (Step 1.1).
- Wire logging: **Step 1.0** only (defaults `Off`; filter by `vim_rs::wire::json` / `soap` targets).

**Dependencies needed:**
```toml
[dependencies]
vim_rs = "0.4"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
env_logger = "0.11"
log = "0.4"
```

### Step 1.0: Wire logging (transport debugging only)

`ClientBuilder::wire_logging` — default **`Off`**. Import `WireLoggingMode`; start with **`Summary`** (`Debug` on `vim_rs::wire::json` / `vim_rs::wire::soap`), then **`Detailed`** if needed (`Trace` bodies where allowed; **`SessionManager`** never logs bodies). Prefer `RUST_LOG=vim_rs::wire::json=debug` over blanket `vim_rs=trace`.

### Step 1.1: XML Transport (ONLY WHEN YOU ACTUALLY NEED IT)

Enable XML only when the target requires it, most notably for direct ESXi connections. The XML path is **optional and experimental**.

**How to enable it:**

```toml
[dependencies]
vim_rs = { version = "0.4", features = ["xml"] }
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
env_logger = "0.11"
log = "0.4"
```

**How to select transport:**

```rust
use anyhow::{Context, Result};
use std::env;
use std::sync::Arc;
use vim_rs::core::{Client, ClientBuilder};
use vim_rs::core::client::TransportMode;

pub async fn connect_auto(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").context("VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").context("VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").context("VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
        .transport(TransportMode::Auto)
        // Optional: `.wire_logging(vim_rs::WireLoggingMode::Summary)` — see Step 1.0
        .build()
        .await?;

    Ok(client)
}
```

Use `TransportMode::Auto` when the target may be either vCenter or ESXi:
- It probes the vCenter Hello System JSON API first.
- If that API is unavailable, it falls back to SOAP/XML automatically.

Use `TransportMode::Soap` when you know you are talking directly to ESXi or you explicitly want SOAP/XML.

**Critical XML caveats:**
- XML currently works only for the core VIM APIs.
- VSAN, SPBM/PBM, SMS, VSLM, EAM, and other non-VIM APIs will return errors over XML transport.
- XML support is experimental. If it fails, enable **wire logging** (`ClientBuilder::wire_logging` with `WireLoggingMode::Summary` or `Detailed`) and filter `vim_rs::wire::soap` / `vim_rs::wire::json` (see Step 1.0). Do not rely on generic `trace` for the whole crate as the primary transport capture mechanism.
- Enabling `xml` increases release binary size by about 500 KB and increases debug build times by about 30-40%.
- If the `xml` feature is not enabled, vim_rs returns to `0.4.0` transport, size, and build-time characteristics.

**Versioning caveat with XML:**
- XML transport does not use Hello System negotiation.
- `client.api_release()` is therefore not a reliable indicator of remote server capability when XML is active.
- Use `client.service_content().about.api_version` when you need to reason about server capabilities.

**Deserialization caveat with XML enabled:**
- If you manually deserialize polymorphic JSON while the `xml` feature is enabled, `_typeName` must appear before subtype-specific fields.

### Step 1.2: govc vcsim (optional — SOAP/XML simulator)

Use this when integration tests target **[govc vcsim](https://github.com/vmware/govmomi)** over SOAP/XML (`xml` feature + `TransportMode::Soap` or `Auto`). Production vCenter/ESXi usually do not need these knobs.

1. **`vcsim_compat` feature** (requires `xml`): enables tolerant client-internal SOAP unmarshalling so incomplete XML fragments (for example `HostConfigInfo.optionDef` without `optionType`) can be dropped instead of failing the entire response. Scoped helpers such as `vim_rs::xml::de::from_xml_with` exist for explicit control.

   ```toml
   vim_rs = { version = "0.4", features = ["xml", "vcsim_compat"] }
   ```

2. **Macro path suffix `?`:** In `vim_retrievable!` and `vim_updatable!`, append `?` to the **quoted** property path string to force `Option<T>` even when the OpenAPI spec marks the property as required—for example `effective_cpu = "summary.quick_stats.overall_cpu_usage"?`. vcsim (and occasionally real hosts) omit “required” fields; without `?`, `TryFrom` fails and whole rows are dropped during retrieval or cache updates.

3. **`CacheManager::set_cancel_wait_on_filter_change(true)`:** vcsim often does not merge newly registered property filters into an in-flight `WaitForUpdatesEx` long poll the way production servers do. When this flag is **true**, filter topology changes issue a best-effort `CancelWaitForUpdates` so the next `Monitor::wait_updates` iteration sees new filters. It defaults to **`false`**. Enable it in vcsim-driven tests that call `add_cache` / `remove_cache` while a monitor loop is running.

4. **Typed cancel faults on SOAP:** For `RequestCanceled` after `cancel_wait_for_updates`, use `vim_rs::core::client::is_request_canceled_error` when matching errors (vcsim often returns an empty SOAP `faultstring`; the library maps typed faults from `<detail>`).

### Step 2: Call APIs (ALWAYS USE THIS PATTERN)

To call any API on vSphere:

1. Create a proxy to a managed object using a `vim_rs::core::Client` and id from `ManagedObjectReference`.
2. Call the method and await the results
3. **If the method ends in `*_Task`, use `TaskTracker` to wait for completion** (see Step 4)

**Example - Property Accessor (returns immediately):**
```rust
// Create a VirtualMachine managed object from the reference
let vm = vim_rs::mo::VirtualMachine::new(client.clone(), &vm_ref.value);

// Call a property accessor - returns immediately
let name = vm.name().await?;
```

**Example - Task Method (returns Task reference):**
```rust
// Create a VirtualMachine managed object from the reference
let vm = vim_rs::mo::VirtualMachine::new(client.clone(), &vm_ref.value);

// Call power_on_vm_task - returns a Task reference
let task_ref = vm.power_on_vm_task(None).await?;

// ⚠️ MUST wait for task completion! (see Step 4)
let task_tracker = TaskTracker::new(client.clone());
task_tracker.wait::<()>(task_ref).await?;
```

Notes:
* **⚠️ IMPORTANT:** Managed object types reside in `vim_rs::mo` module.
* Managed Objects are proxies to objects living on the server like `VirtualMachine`, `Folder` etc.
* Managed Objects expose methods that call the remote APIs.
* Methods ending in `*_Task` are asynchronous operations that return a Task reference - you MUST wait for completion using TaskTracker (see Step 4).
* Property accessors (like `name()`, `runtime()`) return values directly without tasks.
* The generated stubs store an `Arc<dyn VimClient>` internally. The concrete `Client` implements
  `VimClient`, so passing `client.clone()` works without changes.
* `RootObjects` and `VsanObjectCatalog` (`vim_rs::core`) also use `VimClientHandle` (`Arc<dyn VimClient>`).
  You can pass `Arc<Client>` into their constructors via coercion. Their `client()` method returns
  `VimClientHandle`, not `Arc<Client>`—keep your own `Arc<Client>` if downstream code needs the concrete type.

### Step 3: Data Retrieval with Property Collector (ALWAYS USE THIS PATTERN)

To fetch data from vSphere, **always use the `vim_retrievable!` macro**. This is the correct, efficient way.

**DO NOT:**
- Manually construct PropertyCollector specs
- Call individual Get methods for each object
- Use session methods directly

**DO:**
- Use `vim_retrievable!` macro to define your struct
- Use `ObjectRetriever` to fetch data in batches

**Example: Retrieve Host Information**

```rust
use anyhow::Result;
use std::env;
use log::info;
use vim_rs::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

// Define what properties you want to retrieve
vim_retrievable!(
    struct Host: HostSystem {
        overall_status = "summary.overall_status",
        connection_state = "runtime.connection_state",
        name = "name",
        version = "config.product.version",
        cpu_usage = "summary.quick_stats.overall_cpu_usage",
        memory_usage = "summary.quick_stats.overall_memory_usage",
        uptime = "summary.quick_stats.uptime",
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    // Create retriever
    let retriever = ObjectRetriever::new(client.clone())?;

    // Fetch all hosts from root folder
    let hosts: Vec<Host> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;

    // Use the data
    for host in hosts {
        info!(
            "Host ({}): {}, {:?}, {:?}, {:?}",
            host.id.value,
            host.name,
            host.connection_state,
            host.overall_status,
            host.version
        );
    }

    Ok(())
}
```

**Note:** The `vim_retrievable!` macro is included in `vim_rs` — no additional dependency needed.

**Key Points:**
- `vim_retrievable!` generates a struct with property mappings
- Property paths use Rust formatted names (e.g., "summary.overall_status")
    - ❌ DO NOT use Java style names like "summary.overallStatus"
- `ObjectRetriever` handles batching automatically (efficient!)
- Use `retrieve_objects_from_container()` to fetch all objects of a type
- Use `retrieve_objects_from_list()` when you already have a slice/vec of `ManagedObjectReference`
- Use **`retrieve_object()`** for a **single** MoRef: one `RetrievePropertiesEx` round trip and **no** `ListView` / `ContainerView` lifecycle (prefer this over `retrieve_objects_from_list(&[one])` for one-off lookups)
- Optional macro suffix **`?`** after the path string forces `Option<T>` even when the spec marks the field required—see **Step 1.2** (govc vcsim) for when this matters
- The macro generates `id` field automatically (ManagedObjectReference)

### Quick patterns (`ObjectRetriever`)

- **Filter in memory:** `retrieve_objects_from_container` → `iter().filter(...)`.
- **One MoRef:** `retrieve_object(&mor).await?` → `Option<T>` (no view).
- **Several MoRefs:** `retrieve_objects_from_list(&[...]).await?`.
- **Hierarchy:** define another `vim_retrievable!` type (e.g. `Datacenter`) and `retrieve_objects_from_container(root)`.

### Step 3.5: Working with Polymorphic Types (CRITICAL)

**⚠️ EXTREMELY IMPORTANT: vim_rs uses TRAITS for polymorphic types, NOT ENUMS!**

Many vSphere properties return polymorphic types (types that can be one of several subtypes). For example:
- `config.hardware.device` returns `Vec<Box<dyn VirtualDeviceTrait>>` (not an enum!)
- Device subtypes include: VirtualEthernetCard, VirtualDisk, VirtualCdrom, etc.

**The WRONG way:**
```rust
// ❌ THIS DOES NOT WORK - VirtualDevice is NOT an enum!
match device {
    VirtualHardwareDevice::VirtualEthernetCard(card) => { /* ... */ }
    _ => {}
}
```

**The CORRECT way - Using Traits:**

vim_rs provides **three ways** to work with polymorphic trait types:

#### Option 1: Cast to a More Specific Trait (MOST COMMON)

Use `CastInto` trait to convert between trait types:

```rust
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;

vim_retrievable!(
    struct Vm: VirtualMachine {
        name = "name",
        devices = "config.hardware.device",  // Returns Vec<Box<dyn VirtualDeviceTrait>>
    }
);

// Iterate through devices and filter for Ethernet cards
if let Some(devices) = vm.devices {
    for device in devices {
        // Try to cast VirtualDeviceTrait -> VirtualEthernetCardTrait
        let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref() else {
            continue;  // Not an ethernet card, skip
        };
        
        // Access fields directly via Deref coercion
        if let Some(mac) = &eth.mac_address {
            println!("MAC: {}", mac);
        }
    }
}
```

#### Method 2: Downcast to a concrete struct

When you need one specific type: `device.as_any_ref().downcast_ref::<VirtualE1000>()`.

#### Method 3: Base fields on any device

`VirtualDeviceTrait` derefs to `VirtualDevice`; use `device.key`, `device.controller_key`, etc.

**NIC / MAC (most common pitfall):** devices are **`VirtualDeviceTrait`**, not an enum. Do **not** `match` on every NIC struct. Cast once to **`VirtualEthernetCardTrait`** (Option 1) and read `mac_address`.

#### When to Use Which Method

| Use Case | Method | Example |
|----------|--------|---------|
| Filter by category | Cast to trait | Get all ethernet cards from devices |
| Check specific type | Downcast to struct | Find VirtualE1000 specifically |
| Access base fields | Direct field access | Get device key from any device via `device.key` |

**Summary:** `Box<dyn Trait>` not enums; `CastInto` + `.as_ref().into_ref()`; `as_any_ref().downcast_ref::<T>()`; fields via Deref (`device.key`, `eth.mac_address`).

## Step 3.6: Compositional Inheritance — How Structs and Traits Use Deref

vim_rs uses **compositional inheritance** via Rust's `Deref` trait. Child structs do NOT have parent fields expanded inline. Instead, they contain a single parent field with an underscore suffix.

### The Parent Field Convention

- `VirtualEthernetCard` has field `virtual_device_: VirtualDevice`
- `TraversalSpec` has field `selection_spec_: SelectionSpec`
- `VirtualE1000` has field `virtual_ethernet_card_: VirtualEthernetCard`

The naming convention is: `snake_case(ParentTypeName) + "_"`

### Constructing Child Structs

**Example: TraversalSpec (has parent field)**

```rust
use vim_rs::types::structs::{TraversalSpec, SelectionSpec};

let spec = TraversalSpec {
    // Parent field — REQUIRED, uses underscore suffix naming
    selection_spec_: SelectionSpec {
        name: Some("traverseEntities".to_string()),
    },
    // Own fields
    r#type: "ContainerView".to_string(),
    path: "view".to_string(),
    skip: Some(false),
    select_set: None,
};
```

**Example: VirtualDeviceConfigSpec (no parent field — DataObject was pruned)**

```rust
use vim_rs::types::structs::VirtualDeviceConfigSpec;
use vim_rs::types::enums::VirtualDeviceConfigSpecOperationEnum;

let spec = VirtualDeviceConfigSpec {
    // No parent field — DataObject was pruned (has no meaningful fields)
    operation: Some(VirtualDeviceConfigSpecOperationEnum::Edit),
    device: my_device,
    file_operation: None,
    profile: None,
    backing: None,
    filter_spec: None,
    change_mode: None,
};
```

**Using the `defaults` feature for easier construction**

Enable the `defaults` feature in your `Cargo.toml`:
```toml
vim_rs = { version = "0.4", features = ["defaults"] }
```

Then use `..Default::default()` to fill in optional fields:

```rust
let spec = VirtualDeviceConfigSpec {
    operation: Some(VirtualDeviceConfigSpecOperationEnum::Edit),
    device: my_device,
    ..Default::default()
};
```

For structs with a parent field: `VirtualE1000::default()` or `VirtualEthernetCard { virtual_device_: VirtualDevice::default(), ..Default::default() }`.

### Reading via Deref Chain

Each child struct implements `Deref` to its parent, forming an access chain:

```
VirtualE1000 --Deref--> VirtualEthernetCard --Deref--> VirtualDevice
```

So given a `VirtualE1000`:
- `e1000.mac_address` works (from VirtualEthernetCard)
- `e1000.key` works (from VirtualDevice, two Deref levels deep)

Trait objects also Deref to their struct:
- `dyn VirtualDeviceTrait` —Deref→ `VirtualDevice`
- `dyn VirtualEthernetCardTrait` —Deref→ `VirtualEthernetCard`

So `device.key` works on `&dyn VirtualDeviceTrait` and `eth.mac_address` works on `&dyn VirtualEthernetCardTrait`.

### When Parent Fields Exist vs Don't

**Parent field exists** when the parent type has fields of its own:
- `VirtualEthernetCard.virtual_device_` (VirtualDevice has key, backing, etc.)
- `TraversalSpec.selection_spec_` (SelectionSpec has name)

**Parent field is absent** when the parent is pruned (no own fields):
- `VirtualMachineConfigSpec` — parent DataObject was pruned
- `EventFilterSpec` — parent DataObject was pruned

Use `get(id="StructName")` in the MCP tools to see if a struct has a parent field.

## Step 4: TaskTracker (`*_Task` methods)

Methods ending in `*_Task` return a **Task** MoRef; the operation is **not** done until you wait.

```rust
use vim_rs::core::tasks::TaskTracker;

let task_tracker = TaskTracker::new(client.clone()); // once per client; reuse in loops
let task_ref = vm.power_on_vm_task(None).await?;
task_tracker.wait::<()>(task_ref).await?;
```

- **`wait::<T>()`** — deserialize result (`()`, `ManagedObjectReference`, or a concrete result type).
- **`wait_any()`** — `Option<VimAny>` when you want to avoid extra allocation / handle polymorphic results manually.

Property accessors (e.g. `vm.name().await?`) are **not** tasks—no `TaskTracker`.

## Anti-patterns (dense checklist)

| Wrong | Right |
|--------|--------|
| Construct `Client` manually | `ClientBuilder` (Step 1) |
| Hand-built `PropertyCollector` specs | `vim_retrievable!` + `ObjectRetriever` (Step 3) |
| N per-object property fetches in a loop | `retrieve_objects_from_list` / `from_container` |
| `retrieve_objects_from_list(&[one])` for a single MoRef | `retrieve_object` |
| `#[tokio::main]` missing / `async` missing in `main` | `#[tokio::main] async fn main` |
| `match device` as if devices were enums | **Step 3.5** — `CastInto` + `VirtualEthernetCardTrait` for NICs |
| `*_Task` without `TaskTracker::wait` | Step 4 |
| New `TaskTracker` inside hot loops | One tracker, many `wait` calls |
| `enum.into()` / `From` for enum → `&str` | **`.as_str()`** on enums and `ValueElements` |
| `TraversalSpec` without parent field | **Step 3.6** — `selection_spec_: SelectionSpec { ... }` |

```rust
// Enum → string (removed From/Into)
let s = MoTypesEnum::VirtualMachine.as_str();
```

## Understanding API Navigation Paths

MCP `get` / `search` results use **paths** (below) to show how to reach a field from a managed object—translate them into **snake_case** `vim_retrievable!` strings.

### Path Notation

```
VirtualMachine::config?.hardware.device[*]→VirtualEthernetCard
```

| Symbol | Meaning | Example |
|--------|---------|---------|
| `::` | Scope resolution (property or method on MO) | `VirtualMachine::config` |
| `.` | Field access | `.hardware.device` |
| `?` | Optional field (may be None) in **navigation paths** (MCP / docs) | `config?` |
| `[*]` | Array iteration | `device[*]` (iterate over devices) |
| `→` | Downcast to concrete struct | `→VirtualEthernetCard` |
| `⇒` | Cast to trait (polymorphic) | `⇒VirtualDeviceTrait` |
| `()` | Method call (vs property) | `::reconfigure_vm()` |
| `(param)` | Method input parameter | `::reconfigure_vm(spec)` |

### How paths map to `vim_retrievable!`

Use MCP paths to pick **Rust snake_case** property strings (e.g. `config.hardware.num_cpu`, not JavaCase). `→` / `⇒` mean you will cast (`downcast_ref`) or use `CastInto` (Step 3.5).

**Navigation `?` vs macro `?`:** In MCP trees, `?` means the API field is optional. In `vim_retrievable!`, a trailing `?` on the **quoted** string forces `Option<T>` even when the spec says required (Step 1.2 / vcsim).

## Quick Reference: Essential Managed Objects

**Inventory Objects:**
- `Datacenter` - Top-level container for hosts and VMs
- `Folder` - Organizational container
- `ClusterComputeResource` - Cluster of hosts
- `HostSystem` - ESXi host
- `VirtualMachine` - VM instance
- `Datastore` - Storage

**Service Objects:**
- `ServiceInstance` - Root of inventory (from `client.service_content()`)
- `PropertyCollector` - Bulk property retrieval (use `ObjectRetriever` wrapper)
- `Task` - Represents async operations (use `TaskTracker` to wait for completion)
- `TaskManager` - Task tracking
- `SessionManager` - Session handling (handled by ClientBuilder)
- `SearchIndex` - Lookup VMs, hosts, datastores by inventory path, IP, DNS, UUID, etc.

## MCP tools (vim_rs MCP server)

| Call | Use |
|------|-----|
| `get_starter_guide()` | This document |
| `search(query, filter=...)` | Discover IDs (`managed_objects`, `methods`, `structures`, `examples`, …) |
| `get(id=...)` | Full doc: `VirtualMachine`, `VirtualMachine::power_on_vm_task`, `VirtualHardware::device`, `example::…` |
| `list_property_collector_root_types()` | MO types valid as collector roots |
| `get_property_tree(managed_object, start_path?)` | Property tree → pick **snake_case** strings for `vim_retrievable!` |
| `get_property_path(managed_object, property_path)` | One path (VIM wire name vs Rust type) |

Flow: `search` → copy **id** → `get(id)`. For macro paths use `get_property_tree` / `get_property_path` (see *Understanding API Navigation Paths*).

**App skeleton:** **Step 1** `connect` + **Step 3** `vim_retrievable!` / `ObjectRetriever` + **Step 4** `TaskTracker` only for `*_Task`. Same `Cargo.toml` as Step 1.

## Golden rules

`ClientBuilder` → `vim_retrievable!` + `ObjectRetriever` → `TaskTracker` for `*_Task` (create once, reuse) → `search` / `get` before inventing APIs → **Anti-patterns** table. SOAP tests against **govc vcsim**: **Step 1.2**.
