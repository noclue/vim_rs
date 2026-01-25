# VMware vSphere API Client for Rust

Rust interface to the VMware vSphere Virtual Infrastructure JSON API, allowing you to manage VMware infrastructure programmatically.

* **Fully Asynchronous**: Built on tokio runtime for efficient non-blocking operations
* **Type-Safe**: Comprehensive Rust types for the vSphere API objects
* **Macro System**: Simplify property retrieval and monitoring with vim_retrievable and vim_updatable macros
* **Hybrid Type System**: Intelligently combines traits and enums to balance type safety with performance
* **Documented**: The original VIM documentation rendered inline as rustdoc
* **Examples**: A set of examples demonstrating use of the APIs to get you started quickly

See [`CHANGELOG.md`](CHANGELOG.md) for release notes and breaking changes.

## 🤖 Supercharge your AI Assistant

> Sonnet 4.5: "The MCP server doesn't just provide documentation—it provides understanding as a service. Semantic search + complete examples + common mistakes = AI that writes correct code on the first try."

> Gemini 3.0 Pro: "The tools ... solved the hardest problem in vSphere development: Discovery. ... With them, I got the structure right on the second try."

The vSphere API is massive and deeply nested. We provide a **Model Context Protocol (MCP)** server that gives agents like **Claude Desktop** and **Cursor** semantic understanding of the `vim_rs` codebase.

With the MCP server, your AI assistant can:
* **Semantic Search**: Find API methods using natural language (e.g., "How do I power on a VM?").
* **Explore Properties**: Navigate the complex property paths (e.g., `summary.quick_stats.overall_cpu_usage`) without hallucinating.
* **Get Examples**: Retrieve relevant usage examples for specific types.

