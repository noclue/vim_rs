# vim_rs Workflow Guide for LLMs

**CRITICAL: Read this entire guide before writing any vim_rs code. Following this workflow will ensure success on the first try.**

## Overview

vim_rs is a Rust SDK for the vSphere API. It provides:
- Type-safe bindings for vSphere managed objects, data structures, and enums
- Property collector macros for efficient bulk data retrieval
- Async client with connection pooling
- Comprehensive error handling

## Three-Tier Knowledge System

When building vim_rs applications, use these resources in order:

1. **API Reference** (search with filter='managed_objects', 'methods', 'structures', 'enums', 'traits')
   - WHAT exists: managed objects, methods, types, enums
   - Use to find the correct Rust types and method names

2. **Code Examples** (search with filter='examples', then get by ID)
   - HOW to code it: working patterns for common tasks
   - Use to understand proper usage patterns

3. **Admin Guides** (search with filter='guides', then get by ID)
   - WHEN/WHY/GOTCHAS: conceptual knowledge, best practices, limitations
   - Use to understand vSphere concepts and constraints

## Standard Workflow for vim_rs Applications

### Step 1: Client Connection (ALWAYS USE THIS PATTERN)

Every vim_rs application starts by connecting to vCenter. **Always use this exact pattern:**

```rust
use anyhow::{Context, Result};
use std::env;
use std::sync::Arc;
use vim_rs::Client;
use vim_rs:core::ClientBuilder;

pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").context("VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").context("VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").context("VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
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

**Dependencies needed:**
```toml
[dependencies]
vim_rs = "0.2"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
env_logger = "0.11"
log = "0.4"
```
### Step 2: Call APIs (ALWAYS USE THIS PATTERN)

To call any API on vSphere:

1. Create a proxy to a managed object using a `vim_rs::core::Client` and id from `ManagedObjectReference`.
2. Call the method and await the results

**Example:**
```rust
// Create a VirtualMachine managed object from the reference
let vm = vim_rs::mo::VirtualMachine::new(client.clone(), &vm_ref.value);

// Call power_on_vm_task with None for host (let vCenter choose)
let task_ref = vm.power_on_vm_task(None).await?;
```
Notes:
* **⚠️ IMPORTANT:** Managed object types reside in `vim_rs::mo` module.
* Managed Objects are proxies to objects living on the server like `VirtualMachine`, `Folder` etc.
* Managed Objects expose methods that call the remote APIs.

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
use vim_macros::vim_retrievable;
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

**Additional dependency:**
```toml
vim_macros = "0.2"
```

**Key Points:**
- `vim_retrievable!` generates a struct with property mappings
- Property paths use Rust formatted names (e.g., "summary.overall_status")
    - ❌ DO NOT use Java style names like "summary.overallStatus"
- `ObjectRetriever` handles batching automatically (efficient!)
- Use `retrieve_objects_from_container()` to fetch all objects of a type
- The macro generates `id` field automatically (ManagedObjectReference)


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

#### Option 1: Cast to a More Different Trait (MOST COMMON)

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
        
        // Now we can use VirtualEthernetCardTrait methods
        if let Some(mac) = eth.get_mac_address() {
            println!("MAC: {}", mac);
        }
    }
}
```

**Key points:**
- Import `vim_rs::types::convert::CastInto`
- Import the target trait (e.g., `VirtualEthernetCardTrait`)
- Use `.as_ref().into_ref()` to get `Option<&dyn TargetTrait>`
- Use `let Some(...) = ... else { continue }` pattern to handle failed trait cast

#### Method 2: Downcast to Concrete Struct Type

Use `std::any:Any` `as_any_ref()` and `downcast_ref()` to get a specific struct type:

```rust
use vim_rs::types::structs::VirtualE1000;

if let Some(devices) = vm.devices {
    for device in devices {
        // Try to downcast to specific VirtualE1000 type
        if let Some(e1000) = device.as_any_ref().downcast_ref::<VirtualE1000>() {
            println!("Found E1000 with MAC: {:?}", e1000.mac_address);
        }
    }
}
```

#### Method 3: Use Trait Getter Methods

All traits provide `get_*()` methods to access fields from the struct type:

```rust
use vim_rs::types::traits::VirtualDeviceTrait;

if let Some(devices) = vm.devices {
    for device in devices {
        // VirtualDeviceTrait provides get_key(), get_backing(), etc.
        let key = device.get_key();
        let controller_key = device.get_controller_key();
        println!("Device {} on controller {:?}", key, controller_key);
    }
}
```

#### Complete Working Example: Collecting MAC Addresses

