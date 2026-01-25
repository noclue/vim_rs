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
- Managed-object stubs store an `Arc<dyn VimClient>` internally; `Client` implements `VimClient`.

**Dependencies needed:**
```toml
[dependencies]
vim_rs = "0.3"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
env_logger = "0.11"
log = "0.4"
```
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
vim_macros = "0.3"
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

**Key points:**
- Import `vim_rs::types::convert::CastInto`
- Import the target trait (e.g., `VirtualEthernetCardTrait`)
- Use `.as_ref().into_ref()` to get `Option<&dyn TargetTrait>`
- Use `let Some(...) = ... else { continue }` pattern to handle failed trait cast
- Access fields directly (e.g., `eth.mac_address`) - no getter methods needed

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

#### Method 3: Use Direct Field Access via Deref

Structs use compositional inheritance with Deref, allowing direct field access through the inheritance chain:

```rust
if let Some(devices) = vm.devices {
    for device in devices {
        // Access fields directly - Deref provides access to parent fields
        let key = device.key;
        let controller_key = device.controller_key;
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
                
                // Access mac_address field directly via Deref
                if let Some(mac) = &eth.mac_address {
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
| Access base fields | Direct field access | Get device key from any device via `device.key` |

**🔑 Key Takeaway:**
- Polymorphic types are `Box<dyn SomeTrait>`, not enums
- Use `.as_ref().into_ref()` to cast between traits
- Use `.as_any_ref().downcast_ref::<Type>()` to get concrete types
- Always import `vim_rs::types::convert::CastInto`
- Access fields directly via Deref (e.g., `device.key`, `eth.mac_address`)


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
    
    // Access mac_address field directly - works for E1000, E1000e, Vmxnet3, etc.
    if let Some(mac) = &eth.mac_address {
        println!("MAC: {}", mac);
    }
}
```

**Remember:**
- Import `vim_rs::types::convert::CastInto`
- Import target trait `vim_rs::types::traits::VirtualEthernetCardTrait`
- Use `device.as_ref().into_ref()` to cast
- Access `&eth.mac_address` directly (fields available via Deref)

---

## Step 4: Awaiting Task Completion with TaskTracker (CRITICAL FOR ASYNC OPERATIONS)

**⚠️ IMPORTANT: Many vSphere operations are ASYNCHRONOUS and return Task references!**

### Understanding vSphere Tasks

Methods ending in `*_Task` (e.g., `power_on_vm_task`, `rename_task`, `reconfigure_vm_task`) return a `ManagedObjectReference` to a `Task` object instead of completing immediately. You MUST wait for these tasks to complete to know if the operation succeeded.

**The WRONG way:**
```rust
// ❌ DON'T DO THIS - The operation is NOT complete yet!
let task_ref = vm.power_on_vm_task(None).await?;
println!("VM powered on!");  // WRONG - task might still be running!
```

**The CORRECT way:**
```rust
// ✅ DO THIS - Wait for the task to complete
use vim_rs::core::tasks::TaskTracker;

let task_tracker = TaskTracker::new(client.clone());
let task_ref = vm.power_on_vm_task(None).await?;
task_tracker.wait::<()>(task_ref).await?;
println!("VM powered on!");  // NOW it's actually powered on
```

### ALWAYS USE THIS PATTERN for Tasks

**Step 1: Create a TaskTracker once and reuse it**

```rust
use vim_rs::core::tasks::TaskTracker;

// Create once per Client
let task_tracker = TaskTracker::new(client.clone());

// Reuse for multiple tasks
task_tracker.wait::<()>(task1_ref).await?;
task_tracker.wait::<()>(task2_ref).await?;
```

**Step 2: Call the *_Task method**

```rust
// Methods ending in *_Task return ManagedObjectReference to a Task
let task_ref = vm.power_on_vm_task(None).await?;
let task_ref = vm.rename_task("NewName").await?;
let task_ref = vm.reconfigure_vm_task(spec).await?;
```

**Step 3: Wait for completion using one of two APIs**

### API Option 1: `wait::<T>()` - Most Common

Use `wait::<T>()` when you know the expected result type. It automatically deserializes the result.

```rust
// For tasks that return nothing (most common)
task_tracker.wait::<()>(task_ref).await?;

// For tasks that return a ManagedObjectReference
let vm_ref: ManagedObjectReference = task_tracker.wait(task_ref).await?;

// For tasks that return custom data
let result: CustomType = task_tracker.wait(task_ref).await?;
```

**Common task return types:**
- `()` - No return value (rename, power operations, reconfigure, etc.)
- `ManagedObjectReference` - Created object reference (create VM, clone VM, etc.)
- Custom types - Depends on the specific operation

### API Option 2: `wait_any()` - Zero-Allocation Path

Use `wait_any()` for maximum efficiency when you want to handle `VimAny` directly:

