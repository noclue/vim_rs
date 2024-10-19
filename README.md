# VIM-RS

This repository contains Rust crates for working with the Virtual Infrastructure API.

There are 3 crates:

1. `gen` - a code generation tool that read VI-JSON OpenAPI specs and turns them to library code
2. `vim` - the library code generated from `gen`. Contains data types and stub for the VIM API
3. `vim_tests` - binding integration tests that uses `vim`. This requires a vCenter server.

Typically one will load the `gen` and `vim_tests` projects in an IDE and avoid loading `vim`. `vim` is too big and compiles slow. When loaded as source into IDE it may be beneficial to turn off tools like `rust-analyzer` as it may take forever to do anything.

On a decent box compiling `vim-tests` for first time with new `vim` version takes 5 to 20 minutes. Time between successive runs of tests from the IDE may take 1-2 minutes.

## Generating bindings

To generate new `vim` content run `gen/src/main`.

## Updating the OpenAPI specification

The OpenAPI specification is held in `gen/data`. We use a JSON conversion of the original specification as `DatastoreAccessible_enum` with values `True` and `False` is ambigously rendered in YAML i.e. as per the YAML specs the values are interpreted as boolean constants and not strings.

To add a newer version of the OpenAPI spec use tool like `yq` to convert to JSON e.g. `yq -o=json eval vi_json_openapi_specification_v8_0_2_0.yaml`. Review if `DatastoreAccessible_enum` values are rendered as boolean flags or strings. The correct is to have Strings - `True` and `False`.

## `gen` structure

The generator has three packages:

1. `oas30` - an indigenous OepnAPI 3.0.x data library used to load the OpenAPI docs. In addition to the base OpenAPI syntax it allows for documentation string on fields in structures when those referred to predefined types. The library has some rudimentary validationn logic. I tried formerly to use ready OpenAPI Rust crates and none had sufficient maturity. In some of the crates data elements would be missing. In other the model is tailored to specific flavour of OpenAPI making the use of VIM tedious e.g. for lacking support around inheritance via `allOf`.
1. `vim_model` - contains an API model that is closer to the Rust semantics and logic to convert OpenAPI 3.0.x. The loader.rs converts the OpenAPI model to the Rust like model.
1. `rs_emitter` - contains code generation logic reading from `vim_model` and rendering the actual Rust code for `vim` crate.

In additon `printer.rs` provides basic wrapper on an output stream for code generation.