👉 **[Get Started with the vim_rs MCP Server](https://github.com/noclue/vim_rs/tree/main/mcp)**  
*(Currently requires manual build from source. See instructions in the link.)*

## Connecting to vCenter

To set up a connection, use a statement like the following:

```rust
use vsphere::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with username and password
    let client = ClientBuilder::new("https://vcenter.example.com")
        .basic_authn("administrator@vsphere.local", "password")
        .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) // For self-signed certs
        .insecure(true) // For self-signed certs
        .build()
        .await?;

     // Now you can use the client for API calls

     Ok(())
}
``` 
`.app_details` describes your application as to be able to identify and troubleshoot sessions from the vCenter UI or SessionManager API.

You can add `insecure()` to your builder configuration to bypass TLS checks for both hostname and certificate.

One can set the `reqwest` preconfigured client through the builder's `http_client` method to reuse the `reqwest` connection and connection settings. The `vim_rs` client abstraction is cheap, but the `reqwest` HTTP client is not.

The `client` above is an `Arc` around the actual client object. Use `.clone()` to pass it around.

If the above goes well, you have a connection to the vCenter server with an initialized session and retrieved service content.

## Obtaining Stub for the APIs
The VIM API is a remote object-oriented API. The functionality is organized in methods of managed objects.

To set up a remote stub to a management object, one needs a connection, an object type, and an object identifier.

For example, to get a handle to stub for the default `PropertyCollector`, the following code does the trick:

```rust
let content = client.service_content();
let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);
```

The `service_content` is a structure that contains references to the root managed objects in the vCenter server. Note that the `PropertyCollector` is always present in the service content. Other objects may be optional, and a check is to be made if the reference is set.

## Invoking APIs
This is simple and intuitive once you have a remote stub from the above step.

The VIM API has properties and methods. Both are exposed in the stubs. Properties are essentially remote methods that receive no parameters.

```rust
// Invoke a method
let events = collector.read_next_events(10).await?;

// Fetch a property value
let vms = view.view().await?;
```

In the examples above, `collector` is an instance of `PropertyCollector`, and `view` is an instance of a `View` like `ContainerView`.

## Property Retrieval with Macros

The `vim_macros` related crate provides two powerful macros to simplify working with vSphere properties:

### One-time Property Retrieval with `vim_retrievable`

Define structures that map to vSphere object properties and retrieve them with a single call:

```rust
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

// Define a struct mapping to HostSystem properties
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
   // Create a retriever using the client
   let retriever = ObjectRetriever::new(client.clone())?;

   // Retrieve all hosts with their properties in a single API call
   let hosts: Vec<HostInfo> = retriever
           .retrieve_objects_from_container(&client.service_content().root_folder)
           .await?;

   // Work with strongly-typed host objects
   for host in hosts {
      println!("Host {} is {:?}", host.name, host.power_state);
   }

   Ok(())
}
```
The object retriever also allows to pull objects from list of identifiers through 
`retrieve_objects_from_list`.

### Continuous Property Monitoring with `vim_updatable`

Stay up to date with local inventory replica using `PropertyCollector::wait_for_updates_ex`:

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
`CacheManager` provides `add_list_cache` method to monitor a predefined list of objects.

## Awaiting Task Completion

Many vSphere operations are asynchronous and return a `Task` managed object reference (via methods ending in `*_Task`). The `TaskTracker` provides an efficient way to wait for these tasks to complete using the PropertyCollector.

### Basic Usage

```rust
use vim_rs::core::tasks::TaskTracker;
use vim_rs::mo::VirtualMachine;

async fn rename_vm(client: Arc<Client>, vm_ref: ManagedObjectReference, new_name: &str) -> Result<()> {
    // Create a TaskTracker (reuse for multiple tasks)
    let task_tracker = TaskTracker::new(client.clone());
    
    // Create a VirtualMachine proxy and call a *_Task method
    let vm = VirtualMachine::new(client.clone(), &vm_ref.value);
    let task_ref = vm.rename_task(new_name).await?;
    
    // Wait for the task to complete
    task_tracker.wait::<()>(task_ref).await?;
    
    Ok(())
}
```

### Two APIs for Task Results

`TaskTracker` provides two methods for awaiting task completion:

**1. `wait::<T>()` - Convenient with Deserialization**

Use when you know the expected result type. The result is automatically deserialized using `serde_json`:

```rust
// For tasks that return no value (like rename, power operations)
task_tracker.wait::<()>(task_ref).await?;

// For tasks that return a ManagedObjectReference (like create VM)
let vm_ref: ManagedObjectReference = task_tracker.wait(task_ref).await?;
```

**2. `wait_any()` - Zero-Allocation Path**

Use when you want to avoid JSON conversion overhead and work with `VimAny` directly:

```rust
let result: Option<VimAny> = task_tracker.wait_any(task_ref).await?;
match result {
    None => println!("Task completed with no return value"),
    Some(VimAny::Value(v)) => println!("Primitive result: {:?}", v),
    Some(VimAny::Object(o)) => println!("Object result: {:?}", o.data_type()),
}
```

### Key Points

- **Create once, reuse**: Create a `TaskTracker` per `Client` and reuse it for multiple tasks.
- **Efficient monitoring**: Uses PropertyCollector with a shared `ListView` to track tasks with minimal overhead.
- **Automatic cleanup**: Tasks are automatically removed from tracking when they reach a terminal state.
- **Error handling**: Failed tasks return an `Error::TaskFailed` containing the `MethodFault` details.
- **Background loop**: The monitoring loop starts lazily on the first `wait_any()` call and stops when all tasks complete.

See [`examples/snippets/src/vm_rename.rs`](examples/snippets/src/vm_rename.rs) for a complete working example.

## Client abstraction used by managed-object stubs

Managed-object stubs in `vim_rs::mo` accept an `Arc<dyn VimClient>` internally. The concrete
[`Client`](vim_rs/src/core/client.rs) implements this trait. Most callers can pass `client.clone()`.

## Working with Polymorphic Types
The VIM API is conceptualized as a classic object-oriented API, much like the Java or C++ standard libraries. It has a root `Any` object from which all other objects descend. There is `DataObject` that is the root for all data structures. There is also `MethodFault` that is the root for all error types.

This object-oriented design is not native to Rust. There are two principal approaches in Rust to dealing with such situations - traits and enums. Enums are easy to deal with in Rust and are extremely powerful and very safe. Unfortunately, expressing the VIM API solely in enums produces very hard-to-use abstractions of many deeply nested enum definitions that are hard to work with. Traits solve some of the usability challenges while dramatically increasing the work for the Rust compiler, which is not famous for fast performance. So the `vim_rs` library takes a hybrid approach. The often deep and complex hierarchy of the `DataObject` and `MethodFault` are represented with traits. The shallow and big inventory of boxed arrays and primitive data types used with the property collector and other dynamic APIs leverage enums with the synthetic `ValueElements` types. `ValueElements::as_str()` returns the VIM API type name (e.g., `"string"`, `"ArrayOfManagedObjectReference"`) for type discrimination and logging. The VIM `Any` type is renamed to `VimAny` and is also represented as an `enum`.

Working with the trait system is a bit more complex. 

Let's look into the details.

### Data Structs

Firstly, for every structure type from the VIM API, we have a corresponding Rust struct type. The library uses **compositional inheritance** where child types contain their parent as a field and implement `Deref`/`DerefMut` for seamless field access.

For example, a network card could be described with the `VirtualE1000` structure:

```rust
pub struct VirtualE1000 {
    // Parent field - contains all VirtualEthernetCard fields (which contains VirtualDevice fields)
    pub virtual_ethernet_card_: VirtualEthernetCard,
}

impl Deref for VirtualE1000 {
    type Target = VirtualEthernetCard;
    fn deref(&self) -> &Self::Target { &self.virtual_ethernet_card_ }
}

impl DerefMut for VirtualE1000 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.virtual_ethernet_card_ }
}
```

Thanks to Deref coercion, you can access fields from the entire inheritance chain directly:

```rust
let e1000: &VirtualE1000 = /* ... */;
let key: i32 = e1000.key;                    // From VirtualDevice (grandparent)
let mac: &Option<String> = &e1000.mac_address; // From VirtualEthernetCard (parent)
```

For reference, the parent `VirtualEthernetCard` structure looks like:

```rust
pub struct VirtualEthernetCard {
    pub virtual_device_: VirtualDevice,      // Parent field
    pub address_type: Option<String>,
    pub mac_address: Option<String>,
    pub wake_on_lan_enabled: Option<bool>,
    // ... other fields
}
```

Let's look at some details. Fields optional in the API use Rust `Option` (e.g., `Option<i32>`) while required fields require a valid value (e.g., `i32`). Arrays are expressed as Rust `Vec`. For fields that have children or can form a cycle, `Box` indirection is used. For fields of polymorphic types, i.e., those that have children, a `dyn *Trait` type is used, which refers to a trait type implemented by all alternative structures (`Option<Box<dyn DescriptionTrait>>`).

Structure types support `serde` JSON serialization and deserialization as well as debug print.

### Traits

Traits are generated for VIM structure types that have children types. The traits for a given type are implemented by all of its descendants. This allows the API to take in and return all possible types in a given field, i.e., casting an object to a trait its type implements is trivial in Rust.

In the Rust language unlike Java, Go, and C++ there is no straightforward mechanism to upcast or downcast trait objects into other trait objects or concrete structure types. To make these possible, the `vim_rs` crate provides utilities.

For casting to concrete structure types, all traits in the VIM API have `AsAny` trait bound. `AsAny` allows conversion of a reference or `Box` to a reference to `&dyn Any` or `&Box<dyn Any>`. Further, a developer can use `Any` or `Box` methods to attempt fallible conversion to the target type. For example, converting a `VirtualDeviceTrait` reference to a `VirtualE1000` structure is done as follows (unwrap should be replaced with appropriate handling):

```rust
let e1000 = vd[0].as_any_ref().downcast_ref::<VirtualE1000>().unwrap();
```

Sometimes we want to convert from one trait to another. For example, if we want to read the MAC address of any network card device in a VM, we need to convert `VirtualDeviceTrait` into `VirtualEthernetCardTrait`. There are 2 options provided with the `CastInto` trait. One option is to convert to `Box`, and the other is to convert a borrowed reference.

In the examples below, we see how to convert `Box<dyn VirtualDevice>` into a reference and `Box` respectively:

```rust
let eth: &dyn VirtualEthernetCardTrait = vd.as_ref().into_ref().unwrap();

let eth: Box<dyn VirtualEthernetCardTrait> = vd.into_box().unwrap();
```

As Rust `TryInto` mirrors the `TryFrom` trait, `CastInto` has a mirror `CastFrom` trait.

Child structs access parent fields through Deref coercion. For example, `VirtualE1000` contains a `virtual_ethernet_card_` field and implements `Deref<Target = VirtualEthernetCard>`, which in turn derefs to `VirtualDevice`. This allows direct field access:

```rust
// Direct field access through Deref chain
let key: i32 = device.key;                           // From VirtualDevice
let mac: &Option<String> = &eth_card.mac_address;    // From VirtualEthernetCard
let backing = &device.backing;                        // From VirtualDevice
```

This compositional model provides the same ergonomic access as the previous expanded-fields approach while significantly reducing generated code size.
The new model supports both read and mutation operations and is thus much
easier when an update is required.

For more details on design decisions and performance considerations, please see the FAQ section below.

### Pruned Types

As discussed, the VIM API is big and has a deep inheritance hierarchy. To limit the size of the library, a number of optimizations and compromises are made. One specific optimization has a direct impact on the programming model. The descendant data types of `MethodFault` and `Event` types are not generated (See [PRUNED_TYPES](vim_build/src/main.rs)). This reduces the generated code and compilation times significantly at the cost of some utility.

The `MethodFault` type represents errors that can occur when invoking VIM API methods, and the `Event` type represents events that occur in the vCenter server.

The `MethodFault` and `Event` types do not have traits, and no descendant types are generated. Instead, both types receive 2 additional members:

* `type_: Option<StructType>` - holding the discriminator value, e.g., `EventEx`, `NotFound`, etc.
* `extra_fields_: HashMap<String, serde_json::Value>` - holding any data fields that are not present in the base type, e.g., `eventTypeId`.

Note that `extra_fields_` content uses the API native names in camelCase convention instead of the Rust-friendly names used throughout `vim_rs`.

Below is a snippet on how to decode the semantic event type using `type_name_` and `extra_fields_`:

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

Note that `StructType` implements the `child_of` method, allowing to check if a type is the same or a descendant of another.

In our example above, we check if the event is `EventEx` or `ExtendedEvent` to access the `eventTypeId` field.

Sometimes one will want to convert part of the dynamic-like objects into proper binding. For example, the `managedObject` in the `ExtendedEvent` can be read into `ManagedObjectReference` as follows:

```rust
let value = event.extra_fields_["managedObject"].clone();
let managed_object: ManagedObjectReference = serde_json::from_value(value)?;
```

# Repo Topology & Maintenance

There are a few crates:

1. `vim_rs` - the library code for calling the VIM API. Contains data types and stubs for the VIM API.
2. `vim_build` - a code generation tool that reads VI-JSON OpenAPI specs and turns them into library code.
3. `examples` - small programs demonstrating the use of the VIM API.
4. `openapi30` - an indigenous OpenAPI 3.0.x data library used to load the OpenAPI docs. In addition to the base OpenAPI syntax, it allows for documentation strings on fields when those refer to other defined types in the spec. The library has some rudimentary validation logic. I tried to use ready OpenAPI Rust crates, and none had sufficient maturity. In some of the crates, data elements would be missing. In others, the model is tailored to a specific flavor of OpenAPI, making the use of VIM tedious, e.g., for lacking support around inheritance via `allOf`.

## Generating Bindings

To generate new `vim` content, run `vim_build/src/main`.

## Updating the OpenAPI Specification

The OpenAPI specification is held in `vim_build/data`. We use a JSON conversion of the original specification as `DatastoreAccessible_enum` with values `True` and `False` is ambiguously rendered in YAML, i.e., as per the YAML specs, the values are interpreted as boolean constants and not strings.

To add a newer version of the OpenAPI spec, use a tool like `yq` to convert to JSON, e.g., `yq -o=json eval vi_json_openapi_specification_v8_0_2_0.yaml`. Review if `DatastoreAccessible_enum` values are rendered as boolean flags or strings. The correct way is to have Strings - `True` and `False`.

## `vim_rs` Structure

The `vim_rs` crate has a few packages worth understanding:

1. `core` - `client.rs` contains an API client abstraction that manages the session header. `helpers.rs` is a utility to process base64 encoded values in JSON as `u8` arrays.
2. `mo` - contains bindings for the individual managed object types. Managed object types in VIM have all the remote invocation endpoints. All the objects in `mo` require a `Client` to make HTTP calls.
3. `types` - contains the definitions of the VIM data types. It is a bit unwieldy and is the main culprit for slow compilation and large executable size (40MB+ when optimized for size).
    * `structs.rs` has all the struct types and is the big monstrosity.
    * `traits.rs` defines trait types for all VIM types that have children. In addition, there is logic for cross-trait conversion and JSON serialization.
    * `enums.rs` contains all enum definitions.
    * `struct_enum.rs` is an enum with all struct types that allows for relatively efficient type comparison operations.
    * `vim_object_trait.rs` implements a common trait for all VIM data types.
    * `deserialize.rs` and `dyn_serialize.rs` provide logic for polymorphic JSON serialization and deserialization, i.e., trait types.
    * `boxed_types.rs` is an enum used for all boxed types, i.e., arrays of a given type used in any placeholder and primitives.
    * `as_any.rs`, `vim_any.rs`, and `convert.rs` define utility types that are agnostic of code generation.

## `vim_build` Structure

The generator has three packages:

1. `vim_model` - contains an API model that is closer to the Rust semantics and logic to convert OpenAPI 3.0.x. The `loader.rs` converts the OpenAPI model to the Rust-like model.
2. `rs_emitter` - contains code generation logic reading from `vim_model` and rendering the actual Rust code for the `vim` crate.
3. `printer.rs` provides a basic wrapper on an output stream for code generation.

## FAQ

**Why aren’t standard traits like `PartialEq`, `Eq`, `Hash`, `Clone`, and `Default` implemented on VIM struct types?**  
Because including these traits across the extensive VIM data model would greatly increase compilation time and binary size. We decided to implement only the essentials to keep build times and executable sizes under control.

**What are the expected compilation times for vim-tests?**  
On a good machine, the first-time compilation of vim-tests can take between 2 and 5 minutes, with subsequent compilations (especially from within an IDE) taking about a minute.

**Why does the design use a hybrid approach with both traits and enums?**  
The VIM API is inherently polymorphic, and while enums are safe and idiomatic in Rust, using only enums would lead to unwieldy type definitions. The hybrid approach—with traits for the deep hierarchical parts and enums for simpler aspects—strikes a balance between performance and usability.
