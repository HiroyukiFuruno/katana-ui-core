#!/usr/bin/env python3
import unittest
from pathlib import Path

from test_storybook_ui_harness import StorybookUiHarness, write_minimal_repo, write_text


class StorybookUiHarnessPublicOptionsTest(unittest.TestCase):
    def test_rejects_button_command_public_option_without_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/options.rs",
                "impl Button { pub fn command(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "button: public option `pub fn command` "
                "missing Storybook Inspector option `button.command`",
                failures,
            )

    def test_accepts_button_public_options_when_storybook_options_exist(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/options.rs",
                "impl Button {\n"
                "    pub fn command(mut self) -> Self { self }\n"
                "    pub fn keyboard_activation(mut self) -> Self { self }\n"
                "    pub fn icon_position(mut self) -> Self { self }\n"
                "    pub fn layout_preset(mut self) -> Self { self }\n"
                "}\n",
            )
            add_button_options(
                root,
                (
                    'StorybookUiOptionContract::new("button.command", "save", "open"),',
                    'StorybookUiOptionContract::new("button.keyboard_activation", "true", "false"),',
                    'StorybookUiOptionContract::new("button.icon_position", "leading", "trailing"),',
                    'StorybookUiOptionContract::new("button.layout_preset", "page", "dense"),',
                ),
            )

            failures = StorybookUiHarness(root).failures()

            self.assertFalse(
                any("button: public option" in failure for failure in failures),
                failures,
            )

    def test_rejects_icon_svg_public_option_without_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "impl Icon { pub fn icon_view_box(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "icon: public option `pub fn icon_view_box` "
                "missing Storybook Inspector option `icon.view_box`",
                failures,
            )

    def test_accepts_icon_svg_public_option_when_storybook_option_exists(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "impl Icon { pub fn icon_view_box(mut self) -> Self { self } }\n",
            )
            add_option_page(
                root,
                "icon",
                "ICON_OPTIONS",
                'StorybookUiOptionContract::new("icon.view_box", "16", "24"),',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "icon: public option `pub fn icon_view_box` "
                "missing Storybook Inspector option `icon.view_box`",
                failures,
            )

    def test_rejects_text_line_metrics_without_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/options.rs",
                "impl Text { pub fn line_metrics(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "text: public option `pub fn line_metrics` "
                "missing Storybook Inspector option `text.line_metrics`",
                failures,
            )

    def test_accepts_text_line_metrics_when_storybook_option_exists(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/options.rs",
                "impl Text { pub fn line_metrics(mut self) -> Self { self } }\n",
            )
            add_option_page(
                root,
                "text",
                "TEXT_OPTIONS",
                'StorybookUiOptionContract::new("text.line_metrics", "default", "compact"),',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "text: public option `pub fn line_metrics` "
                "missing Storybook Inspector option `text.line_metrics`",
                failures,
            )

    def test_rejects_toolbar_public_option_without_storybook_inspector_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/toolbar/options.rs",
                "impl ToolbarOptions { pub fn display_mode(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "toolbar: public option `pub fn display_mode` "
                "missing Storybook Inspector option `toolbar.display_mode`",
                failures,
            )

    def test_accepts_toolbar_public_option_when_storybook_inspector_option_exists(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/toolbar/options.rs",
                "impl ToolbarOptions { pub fn display_mode(mut self) -> Self { self } }\n",
            )
            add_option_page(
                root,
                "toolbar",
                "TOOLBAR_OPTIONS",
                'StorybookUiOptionContract::new("toolbar.display_mode", "IconLeading", "IconOnly"),',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "toolbar: public option `pub fn display_mode` "
                "missing Storybook Inspector option `toolbar.display_mode`",
                failures,
            )

    def test_rejects_window_control_public_field_without_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/selection/window_control_button_group/options.rs",
                "pub struct WindowControlButtonGroupOptions { pub controls: Vec<String> }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "window-control-button-group: public option `pub controls:` "
                "missing Storybook Inspector option `window_control.controls`",
                failures,
            )

    def test_accepts_multiline_window_control_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/selection/window_control_button_group/options.rs",
                "pub struct WindowControlButtonGroupOptions { pub controls: Vec<String> }\n",
            )
            add_option_page(
                root,
                "window-control-button-group",
                "WINDOW_CONTROL_OPTIONS",
                "StorybookUiOptionContract::new(\n"
                '    "window_control.controls",\n'
                '    "Close+Minimize+Maximize",\n'
                '    "Close",\n'
                "),",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "window-control-button-group: public option `pub controls:` "
                "missing Storybook Inspector option `window_control.controls`",
                failures,
            )

    def test_rejects_context_menu_public_option_without_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/context_menu/options.rs",
                "impl ContextMenu { pub fn placement_used(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "context-menu: public option `pub fn placement_used` "
                "missing Storybook Inspector option `context_menu.placement_used`",
                failures,
            )

    def test_rejects_accordion_public_option_without_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/disclosure/model.rs",
                "impl Accordion { pub fn trigger_area(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "accordion: public option `pub fn trigger_area` "
                "missing Storybook Inspector option `accordion.trigger_area`",
                failures,
            )

    def test_rejects_shortcut_cheatsheet_public_option_without_storybook_option(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/shortcut_cheatsheet.rs",
                "impl ShortcutCheatsheet { pub fn group_layout(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "shortcut-cheatsheet: public option `pub fn group_layout` "
                "missing Storybook Inspector option `shortcut_cheatsheet.group_layout`",
                failures,
            )

    def test_accepts_context_menu_public_option_when_storybook_option_exists(self) -> None:
        with self.temporary_repo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/context_menu/options.rs",
                "impl ContextMenu { pub fn placement_used(mut self) -> Self { self } }\n",
            )
            add_option_page(
                root,
                "context-menu",
                "CONTEXT_MENU_OPTIONS",
                'StorybookUiOptionContract::new("context_menu.placement_used", "BelowStart", "AboveEnd"),',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "context-menu: public option `pub fn placement_used` "
                "missing Storybook Inspector option `context_menu.placement_used`",
                failures,
            )

    def temporary_repo(self):
        return TemporaryStorybookRepo()


