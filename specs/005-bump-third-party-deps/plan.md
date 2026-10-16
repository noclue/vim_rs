# Implementation Plan: Third-Party Dependency Refresh

**Branch**: `005-bump-third-party-deps` | **Date**: 2026-10-15 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/005-bump-third-party-deps/spec.md`

**Note**: Filled by `/speckit-plan`. Workflow: `.specify/templates/plan-template.md`.

## Summary

Refresh every in-repo lockfile to the newest crates.io releases still allowed by current compatibility declarations (`cargo update` in each package root). Then widen **quick-xml** from `0.40` to **`0.42`** and migrate the hand-written XML transport off the removed byte-oriented APIs. **rmcp** `3.4.0` → **`3.5.1`** and **anyhow** `1.0.103` → **`1.0.104`** fall out of the compatible refresh; their declarations stay. The core library version stays **0.6.0**. Generated bindings are not touched.

A 2026-10-15 dry-run is the expected shape, not a frozen pin list. Implementation re-runs `cargo update` and takes whatever newest compatible releases exist that day. See [research.md](./research.md).

## Technical Context

**Language/Version**: Rust 2021, stable toolchain (CI: `dtolnay/rust-toolchain@stable`; local rustc 1.95.0). No `rust-version` field exists and none is added. quick-xml 0.42 raises its own MSRV to **1.86**. rmcp 3.4.0 already required **1.88**, so 3.5.1 does not raise the MCP floor further.

**Primary Dependencies**: Third-party crates in seven lockfiles. Named targets: `quick-xml` 0.42.0 (`vim_rs` `xml` feature), `rmcp` 3.5.1 + `rmcp-macros` 3.5.1 (`mcp/server`), `anyhow` 1.0.104 (examples and MCP packages).

**Storage**: N/A (lockfiles and manifests only)

**Testing**: `cargo test` per package root; `cargo test --features xml` and the vcsim XML regression in `vim_rs`; MCP server build. Live vCenter / ESXi tests stay `#[ignore]` unless parsed SOAP values diverge from fixtures.

**Target Platform**: Cross-platform library and tools (Linux/macOS/Windows), same as today

**Project Type**: Rust library (`vim_rs`) plus sibling crates (`vim_macros`, `vim_build`, `openapi30`), examples workspace, MCP workspace, `tls_rustls_only`

**Performance Goals**: No substantial Principle III regression. quick-xml is optional, so the default `vim_rs` build should not grow because of the 0.42 migration. A native bump of `aws-lc-sys` 0.41 → 0.46 (via the compatible refresh) is the main build-time risk.

**Constraints**: No generated `types/` or `mo/` edits. No `vim_rs` / `vim_macros` version bump. No new or removed third-party libraries. Do not widen declarations except quick-xml, unless a named-target compile fix truly needs a higher floor (`rmcp` `3.5`) or an upstream crate demands a replacement. Do not adopt yanked releases. Do not move `bincode` to 3 (unmaintained placeholder; already rejected in the 0.6.0 changelog).

**Scale/Scope**: Seven lockfiles. One manifest edit (`vim_rs` quick-xml). Call-site edits in `vim_rs/src/xml/`. Changelog under `[Unreleased]`. Dry-run moved 131 (`vim_rs`), 11 (`vim_macros`), 33 (`vim_build`), 14 (`openapi30`), 104 (`tls_rustls_only`), and 188 (`mcp`) packages, plus the examples lockfile.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from specs** | Pass | Lockfiles, `vim_rs/Cargo.toml`, hand-written `vim_rs/src/xml/*`, changelog. No generator run. |
| **II. Complete type-safe surface** | Pass | Public VIM types and managed objects unchanged. |
| **III. Build-time & binary budgets** | Pass | No generated expansion. Default build does not link quick-xml. Measure only if a release build is substantially slower or larger; justify or revert that bump (FR-016). `aws-lc-sys` is the watch item. |
| **IV. Multi-transport parity** | Pass | JSON path is lockfile-only. SOAP path must pass `--features xml` tests, including `vcsim_xml_regression`. Live SOAP run only if parsed values change. |
| **V. Documented public surfaces** | Pass | No new public API. Changelog lists the three named old→new versions and one summary line. |
| **VI. Readable source** | Pass | 0.42 migration deletes `from_utf8` wrappers around names that are already `&str`. |
| **VII. Ecosystem tooling** | Pass | MCP server resolves rmcp 3.5.1; examples resolve anyhow 1.0.104; macros/generator/OpenAPI parser get compatible lockfile updates only. |
| **Async-first I/O** | Pass | Tokio stays 1.x (`1.52.3` → `1.53.3` in the dry-run). No new blocking paths. |
| **Marshalling (miniserde)** | Pass | `vim_rs` stays on miniserde (`0.1.45` → `0.1.46`). quick-xml `serde` feature is not enabled. |
| **Security & secrets** | Pass | quick-xml 0.42 is the fix for the nesting / namespace-depth DoS in 0.1.4–0.41.0. No credential or logging changes. |
| **Feature flags** | Pass | `xml` / `vcsim_compat` stay opt-in. No new feature. |
| **Errors** | Pass | XML failures still map to the existing `Error` type. |

