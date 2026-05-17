# Contributing to katana-ui-core

Thank you for your interest in contributing!

## Development Setup

```bash
# Install development dependencies
cargo install just
just ast-lint-install

# Run quality checks
just check
```

## Workflow

1. Fork the repository and create your branch from `master`.
2. Make your changes, following the widget hierarchy rules (see `docs/directory-structure.md`).
3. Ensure `just check` passes (fmt, types, lint, ast-lint, tests).
4. Open a pull request.

## Widget Hierarchy

See `docs/directory-structure.md` for the full hierarchy and dependency direction rules.

## Storybook

When adding a new widget, add a corresponding page to `storybook/src/pages/`. Run with:

```bash
just storybook
```

## Code of Conduct

Be respectful and constructive in all interactions.
