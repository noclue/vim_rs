# Repository and code generation

`vim_rs` is generated. Hand-edits under `vim_rs/src/mo/` and `vim_rs/src/types/` are overwritten by the next `vim_build` run. Changes to the binding belong in the generator, in `openapi30`, or in the OpenAPI input.

## Crates

The repository root is a Cargo workspace for the library and its generator. `examples/` and `mcp/` are separate workspaces.

| Crate | Role |
|-------|------|
| `vim_rs` | Client, managed-object stubs, and the generated type system. Published to crates.io. |
| `vim_macros` | Procedural macros `vim_retrievable!` and `vim_updatable!`. Re-exported from `vim_rs`, so applications depend on `vim_rs` only. |
| `vim_build` | Reads a VI JSON OpenAPI document and writes the `vim_rs` sources. |
| `openapi30` | OpenAPI 3.0 model used by `vim_build`. It keeps documentation strings on fields that `$ref` another type, and it accepts VIM’s inheritance via `allOf`. Existing OpenAPI crates were a poor fit: some dropped model elements, others assumed a dialect that does not express this spec. |
| `examples` | `snippets` binaries and the `vtui` browser. See [examples/README.md](../../examples/README.md). |
| `mcp/server` | MCP server for coding agents. See [mcp/README.md](../../mcp/README.md). |

## `vim_rs` source

| Module | Contents |
|--------|----------|
| `core` | `Client` / `ClientBuilder`, session handling, `TaskTracker`, property retrieval and cache, errors. `helpers` decodes base64 JSON strings as `u8` arrays. |
| `mo` | One stub per managed object. Methods and properties are remote calls. Every stub holds `Arc<dyn VimClient>`. |
| `types` | Generated data model. This is most of the compile time and of the optimized binary size. |
| `xml` | SOAP transport and the XML deserializer. Compiled with the `xml` feature. |

Inside `types/`:

| File | Contents |
|------|----------|
| `structs.rs` | Every VIM struct. The largest generated file. |
| `defaults.rs` | `Default` impls. Only with the `defaults` feature. |
| `traits.rs` | Traits for VIM types that have children, plus cross-trait casts and JSON hooks. |
| `enums.rs` | Generated enums. |
| `struct_enum.rs` | `StructType`, an enum of struct names used for cheap type comparisons (`child_of`). |
| `vim_object_trait.rs` | Common trait implemented by VIM data types. |
| `deserialize.rs`, `dyn_serialize.rs` | Polymorphic JSON for trait objects. |
| `boxed_types.rs` | Enum of boxed arrays and primitives used in dynamic slots (`ValueElements`). |
| `as_any.rs`, `vim_any.rs`, `convert.rs` | Hand-written (or generator-stable) casting helpers: `AsAny`, `VimAny`, `CastInto`. |

The client facade is described in [CLIENT.md](CLIENT.md). The trait and enum model is [TYPE_SYSTEM.md](TYPE_SYSTEM.md).

## `vim_build` source

| Piece | Role |
|-------|------|
| `vim_model` | A Rust-shaped API model. `loader.rs` converts the OpenAPI model into it. |
| `rs_emitter` | Renders `vim_model` into the `vim_rs` sources. |
| `printer.rs` | Thin writer used by the emitter. |

## Generating bindings

From the repository root:

```bash
cargo run -p vim_build
```

The generator reads `vim_build/data/`. The current binding target is **vSphere API 9.1.0.0**, file `vi_json_openapi_specification_v9_1_0_0.json`.

## Updating the OpenAPI specification

The canonical input is JSON. `DatastoreAccessible_enum` has the values `True` and `False`. In YAML those tokens are booleans, so a YAML source cannot be the input the generator sees.

Convert with `yq`, then check the enum:

```bash
yq -o=json eval vi-json.yaml > vi_json_openapi_specification_v9_1_0_0.json
```

On Windows, confirm the redirect wrote UTF-8 rather than UTF-16. In the JSON, `DatastoreAccessible_enum` must be the strings `"True"` and `"False"`. If they were emitted as JSON booleans, fix them before running `vim_build`.

A clean build of the `vim_rs` tests is typically a few minutes. See [TYPE_SYSTEM.md](TYPE_SYSTEM.md#compile-time).
