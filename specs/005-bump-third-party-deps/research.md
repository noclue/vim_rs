# Research: Third-Party Dependency Refresh

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

**Date**: 2026-10-15. Versions below are from crates.io and `cargo update --dry-run` on that date. Implementation re-runs the update and takes the newest compatible releases available then. Dry-run did not modify lockfiles.

## R1 — quick-xml 0.42.0

**Decision**: Widen `vim_rs` `quick-xml` from `"0.40"` to `"0.42"` and resolve **0.42.0**.

**Rationale**: `cargo update -p quick-xml` in `vim_rs` locks **0 packages**. The declaration `0.40` is `^0.40` (`>=0.40.0, <0.41.0`), and 0.40.1 (2026-05-15) is already the newest 0.40 release. The newest publish is **0.42.0** (2026-08-22). 0.41.0 (2026-06-29) is also outside the declaration, and it does not fix the nesting / namespace-depth DoS that 0.42.0 fixes (affected range 0.1.4–0.41.0). MSRV of 0.42.0 is Rust 1.86. CI uses stable; local rustc is 1.95.0. No project `rust-version` field is added.

**Alternatives considered**:

- Stay on 0.40.1. Misses the named bump and the security fix.
- Stop at 0.41.0. Still a declaration widen, still vulnerable, not the newest release.
- Enable quick-xml's `serde` feature. Rejected. `vim_rs` marshalling stays on miniserde (constitution).

**Behavior we accept and disclose**:

- `escape()` (used by `vim_rs/src/xml/ser.rs`) encodes `\r` as `&#13;`. Parsed values for fixtures without CR stay the same. This is not a consumer source break, so `vim_rs` stays 0.6.0.
- Default in-scope namespace bindings cap is 128 (was unbounded on 0.40). vSphere SOAP envelopes do not approach that. If a fixture hits `TooManyBindings`, raise `NamespaceResolver::set_max_namespace_bindings`.
- `Reader::from_reader` now rejects non-UTF-8. This crate uses `from_str` / `NsReader::from_str`, which already require UTF-8.

## R2 — quick-xml call-site migration

**Decision**: Edit hand-written `vim_rs/src/xml/de.rs` and `vim_rs/src/xml/client.rs`. Touch `soap.rs` or `ser.rs` only if they fail to compile.

**Rationale**: 0.42.0 replaces `&[u8]` names and event bodies with `&str` / `Cow<str>` and removes `BytesText::decode`. Current code compares local names to `b"type"` / `b"nil"` and UTF-8-decodes `as_ref()` results. After the bump those values are already strings. `Attribute::normalized_value(XmlVersion::Implicit1_0)` is the non-deprecated unescape path and stays.

**Alternatives considered**:

- A compatibility shim that re-wraps strings as bytes. Rejected. It keeps the old shape without any caller that still needs bytes.
- Regenerating bindings. Rejected. XML transport code is hand-written; generated `types/` and `mo/` do not import quick-xml.

## R3 — rmcp 3.5.1

**Decision**: Let `cargo update` in `mcp/` move `rmcp` **3.4.0 → 3.5.1** and `rmcp-macros` **3.4.0 → 3.5.1**. Leave `mcp/server/Cargo.toml` at `version = "3.4"`.

**Rationale**: `"3.4"` is `^3.4` (`>=3.4.0, <4.0.0`), so 3.5.1 is inside the declaration. Dry-run selects exactly those two plus anyhow. Upstream changelog 3.4.0 → 3.5.1 has protocol and header fixes and no listed Rust API break. Direct dependencies are unchanged in kind: `schemars ^1.0`, `tokio ^1`, `serde ^1`. MSRV stays 1.88, which 3.4.0 already required. `mcp/server/src/main.rs` uses `#[tool]`, `#[tool_router]`, `#[tool_handler]`, `ServerHandler`, and `ServiceExt`.

**Alternatives considered**:

- Raise the floor to `"3.5"` immediately. Unnecessary unless a compile fix calls a 3.5-only API. Do that only then.
- Jump to a 4.x line. None is published (newest is 3.5.1).

## R4 — anyhow 1.0.104

**Decision**: Let `cargo update` move anyhow **1.0.103 → 1.0.104** in `mcp/Cargo.lock` and `examples/Cargo.lock`. Leave every `anyhow = "1.0"` declaration as-is.

**Rationale**: `"1.0"` already allows any 1.x. 1.0.104 (2026-07-18) is the newest stable. MSRV stays 1.68. Dry-run updates one package in `examples/` and includes anyhow in the `mcp/` refresh. Direct users today: `examples/snippets`, `examples/vtui`, `mcp/server`, `mcp/api_database`, and the `mcp/data_processing/*` packages. They must resolve the same release (one lockfile per workspace, so this is automatic).

**Alternatives considered**:

- Pin `=1.0.104`. Rejected. The existing caret is the project style and still resolves the newest 1.x on the next update.

## R5 — Compatible refresh of every other library