```rust
let result: Option<VimAny> = task_tracker.wait_any(task_ref).await?;
match result {
    None => println!("Task completed with no return value"),
    Some(VimAny::Value(v)) => {
        // Handle primitive/boxed-array result
        println!("Primitive result: {:?}", v);
    }
    Some(VimAny::Object(o)) => {
        // Handle data object result
        println!("Object type: {:?}", o.data_type());
        // Downcast to concrete type if needed
        if let Some(mor) = o.as_any_ref().downcast_ref::<ManagedObjectReference>() {
            println!("Created object: {}", mor.value);
        }
    }
}
```

### Step 5: Common Patterns

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
// Access fields directly via Deref: eth.mac_address, eth.key, etc.
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
// Access mac_address field directly via Deref
if let Some(mac) = &eth.mac_address {
    println!("MAC: {}", mac);
}
```

---

❌ **DON'T** forget to await task completion:
```rust
// WRONG - Task might still be running!
let task_ref = vm.power_on_vm_task(None).await?;
println!("VM powered on!");  // Operation may not be complete yet!
```

✅ **DO** use TaskTracker to wait for tasks:
```rust
use vim_rs::core::tasks::TaskTracker;

let task_tracker = TaskTracker::new(client.clone());
let task_ref = vm.power_on_vm_task(None).await?;
task_tracker.wait::<()>(task_ref).await?;
println!("VM powered on!");  // Now it's actually complete
```

---

❌ **DON'T** create a new TaskTracker for every task:
```rust
// WRONG - inefficient, creates new ListView for each task
for vm in vms {
    let tracker = TaskTracker::new(client.clone());  // Don't do this in a loop!
    let task = vm.power_on_vm_task(None).await?;
    tracker.wait::<()>(task).await?;
}
```

✅ **DO** create once and reuse:
```rust
// RIGHT - single TaskTracker for all tasks
let task_tracker = TaskTracker::new(client.clone());
for vm in vms {
    let task = vm.power_on_vm_task(None).await?;
    task_tracker.wait::<()>(task).await?;
}
```

---

❌ **DON'T** confuse property accessors with task methods:
```rust
// Property accessor - returns value directly, no task
let name: String = vm.name().await?;

// Task method - returns Task reference, must wait for completion
let task_ref = vm.rename_task("NewName").await?;
// ❌ WRONG: Missing task wait!
```

✅ **DO** understand the difference:
```rust
// Property accessor - no TaskTracker needed
let name: String = vm.name().await?;

// Task method - MUST use TaskTracker
let task_tracker = TaskTracker::new(client.clone());
let task_ref = vm.rename_task("NewName").await?;
task_tracker.wait::<()>(task_ref).await?;  // ✅ Correct!
```

---

❌ **DON'T** use `.into()` or `From::from()` to convert enums to strings:
```rust
// WRONG - these implementations have been removed
let type_str: &'static str = mo_type.into();
let type_str: &'static str = From::from(&mo_type);
let type_str = Into::<&str>::into(MoTypesEnum::VirtualMachine);
```

✅ **DO** use `.as_str()` for enum-to-string conversion:
```rust
// Convert enum to string
let type_str = mo_type.as_str();

// If you need an owned String
let type_string = mo_type.as_str().to_string();

// Works for all vim_rs enums
let vm_type = MoTypesEnum::VirtualMachine.as_str();  // "VirtualMachine"
let status = ManagedEntityStatus::Green.as_str();    // "green"