```rust
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;

vim_retrievable!(
    struct Vm: VirtualMachine {
        name = "name",
        devices = "config.hardware.device",
    }
);

async fn get_vm_macs(client: Arc<Client>) -> Result<Vec<(String, String)>> {
    let retriever = ObjectRetriever::new(client.clone())?;
    let vms: Vec<Vm> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;
    
    let mut results = Vec::new();
    
    for vm in vms {
        if let Some(devices) = vm.devices {
            for device in devices {
                // Cast to VirtualEthernetCardTrait
                let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref() else {
                    continue;
                };
                
                // Use trait getter method
                if let Some(mac) = eth.get_mac_address() {
                    results.push((vm.name.clone(), mac.clone()));
                }
            }
        }
    }
    
    Ok(results)
}
```

#### When to Use Which Method

| Use Case | Method | Example |
|----------|--------|---------|
| Filter by category | Cast to trait | Get all ethernet cards from devices |
| Check specific type | Downcast to struct | Find VirtualE1000 specifically |
| Access base fields | Use trait getters | Get device key from any device |

**🔑 Key Takeaway:**
- Polymorphic types are `Box<dyn SomeTrait>`, not enums
- Use `.as_ref().into_ref()` to cast between traits
- Use `.as_any_ref().downcast_ref::<Type>()` to get concrete types
- Always import `vim_rs::types::convert::CastInto`


---

## ⚠️ SPECIAL CASE: Getting MAC Addresses from VirtualMachine Devices

**THIS IS THE #1 MOST COMMON MISTAKE - READ CAREFULLY!**

When you retrieve `config.hardware.device` from a VirtualMachine, you get `Vec<Box<dyn VirtualDeviceTrait>>`. This includes disks, controllers, NICs, etc.

**❌ WRONG APPROACH - DON'T DO THIS:**
```rust
// ❌ This is what 20B keeps trying - DON'T DO THIS!
for device in devices {
    match device.as_any_ref() {
        Some(v): &VirtualE1000 => v.mac_address.clone(),
        Some(v): &VirtualE1000E => v.mac_address.clone(),
        Some(v): &VirtualVmxnet3 => v.mac_address.clone(),
        // ... listing every single NIC type
        _ => None,
    }
}
// This won't compile AND you'll miss new NIC types!
```

**✅ CORRECT APPROACH - DO THIS INSTEAD:**
```rust
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;

for device in devices {
    // Cast to VirtualEthernetCardTrait - works for ALL ethernet card types!
    let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref() else {
        continue;  // Not an ethernet card (disk, controller, etc.)
    };
    
    // Use the trait method - works for E1000, E1000e, Vmxnet3, etc.
    if let Some(mac) = eth.get_mac_address() {
        println!("MAC: {}", mac);
    }
}
```

**Remember:**
- Import `vim_rs::types::convert::CastInto`
- Import target trait `vim_rs::types::traits::VirtualEthernetCardTrait`
- Use `device.as_ref().into_ref()` to cast
- Use `eth.get_mac_address()` to get the MAC

### Step 4: Common Patterns

**Pattern: Filter objects by property**
```rust
let hosts: Vec<Host> = retriever
    .retrieve_objects_from_container(&client.service_content().root_folder)
    .await?;

let connected_hosts: Vec<&Host> = hosts
    .iter()
    .filter(|h| h.connection_state == Some(HostSystemConnectionState::Connected))
    .collect();
```

**Pattern: Retrieve specific object by MoRef**
```rust
// If you have a ManagedObjectReference
let host_ref = /* ... */;
let hosts: Vec<Host> = retriever
    .retrieve_objects_from_list(&[host_ref])
    .await?;
```

**Pattern: Navigate inventory hierarchy**
```rust
// Start from root folder
let root = &client.service_content().root_folder;

// Get datacenters
vim_retrievable!(
    struct Datacenter: Datacenter {
        name = "name",
        host_folder = "hostFolder",
        vm_folder = "vmFolder",
    }
);

let datacenters: Vec<Datacenter> = retriever
    .retrieve_objects_from_container(root)
    .await?;
```

## Common Mistakes to Avoid

❌ **DON'T** manually create Client:
```rust
// WRONG
let client = Client { /* manual construction */ };
```

✅ **DO** use ClientBuilder:
```rust
let client = ClientBuilder::new(server)
                .basic_authn(username.as_str(), pwd.as_str())
                .build().await?;
```

---

❌ **DON'T** manually construct PropertyCollector specs:
```rust
// WRONG - too complex, error-prone
let spec = PropertySpec { /* manual construction */ };
```

✅ **DO** use vim_retrievable! macro:
```rust
vim_retrievable!(
    struct VM: VirtualMachine {
        name = "name",
        power_state = "runtime.powerState",
    }
);
```