class TemporaryStorybookRepo:
    def __enter__(self) -> Path:
        import tempfile

        self.directory = tempfile.TemporaryDirectory()
        root = Path(self.directory.name)
        write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
        return root

    def __exit__(self, exc_type, exc, traceback) -> None:
        self.directory.cleanup()


def add_option_page(root: Path, page: str, array_name: str, row: str) -> None:
    path = root / "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs"
    source = path.read_text(encoding="utf-8")
    path.write_text(
        source.replace(
            '"button" => &BUTTON_OPTIONS,',
            f'"button" => &BUTTON_OPTIONS, "{page}" => &{array_name},',
        ),
        encoding="utf-8",
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/visual/storybook_ui_runtime_options.rs",
        f"const {array_name}: [StorybookUiOptionContract; 4] = ["
        f"{row}"
        'StorybookUiOptionContract::new("b", "0", "1"),'
        'StorybookUiOptionContract::new("c", "0", "1"),'
        'StorybookUiOptionContract::new("d", "0", "1"),];\n',
    )


def add_button_options(root: Path, rows: tuple[str, ...]) -> None:
    path = root / "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs"
    source = path.read_text(encoding="utf-8")
    path.write_text(
        source.replace(
            '"button" => &BUTTON_OPTIONS,',
            '"button" | "text-button" | "svg-button" | "icon-text-button" => &BUTTON_OPTIONS,',
        ).replace(
            "const BUTTON_OPTIONS: [StorybookUiOptionContract; 4] = [",
            f"const BUTTON_OPTIONS: [StorybookUiOptionContract; {4 + len(rows)}] = ["
            + "".join(rows),
        ),
        encoding="utf-8",
    )
    preset_path = root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs"
    preset_source = preset_path.read_text(encoding="utf-8")
    extra_labels = ",".join(f'"button option {index}"' for index in range(len(rows)))
    preset_path.write_text(
        preset_source.replace(
            '"button" => &["a","b","c","d"]',
            f'"button" => &["a","b","c","d",{extra_labels}]',
        ),
        encoding="utf-8",
    )


if __name__ == "__main__":
    unittest.main()
