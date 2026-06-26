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

## Generated surface contract

| Surface | Regeneration | Hand-edits allowed |
|---------|--------------|-------------------|
| `vim_rs/src/types/*` | `vim_build` generator | **Never** |
| `vim_rs/src/mo/*` | `vim_build` generator | **Never** |
| `vim_macros/src/field_data.rs` | Same generator run | **Never** |
| `mcp/server/src/field_data.rs` | Same generator run | **Never** |
| `mcp/data/api_database.bin` | `data-transformer` | **Never** (rebuild) |

**Idempotency**: Running the generator twice against unchanged input MUST produce identical generated output (SC-005).

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
| MCP server | `cargo build -p vim_mcp_server` after data-transformer | SC-007 |
| Regen idempotency | Second generator run → no diff | SC-005 |
| Changelog | `[0.6.0]` entry with API version + breaking changes | SC-006 |
| Build metrics | vs 0.5.0 baseline documented | SC-008 |
| Semver audit | No `0.5.1` in manifests/release docs | SC-011 |

## Non-goals (stable for 0.6.0)

- No change to transport layer (VI/JSON, SOAP/XML), TLS, or client builder behavior.
- No change to `PRUNED_TYPES` unless generator is blocked.
- No mandatory live vCenter test as part of this release contract (recommended post-release per constitution Principle IV).

## Consumer upgrade path

```toml
vim_rs = "0.6.0"
vim_macros = "0.6.0"  # if used directly
```

Review CHANGELOG for spec-driven breaking symbol changes before upgrading automation that references specific VIM types or managed-object methods. Upgrading from **0.5.0** requires addressing struct literal breaks (e.g. `EventFilterSpec` new fields) or enabling the `defaults` feature.