**Post-design re-check**: [research.md](./research.md), [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), and [quickstart.md](./quickstart.md) match these gates. No violations.

## Project Structure

### Documentation (this feature)

```text
specs/005-bump-third-party-deps/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1
├── spec.md
├── checklists/requirements.md
└── tasks.md             # /speckit-tasks (not created by /speckit-plan)
```

### Source Code (repository root)

```text
vim_rs/Cargo.toml                 # quick-xml "0.40" → "0.42"
vim_rs/Cargo.lock                 # cargo update, then quick-xml 0.42
vim_rs/src/xml/de.rs              # str names; decode() removed
vim_rs/src/xml/client.rs          # str names on fault extraction
vim_rs/src/xml/ser.rs             # escape() still used; CR encoding documented
vim_rs/src/xml/soap.rs            # only if 0.42 fails to compile
vim_macros/Cargo.lock
vim_build/Cargo.lock
openapi30/Cargo.lock
tls_rustls_only/Cargo.lock
examples/Cargo.lock               # anyhow 1.0.104; quick-xml follows vim_rs
examples/snippets/Cargo.toml      # anyhow = "1.0" unchanged
examples/vtui/Cargo.toml          # anyhow = "1.0" unchanged
mcp/Cargo.lock                    # rmcp 3.5.1, anyhow 1.0.104, rest of refresh
mcp/server/Cargo.toml             # rmcp = "3.4" unchanged unless 3.5-only APIs are required
mcp/server/src/main.rs            # only if rmcp 3.5 fails to compile
CHANGELOG.md                      # [Unreleased] Dependencies + Security
```

**Structure Decision**: Seven independent package roots, each with its own lockfile. The only planned manifest edit is the quick-xml requirement on `vim_rs`. XML call sites live in hand-written `vim_rs/src/xml/`, not in generated code.

## Complexity Tracking

No constitution violations. The quick-xml `\r` → `&#13;` serialization change is disclosed in the changelog and does not require a core-library version bump (consumers do not change source). See research R1.

---

## Phase 0: Research

**Status**: Complete → [research.md](./research.md)

| ID | Topic | Outcome |
|----|-------|---------|
| R1 | quick-xml target | **0.42.0**. Widen `vim_rs` declaration `0.40` → `0.42`. 0.40.1 is already the newest 0.40.x; `cargo update -p quick-xml` locks 0 packages. |
| R2 | quick-xml migration | Names and text are `&str` / `Cow<str>`. Drop `from_utf8` and `BytesText::decode`. Keep `normalized_value(XmlVersion::Implicit1_0)`. Do not enable the serde feature. |
| R3 | rmcp target | **3.5.1** (and `rmcp-macros` 3.5.1) via `cargo update`. Declaration stays `"3.4"` (`^3.4` includes 3.5). schemars stays `^1.0`. |
| R4 | anyhow target | **1.0.104** via `cargo update` in `mcp/` and `examples/`. Declaration stays `"1.0"`. Same release in every direct user. |
| R5 | Everything else | `cargo update` in all seven roots. Do not widen other declarations. Record resolver leftovers (base64 0.23, syn 3, bincode 3, fastembed 7, …) as in-declaration, not skips. |
| R6 | Compile-risk edges | Transitive `syn` 3 in `openapi30`, `miniserde` 0.1.46, `aws-lc-sys` 0.46, `getrandom` 0.4 / `rand` 0.10. On failure, `--precise` back and record an exception. |
| R7 | Behavior gate | `vcsim_xml_regression` plus `cargo test --features xml`. Parsed VIM values must match. Live SOAP only if they do not. |
| R8 | Changelog / version | `[Unreleased]` only. `vim_rs` stays 0.6.0. Security note for the quick-xml DoS fix. |

No open **NEEDS CLARIFICATION** items.

---

## Phase 1: Design

