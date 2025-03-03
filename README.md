# VIM-RS

This repository contains Rust crates for working with the Virtual Infrastructure API.

# Crash course

## Connecting to vCenter
To work with the API one needs the `vim_rs` crate from the `vim_rs` folder of this repo.

The vim_rs library is all asynchronous and uses the `tokio` runtime and `reqwest` HTTP library to
invoke remote APIS.

To set up a connection use statement like the following:

```rust
    let vim_client = ClientBuilder::new(&vc_server)
        .basic_authn(&username, &pwd)
        .app_details(APP_NAME, APP_VERSION)
        .build().await?;
```
Where APP_NAME and APP_VERSION describe your application so to be able to identify and trouble shoot
session form the vCenter UI or SessionManager API. An easy way to configure them with your Cargo
project settings is as follows:
```rust
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
```
You can add `insecure()` to you builder configuration to by pass TLS check for both hostname and
certificate.

Optionally one can set `reqwest` client through the client builder as to reuse the `reqwest`
connection and connection settings across multiple `vim_rs` client instances. The `vim_rs` client
abstraction is cheap. `request` HTTP client is not cheap.

Teh `vim_client` above is an `Arc` around the actual client object. Use `.clone()` to pass it
around.

If the above goes well you have a connection to the vCenter server with initialized session and
retrieved service content

## Obtaining stub for the APIs
The VIM API is a remote object oriented API. The functionality is organized in methods of managed
objects.

To set up a remote proxy to a management object one needs a connection, an object type and object
identifier.

For example to get handle to the `PropertyCollector` API the following code does the trick:
```rust
let content = client.service_content();
let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);
```
Note that the `PropertyCollector` is always present in the service content. Other object may be
optional and a check is to be made if the reference is set.


## Invoking APIs
This is simple and intuitive once you have remote stub from the above step.

The vim API has properties and methods. Both are exposed in the stubs. Properties are essentially
remote methods that receive no parameters.

```rust
// Invoke a method
let events = collector.read_next_events(10).await?;

// Fetch a property value
let vms = view.view().await?;
```

## Working with polymorphic types
The VIM API is conceptualized as a classic object oriented API much like the Java or C++ standard
libraries. It has a root `Any` object from which all other objects descend. There is `DataObject`
that is root for all data structures. There is also `MethodFault` that is root for all error types.

This object oriented design is not native to Rust. There are two principal approaches in Rust to
dealing with such situation - traits and enums. Enums are easy to deal with Rust and are extremely
powerful and most of all very safe. Unfortunately expressing the VIM API solely in enums produces
very hard to use abstraction of many deeply nested enum definitions that are hard to work with. 
Trait solve some of the usability challenges while increasing dramatically the work of r the Rust
compiler that is not famous for fast performance. So the `vim_rs` library take a hybrid approach.
The often deep and complex hierarchy of the `DataObject` and `MethodFault` are represented with
traits. The shallow and big inventory of boxed array and primitive data types used with property 
collector and other dynamic APIs leverages enums with the synthetic `ValueElements` types. The VIM
`Any` type is renamed to `VimAny` and is too represented as an `enum`.

Working wit the trait system is a bit more complex. So lets look in details. 

> TODO explain the trait and struct system and how to cast between traits, how to cast trait to
> concrete type, how to access data through traits 



# Repo topology & maintenance

There are few crates:

1. `vim_rs` - the library code for calling the VIM API. Contains data types and stubs for the VIM
   API
2. `gen` - a code generation tool that reads VI-JSON OpenAPI specs and turns it to library code
3. `examples` - small programs demonstrating the use of the VIM API
5. `openapi30` - an indigenous OpenAPI 3.0.x data library used to load the OpenAPI docs. In addition
   to the base OpenAPI syntax it allows for documentation string on fields when those refer to other
   defined in the spec types. The library has some rudimentary validation logic. I tried to use
   ready OpenAPI Rust crates and none had sufficient maturity. In some of the crates data elements
   would be missing. In other the model is tailored to specific flavour of OpenAPI making the use of
   VIM tedious e.g. for lacking support around inheritance via `allOf`.

On a decent box compiling `vim-tests` for first time with new `vim` version takes 5 to 20 minutes.
Time between successive runs of tests from the IDE may take 1-2 minutes.

## Generating bindings

To generate new `vim` content run `gen/src/main`.

## Updating the OpenAPI specification

The OpenAPI specification is held in `gen/data`. We use a JSON conversion of the original
specification as `DatastoreAccessible_enum` with values `True` and `False` is ambiguously rendered in
YAML i.e. as per the YAML specs the values are interpreted as boolean constants and not strings.

To add a newer version of the OpenAPI spec use tool like `yq` to convert to JSON e.g. `yq -o=json
eval vi_json_openapi_specification_v8_0_2_0.yaml`. Review if `DatastoreAccessible_enum` values are
rendered as boolean flags or strings. The correct is to have Strings - `True` and `False`.

## `vim_rs` structure

The `vim_rs` crate has few packages worth understanding:

1. `core` - `client.rs` contains an API client abstraction that manages the session header.
   `helpers.rs` is utility to process base64 encoded values in JSON as `u8` arrays.
2. `mo` - contains bindings for the individual managed object types. Managed object types in VIM
   have all the remote invocation endpoints. All the objects in `mo` require a `Client` to make HTTP
   calls.
3. `types` - contains the definitions of the VIM data types. It is a bit unwieldy and is the main
   culprit for slow compilation and large executable size (40MB+ when optimized for size). 
    * `structs.rs` has all the struct types and is the big monstrosity. 
    * `traits.rs` defines trait types for all VIM types that have children in addition there is
      logic for cross trait conversion and JSON serialization. 
    * `enums.rs` contains all enum definitions. 
    * `struct_enum.rs` is an enum with all struct types that allows for relatively efficient type
      comparison operations. 
    * `vim_object_trait.rs` implements a common trait for all VIM data types. 
    * `deserialize.rs` and `dyn_serialize.rs` provide logic for polymorphic JSON serialization and
      deserialization i.e. trait types. 
    * `boxed_types.rs` is an enum used for all boxed types i.e. arrays of given type used in any
      placeholder and primitives. 
    * `as_any.rs`, `vim_any.rs` and `convert.rs` define utility types that are agnostic of code
      generation.
 
## `gen` structure

The generator has three packages:

1. `vim_model` - contains an API model that is closer to the Rust semantics and logic to convert
   OpenAPI 3.0.x. The loader.rs converts the OpenAPI model to the Rust like model.
1. `rs_emitter` - contains code generation logic reading from `vim_model` and rendering the actual
   Rust code for `vim` crate.

In addition `printer.rs` provides basic wrapper on an output stream for code generation.