---

❌ **DON'T** fetch objects one by one:
```rust
// WRONG - N API calls (slow!)
for vm_ref in vm_refs {
    let vm = get_vm_properties(&vm_ref).await?;
}
```

✅ **DO** use ObjectRetriever for batch fetching:
```rust
// RIGHT - 1 API call (fast!)
let vms: Vec<VM> = retriever
    .retrieve_objects(&vm_refs)
    .await?;
```

---

❌ **DON'T** ignore async/await:
```rust
// WRONG - won't compile
fn main() {
    let client = connect("app", "1.0").await?;
}
```

✅ **DO** use #[tokio::main]:
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let client = connect("app", "1.0").await?;
    Ok(())
}
```

---

❌ **DON'T** treat polymorphic types as enums:
```rust
// WRONG - VirtualDevice is NOT an enum!
match device {
    VirtualHardwareDevice::VirtualEthernetCard(card) => { /* ... */ }
    _ => {}
}
```

✅ **DO** use CastInto trait to work with polymorphic types:
```rust
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;

// Cast trait -> more specific trait
let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref() else {
    continue;
};
// Now use eth.get_mac_address(), etc.
```



---

❌ **DON'T** downcast to every NIC type to get MAC addresses:
```rust
// WRONG - tedious, error-prone, won't compile
match device.as_any_ref() {
    Some(v): &VirtualE1000 => v.mac_address.clone(),
    Some(v): &VirtualE1000E => v.mac_address.clone(),
    Some(v): &VirtualVmxnet3 => v.mac_address.clone(),
    _ => None,
}
```

✅ **DO** cast to VirtualEthernetCardTrait once:
```rust
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;

// Cast to trait - works for ALL NIC types!
let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref() else {
    continue;
};
// Get MAC using trait method
if let Some(mac) = eth.get_mac_address() {
    println!("MAC: {}", mac);
}
```

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
- `TaskManager` - Task tracking
- `SessionManager` - Session handling (handled by ClientBuilder)
- `SearchIndex` - Lookup Virtual Machines, Hosts, Datatores by inventory path, IP, DNS, UUID etc.

## MCP Server Tools and Workflows

### Unified API: Two Core Tools

The MCP server provides a simple, unified interface:

| Tool | Purpose |
|------|---------|
| `search(query, filter)` | Find items by natural language query |
| `get(id)` | Get detailed documentation for any item by ID |

Plus specialized tools for property path exploration:
- `get_starter_guide()` - This guide
- `get_property_info(managed_object, property_path)` - Explore property paths
- `list_property_collector_root_types()` - List available root types

### ID Format Reference

Every item has a unique ID you can pass to `get()`:

| Item Type | ID Format | Example |
|-----------|-----------|---------|
| Managed Object | `{rust_struct}` | `VirtualMachine` |
| Method | `{mo}::{method}` | `VirtualMachine::power_on_vm_task` |
| Structure | `{rust_name}` | `VirtualDevice` |
| Field | `{struct}::{field}` | `VirtualHardware::device` |
| Enum | `{rust_name}` | `ManagedEntityStatus` |
| Trait | `{rust_name}` | `VirtualDeviceTrait` |
| Example | `example::{name}` | `example::connection_basic` |
| Guide | `guide::{chunk_id}` | `guide::installing-esx-auto-deploy` |

### Workflow 1: Start Here - Get the Starter Guide
**Always call this first when beginning with vim_rs:**

```
get_starter_guide()
```

Returns this complete vim_rs starter guide with connection patterns, property collector usage, code snippets, and best practices.

---

### Workflow 2: Explore API, Examples, and Documentation

**Step 1: Search semantically** (finds items by meaning, not just keywords)
```
search(query="how to power on a virtual machine", limit=10, filter="all")
```

**Filters available:**
- `all` - Search everything (default)
- `managed_objects` - Managed object types (VirtualMachine, HostSystem, etc.)
- `methods` - API methods
- `structures` - Data structures/types
- `fields` - Structure fields
- `enums` - Enumerations
- `traits` - Trait definitions
- `examples` - Code examples
- `guides` - vSphere/VCF admin documentation

Search results include the **ID** for each item. Use this ID with `get()`.

**Step 2: Get detailed information using the ID from search results:**

```
get(id="VirtualMachine")                          # Managed object
get(id="VirtualMachine::power_on_vm_task")        # Method
get(id="VirtualDevice")                           # Structure  
get(id="VirtualHardware::device")                 # Field
get(id="VirtualDeviceTrait")                      # Trait
get(id="ManagedEntityStatus")                     # Enum
get(id="example::connection_basic")               # Code example
get(id="guide::installing-esx-auto-deploy")       # Guide section
```

**Tool Details:**

- **`search(query, limit, filter)`** - Natural language semantic search
  - Returns: Matching items with IDs, types, and brief summaries
  - Each result includes the ID needed for `get()`
  
- **`get(id)`** - Get comprehensive documentation for any item
  - Returns: Full details including description, fields/methods, usage examples, related types
  - Works for all item types (managed objects, methods, structures, fields, enums, traits, examples, guides)

---

#### Workflow 3: Build Property Collector Paths

Use these tools to discover valid property paths for `vim_retrievable!` macro:

**Step 1: List available root types**
```
list_property_collector_root_types()
```

Returns all supported managed object types (VirtualMachine, HostSystem, Datacenter, Datastore, ClusterComputeResource, etc.) that can be used as roots in the property collector.

**Step 2: Explore property paths**
```
get_property_info(managed_object="VirtualMachine", property_path="")
```

Returns detailed information about a property path including:
- VIM path (e.g., `guest.ipAddress`)
- Rust type (e.g., `Option<String>`)
- Whether it's optional
- Documentation
- Child fields (if it's a complex type)
- Example usage in `vim_retrievable!` macro

**Examples:**
```
# List all top-level properties for VirtualMachine
get_property_info(managed_object="VirtualMachine", property_path="")

