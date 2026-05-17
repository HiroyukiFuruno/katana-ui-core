# kuw-workflow-guide

Operational rules and conventions for the `katana-ui-core` repository.

## Framework

**Floem only.** All widgets target [Floem](https://github.com/lapce/floem) (native Rust UI framework).
egui compatibility layers are out of scope.

## Widget Hierarchy

```
theme  ←  primitive  ←  composite  ←  layout
```

| Layer      | Path                        | May depend on               |
|------------|-----------------------------|-----------------------------|
| `theme`    | `src/theme/`                | nothing                     |
| `primitive`| `src/primitive/`            | `theme` only                |
| `composite`| `src/composite/<category>/` | `theme`, `primitive`        |
| `layout`   | `src/layout/`               | `theme`, `primitive`, `composite` |

### Composite subcategories

- `button/` — svg, text, icon_text
- `selector/` — toggle, segmented, select, color
- `input/` — text, search
- `indicator/` — tooltip, badge, key_cap

**Cross-subcategory references are forbidden.** A widget in `composite/button/` must not reference anything in `composite/selector/`. If a common part emerges, demote it to `primitive/` or `theme/`.

**Intra-subcategory references are allowed.** `composite/input/search/` may use `composite/input/text/`.

## Widget Extraction Policy

✅ **Extract** a widget when it:
- Has no dependency on Katana domain logic
- Is complete with Floem alone
- Could be useful in other Floem-based projects

❌ **Do NOT extract** widgets that involve:
- Markdown rendering / KMM integration
- Chat composer / vendor UI controls
- Linter result display
- Workspace file tree or Katana-specific panels

Reference implementations (inspect, do not copy):
- `../katana/crates/katana-ui/src/widgets/` (egui — adapt to Floem)
- `../katana-chat-ui/crates/katana-chat-ui-floem/src/widget/` (Floem)

## Storybook

Storybook lives at `storybook/` — **outside** `crates/`, **not** a workspace member.
Run with `just storybook`, check with `just storybook-check`.

**Every widget change must add a Storybook page** (`storybook/src/pages/<widget_name>.rs`).
Each page must show at minimum:
1. Default state
2. Key variants (prop differences)
3. Interactive states (hover / focus / disabled / active, where applicable)

## Per-Widget Change Checklist (DoD)

- [ ] Implementation under the correct hierarchy directory
- [ ] Storybook page added (`storybook/src/pages/<widget_name>.rs`)
- [ ] Unit tests (in `tests.rs` or `#[cfg(test)]` block)
- [ ] `just check` passes (fmt + types + lint + ast-lint + tests)
- [ ] `just storybook-check` passes

## Directory Size Rule

When a directory exceeds 10 files, split by concern.
Typical per-widget module layout:

```
<widget>/
├── mod.rs      — public re-exports
├── types.rs    — data types / enums
├── ops.rs      — business logic / state
├── view.rs     — Floem view construction
└── tests.rs    — unit tests (or inline #[cfg(test)])
```
