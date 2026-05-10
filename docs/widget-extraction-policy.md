# Widget Extraction Policy

This document defines the criteria for which widgets are in scope for `katana-ui-widget`.

## Extraction Criteria

A widget is **eligible** for extraction when it meets all of the following:

1. **Floem-complete**: The implementation relies only on Floem and standard Rust. No platform-specific extensions beyond what Floem provides.
2. **Domain-agnostic**: No dependency on Katana business logic (documents, editors, linter rules, chat sessions, etc.).
3. **Generally useful**: The widget could reasonably appear in other Floem-based applications — not something inherently tied to the Katana UX.

## Exclusion Examples

The following are explicitly **out of scope**, regardless of reuse potential:

| Widget / Feature | Reason for exclusion |
|-----------------|----------------------|
| Markdown rendering panel | Depends on KME (Katana Markdown Engine) |
| Chat composer | Vendor UI control with Katana-specific protocol |
| Linter result list | Katana AST lint domain object |
| Workspace file tree | Katana project model |
| Editor gutter / ruler | Katana document model |
| Language server status | LSP integration specific to Katana |

## Reference Implementations

These repos contain existing implementations to **inspect for spec**, but code must not be copied directly — re-implement from scratch in Floem:

- `../katana/crates/katana-ui/src/widgets/` — egui-based; adapt mental model to Floem
- `../katana/crates/katana-ui/src/views/` — egui views
- `../katana-chat-ui/crates/katana-chat-ui-floem/src/widget/` — Floem-based; closest reference

## KML Diff Exclusions

The following KML (katana-markdown-linter) conventions were reviewed and **intentionally not mirrored** to KUW:

| KML artifact | KUW decision |
|---|---|
| `docs/dogfooding.md` | N/A — KUW is a library, not self-applying |
| `docs/mcp-*.md` | N/A — KUW has no MCP server |
| `scripts/release/homebrew-publish-gate.sh` | N/A — no Homebrew distribution |
| `scripts/release/verify-npm-*.js` | N/A — Rust-only |
| `action.yml` | N/A — no GitHub Action distribution |
| `wrappers/` | N/A — no language wrapper layer |
