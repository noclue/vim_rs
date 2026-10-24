# vim-rs

Rust client for the VMware vSphere API. Async, type-safe bindings for the VI JSON API, plus an optional experimental SOAP/XML transport when you need to talk to ESXi directly.

The vSphere API is huge: a deep object model, thousands of types, and a property collector that expects you to name every field you want back. vim-rs turns that surface into a Rust library — generated types with the original VIM documentation in rustdoc, a small session client, and macros that turn property paths into structs.

[![How to Fit an Elephant in a Rusty Refrigerator — EuroRust 2025](https://img.youtube.com/vi/C87GIl1n8v4/hqdefault.jpg)](https://www.youtube.com/watch?v=C87GIl1n8v4)

**[How to Fit an Elephant in a Rusty Refrigerator](https://www.youtube.com/watch?v=C87GIl1n8v4)** — Kiril Karaatanasov, EuroRust 2025. How this binding is generated, and the compromises that keep a model this large compilable.

## Start here

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

    client.close().await?;
    Ok(())
}
```

The [crate guide](vim_rs/README.md) is the developer walkthrough: your own TLS stack, property retrieval and monitoring, tasks, and the XML transport. API reference is on [docs.rs](https://docs.rs/vim_rs). Release notes are in [`CHANGELOG.md`](CHANGELOG.md). Runnable programs are in [`examples/`](examples/README.md).

## What you get

- **Async** client on Tokio, shared as `Arc<Client>`
- **Generated types** for the vSphere 9.1 VI JSON API, with VIM docs rendered as rustdoc
- **`vim_retrievable!` and `vim_updatable!`** for one-shot reads and a local inventory cache
- **`TaskTracker`** for `*_Task` methods, via PropertyCollector
- **Hybrid type system** — traits where the inheritance is deep, enums where the branching is wide
- **MCP server** so a coding agent can search the API instead of guessing property paths

## Features

| Feature | Default | Cost |
|---------|---------|------|
| `default-client` | on | Builds a `reqwest` client with **rustls** (reqwest 0.13). Turn it off to bring your own TLS stack. |
| `xml` | off | Experimental SOAP/XML transport. About **+500 KB** on a release binary and about **+30–40%** debug build time. |
| `defaults` | off | `Default` for generated types, so struct-update syntax works. Adds a large amount of generated code and compile time. |
| `vcsim_compat` | off | Tolerant SOAP parsing for [govc vcsim](https://github.com/vmware/govmomi). Requires `xml`. |

How to enable each one, and the XML and vcsim caveats, is in the [crate guide](vim_rs/README.md).

SOAP/XML covers the core VIM API. VSAN, SPBM/PBM, SMS, VSLM, and EAM stay on VI/JSON. XML does not negotiate the API release through the Hello call; use `client.service_content().about.api_version` when the server’s capabilities matter. Full write-up: [XML transport](vim_rs/README.md#optional-xml-transport).

## MCP server

> Sonnet 4.5: "The MCP server doesn't just provide documentation—it provides understanding as a service. Semantic search + complete examples + common mistakes = AI that writes correct code on the first try."

> Gemini 3.0 Pro: "The tools ... solved the hardest problem in vSphere development: Discovery. ... With them, I got the structure right on the second try."

The API is deeply nested. The [MCP server](mcp/README.md) gives agents such as Claude Desktop and Cursor a semantic index of `vim_rs`:

- **Search** in natural language ("How do I power on a VM?")
- **Walk property paths** such as `summary.quick_stats.overall_cpu_usage` without inventing fields
- **Pull examples** for a specific type

Build it from source: [mcp/README.md](mcp/README.md).

## Layout

| Path | Role |
|------|------|
| [`vim_rs`](vim_rs/README.md) | The library. This is the crates.io package. |
| [`vim_macros`](vim_macros/README.md) | `vim_retrievable!` and `vim_updatable!`. Re-exported by `vim_rs`. |
| [`vim_build`](vim_rs/docs/REPOSITORY.md#generating-bindings) | Generates `vim_rs` from the VI JSON OpenAPI spec. |
| [`openapi30`](vim_rs/docs/REPOSITORY.md#crates) | OpenAPI 3.0 parser used by the generator, including VIM inheritance via `allOf`. |
| [`examples`](examples/README.md) | Snippets and the `vtui` inventory browser. |
| [`mcp`](mcp/README.md) | MCP server and the tools that build its API database. |

Design notes — type system, client architecture, generator, XML deserializer — are in [`vim_rs/docs`](vim_rs/docs/README.md).

Bindings are generated. To regenerate them or to ingest a newer OpenAPI spec (watch `DatastoreAccessible_enum`: the values must stay the strings `"True"` and `"False"`), see [Repository and code generation](vim_rs/docs/REPOSITORY.md).

## License

Apache-2.0. See [LICENSE](LICENSE).
