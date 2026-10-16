# Feature Specification: Third-Party Dependency Refresh

**Feature Branch**: `005-bump-third-party-deps`

**Created**: 2026-10-14

**Status**: Draft

**Input**: User description: "we need to review 3rd party deps and bump them up. We should run `cargo update`. Notably I want to bump quick-xml, rmcp, anyhow"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Project runs on current compatible libraries (Priority: P1)

A maintainer reviews every outside library the repository depends on and brings each resolved version up to the newest release still allowed by that library's current compatibility declaration. After the refresh, the core library, generator, macros, samples, and MCP tooling still build and their existing automated tests still pass.

**Why this priority**: Stale resolved versions accumulate security and maintenance debt. A compatible refresh is the baseline the maintainer asked for, and it is useful even before any declared compatibility range is widened.

**Independent Test**: Compare resolved versions before and after the refresh. Every outside library is on the newest release its current declaration allows, or the review record explains why a newer allowed release was not adopted. A full build and the existing automated tests complete with no new failures.

**Acceptance Scenarios**:

1. **Given** the repository's current dependency declarations, **When** the compatible refresh is applied, **Then** each resolved outside library is the newest published release still inside its declaration, except where an exception is written down.
2. **Given** a newer allowed release that fails to build or changes behavior users of the core library can observe, **When** the refresh is finished, **Then** that release is not adopted, the previous acceptable release remains, and the review record states why.
3. **Given** the refresh is complete, **When** maintainers build every in-repository package and run existing automated tests, **Then** there are no new build errors or test failures caused by the version changes.
4. **Given** a library that was already on the newest allowed release, **When** the review is recorded, **Then** it is listed as already current and is not given a fake version change.

---

### User Story 2 - Named libraries move to the newest adoptable release (Priority: P1)

A maintainer specifically advances three libraries: **quick-xml** (XML reading and writing used by the optional XML transport), **rmcp** (the protocol library behind the MCP exploration server), and **anyhow** (error handling in sample programs and MCP tooling). Each one ends on the newest published release that the project can adopt, even when that means widening the compatibility declaration and adjusting in-project usage.

**Why this priority**: These three were called out by name. A general compatible refresh can leave them unchanged when the declaration is already tight (notably quick-xml, declared as 0.40, currently resolved at 0.40.1) or when only a newer major line exists (rmcp is declared as 3.4 and resolved at 3.4.0; anyhow is declared as 1.0 and resolved at 1.0.103).

**Independent Test**: For each of the three libraries, compare the resolved version before the change with the newest published release. The resolved version matches the newest release that still preserves behavior users of the core library can observe, and the areas that use that library still build and pass their tests.

**Acceptance Scenarios**:

1. **Given** a newer published release of quick-xml, rmcp, or anyhow than the one currently resolved, **When** the bump is applied, **Then** the project resolves that newer release, widening the declared compatibility range when the newer release sits outside it.
2. **Given** the newer release requires changes to how the project calls it, **When** the bump is applied, **Then** those in-project call sites are updated so the optional XML transport, the MCP server, and the sample programs still build.
3. **Given** the newest published release would change behavior that users of the core library can observe, **When** the bump is finished, **Then** the project uses the newest release that preserves that behavior, and the review record explains what was skipped and why.
4. **Given** anyhow is used by more than one package, **When** its version changes, **Then** every package that declares it resolves the same anyhow release.

---

### User Story 3 - Consumers and reviewers can see what changed (Priority: P2)

A downstream user of the core library, and a reviewer of the change, can tell which outside libraries moved, with the three named libraries listed as old version to new version. Users of the core library do not have to change their own programs because of this refresh.

**Why this priority**: A dependency refresh is only safe to ship when reviewers can see the delta and consumers are not surprised by a required source change.

**Independent Test**: Read the project changelog and the dependency review record. Confirm the three named libraries have old-to-new versions, other moves are summarized, and no core-library public behavior change is claimed or required of consumers.

