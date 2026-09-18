# Data model: vSphere 9.1.0.0 binding regeneration

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

This feature transforms an **OpenAPI specification artifact** into **generated Rust surfaces** and a **rebuilt MCP database**. No runtime persistent storage. Entities below are **build artifacts** and **configuration**.

---

## Entities

### VI/JSON OpenAPI Definition (9.1.0.0)

| Attribute | Description |
|-----------|-------------|
| **Path** | `vim_build/data/vi_json_openapi_specification_v9_1_0_0.json` |
| **Source archive** | `vim_build/data/vi-json.yaml` (reference; not generator input) |
| **Prior version** | `vi_json_openapi_specification_v9_0_0_0_24798170.json` (reference only) |
| **Validation** | JSON parseable; `DatastoreAccessible_enum` uses string `"True"`/`"False"` |
| **Role** | Single authoritative input for all generated artifacts (FR-001) |

### `vim_model::Model` (intermediate)

| Attribute | Description |
|-----------|-------------|
| **Produced by** | `vim_model::load_vim_model()` from `openapi30::OpenAPI` |
| **Consumed by** | All `rs_emitter` modules + `FieldDataEmitter` |
| **Pruning** | `PRUNED_TYPES`: `MethodFault`, `Event` — descendants not emitted as separate types |
| **Lifecycle** | In-memory only during generator run |

### Generated Bindings (`vim_rs/src/`)

| Artifact | Path pattern | Mutable by hand? |
|----------|--------------|------------------|
| Structs | `types/structs.rs` | **No** |
| Enums | `types/enums.rs` | **No** |
| Traits | `types/traits.rs`, `types/*_trait.rs` | **No** |
| Ser/de | `types/deserialize.rs`, `types/dyn_serialize.rs` | **No** |
| Defaults | `types/defaults.rs` | **No** |
| Managed objects | `mo/*.rs`, `mo/mod.rs` | **No** |
| Supporting types | `struct_enum.rs`, `boxed_types.rs`, `vim_any.rs`, … | **No** |

**Relationships**: One OpenAPI schema → one or more Rust types/enums/traits following compositional inheritance and hybrid polymorphism (Principle II).

### Field-Path Reference Data

| Attribute | `vim_macros` | MCP server |
|-----------|--------------|------------|
| **Path** | `vim_macros/src/field_data.rs` | `mcp/server/src/field_data.rs` |
| **Emitter** | `FieldDataEmitter` (same run) | `FieldDataEmitter` (same run) |
| **Purpose** | `vim_retrievable!` / `vim_updatable!` path resolution | Property tree exploration |
| **Hand-editable** | **No** | **No** |

**Invariant**: Both files MUST be byte-identical in structure from the same generator run (same model, same emitter).

### MCP API Database

| Attribute | Description |
|-----------|-------------|
| **Path** | `mcp/data/api_database.bin` |
| **Git tracking** | **No** — `mcp/data/` is `.gitignore`d (R12) |
| **Built by** | `data-transformer` (`cargo run -p data-transformer --release --features cuda`) |
| **Inputs** | 9.1.0.0 OpenAPI JSON, `examples/` sources, embedding model cache |
| **Serialization** | `bincode` **3.x** (after FR-018 migration) |
| **Validated by** | `mcp/server/build.rs` staleness check at compile time |
| **Hand-editable** | **No** (rebuild via pipeline) |

### Dependency Refresh Matrix

| Package | From | To | Crates affected |
|---------|------|-----|-----------------|
| `phf` | 0.11 / 0.13 | **0.14** | `vim_macros`, `vim_rs`, `mcp/server` |
| `phf_codegen` | 0.11 | **0.14** | `vim_build` |
| `quick-xml` | 0.39 | **0.40** | `vim_rs` (`xml`) |
| `criterion` | 0.5 | **0.8** | `vim_rs` (dev) |
| `convert_case` | 0.8 | **0.11** | `vim_build` |
| `check_keyword` | 0.3 | **0.4** | `vim_build` |
| `bincode` | 1.3 | **3** | `mcp/server`, `data-transformer` |
| `tera` | 1.20 | **2** | `mcp/server` (`web-ui`) |
| Minor/patch | various | latest | All in-scope crates — see [research R16](./research.md#r16--dependency-refresh-matrix) |

**Invariant**: After refresh, `vim_rs/Cargo.lock` contains exactly one resolved `phf` major version (0.14.x) — SC-013.

### Release Artifacts (0.6.0)

| Crate / doc | Version field | Published? |
|-------------|---------------|------------|
| `vim_rs` | `0.6.0` | Yes (consumer library) |
| `vim_macros` | `0.6.0` | Yes (proc-macro companion) |
| `examples/*` | path dep `0.6.0` | No (local samples) |
| `CHANGELOG.md` | `[0.6.0]` section | Yes (release notes) |
| `mcp/README.md` | aligned mention | Yes (LLM guidance) |
| MCP sub-crates | unchanged `0.1.0` | Internal tooling |

**Invariant**: Version **0.5.1 MUST NOT** remain as a release target in any manifest or release doc (SC-011).

---

## State transitions (build pipeline)

```text
[vi-json.yaml] ──yq──► [9.1.0.0 JSON] ──validate──► [OK | fix spec]
                              │
                    bump phf_codegen 0.14 (vim_build)
                              │
                              ▼
                    vim_build::emit_vim_bindings
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
        vim_rs/src/   vim_macros/      mcp/server/
        (bindings)    field_data.rs    field_data.rs
                              │
              bump remaining deps + lockfiles
                              │
                              ▼
                    data-transformer (bincode 3)
                              │
                              ▼
              mcp/data/api_database.bin  (gitignored, local)
                              │
                              ▼
                    vim_mcp_server build (manual test)
                              │
                              ▼
              cargo test / cargo build (full validation)
```

**Failure triage** (FR-013): any transition failure → check `[9.1.0.0 JSON]` validity before modifying generator or hand-written crate code.

---

## Validation rules (from spec)

| Rule | Enforcement |
|------|-------------|
| FR-001 | Generator reads only 9.1.0.0 JSON |
| FR-002 | No manual edits post-generation |
| FR-003/004 | field_data from same `emit_vim_bindings` call |
| FR-005 | data-transformer run produces fresh `api_database.bin` |
| FR-006/007 | Cargo.toml versions = **0.6.0** |
| FR-015 | No 0.5.1 release-target references in manifests/docs (SC-011) |
| FR-016–FR-020 | Full dependency refresh; `phf` 0.14 alignment; lockfiles committed |
| FR-021 | `quickstart.md` documents local MCP DB regen |
| FR-008 | Full monorepo compile matrix passes |
| FR-013 | Spec triage documented on failure (SC-009) |
| SC-005 | Re-run generator → zero diff on generated files |
| SC-012 | `cargo outdated --root-deps-only` clean per in-scope crate |
| SC-013 | Single `phf` 0.14.x in `vim_rs/Cargo.lock` |
