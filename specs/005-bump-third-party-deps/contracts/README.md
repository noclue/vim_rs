# Contracts: Third-party dependency refresh

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Consumer-visible contract for this change. The core library's public API does not change.

## Version and features

| Item | Contract |
|------|----------|
| `vim_rs` version | Stays **0.6.0**. This refresh is not a release by itself. |
| `vim_macros` version | Stays **0.6.0**. |
| Cargo features | Unchanged. `default-client`, `defaults`, `xml`, `vcsim_compat` keep their current meaning. |
| quick-xml | Still optional, still only the `xml` feature (and `vcsim_compat`, which implies `xml`). Not part of the public API. |
| Public types and methods | No additions, removals, or signature changes. `Client::close` / `VimClient::close` from the in-progress 0.6.x work are untouched. |

Users of the published core library do not change their own source because of this refresh.

## Resolved versions that must move

| Package | From | To | Who resolves it |
|---------|------|-----|-----------------|
| `quick-xml` | 0.40.1 | **0.42.0** | `vim_rs` (`xml`), then `examples` via the path dependency |
| `rmcp` | 3.4.0 | **3.5.1** | `mcp/server` |
| `rmcp-macros` | 3.4.0 | **3.5.1** | transitive, `rmcp` `macros` feature |
| `anyhow` | 1.0.103 | **1.0.104** | `examples` and `mcp` lockfiles, same release in both |

If implementation day has a newer adoptable publish (a newer 0.42.x, 3.5.x, or 1.0.x), take that newer publish instead and say so in the changelog. Do not stay on the baseline versions above.

## Behavior

| Surface | Contract |
|---------|----------|
| JSON transport | Same requests, responses, and errors. miniserde stays the marshaller. A miniserde patch that changes parsed values is reverted. |
| XML transport, parsed values | Existing fixtures, including vcsim `_XMLSchema-instance:type` and vCenter `xsi:type`, deserialize to the same VIM values. |
| XML transport, serialized `\r` | A string containing CR may be written as `&#13;` instead of a raw CR. Disclosed in the changelog. Not a source break for consumers. |
| XML limits | Documents that declare more than 128 in-scope namespace bindings can now fail. vSphere envelopes are not in that class. If a real fixture hits the cap, the limit is raised in the reader; parsed values are not dropped. |
| MCP server | Same tools (`search`, `get`, starter guide, property paths). rmcp 3.5.x protocol fixes are internal. A tool-signature change would be a failed bump, not an accepted change. |
| Examples | Still build against the path `vim_rs`. anyhow 1.0.104 does not change sample behavior. |

## Changelog shape (`## [Unreleased]`)

Required sections, in the existing Keep-a-Changelog style:

1. **Dependencies** — a row each for quick-xml 0.40.1 → 0.42.0, rmcp 3.4.0 → 3.5.1, anyhow 1.0.103 → 1.0.104. One summary sentence for every other lockfile update across the seven package roots.
2. **Security** — quick-xml 0.42.0 fixes unbounded nesting / namespace-depth handling present in 0.40.1 through 0.41.0. The optional XML transport parses server-supplied SOAP, so the fix is in scope.
3. **Exceptions** — only if a package was precise-pinned back. Name the rejected version and the reason. Omit the section when there are none.

Do not list `bincode` 3, `fastembed` 7, `base64` 0.23, `syn` 3 (for `vim_macros`), or `convert_case` 0.12 as upgrades. They stay outside current declarations. See [research.md](../research.md) R5.

## Non-goals

- Regenerating VIM bindings or the OpenAPI input.
- Publishing a new crates.io release.
- Enabling quick-xml `serde`.
- Adding a `rust-version` field.
- Live vCenter as a default gate. It becomes required only when XML fixture values change and the bump is still under consideration.
- Unifying `convert_case` between `vim_build` (0.11) and `mcp` (0.10).
