# Contracts: vSphere 9.1.0.0 release (0.6.0)

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Public contract for consumers and maintainers of the **0.6.0** release targeting **vSphere API 9.1.0.0**.

## Version contract

| Artifact | Version | Notes |
|----------|---------|-------|
| `vim_rs` crate | **0.6.0** | Primary consumer library |
| `vim_macros` crate | **0.6.0** | Must match `vim_rs` dependency version |
| VIM API target | **9.1.0.0** | VI/JSON OpenAPI specification generation source |

**Semver**: **Minor breaking release** (not 0.5.1 patch). The 9.1.0.0 spec diff introduces compile-breaking changes on generated structs (e.g. new fields on `EventFilterSpec`) that require consumer source updates or the `defaults` feature. All such changes MUST be listed in CHANGELOG under `[0.6.0]`. Version **0.5.1 MUST NOT** be published.

## Dependency contract (0.6.0)

| Package | Prior | Target | Scope |
|---------|-------|--------|-------|
| `phf` | 0.11 / 0.13 | **0.14** | `vim_macros`, `vim_rs`, `mcp/server` |
| `phf_codegen` | 0.11 | **0.14** | `vim_build` |
| `quick-xml` | 0.39 | **0.40** | `vim_rs` (`xml` feature) |
| `criterion` | 0.5 | **0.8** | `vim_rs` dev-deps |
| `convert_case` | 0.8 | **0.11** | `vim_build` |
| `check_keyword` | 0.3 | **0.4** | `vim_build` |
| `bincode` | 1.3 | **3** | `mcp/server`, `data-transformer` |
| `tera` | 1.20 | **2** | `mcp/server` (`web-ui` feature) |

Minor/patch bumps across all in-scope crates per [research R16](../research.md#r16--dependency-refresh-matrix).

**CHANGELOG obligation** (FR-011): each major bump above listed as **old→new** in `[0.6.0]`; minor/patch summarized in one line.

**Lockfile obligation** (FR-020): in-scope `Cargo.lock` files committed; `vim_rs/Cargo.lock` MUST NOT resolve multiple `phf` majors (SC-013).

## Generated surface contract

| Surface | Regeneration | Hand-edits allowed | Git tracked |
|---------|--------------|-------------------|-------------|
| `vim_rs/src/types/*` | `vim_build` generator | **Never** | Yes |
| `vim_rs/src/mo/*` | `vim_build` generator | **Never** | Yes |
| `vim_macros/src/field_data.rs` | Same generator run | **Never** | Yes |
| `mcp/server/src/field_data.rs` | Same generator run | **Never** | Yes |
| `mcp/data/api_database.bin` | `data-transformer` (local) | **Never** (rebuild) | **No** (`.gitignore`) |

**Idempotency**: Running the generator twice against unchanged input MUST produce identical generated output (SC-005).

**PHF alignment**: Regenerate PHF-backed files only after `phf_codegen` 0.14 is wired in `vim_build` (SC-014).

## OpenAPI input contract

| Requirement | Detail |
|-------------|--------|
| Canonical file | `vim_build/data/vi_json_openapi_specification_v9_1_0_0.json` |
| Format | JSON (converted from VMware YAML via `yq`) |
| Enum strings | `DatastoreAccessible_enum` values MUST be `"True"` / `"False"` strings, not JSON booleans |
| Failure triage | Spec defects fixed at input layer; not patched into generated Rust (FR-013) |

## Pruned types (unchanged)

| Type | Behavior |
|------|----------|
| `MethodFault` | `type_` + `extra_fields_` for descendants |
| `Event` | `type_` + `extra_fields_` for descendants |

Descendant types in these hierarchies are NOT emitted as separate Rust structs (existing `PRUNED_TYPES` policy).

## Validation gates (release checklist)

| Gate | Command / check | Spec ref |
|------|-----------------|----------|
| Core build | `cargo build -p vim_rs` | SC-001 |
| All features | `cargo build -p vim_rs --all-features` | SC-002 |
| Core tests | `cargo test -p vim_rs` | SC-004 |
| Macro tests | `cargo test -p vim_macros` | SC-004 |
| Examples | `cargo build --bins` in `examples/` | SC-001 |
| MCP server | Local data-transformer + `cargo build -p vim_mcp_server` | SC-007 |
| Regen idempotency | Second generator run → no diff | SC-005 |
| PHF lockfile | Single `phf` 0.14.x in `vim_rs/Cargo.lock` | SC-013 |
| Dependency freshness | `cargo outdated --root-deps-only` per in-scope crate | SC-012 |
| Changelog | `[0.6.0]` with API version, breaks, **major dep lines**, minor summary | SC-006 |
| Build metrics | vs 0.5.0 baseline documented | SC-008 |
| Semver audit | No `0.5.1` in manifests/release docs | SC-011 |

## Non-goals (stable for 0.6.0)

- No change to transport layer (VI/JSON, SOAP/XML), TLS, or client builder behavior.
- No change to `PRUNED_TYPES` unless generator is blocked.
- No mandatory live vCenter test as part of this release contract (recommended post-release per constitution Principle IV).
- No committing `mcp/data/api_database.bin` (local artifact only).

## Consumer upgrade path

```toml
vim_rs = "0.6.0"
vim_macros = "0.6.0"  # if used directly
```

Review CHANGELOG for:

1. Spec-driven breaking symbol changes (e.g. `EventFilterSpec` new fields)
2. Major third-party dependency bumps affecting your transitive graph

Upgrading from **0.5.0** requires addressing struct literal breaks or enabling the `defaults` feature.

## MCP maintainer note

After upgrading to 0.6.0 sources, regenerate the local API database before building `vim_mcp_server`:

```bash
cd mcp
cargo run -p data-transformer --release --features cuda
cargo build --release -p vim_mcp_server
```

See [quickstart.md](../quickstart.md) Step 4.
