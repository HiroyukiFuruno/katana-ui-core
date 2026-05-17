# Directory Structure

## Crate Hierarchy

```
crates/katana-ui-core/src/
├── lib.rs
├── theme/                         # Design tokens — no deps
│   ├── color/mod.rs
│   ├── spacing/mod.rs
│   └── typography/mod.rs
├── primitive/                     # Atomic widgets — may use theme only
│   ├── icon/mod.rs
│   ├── spinner/mod.rs
│   └── text/mod.rs
├── composite/                     # Composed widgets — may use theme + primitive
│   ├── button/
│   │   ├── svg/mod.rs
│   │   ├── text/mod.rs
│   │   └── icon_text/mod.rs
│   ├── selector/
│   │   ├── toggle/mod.rs
│   │   ├── segmented/mod.rs
│   │   ├── select/mod.rs
│   │   └── color/mod.rs
│   ├── input/
│   │   ├── text/mod.rs
│   │   └── search/mod.rs
│   └── indicator/
│       ├── tooltip/mod.rs
│       ├── badge/mod.rs
│       └── key_cap/mod.rs
└── layout/                        # Layout containers — may use theme + primitive + composite
    ├── card/mod.rs
    ├── accordion/mod.rs
    ├── split/mod.rs
    ├── modal/mod.rs
    └── popover/mod.rs
```

## Dependency Direction

```
theme  ←  primitive  ←  composite  ←  layout
               ↑              ↑
           (may use)      (may use)
             theme       theme + primitive
```

Rules:
- **`theme/`** depends on nothing within this crate.
- **`primitive/`** may depend on `theme/` only.
- **`composite/<category>/`** may depend on `theme/` and `primitive/`. Cross-subcategory references (e.g., `button/` → `selector/`) are **forbidden**. Intra-subcategory references (e.g., `input/search/` → `input/text/`) are allowed.
- **`layout/`** may depend on `theme/`, `primitive/`, and any `composite/` subcategory.

Enforcement: by code review and convention. Mechanical enforcement (via `katana-ast-lint` dependency direction rule) is planned for a future kal release.

## Theme Token Convention

**Never hard-code numeric values (colors, sizes, spacing) directly in widget code.**

All widgets must reference values through `theme/` tokens:

```rust
// ❌ Forbidden
.style(|s| s.background(Color::rgb8(59, 130, 246)).padding(8.0))

// ✅ Required
let theme = use_theme();
.style(move |s| {
    s.background(/* convert theme.color.accent */)
     .padding(theme.spacing.sm)
})
```

- Color values → `theme.color.*`
- Spacing values → `theme.spacing.*`
- Font sizes / weights → `theme.typography.*`

Violation detection is planned via `katana-ast-lint` in a future kal release.

## Storybook

```
storybook/                         # Independent Cargo project — NOT a workspace member
├── Cargo.toml                     # [workspace] + bin crate
├── Cargo.lock                     # committed — independent lock from crates/
└── src/
    ├── main.rs                    # Floem app entry (sidebar + content area)
    └── pages/
        ├── mod.rs                 # re-exports one pub fn per page
        └── welcome.rs             # placeholder welcome page
```

**Convention:** one widget = one page (`pages/<widget_name>.rs`).

Run: `just storybook`  
Check (CI): `just storybook-check`

## Per-Widget Module Layout

When a widget directory grows beyond 10 files, split by concern:

```
<widget>/
├── mod.rs      # pub re-exports only
├── types.rs    # data types, enums, constants
├── ops.rs      # state management, business logic
├── view.rs     # Floem view construction
└── tests.rs    # unit tests (or inline #[cfg(test)])
```