# Get details about a specific nested property
get_property_info(managed_object="VirtualMachine", property_path="guest.ip_address")

# Explore nested structures
get_property_info(managed_object="VirtualMachine", property_path="config.hardware.device")
```

This workflow helps you build correct property paths like:
```rust
vim_retrievable!(
    struct MyVirtualMachine: VirtualMachine {
        name = "name",
        power_state = "runtime.powerState",
        ip_address = "guest.ipAddress",
        num_cpu = "config.hardware.numCPU",
    }
);
```

---

### Quick Reference: Task to Tool Mapping

| Task | Tool | Example |
|------|------|---------|
| **Getting Started** | `get_starter_guide()` | Learn vim_rs patterns first |
| **Find anything** | `search(query="...")` | `search(query="power on vm")` |
| **Find working code** | `search(filter="examples")` | `search(query="create vm", filter="examples")` |
| **Find admin concepts** | `search(filter="guides")` | `search(query="drs", filter="guides")` |
| **Get item details** | `get(id="...")` | `get(id="VirtualMachine")` |
| **Get example code** | `get(id="example::...")` | `get(id="example::property_collector_macro")` |
| **Get guide section** | `get(id="guide::...")` | `get(id="guide::vcenter-concepts")` |
| **Understand a type** | `get(id="...")` | `get(id="VirtualMachineConfigSpec")` |
| **Get method details** | `get(id="Mo::method")` | `get(id="VirtualMachine::power_on_vm_task")` |
| **Get field details** | `get(id="Struct::field")` | `get(id="VirtualHardware::device")` |
| **List property roots** | `list_property_collector_root_types()` | See all available managed object types |
| **Build property paths** | `get_property_info(...)` | `get_property_info(managed_object="VirtualMachine", property_path="guest.ip_address")` |

## Complete Minimal Example Template

Use this as a starting point for any vim_rs application:

```rust
use anyhow::{Context, Result};
use std::env;
use std::sync::Arc;
use log::info;
use vim_rs::core::{Client, ClientBuilder};
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

// 1. Connection helper
pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").context("VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").context("VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").context("VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
        .build()
        .await?;

    info!("Connected to {}", client.service_content().about.full_name);
    Ok(client)
}

// 2. Define data structure
vim_retrievable!(
    struct MyObject: HostSystem {  // Change type as needed
        name = "name",
        // Add more properties here
    }
);

// 3. Main logic
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Connect
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    // Retrieve data
    let retriever = ObjectRetriever::new(client.clone())?;
    let objects: Vec<MyObject> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;

    // Process data
    for obj in objects {
        info!("Object: {} ({})", obj.name, obj.id.value);
    }

    Ok(())
}
```

**Cargo.toml dependencies:**
```toml
[dependencies]
vim_rs = "0.2"
vim_macros = "0.2"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
env_logger = "0.11"
log = "0.4"
```

## Summary: The Golden Rules

1. **Always** use `ClientBuilder` for connections
2. **Always** use `vim_retrievable!` macro for data retrieval
3. **Always** use `ObjectRetriever` for fetching objects
5. **Always** check code examples first before inventing patterns
6. **Never** manually construct PropertyCollector specs
7. **Never** fetch objects one-by-one in loops
8. **Never** use `.unwrap()` in production code


This workflow ensures your vim_rs code will work correctly on the first try!
