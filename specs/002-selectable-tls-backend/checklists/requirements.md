# Specification Quality Checklist: Selectable TLS Backend / Opt-Out of Default Client Configuration

**Purpose**: Validate specification completeness and quality before proceeding to planning
**Created**: 2026-06-01
**Feature**: [spec.md](../spec.md)

## Content Quality

- [ ] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

## Requirement Completeness

- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Success criteria are technology-agnostic (no implementation details)
- [x] All acceptance scenarios are defined
- [x] Edge cases are identified
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

## Feature Readiness

- [x] All functional requirements have clear acceptance criteria
- [x] User scenarios cover primary flows
- [x] Feature meets measurable outcomes defined in Success Criteria
- [ ] No implementation details leak into specification

## Notes

- Items marked incomplete require spec updates before `/speckit-clarify` or `/speckit-plan`.
- **Content Quality / "No implementation details"** is intentionally left unchecked: this is a library dependency-wiring and TLS-backend feature, so the spec necessarily references the HTTP client crate, its TLS backends, feature flags, and `core/client.rs`. These are the subject matter of the feature (the issue is itself a `Cargo.toml`/feature request), not incidental tech leakage. Success criteria remain outcome-based (dependency-tree contents, cross-compile, behavior parity).
- **Clarification resolved (2026-06-01)**: Turnkey TLS default strategy (FR-011/FR-012) — the turnkey path enables the HTTP client crate's default feature set, so on 0.13 the default backend becomes rustls; `vim_rs` does not pin native-TLS and does not expose per-backend toggles. Documented as a notable change. No `[NEEDS CLARIFICATION]` markers remain.
- **Clarification resolved (2026-06-01)**: Opt-out HTTP client (FR-003/FR-006/FR-009) — `ClientBuilder::new(server, http_client)` required when `default-client` is off; compile-time enforcement; `http_client()` override only on turnkey path.
- **Clarification resolved (2026-06-01)**: Cookie store (FR-007/FR-008) — `.cookie_store(true)` and `reqwest/cookies` only for `xml`/SOAP; VI/JSON uses `vmware-api-session-id` header, not cookies. Corrects earlier mistaken “cookies required for JSON” wording.
