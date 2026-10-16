# Data Model: Third-Party Dependency Refresh

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

No runtime objects are added. The model is the dependency review: what is declared, what is resolved, and what may change.

## Third-party library

An outside package resolved by one of the seven lockfiles.

| Field | Meaning |
|-------|---------|
| Name | crates.io package name |
| Declaration | Version requirement in a `Cargo.toml`, if this repo states one. Transitive packages have none of their own. |
| Resolved version | Concrete version in a lockfile |
| Package root | Which lockfile: `vim_rs`, `vim_macros`, `vim_build`, `openapi30`, `tls_rustls_only`, `examples`, `mcp` |
| Class | `named` (quick-xml, rmcp, anyhow) or `compatible` (everything else) |

**Validation**:

- A `compatible` library's resolved version is the newest publish its current declaration (and its parents' declarations) allow.
- A yanked version is never selected.
- A library is not added or removed unless quick-xml, rmcp, or anyhow upstream requires a replacement. This refresh does not expect that.

## Named bump target

| Library | Where declared | Declaration after | Resolved before | Resolved after | Declaration widened? |
|---------|----------------|-------------------|-----------------|----------------|----------------------|
| quick-xml | `vim_rs` (`xml` feature); `examples` lockfile follows the path dependency | `"0.42"` | 0.40.1 | 0.42.0 | Yes |
| rmcp | `mcp/server` | `"3.4"` (or `"3.5"` only if a compile fix needs a 3.5-only API) | 3.4.0 | 3.5.1 | No |
| rmcp-macros | transitive of rmcp `macros` feature | (none) | 3.4.0 | 3.5.1 | No |
| anyhow | `examples/snippets`, `examples/vtui`, `mcp/server`, `mcp/api_database`, `mcp/data_processing/*` | `"1.0"` | 1.0.103 | 1.0.104 | No |

**Validation**:

- Each named resolved version is the newest adoptable publish, or the review says the baseline was already that publish.
- Every direct anyhow user resolves **the same** anyhow release.
- quick-xml stays optional. Default `vim_rs` builds do not enable it.

**Relationship**: quick-xml is used only by the optional XML transport (`vim_rs/src/xml/`). rmcp is used only by the MCP server binary. anyhow is used by samples and MCP tooling, not by `vim_rs`'s public error type.

## Dependency review record

The changelog entry plus any precise-pin exceptions. It is not a new file format.

| Field | Meaning |
|-------|---------|
| Named lines | old → new for quick-xml, rmcp, anyhow |
| Summary line | One sentence covering every other resolved-version change |
| Security note | quick-xml 0.42 fixes the nesting / namespace-depth DoS present through 0.41.0 |
| Exception | Package, rejected version, previous version kept, reason (build failure or parsed-value change) |
| Intentionally not widened | The table in [research.md](./research.md) R5. These are not exceptions. |

**Validation**:

- An exception names both the rejected release and the reason.
- "Already current" is used only when the resolved version equals the newest adoptable publish. That is not expected for the three named libraries on this date.

## Affected product area

| Area | Libraries that touch it | What must remain true |
|------|-------------------------|------------------------|
| Core library, JSON transport | miniserde, reqwest, tokio, rustls (compatible refresh) | Public types, `Client` / `VimClient`, and JSON tests unchanged in behavior. Version stays 0.6.0. |
| Optional XML transport | quick-xml 0.42 | `cargo test --features xml` passes. vcsim and `xsi:type` fixtures parse to the same VIM values. |
| Examples | anyhow 1.0.104, plus the same HTTP stack as `vim_rs` | Sample crates build. anyhow release matches MCP. |
| MCP server and data tools | rmcp 3.5.1, anyhow 1.0.104, actix-web, tera, fastembed (compatible) | `vim_mcp_server` builds. Tool macros still expand. `bincode` stays 2.x. |
| Macros, generator, OpenAPI parser | compatible refresh only | `cargo test` in each root passes. `vim_macros` stays on `syn` 2. |

## State transitions

```text
declared + locked
    → compatible cargo update          (all seven lockfiles)
    → build/test
        → green: keep
        → red or parsed-value change: precise-pin back, record exception
    → widen quick-xml to 0.42          (vim_rs manifest + vim_rs and examples lockfiles)
    → migrate xml call sites
    → xml tests
        → green: keep 0.42.0
        → parsed values differ: do not paper over; stop that release and record why
    → changelog [Unreleased]
```

No transition changes `vim_rs` or `vim_macros` package versions, generated sources, or the set of Cargo features.
