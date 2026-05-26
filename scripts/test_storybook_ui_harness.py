#!/usr/bin/env python3
import tempfile
import unittest
import importlib.util
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("assert-storybook-ui-harness.py")
SPEC = importlib.util.spec_from_file_location("assert_storybook_ui_harness", MODULE_PATH)
assert SPEC is not None
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)
StorybookUiHarness = MODULE.StorybookUiHarness


def write_text(path: Path, source: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


class StorybookUiHarnessTest(unittest.TestCase):
    def test_accepts_required_pages_with_presets_options_and_inspector_route(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')

            self.assertEqual([], StorybookUiHarness(root).failures())

    def test_rejects_missing_option_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
            source = option_contract_source('"button" => &BUTTON_OPTIONS,')
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs",
                source.replace('"text" => &TEXT_OPTIONS,', ""),
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn("text: missing Storybook UI option contract", failures)

    def test_rejects_low_option_count(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
            path = root / "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs"
            source = path.read_text(encoding="utf-8")
            path.write_text(source.replace("new(\"d\", \"0\", \"1\"),", ""), encoding="utf-8")

            failures = StorybookUiHarness(root).failures()

            self.assertIn("text: Storybook option contract must cover at least 4 options", failures)

    def test_accepts_option_counts_from_split_source_file(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
            path = root / "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs"
            source = path.read_text(encoding="utf-8")
            path.write_text(
                source.split("const BUTTON_OPTIONS", 1)[0],
                encoding="utf-8",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/storybook_ui_runtime_options.rs",
                "const BUTTON_OPTIONS: [StorybookUiOptionContract; 4] = ["
                'StorybookUiOptionContract::new("a", "0", "1"),'
                'StorybookUiOptionContract::new("b", "0", "1"),'
                'StorybookUiOptionContract::new("c", "0", "1"),'
                'StorybookUiOptionContract::new("d", "0", "1"),];\n',
            )

            self.assertEqual([], StorybookUiHarness(root).failures())

    def test_rejects_low_preset_count(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
            path = root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs"
            source = path.read_text(encoding="utf-8")
            path.write_text(
                source.replace('"button" => &["a","b","c","d"]', '"button" => &["a","b","c"]'),
                encoding="utf-8",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn("button: Storybook presets must expose at least 4 tabs", failures)

    def test_rejects_missing_leaf_change(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
            leaf = root / "openspec/changes/storybook-page-text/proposal.md"
            leaf.unlink()

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "text: leaf change `storybook-page-text` missing openspec/changes/storybook-page-text/proposal.md",
                failures,
            )

    def test_rejects_missing_priority_number(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
            write_text(
                root
                / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md",
                "| priority | menu page | leaf change | reason |\n"
                "| --- | --- | --- | --- |\n"
                "| SB-001 | `text` | `storybook-page-text` | test |\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn("button: Storybook menu page missing priority number", failures)

    def test_rejects_missing_dedicated_draw_branch(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, option_arm='"button" => &BUTTON_OPTIONS,')
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/dedicated.rs",
                'fn draw_page(page: &str) { match page { "text" => text(), _ => draw() } }\n',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "button: split table says page-specific rendering exists, but draw_page has no branch",
                failures,
            )


def write_minimal_repo(root: Path, option_arm: str) -> None:
    write_text(
        root / "crates/katana-ui-core-storybook/src/requirements.rs",
        'const REQUIRED_PAGES: &[&str] = &["text", "button"];\nconst MIN_SINGLE_NODE: usize = 1;\n',
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
        'fn for_page(page: &str) { match page { "text" => &["a","b","c","d"], "button" => &["a","b","c","d"], _ => &[] } }\n',
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/visual/storybook_ui_option_contract.rs",
        option_contract_source(option_arm),
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/visual/inspector_rows.rs",
        "fn x() { storybook_ui_option_contract::settings_rows_for(); }\n",
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/visual/dedicated.rs",
        'fn draw_page(page: &str) { match page { "text" => text(), "button" => button(), _ => draw() } }\n',
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/catalog/story_paths_atoms.rs",
        'const PATHS: &[StoryPath] = &[StoryPath { page: "text" }, StoryPath { page: "button" }];\n',
    )
    write_text(
        root / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md",
        "| group | menu page | leaf change | input | status |\n"
        "| --- | --- | --- | --- | --- |\n"
        "| Atoms | `text` | `storybook-page-text` | test | page別描画あり |\n"
        "| Atoms | `button` | `storybook-page-button` | test | page別描画あり |\n",
    )
    write_text(
        root / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md",
        "| priority | menu page | leaf change | reason |\n"
        "| --- | --- | --- | --- |\n"
        "| SB-001 | `text` | `storybook-page-text` | test |\n"
        "| SB-002 | `button` | `storybook-page-button` | test |\n",
    )
    write_leaf_change(root, "storybook-page-text")
    write_leaf_change(root, "storybook-page-button")


def write_leaf_change(root: Path, change: str) -> None:
    write_text(root / f"openspec/changes/{change}/.openspec.yaml", "schema: spec-driven\n")
    write_text(root / f"openspec/changes/{change}/proposal.md", "# Proposal\n")
    write_text(root / f"openspec/changes/{change}/tasks.md", "# Tasks\n")
    write_text(root / f"openspec/changes/{change}/specs/{change}/spec.md", "## ADDED Requirements\n")


def option_contract_source(option_arm: str) -> str:
    return (
        "struct StorybookUiOptionContract;\n"
        "impl StorybookUiOptionContract { fn new(_: &str, _: &str, _: &str) -> Self { Self } }\n"
        "fn options_for_page(page: &str) { match page { "
        '"text" => &TEXT_OPTIONS, '
        f"{option_arm} _ => &[] }} }}\n"
        "const TEXT_OPTIONS: [StorybookUiOptionContract; 4] = ["
        'StorybookUiOptionContract::new("a", "0", "1"),'
        'StorybookUiOptionContract::new("b", "0", "1"),'
        'StorybookUiOptionContract::new("c", "0", "1"),'
        'StorybookUiOptionContract::new("d", "0", "1"),];\n'
        "const BUTTON_OPTIONS: [StorybookUiOptionContract; 4] = ["
        'StorybookUiOptionContract::new("a", "0", "1"),'
        'StorybookUiOptionContract::new("b", "0", "1"),'
        'StorybookUiOptionContract::new("c", "0", "1"),'
        'StorybookUiOptionContract::new("d", "0", "1"),];\n'
    )


if __name__ == "__main__":
    unittest.main()