**Acceptance Scenarios**:

1. **Given** the refresh is complete, **When** a reviewer reads the changelog, **Then** quick-xml, rmcp, and anyhow each appear as old version to new version, and every other resolved-version change is covered by one summary statement.
2. **Given** a consumer of the published core library, **When** they upgrade to the refreshed revision, **Then** their existing programs do not need source changes caused by this refresh.
3. **Given** an exception was recorded (a release skipped because it breaks the build or changes observable behavior), **When** a reviewer reads the review record, **Then** the skipped release and the reason are both present.

---

### Edge Cases

- **Already current**: If quick-xml, rmcp, or anyhow is already the newest adoptable release, the review says so. The work still completes the compatible refresh of every other outside library.
- **Breaking newer release**: Widening a declaration (for example a new quick-xml 0.x line, or an rmcp major) may force call-site edits. Those edits are in scope. Edits that would change what core-library users observe are not; the bump stops at the newest release that preserves that behavior.
- **Optional XML transport**: quick-xml is used only when the XML transport is enabled. That transport must still build and its existing tests must still pass after the bump. A quick-xml change that alters XML parsing outcomes users can observe is treated as an observable behavior change and is not adopted silently.
- **MCP server**: rmcp is used by the MCP exploration server. That server must still build after the bump. If the newer rmcp release changes tool behavior users of the server can observe, the review record must say so before that release is kept.
- **Split anyhow versions**: Sample programs and several MCP packages declare anyhow independently. The refresh must not leave them on different anyhow releases.
- **Transitive breakage**: A compatible refresh may move a library the project does not declare directly and break the build. The project is restored to a green build, either by adapting usage or by keeping the last working release with a written exception.
- **Yanked or known-vulnerable releases**: A yanked release is never selected. When a published security fix exists inside the adoption rules above, that fixed release is preferred over an older vulnerable one.
- **Build cost**: If the refresh makes the core library substantially slower to build or larger to ship, the regression is measured and either justified in writing or the responsible bump is reverted.
- **No public version bump**: This refresh does not by itself publish a new core-library version. A version bump happens only if a later decision finds that user-visible behavior actually changed.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Maintainers MUST review every third-party library resolved by the repository and record the resolved version before and after the refresh.
- **FR-002**: Each resolved third-party library MUST be updated to the newest published release still allowed by its current compatibility declaration, covering every independently built part of the repository (core library, macros, generator, OpenAPI parser, TLS sample, examples, and MCP tooling).
- **FR-003**: quick-xml, rmcp, and anyhow MUST each be updated to the newest published release that the project can adopt, including widening the declared compatibility range when the newest release lies outside the current declaration.
- **FR-004**: When a newer release of quick-xml, rmcp, or anyhow requires different call-site usage, in-project usage MUST be updated so the optional XML transport, the MCP exploration server, and the sample programs still build and pass their existing tests.
- **FR-005**: A candidate release MUST NOT be adopted when it changes behavior that users of the core library can observe. The newest release that preserves that behavior MUST be kept instead, and the skipped release MUST be named with a reason in the review record.
- **FR-006**: A candidate release that prevents a successful build, after reasonable in-project usage fixes, MUST NOT be adopted. The last working release MUST remain, with the failure recorded as an exception.
- **FR-007**: Yanked releases MUST NOT be selected. When a security fix for a resolved library is available within the rules in FR-002 and FR-003, that fixed release MUST be preferred.
- **FR-008**: Every package that directly declares anyhow MUST resolve the same anyhow release after the refresh.
- **FR-009**: The optional XML transport MUST build and its existing automated tests MUST pass after the quick-xml bump.
- **FR-010**: The MCP exploration server MUST build after the rmcp bump, and sample programs that use anyhow MUST build after the anyhow bump.
- **FR-011**: Existing automated tests for the core library, macros, generator, and OpenAPI parser MUST pass with no new failures caused by the refresh. Integration tests that require a live vCenter may remain ignored, and MUST NOT be structurally broken.
- **FR-012**: The project changelog MUST list old and new resolved versions for quick-xml, rmcp, and anyhow, and MUST summarize all other resolved-version changes in a single statement. Exceptions from FR-005 and FR-006 MUST appear in the review record.
- **FR-013**: This refresh MUST NOT require users of the published core library to change their own source. The core library's own version number MUST stay unchanged unless a skipped-release decision in FR-005 cannot be honored and a user-visible behavior change is explicitly accepted in the changelog.
- **FR-014**: The refresh MUST NOT add or remove third-party libraries except where adopting the newest release of quick-xml, rmcp, or anyhow requires a replacement the upstream library itself demands.
- **FR-015**: Generated API bindings MUST NOT be hand-edited or regenerated as part of this refresh.
- **FR-016**: If the refresh substantially increases core-library build time or shipped binary size, that regression MUST be measured and either justified in writing or removed by reverting the responsible bump.

