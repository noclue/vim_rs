<!--
SYNC IMPACT REPORT
==================
Version change: (uninitialized template) → 1.0.0
Bump rationale: Initial ratification. Replaces the placeholder template
  with seven concrete, project-specific principles plus Quality &
  Compatibility Standards, Development Workflow & Release Gates, and a
  Governance section.

Modified principles (template → final):
  - [PRINCIPLE_1_NAME]                 → I. Generated From Authoritative Specs
  - [PRINCIPLE_2_NAME]                 → II. Complete, Type-Safe VIM Surface
  - [PRINCIPLE_3_NAME]                 → III. Build-Time & Binary-Size Budgets (NON-NEGOTIABLE)
  - [PRINCIPLE_4_NAME]                 → IV. Multi-Transport Parity Through Real-World Testing
  - [PRINCIPLE_5_NAME]                 → V. Documented Public Surfaces with Runnable Samples
  - (added)                            → VI. Readable Source at Scale
  - (added)                            → VII. Ecosystem Tooling Is Part of the Product

Added sections:
  - Quality & Compatibility Standards (replaces [SECTION_2_NAME])
  - Development Workflow & Release Gates (replaces [SECTION_3_NAME])

Removed sections: none.

Templates requiring updates:
  - .specify/templates/plan-template.md          ✅ aligned (Constitution
    Check is generated against this file at plan time; principles below are
    expressed as testable gates the plan command can lift directly)
  - .specify/templates/spec-template.md          ✅ aligned (no mandatory
    sections added or removed by this constitution)
  - .specify/templates/tasks-template.md         ✅ aligned (generic phase
    structure still applies; principles do not require new task categories)
  - .specify/templates/checklist-template.md     ✅ aligned (generic
    category structure still applies)
  - .specify/extensions/git/commands/*.md        ✅ aligned (no agent-
    specific names referenced; extension is generic)

Runtime guidance verified consistent with this constitution:
  - README.md, CLAUDE.md, examples/README.md, mcp/README.md,
    examples/vtui/README.md, vim_macros/README.md

Follow-up TODOs: none.
-->

# vim_rs Constitution

## Core Principles

### I. Generated From Authoritative Specs

The `vim_rs` bindings are mechanically generated from publicly available VMware
VIM API definitions (today: the VI/JSON OpenAPI specification under
`vim_build/data/`) by the in-tree generator (`vim_build`) and OpenAPI parser
(`openapi30`). Hand-edits to generated source files (notably
`vim_rs/src/types/structs.rs`, `vim_rs/src/types/enums.rs`,
`vim_rs/src/types/traits.rs`, `vim_rs/src/types/struct_enum.rs`,
`vim_rs/src/types/defaults.rs`, and the per-type files under `vim_rs/src/mo/`)
are forbidden. Required changes to the generated surface MUST be made in the
generator, the OpenAPI input, or the model layer (`vim_build/src/vim_model/`),
and re-emitted.

**Rationale**: Treating the generator as the single source of truth keeps the
binding consistent with the upstream VIM contract, makes spec upgrades
mechanical, and prevents drift across the ~200K LOC generated surface.

### II. Complete, Type-Safe VIM Surface

The full VMware VIM object-oriented data model and remoting model MUST be
accessible from Rust with full static type safety. Polymorphism is expressed
through the documented hybrid model: traits + compositional `Deref` for the
deep `DataObject`-rooted hierarchy; enums (`VimAny`, `ValueElements`,
`StructType`) for shallow but high-cardinality dynamic surfaces; pruned types
(`MethodFault`, `Event`) carry `type_` and `extra_fields_` to preserve
information without exploding generated code. Additions to the public surface
MUST preserve this property; intentional coverage gaps MUST be tracked
explicitly (e.g., `PRUNED_TYPES` in `vim_build/src/main.rs`), not silently
normalized.

**Rationale**: VIM's value comes from its rich, strongly-typed remote object
model. Erasing types or smuggling in stringly-typed escape hatches would
defeat the project's reason for existing.

### III. Build-Time & Binary-Size Budgets (NON-NEGOTIABLE)

Debug compile time, release compile time, and release binary size of the
`vim_rs` crate (and the consumer benchmark `examples/vtui`) are first-class
release artifacts and constraints on every change. Each release MUST measure
these against the previous release. A change that substantially regresses any
of them — for example, blanket-deriving heavy traits across the generated
surface, adopting a heavyweight serialization framework, or expanding
generated code by a multiplicative factor — MUST be justified in writing,
reworked to reduce impact, or yanked. Optional features (e.g., `defaults`,
`xml`, `vcsim_compat`) exist precisely to keep cost off the default path; new
costly capabilities MUST be feature-gated by default.

**Rationale**: This crate is large because the API is large. Every byte and
millisecond of build time has a real downstream cost on every dependent
project's CI and developer feedback loop. Calling this out as
NON-NEGOTIABLE prevents convenience changes from quietly accumulating.

### IV. Multi-Transport Parity Through Real-World Testing

`vim_rs` exposes the VIM API over both VI/JSON and SOAP/XML transports. Both
transports MUST be exercised before every release by:

1. Synthetic tests in `vim_rs/tests/` and per-crate unit tests
   (`cargo test --all-features`); and
2. The comprehensive `examples/snippets` and `examples/vtui` suite executed
   against a real vCenter (and, where relevant, ESXi over SOAP).

Public behavior MUST NOT silently diverge between transports. Where divergence
is unavoidable (e.g., XML transport not negotiating API release via Hello,
vcsim quirks under `vcsim_compat`), it MUST be documented in `README.md` and
the `vim_rs` rustdoc.

**Rationale**: The transport layer is exactly where regressions hide and where
"works on my mock" turns into outages. Real-vCenter execution is the only
honest gate for an enterprise-class binding.

### V. Documented Public Surfaces with Runnable Samples

Every public type, method, trait, macro, and feature in `vim_rs`,
`vim_macros`, `vim_build`, `openapi30`, and the MCP server MUST carry rustdoc
or equivalent documentation. Where the VIM API supplies prose documentation,
the generator MUST pass it through to rustdoc. Major capabilities MUST ship
with at least one runnable example under `examples/snippets/`,
`examples/macro_examples/`, or `examples/vtui/`. Onboarding documentation
(`README.md` and crate-level rustdoc) MUST remain sufficient for a new
developer to build a working program against vCenter without reading the
generator's source.

**Rationale**: A type-safe binding to a 10,000-symbol API is only as good as
its discoverability. Examples are the load-bearing teaching tool, especially
for the `vim_retrievable!` / `vim_updatable!` macros and `TaskTracker`
patterns where signatures alone are not self-explanatory.

### VI. Readable Source at Scale

Despite size, every file in this monorepo MUST remain easy to read and reason
about. Concretely: clear module boundaries (`core/`, `mo/`, `types/`, etc.),
idiomatic Rust (`Deref`-based compositional inheritance, `Arc<dyn VimClient>`,
`async`/`await`, `miniserde` for marshalling, `tokio` for I/O), meaningful
names mirroring VIM concepts, and minimal incidental complexity. Generated
output is held to the same readability standard as hand-written code; a
generator change that produces unreadable output is a defect even if the
output compiles. Comments explain non-obvious intent, trade-offs, or
constraints — not what the code already says.

**Rationale**: The codebase is too large for "trust me, the generator did
it" to be a sustainable answer when something goes wrong. Readability of
generated output is what makes the project maintainable across spec updates
and contributor turnover.

### VII. Ecosystem Tooling Is Part of the Product

The first-party tools that make the VIM API tractable — `vim_macros` (ergonomic
PropertyCollector usage), `mcp/server` (semantic search and exploration for
LLM-assisted development), `openapi30` (VIM-aware OpenAPI loader), and
`vim_build` (the generator) — are part of the product surface, not internal
scaffolding. They MUST follow the same engineering and quality standards as
the `vim_rs` crate: documented public APIs, build-time discipline, tests, and
release coordination. Breaking changes in these tools MUST be communicated in
`CHANGELOG.md` alongside `vim_rs` changes.

**Rationale**: A binding alone is not enough for VIM's scale. The macros,
generator, and MCP server are how teams (and AI assistants) actually become
productive against `vim_rs`; treating them as second-class would silently
shift cost back onto every consumer.

## Quality & Compatibility Standards

The following standards apply across the monorepo and are enforced alongside
the Core Principles:

- **Async-first I/O.** All network code uses `tokio` and `async`/`await`.
  Blocking I/O on the runtime is forbidden in library code paths.
- **Marshalling: miniserde, not serde.** The `vim_rs` crate intentionally uses
  `miniserde` to keep compile time and binary size under control (Principle
  III). New code paths MUST use `miniserde` and the helpers in
  `vim_rs::types::*`. Pulling in `serde`/`serde_json` for the `vim_rs` crate
  itself requires explicit constitutional justification.
- **Errors.** Library errors flow through `vim_rs::core::Error` and the
  `Result` alias defined alongside it. Faults from the server are surfaced as
  `MethodFault`-bearing errors (e.g., `Error::TaskFailed`) rather than being
  flattened into strings.
- **Security & secrets.** Credentials are never logged. `SessionManager`
  traffic is summary-only even under `WireLoggingMode::Detailed`. TLS
  verification is on by default; `insecure(true)` is opt-in and documented.
  No credentials may be committed to the repository.
- **Wire diagnostics.** Transport-level diagnostics use the dedicated targets
  `vim_rs::wire::json` and `vim_rs::wire::soap` so consumers can scope
  log-level changes without raising verbosity for the whole crate.
- **Feature flags.** Optional capabilities that affect compile time, binary
  size, or transport behavior (`defaults`, `xml`, `vcsim_compat`, …) MUST
  remain opt-in and MUST document their cost in `README.md` and rustdoc.
- **OpenAPI ingestion.** When updating the VI/JSON OpenAPI spec under
  `vim_build/data/`, the JSON form is the canonical input;
  `DatastoreAccessible_enum` values MUST be the strings `"True"` /
  `"False"` (not booleans) per `README.md` guidance.

## Development Workflow & Release Gates

- **Source-of-truth changes.** Changes to generated bindings MUST land as
  changes to `vim_build`, `openapi30`, or the OpenAPI input — never as
  hand-edits to generated files (Principle I).
- **Pre-merge validation.** Every change MUST pass `cargo build` and
  `cargo test` for `vim_rs`, `vim_build`, `vim_macros`, `openapi30`, and the
  examples workspace. Changes to client behavior, transport, or generated
  code SHOULD additionally execute the relevant `examples/snippets` binaries
  against a real vCenter.
- **Release gate (per Principle III).** Each release MUST publish, in
  `CHANGELOG.md` or release notes, a comparison against the previous release
  for: (a) `cargo build --release` time on a tracked configuration,
  (b) `cargo build` (debug) time, and (c) the optimized binary size of
  `examples/vtui`. Substantial regressions trigger the rework-or-yank
  obligation in Principle III.
- **Release gate (per Principle IV).** Each release MUST run the
  `examples/snippets` and `examples/vtui` suites against a real vCenter on
  both VI/JSON and SOAP transports (where the API supports SOAP) and confirm
  no behavioral regressions versus the previous release.
- **CHANGELOG discipline.** All user-visible changes to `vim_rs`,
  `vim_macros`, the MCP server, and `vim_build` go in `CHANGELOG.md` with
  notes on breaking changes, build-time/size impact, and feature-flag
  changes.
- **Specification-driven changes.** Non-trivial features and refactors SHOULD
  use the Spec Kit workflow under `.specify/` (`/speckit.specify`,
  `/speckit.plan`, `/speckit.tasks`, `/speckit.implement`). The plan
  command's "Constitution Check" derives gates directly from the principles
  above; violations belong in the plan's Complexity Tracking table.

## Governance

This constitution supersedes informal practices and prior conventions where
they conflict. Day-to-day runtime guidance for contributors and AI assistants
lives in `README.md`, `CLAUDE.md`, and crate-level rustdoc; those documents
MUST stay consistent with this file.

**Amendment procedure.** Amendments are proposed by editing
`.specify/memory/constitution.md` in a pull request. The PR MUST: (1) state
the proposed version bump and rationale, (2) update or add a Sync Impact
Report at the top of this file, and (3) update any dependent templates
(`.specify/templates/*.md`) and runtime guidance (`README.md`, `CLAUDE.md`,
crate-level rustdoc) that the amendment affects.

**Versioning policy.** This constitution is versioned with semantic
versioning:

- **MAJOR** — backward-incompatible governance changes, principle removals,
  or redefinitions that invalidate existing review/approval expectations.
- **MINOR** — a new principle or section, or a materially expanded principle
  (new MUST/SHOULD obligations).
- **PATCH** — clarifications, wording, typo fixes, or non-semantic
  refinements.

**Compliance review.** Every pull request MUST verify compliance with the
Core Principles. Reviewers MUST flag changes that:

- regress build time or binary size without justification (Principle III),
- weaken multi-transport coverage or skip the real-vCenter sample run for
  releases (Principle IV),
- introduce hand-edits to generated files (Principle I),
- erase type safety or normalize coverage gaps (Principle II),
- ship without rustdoc or examples for new public surface (Principle V),
- produce unreadable hand-written or generated code (Principle VI), or
- treat ecosystem tooling as second-class (Principle VII).

Justified deviations MUST be recorded in the affected change's plan
(Complexity Tracking) or `CHANGELOG.md` entry.

**Version**: 1.0.0 | **Ratified**: 2026-05-09 | **Last Amended**: 2026-05-09