**Status**: Complete → [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [quickstart.md](./quickstart.md)

### Order of work

1. Compatible `cargo update` in `vim_rs`, `vim_macros`, `vim_build`, `openapi30`, `tls_rustls_only`, `examples`, `mcp`.
2. Build and test. Precise-pin any package that breaks the build or changes parsed VIM/JSON values. Write the exception down.
3. Set `quick-xml` to `"0.42"` in `vim_rs/Cargo.toml`. Update `vim_rs` and `examples` lockfiles. Migrate `xml/de.rs` and `xml/client.rs`.
4. Confirm rmcp **3.5.1** and anyhow **1.0.104**. Fix `mcp/server` only if 3.5 does not compile. Raise the rmcp floor to `"3.5"` only if the fix needs a 3.5-only API.
5. Re-run the test matrix in [quickstart.md](./quickstart.md).
6. Changelog: three old→new lines, one summary line, security note, any exceptions.

### quick-xml call sites

| Current (0.40) | 0.42 |
|----------------|------|
| `std::str::from_utf8(name.as_ref())` | `name.as_ref()` (`AsRef<str>`) |
| `local.as_ref() == b"type"` / `b"nil"` | `== "type"` / `"nil"` |
| `e.decode()?` on `BytesText` | `&*e` or `e.as_ref()` (`decode` removed) |
| `std::str::from_utf8(&e)` on text/CDATA | `e.as_ref()` |
| `Attribute::normalized_value(XmlVersion::Implicit1_0)` | keep; still the non-deprecated unescape path |
| `quick_xml::escape::escape` | keep; `\r` becomes `&#13;` |

`Reader::from_str` / `NsReader::from_str` stay. Those inputs are already UTF-8, so the new `from_reader` encoding check does not apply.

**Post-design constitution re-check**: Pass. No Complexity Tracking rows.

---

## Phase 2: Implementation tasks (preview for `/speckit-tasks`)

Not created by this command. Suggested groups:

1. **Compatible refresh** — `cargo update` in all seven roots; fix or precise-pin breakages; no manifest edits.
2. **quick-xml 0.42** — manifest, lockfiles, `xml/de.rs`, `xml/client.rs`, xml tests.
3. **Named lockfile check** — rmcp 3.5.1, rmcp-macros 3.5.1, anyhow 1.0.104 everywhere it is declared.
4. **Notes** — `[Unreleased]` changelog: Dependencies, Security, exceptions.
5. **Verification** — quickstart matrix. No version bump, no generator run.

---

## Risk & mitigation

| Risk | Mitigation |
|------|------------|
| quick-xml 0.42 breaks `xsi:type` / vcsim prefix handling | `vcsim_xml_regression` is the gate. Keep local-name + namespace-URI checks; only the `&[u8]` vs `&str` comparison changes. |
| Namespace binding cap drops to 128 | vSphere envelopes are far below that. If a fixture returns `TooManyBindings`, raise `NamespaceResolver::set_max_namespace_bindings` rather than forking parser behavior. |
| `\r` in serialized XML changes wire bytes | Disclose in changelog. Do not bump `vim_rs` version. Fixtures without CR must parse to the same values. |
| Transitive majors (`syn` 3, `getrandom` 0.4, `rand` 0.10, `aws-lc-sys` 0.46) fail to compile | `cargo update -p <crate> --precise <previous>` and record the exception. Do not widen our declarations to chase them. |
| rmcp 3.5 breaks `#[tool]` / `ServerHandler` | Fix `mcp/server/src/main.rs`. Changelog between 3.4.0 and 3.5.1 lists no Rust API break; schemars stays 1.x. |
| Implementer bumps `bincode` to 3 or `fastembed` to 7 | Out of scope. `bincode` 3 is an unmaintained placeholder. `fastembed` 7 is outside `"5.8"`. |
| Principle III regression from `aws-lc-sys` | If the native crate dominates build time, revert that edge and record it. |

## Acceptance mapping (spec → plan)

| Criterion | Plan element |
|-----------|----------------|
| SC-001 | Quickstart build of all seven roots, `vim_rs --features xml`, MCP server |
| SC-002 | Quickstart test matrix; ignored live tests stay ignored |
| SC-003 | R1, R3, R4 targets: quick-xml 0.42.0, rmcp 3.5.1, anyhow 1.0.104 |
| SC-004 | R5 compatible update; leftovers in research are inside their declarations |
| SC-005 | No public API change; `vim_rs` version stays 0.6.0 |
| SC-006 | Changelog shape in [contracts/README.md](./contracts/README.md) |
| SC-007 | No generated growth; measure/revert only a substantial `aws-lc-sys` or xml-feature regression |