// ValueElements also has as_str() for VIM API type names
let value = ValueElements::PrimitiveString("hello".to_string());
let type_name = value.as_str();  // "string"
let arr_value = ValueElements::ArrayOfManagedObjectReference(vec![]);
let arr_type = arr_value.as_str();  // "ArrayOfManagedObjectReference"
```

## Understanding API Navigation Paths

When you use `get()` or `search()`, results for structures and fields include **paths** showing how to reach that type from a managed object. Understanding this notation helps you build property collector queries and navigate the API.

### Path Notation

```
VirtualMachine::config?.hardware.device[*]→VirtualEthernetCard
```

| Symbol | Meaning | Example |
|--------|---------|---------|
| `::` | Scope resolution (property or method on MO) | `VirtualMachine::config` |
| `.` | Field access | `.hardware.device` |
| `?` | Optional field (may be None) | `config?` |
| `[*]` | Array iteration | `device[*]` (iterate over devices) |
| `→` | Downcast to concrete struct | `→VirtualEthernetCard` |
| `⇒` | Cast to trait (polymorphic) | `⇒VirtualDeviceTrait` |
| `()` | Method call (vs property) | `::reconfigure_vm()` |
| `(param)` | Method input parameter | `::reconfigure_vm(spec)` |

### Path Types

**Property paths** (most common for PropertyCollector):
- `VirtualMachine::config` - Property accessor
- `VirtualMachine::config.hardware.num_cpu` - Nested property

**Method paths** (for understanding return types):
- `VirtualMachine::reconfigure_vm()` - Method return value
- `VirtualMachine::reconfigure_vm(spec)` - Method input parameter

### How Paths Help You

1. **Build vim_retrievable! property paths**: The path tells you exactly what to put in the macro
   ```rust
   vim_retrievable!(
       struct Vm: VirtualMachine {
           // Path: VirtualMachine::config.hardware.num_cpu
           num_cpu = "config.hardware.numCPU",
       }
   );
   ```

2. **Understand polymorphic types**: `→` and `⇒` show when downcasting is needed
   ```rust
   // Path: VirtualMachine::config.hardware.device[*]→VirtualEthernetCard
   // This tells you: iterate device array, then cast to VirtualEthernetCard
   ```

3. **Handle optional fields**: `?` in paths indicates you need `Option<T>` handling

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
- `get_property_tree(managed_object, start_path)` - View full property tree with types
- `get_property_path(managed_object, property_path)` - Explore specific property paths
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

**Step 2: View the full property tree**
```
get_property_tree(managed_object="VirtualMachine")
```

Returns a visual tree of all properties up to 5 levels deep with their Rust types:
```
VirtualMachine
├─config: Option<vim_rs::types::structs::VirtualMachineConfigInfo>
│ ├─hardware: Option<VirtualHardware>
│ │ ├─device: Option<Vec<Box<dyn VirtualDeviceTrait>>>
│ │ ├─memory_mb: i32
│ │ └─num_cpu: i32
│ └─name: Option<String>
├─guest: Option<vim_rs::types::structs::GuestInfo>
│ ├─ip_address: Option<String>
│ └─host_name: Option<String>
└─name: String
```

You can also start from a specific path to explore a subtree:
```
get_property_tree(managed_object="VirtualMachine", start_path="config.hardware")
```

**Step 3: Get details about a specific property**
```
get_property_path(managed_object="VirtualMachine", property_path="guest.ip_address")
```

Returns detailed information about a property path including:
- VIM path (e.g., `guest.ipAddress`)
- Rust type (e.g., `Option<String>`)
- Documentation
- Child fields (if it's a complex type)
- Example usage in `vim_retrievable!` macro

**Examples:**
```
# View full property tree for VirtualMachine
get_property_tree(managed_object="VirtualMachine")

# View subtree starting from config.hardware
get_property_tree(managed_object="VirtualMachine", start_path="config.hardware")

# Get details about a specific property
get_property_path(managed_object="VirtualMachine", property_path="guest.ip_address")
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
| **Understand a type** | `get(id="...")` | `get(id="VirtualMachineConfigSpec")` |
| **Get method details** | `get(id="Mo::method")` | `get(id="VirtualMachine::power_on_vm_task")` |
| **Get field details** | `get(id="Struct::field")` | `get(id="VirtualHardware::device")` |
| **List property roots** | `list_property_collector_root_types()` | See all available managed object types |
| **View property tree** | `get_property_tree(...)` | `get_property_tree(managed_object="VirtualMachine")` |
| **Explore subtree** | `get_property_tree(...)` | `get_property_tree(managed_object="VirtualMachine", start_path="config.hardware")` |
| **Get property details** | `get_property_path(...)` | `get_property_path(managed_object="VirtualMachine", property_path="guest.ip_address")` |

## Complete Minimal Example Template

Use this as a starting point for any vim_rs application:

```rust
use anyhow::{Context, Result};
use std::env;
use std::sync::Arc;
use log::info;
use vim_rs::core::{Client, ClientBuilder};
use vim_rs::core::tasks::TaskTracker;
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

    // Create TaskTracker if you'll be calling *_Task methods
    let task_tracker = TaskTracker::new(client.clone());

    // Retrieve data
    let retriever = ObjectRetriever::new(client.clone())?;
    let objects: Vec<MyObject> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;

    // Process data
    for obj in objects {
        info!("Object: {} ({})", obj.name, obj.id.value);
        
        // Example: Call a task method if needed
        // let task_ref = obj_proxy.some_method_task().await?;
        // task_tracker.wait::<()>(task_ref).await?;
    }

    Ok(())
}
```

**Cargo.toml dependencies:**
```toml
[dependencies]
vim_rs = "0.3"
vim_macros = "0.3"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
env_logger = "0.11"
log = "0.4"
```

## Summary: The Golden Rules

1. **Always** use `ClientBuilder` for connections
2. **Always** use `vim_retrievable!` macro for data retrieval
3. **Always** use `ObjectRetriever` for fetching objects
4. **Always** use `TaskTracker` to wait for `*_Task` method completion
5. **Always** create TaskTracker once and reuse it for multiple tasks
6. **Always** check code examples first before inventing patterns
7. **Never** manually construct PropertyCollector specs
8. **Never** fetch objects one-by-one in loops
9. **Never** forget to await task completion after calling `*_Task` methods
10. **Never** use `.unwrap()` in production code


This workflow ensures your vim_rs code will work correctly on the first try!
