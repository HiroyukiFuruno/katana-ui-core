#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("next-storybook-page-change.py")
SPEC = importlib.util.spec_from_file_location("next_storybook_page_change", MODULE_PATH)
assert SPEC is not None
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)
StorybookNextChangeResolver = MODULE.StorybookNextChangeResolver


def write_text(path: Path, source: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


class StorybookNextChangeResolverTest(unittest.TestCase):
    def test_returns_first_incomplete_priority(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_fixture(root)

            row = StorybookNextChangeResolver(root).next_row()

            self.assertIsNotNone(row)
            assert row is not None
            self.assertEqual("SB-002", row.priority)
            self.assertEqual("storybook-page-theme-tokens", row.change)

    def test_returns_none_when_all_complete(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_fixture(root)
            write_text(root / "openspec/changes/storybook-page-theme-tokens/tasks.md", "- [x] done\n")

            row = StorybookNextChangeResolver(root).next_row()

            self.assertIsNone(row)

    def test_feedback_done_marker_is_not_incomplete(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_fixture(root)
            write_text(root / "openspec/changes/storybook-page-theme-tokens/tasks.md", "- [/] feedback done\n")

            row = StorybookNextChangeResolver(root).next_row()

            self.assertIsNone(row)


def write_fixture(root: Path) -> None:
    write_text(
        root / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md",
        "| priority | menu page | leaf change | 実装状況 | DoD 状況 | 次アクション | 並べ替え理由 |\n"
        "| --- | --- | --- | --- | --- | --- | --- |\n"
        "| SB-001 | `panel` | `storybook-page-panel` | page別描画あり | 未完了 | first | test |\n"
        "| SB-002 | `theme-tokens` | `storybook-page-theme-tokens` | page別描画あり | 未完了 | second | test |\n",
    )
    write_text(root / "openspec/changes/storybook-page-panel/tasks.md", "- [x] done\n")
    write_text(root / "openspec/changes/storybook-page-theme-tokens/tasks.md", "- [ ] todo\n")


if __name__ == "__main__":
    unittest.main()
