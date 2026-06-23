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
format_completion_payload = MODULE.format_completion_payload


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

    def test_complete_payload_is_false_when_leaf_queue_is_done_but_kuc_dod_has_handoff_items(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_fixture(root)
            write_text(root / "openspec/changes/storybook-page-theme-tokens/tasks.md", "- [x] done\n")
            write_text(
                root / "docs/reviews/2026-05-31-kuc-remaining-work-handoff.md",
                "- [/] P0-8 adapter: native raster parity remains under audit\n"
                "- [ ] P1-13 commit split: dirty worktree is unresolved\n",
            )

            payload = StorybookNextChangeResolver(root).completion_payload()

            self.assertFalse(payload["complete"])
            self.assertTrue(payload["leaf_queue_complete"])
            self.assertEqual("storybook_page_leaf_changes", payload["completion_scope"])
            self.assertFalse(payload["kuc_dod_complete"])
            self.assertEqual(2, len(payload["remaining_handoff_items"]))
            self.assertIn("P0-8 adapter", payload["remaining_handoff_items"][0])

    def test_payload_names_next_manual_acceptance_page_when_leaf_queue_is_done(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_fixture(root)
            write_text(root / "openspec/changes/storybook-page-theme-tokens/tasks.md", "- [x] done\n")
            write_text(
                root / "docs/reviews/2026-05-31-kuc-remaining-work-handoff.md",
                "- [ ] P0-1 text manual acceptance: pending\n",
            )
            write_manifest_with_manual_pending(root, "text")

            payload = StorybookNextChangeResolver(root).completion_payload()

            self.assertFalse(payload["complete"])
            self.assertNotIn("blocked_reason", payload)
            self.assertEqual("manual_acceptance_pending", payload["pending_reason"])
            self.assertEqual("text", payload["next_manual_acceptance_page"])
            self.assertEqual(["text"], payload["pending_manual_acceptance_pages"])
            self.assertEqual(
                "rtk cargo run --release -p katana-ui-core-storybook --bin "
                "katana-ui-core-storybook --locked -- --open-window text",
                payload["next_command"],
            )
            self.assertEqual("await_user_storybook_confirmation", payload["next_action"])

    def test_human_output_names_manual_acceptance_next_command(self) -> None:
        payload = {
            "complete": False,
            "leaf_queue_complete": True,
            "completion_scope": "storybook_page_leaf_changes",
            "kuc_dod_complete": False,
            "pending_reason": "manual_acceptance_pending",
            "next_manual_acceptance_page": "text",
            "next_command": "rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window text",
            "next_smoke_command": "rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 1 text",
            "manual_gate": "do not proceed to the next UI until this page is approved",
            "next_action": "await_user_storybook_confirmation",
        }

        output = format_completion_payload(payload)

        self.assertIn("completion_scope=storybook_page_leaf_changes", output)
        self.assertIn("kuc_dod_complete=false", output)
        self.assertNotIn("blocked_reason", output)
        self.assertIn("pending_reason=manual_acceptance_pending", output)
        self.assertIn("next_manual_acceptance_page=text", output)
        self.assertIn("--open-window text", output)
        self.assertIn("manual_gate=do not proceed to the next UI until this page is approved", output)
        self.assertIn("next_action=await_user_storybook_confirmation", output)

    def test_complete_payload_is_true_only_when_leaf_queue_and_kuc_dod_are_done(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_fixture(root)
            write_text(root / "openspec/changes/storybook-page-theme-tokens/tasks.md", "- [x] done\n")
            write_text(root / "docs/reviews/2026-05-31-kuc-remaining-work-handoff.md", "")

            payload = StorybookNextChangeResolver(root).completion_payload()

            self.assertTrue(payload["complete"])
            self.assertTrue(payload["leaf_queue_complete"])
            self.assertTrue(payload["kuc_dod_complete"])
            self.assertEqual([], payload["remaining_handoff_items"])

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


def write_manifest_with_manual_pending(root: Path, page: str) -> None:
    command = (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        f"katana-ui-core-storybook --locked -- --open-window {page}"
    )
    write_text(
        root / "docs/storybook-77ui-interaction-manifest.json",
        "{"
        '"ui":[{'
        f'"page":"{page}",'
        '"audit_status":"partial",'
        '"manual_acceptance_order":10,'
        '"dependency_layer":"foundation-text-selection",'
        '"depends_on":[],'
        '"required_operations":["pointer","drag","keyboard"],'
        f'"command":"{command}",'
        f'"smoke_command":"{command.replace(f"--open-window {page}", f"--open-window 1 {page}")}",'
        '"minimum_observation_frames":1,'
        '"acceptance_checks":["text_drag_selection"],'
        '"acceptance_observations":["drag text"],'
        '"manual_gate":"do not proceed to the next UI until this page is approved",'
        '"gaps":["manual_acceptance_pending: user confirmation is required"]'
        "}]"
        "}",
    )


if __name__ == "__main__":
    unittest.main()
