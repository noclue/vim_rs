# VIM-RS

This repository contains Rust crates for working with the VMware by Broadcom Virtual Infrastructure JSON API. The code herein is an example of how to use the VI-JSON OpenAPI specifications found in the [vSphere Management SDK](https://developer.broadcom.com/sdks/vsphere-management-sdk/latest/).

## Connecting to vCenter
To work with the API, one needs the `vim_rs` crate from the `vim_rs` folder of this repo.

The `vim_rs` library is fully asynchronous and uses the `tokio` runtime and `reqwest` HTTP library to invoke remote APIs.

To set up a connection, use a statement like the following:

```rust
let vim_client = ClientBuilder::new(&vc_server)
    .basic_authn(&username, &pwd)
    .app_details(APP_NAME, APP_VERSION)
    .build().await?;
```

Where `vc_server` is the address of the vCenter server, `username` is your vCenter username, and `pwd` is your vCenter password. `APP_NAME` and `APP_VERSION` describe your application to be able to identify and troubleshoot sessions from the vCenter UI or SessionManager API. An easy way to configure them with your Cargo project settings is as follows:

```rust
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
```

You can add `insecure()` to your builder configuration to bypass TLS checks for both hostname and certificate.

One can set the `reqwest` client through the builder's `http_client` method to reuse the `reqwest` connection and connection settings. The `vim_rs` client abstraction is cheap, but the `reqwest` HTTP client is not.

The `vim_client` above is an `Arc` around the actual client object. Use `.clone()` to pass it around.

If the above goes well, you have a connection to the vCenter server with an initialized session and retrieved service content.

## Obtaining Stub for the APIs
The VIM API is a remote object-oriented API. The functionality is organized in methods of managed objects.

To set up a remote proxy to a management object, one needs a connection, an object type, and an object identifier.

For example, to get a handle to the `PropertyCollector` API, the following code does the trick:

```rust
let content = client.service_content();
let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);
```

The `service_content` is a structure that contains references to various managed objects in the vCenter server. Note that the `PropertyCollector` is always present in the service content. Other objects may be optional, and a check is to be made if the reference is set.

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

## Working with Polymorphic Types
The VIM API is conceptualized as a classic object-oriented API, much like the Java or C++ standard libraries. It has a root `Any` object from which all other objects descend. There is `DataObject` that is the root for all data structures. There is also `MethodFault` that is the root for all error types.

This object-oriented design is not native to Rust. There are two principal approaches in Rust to dealing with such situations - traits and enums. Enums are easy to deal with in Rust and are extremely powerful and very safe. Unfortunately, expressing the VIM API solely in enums produces very hard-to-use abstractions of many deeply nested enum definitions that are hard to work with. Traits solve some of the usability challenges while dramatically increasing the work for the Rust compiler, which is not famous for fast performance. So the `vim_rs` library takes a hybrid approach. The often deep and complex hierarchy of the `DataObject` and `MethodFault` are represented with traits. The shallow and big inventory of boxed arrays and primitive data types used with the property collector and other dynamic APIs leverage enums with the synthetic `ValueElements` types. The VIM `Any` type is renamed to `VimAny` and is also represented as an `enum`.

Working with the trait system is a bit more complex. 

Let's look into the details.

### Data Structs

Firstly, for every structure type from the VIM API, we have a corresponding Rust struct type. For example, a network card could be described with the `VirtualE1000` structure. It looks roughly as follows:

```rust
pub struct VirtualE1000 {
    pub key: i32,
    pub device_info: Option<Box<dyn super::traits::DescriptionTrait>>,
    pub backing: Option<Box<dyn super::traits::VirtualDeviceBackingInfoTrait>>,
    pub connectable: Option<VirtualDeviceConnectInfo>,
    pub slot_info: Option<Box<dyn super::traits::VirtualDeviceBusSlotInfoTrait>>,
    pub controller_key: Option<i32>,
    pub unit_number: Option<i32>,
    pub numa_node: Option<i32>,
    pub device_group_info: Option<VirtualDeviceDeviceGroupInfo>,
    pub dynamic_property: Option<Vec<DynamicProperty>>,
    pub address_type: Option<String>,
    pub mac_address: Option<String>,
    pub wake_on_lan_enabled: Option<bool>,
    pub resource_allocation: Option<VirtualEthernetCardResourceAllocation>,
    pub external_id: Option<String>,
    pub upt_compatibility_enabled: Option<bool>,
}
```

Let's look at some details. First, we see that fields optional to the API use Rust `Option` (e.g., `Option<i32>`) while required fields require a valid value (e.g., `i32`). Next, we see that arrays of elements are expressed as Rust `Vec`. For fields that have children or can form a cycle, `Box` indirection is used. For fields of polymorphic types, i.e., those that have children, a `dyn *Trait` type is used, which refers to a trait type implemented by all alternative structures (`Option<Box<dyn DescriptionTrait>>`).

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

Last but not least, the VIM trait provides read-only accessors to the fields of the type they represent. For example, the `VirtualDeviceTrait` looks as follows:

```rust
pub trait VirtualDeviceTrait : super::traits::DataObjectTrait {
    fn get_key(&self) -> i32;
    fn get_device_info(&self) -> &Option<Box<dyn super::traits::DescriptionTrait>>;
    fn get_backing(&self) -> &Option<Box<dyn super::traits::VirtualDeviceBackingInfoTrait>>;
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo>;
    fn get_slot_info(&self) -> &Option<Box<dyn super::traits::VirtualDeviceBusSlotInfoTrait>>;
    fn get_controller_key(&self) -> Option<i32>;
    fn get_unit_number(&self) -> Option<i32>;
    fn get_numa_node(&self) -> Option<i32>;
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo>;
}
```

As we see, the same types are used as in the struct types. The `get_*` methods return borrowed references to complex types.

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
    let s: &'static str = type_.into();
    s.to_string()
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
