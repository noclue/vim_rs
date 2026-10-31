# Contracts: Eval harness interfaces

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

External/operator-facing contracts for the PropertyCollector eval harness. Schemas are JSON Schema drafts for validation during implementation; YAML problems must be convertible to the same shapes.

| File | Purpose |
|------|---------|
| [problem.schema.json](./problem.schema.json) | Problem YAML documents |
| [result.schema.json](./result.schema.json) | Per-run `result.json` |
| [scorecard.schema.json](./scorecard.schema.json) | Aggregated A/B scorecard |
| [runner-cli.md](./runner-cli.md) | Offline grade + matrix CLI |

Goose recipe YAML follows upstream Goose recipe schema (not redefined here); Arm A/B differ only by whether the `vim-rs` stdio extension is listed.