### Key Entities

- **Third-party library**: An outside package the repository resolves, with a declared compatibility range and a concrete resolved version. The review cares about both.
- **Named bump targets**: quick-xml, rmcp, and anyhow. These may move past their current declarations; every other library stays inside its current declaration unless an exception says otherwise.
- **Dependency review record**: The before/after list for the three named libraries, the one-line summary of every other move, and any skipped release with its reason.
- **Affected product areas**: Optional XML transport (quick-xml), MCP exploration server (rmcp), and sample programs plus MCP tooling (anyhow).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After the refresh, 100% of in-repository packages build with zero errors, including the optional XML transport and the MCP exploration server.
- **SC-002**: Existing automated test suites complete with zero new failures attributable to the dependency changes.
- **SC-003**: The resolved versions of quick-xml, rmcp, and anyhow each match the newest published release that preserves core-library user-visible behavior, and each is either newer than the pre-refresh baseline (quick-xml 0.40.1, rmcp 3.4.0, anyhow 1.0.103) or documented as already that release.
- **SC-004**: Every other resolved third-party library matches the newest release allowed by its pre-refresh compatibility declaration, or has a written exception naming the rejected release and the reason.
- **SC-005**: A consumer of the core library can keep their existing source unchanged when taking this refresh. The core library version number is unchanged.
- **SC-006**: The changelog shows old-to-new versions for all three named libraries and one summary statement covering every other resolved-version change. The review record lists every exception.
- **SC-007**: Core-library build time and shipped size are not substantially worse than the pre-refresh baseline, or each substantial regression has a written justification.

## Assumptions

- The audience is project maintainers and downstream users of the core library. "Review" means a recorded before/after of resolved versions, not a new product feature.
- "Run a compatible update" means: move each resolved library to the newest published release still inside its current compatibility declaration, across every independently built part of the repository. That is the rule for libraries other than the three named targets.
- "Bump" quick-xml, rmcp, and anyhow means: take the newest published release even when it sits outside the current declaration (quick-xml is declared 0.40 and resolved 0.40.1; rmcp is declared 3.4 and resolved 3.4.0; anyhow is declared 1.0 and resolved 1.0.103). In-project call-site fixes for those three are in scope.
- Baseline resolved versions above are the versions in the repository at specification time. The implementation compares against the registry as it exists when the refresh is performed, not against these numbers alone.
- Observable behavior of the core library (including XML transport results when that transport is enabled) is preserved. Sample programs and the MCP server may need internal edits, but their existing jobs still succeed.
- This work does not regenerate API bindings, retarget the vSphere specification, or publish a new core-library version.
- Live vCenter runs are not required for completion. They become required only if the review finds that a kept quick-xml release can change XML transport results; in that case the existing SOAP-capable sample checks must be run before the release is accepted.
- Security fixes are adopted when they fall inside FR-002 and FR-003. This refresh is not a separate full advisory audit of the dependency tree.
- Build-time and binary-size comparison follows the project's existing release gate: a substantial regression must be justified or removed.