**Decision**: Run `cargo update` (no `-p` filter) in each package root. Do not edit those manifests. Packages the resolver leaves behind a newer publish stay behind, because their declarations (or a parent's) do not allow the newer release.

**Rationale**: That is the spec's rule for everything except the three named libraries. Dry-run on 2026-10-15, after the index update:

| Root | Packages the update would lock |
|------|--------------------------------|
| `vim_rs` | 131 |
| `vim_macros` | 11 |
| `vim_build` | 33 |
| `openapi30` | 14 |
| `tls_rustls_only` | 104 |
| `examples` | same HTTP/TLS stack as `vim_rs`, plus examples-only crates (for example `tui-tree-widget` 0.24.0 → 0.24.1, `uuid` 1.23.4 → 1.28.0) |
| `mcp` | 188, including anyhow, rmcp, and rmcp-macros |

Direct-ish moves worth watching at compile time (still inside current declarations):

- `vim_rs`: `miniserde` 0.1.45 → 0.1.46, `reqwest` 0.13.4 → 0.13.5, `tokio` 1.52.3 → 1.53.3, `rustls` 0.23.41 → 0.23.46, `aws-lc-sys` 0.41.0 → 0.46.0, `hyper` 1.10.1 → 1.12.0
- `openapi30` lockfile: transitive `syn` 2.0.118 → 3.0.9 (`openapi30` does not depend on `syn` directly)
- `mcp`: `actix-web` 4.14.0 → 4.16.0, `tera` 2.0.0 → 2.4.0, `fastembed` 5.17.2 → 5.17.4, `ort` / `ort-sys` 2.0.0-rc.12 → 2.0.0-rc.13

**Left behind on purpose** (newer publish exists; declaration or graph does not allow it). These are not exceptions under FR-006. Do not widen them in this feature:

| Package | Resolved after compatible update | Newer publish not taken | Why |
|---------|----------------------------------|-------------------------|-----|
| `quick-xml` | 0.40.1 until R1 | 0.42.0 | Taken in R1 by widening `vim_rs` only |
| `base64` (`vim_rs`) | 0.22.1 | 0.23.1 | Declaration `"0.22"` |
| `syn` (`vim_macros`) | 2.x (`"2.0"`) | 3.0.9 | Proc-macro API. `openapi30`'s transitive syn may still resolve to 3 |
| `convert_case` (`vim_build`) | 0.11.0 | 0.12.0 | Declaration just aligned to 0.11 in 0.6.0 |
| `convert_case` (`mcp`) | 0.10.0 | 0.12.0 | Separate declaration; do not drive-by align |
| `actix-files` | 0.6.10 | 0.7.0 | Declaration `"0.6"` |
| `bincode` | 2.0.1 | 3.0.0 | 3.0.0 is an unmaintained placeholder that does not compile (0.6.0 changelog) |
| `fastembed` | 5.17.4 | 7.1.1 | Declaration `"5.8"` |
| `ort` / `ort-sys` | 2.0.0-rc.13 | 2.0.0-rc.14 | Resolver stopped at rc.13 |
| `generic-array` (examples) | 0.14.7 | 0.14.9 | Parent requirement does not allow 0.14.9 |

**Alternatives considered**:

- `cargo update` once at the repo root. There is no root workspace. Each root has its own lockfile.
- Bump every direct dependency to its absolute newest major. That was the 0.6.0 release. This feature's spec keeps non-named libraries inside current declarations.

## R6 — Failure policy

**Decision**: If a compatible bump fails to compile, or changes parsed VIM or JSON values, restore that package with `cargo update -p <name> --precise <previous>` and record it in the changelog exception list. Do not "fix" it by widening an unrelated declaration.

**Rationale**: FR-005 and FR-006. Likely candidates from the dry-run are transitive majors (`syn` 3, `getrandom` 0.3 → 0.4, `rand` 0.9 → 0.10, `r-efi` 5 → 6, `synstructure` 0.13 → 0.14) and the native `aws-lc-sys` 0.46 build. Our own source should not need edits for those, because we do not call them directly except miniserde, reqwest, and tokio, whose updates are minor/patch.

**Alternatives considered**:

- Commit a red tree and fix forward in a follow-up. Rejected. The refresh is done only when the quickstart matrix is green.

## R7 — What proves behavior is preserved

**Decision**: `cargo test --features xml` in `vim_rs`, including `vcsim_xml_regression` (vcsim `_XMLSchema-instance:type` and vCenter `xsi:type`). Default `cargo test` covers JSON/miniserde. `soap_integration` stays ignored unless parsed fixture values change, in which case it becomes required before accepting the quick-xml bump.

**Rationale**: Spec assumption: live vCenter is not the default gate. The committed vcsim capture is the SOAP behavior oracle. JSON transport does not use quick-xml.

**Alternatives considered**:

- Require a live ESXi run for every lockfile refresh. Rejected by the spec unless XML parse results move.

## R8 — Notes and version

**Decision**: Document under `CHANGELOG.md` `## [Unreleased]`. Do not bump `vim_rs` or `vim_macros` (both 0.6.0).

**Rationale**: FR-012 and FR-013. Consumers of the published library do not change source. The three named libraries get old→new lines. Every other lockfile move is one summary sentence. The quick-xml DoS fix is a Security note. Precise-pin exceptions from R6 are listed with the rejected version and the reason.

**Alternatives considered**:

- A new 0.6.1 release as part of this change. Out of scope unless an accepted behavior change forces it. The `\r` encoding note does not, because it is disclosed and does not force consumer source edits.
