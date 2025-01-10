# VIM-RS

This repository contains Rust crates for working with the Virtual Infrastructure API.

There are few crates:

1. `vim` - the library code for calling the VIM API. Contains data types and stubs for the VIM API
2. `gen` - a code generation tool that reads VI-JSON OpenAPI specs and turns it to library code
3. `examples/eventster` - small program illustrating how to pull events from vCenter server. Requires a vCenter server
4. `examples/vim_tests` - binding integration tests that uses `vim`. This requires a vCenter server.
5. `openapi30` - an indigenous OepnAPI 3.0.x data library used to load the OpenAPI docs. In addition to the base OpenAPI syntax it allows for documentation string on fields when those refer to other defined in the spec types. The library has some rudimentary validationn logic. I tried to use ready OpenAPI Rust crates and none had sufficient maturity. In some of the crates data elements would be missing. In other the model is tailored to specific flavour of OpenAPI making the use of VIM tedious e.g. for lacking support around inheritance via `allOf`.

On a decent box compiling `vim-tests` for first time with new `vim` version takes 5 to 20 minutes. Time between successive runs of tests from the IDE may take 1-2 minutes.

## Generating bindings

To generate new `vim` content run `gen/src/main`.

## Updating the OpenAPI specification

The OpenAPI specification is held in `gen/data`. We use a JSON conversion of the original specification as `DatastoreAccessible_enum` with values `True` and `False` is ambigously rendered in YAML i.e. as per the YAML specs the values are interpreted as boolean constants and not strings.

To add a newer version of the OpenAPI spec use tool like `yq` to convert to JSON e.g. `yq -o=json eval vi_json_openapi_specification_v8_0_2_0.yaml`. Review if `DatastoreAccessible_enum` values are rendered as boolean flags or strings. The correct is to have Strings - `True` and `False`.

## `vim` structure

The `vim` crate has few packages worth undertsanding:

1. `core` - `client.rs` contains an HTTP client that manages the session header. `base64.rs` is utility to process base64 encoded values in JSON as `u8` arrays.
2. `mo` - contains bindings for the individual managed object types. Managed object types in VIM have all the remote invocaiton endpoints. All the objects in `mo` require a `Client` to make HTTP calls.
3. `types` - contains the definitons of the VIM datatypes. It is a bit unwieldy and is the main culprit for slow compilaton and large executable size (60MB+). The `structs.rs` has all the struct types and is the big monstrosity with over 200K lines, 2911 structs and 16K functions and around 30K lambda functions, 343 traits all these do not include the types and functions generated with derive macros for serializaiton, deserializaiton and debug printing. `enums.rs` contains all enum definitons.
 
## `gen` structure

The generator has three packages:

1. `vim_model` - contains an API model that is closer to the Rust semantics and logic to convert OpenAPI 3.0.x. The loader.rs converts the OpenAPI model to the Rust like model.
1. `rs_emitter` - contains code generation logic reading from `vim_model` and rendering the actual Rust code for `vim` crate.

In additon `printer.rs` provides basic wrapper on an output stream for code generation.
