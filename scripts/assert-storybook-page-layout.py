#!/usr/bin/env python3
from pathlib import Path
import sys

STORYBOOK_FILES = (
    Path("storybook/Cargo.toml"),
    Path("storybook/src/main.rs"),
    Path("crates/katana-ui-core-storybook/Cargo.toml"),
    Path("crates/katana-ui-core-storybook/src/lib.rs"),
    Path("crates/katana-ui-core-storybook/src/main.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/atoms.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/layouts.rs"),
    Path("crates/katana-ui-core-storybook/src/panel.rs"),
    Path("crates/katana-ui-core-storybook/src/requirements.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/card.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/modal.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/render.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/runtime.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/canvas.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/text.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/types.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window.rs"),
)


def main() -> int:
    source = "\n".join(path.read_text(encoding="utf-8") for path in STORYBOOK_FILES)
    required = (
        "StorybookPanel::verify_theme_variants",
        "ThemeSnapshot::light()",
        "ThemeSnapshot::dark()",
        "StorybookStyleSheet",
        "styled_story_roots",
        "StorybookVisual",
        "--visual-snapshot",
        "--open-modal-window",
        "--runtime-regression",
        "modal_window_opened={}",
        "state_reflected={}",
        "overlay_rendered={}",
        "modal_plan_same_display",
        "render_summary()",
        "katana-ui-core-storybook:",
    )
    missing = [token for token in required if token not in source]
    forbidden = (
        "katana-ui-core-floem",
        "katana_ui_core_floem",
        "floem",
        "floem::",
        "Application::new()",
    )
    leaked = []
    for path in STORYBOOK_FILES:
        candidate = path.read_text(encoding="utf-8")
        for token in forbidden:
            if token in candidate:
                leaked.append(f"{path}:{token}")
    if missing:
        print("storybook core-only layout lint failed", file=sys.stderr)
        for token in missing:
            print(f"- missing token: {token}", file=sys.stderr)
        return 1
    if leaked:
        print("storybook must not render through Floem", file=sys.stderr)
        for token in leaked:
            print(f"- forbidden token: {token}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
