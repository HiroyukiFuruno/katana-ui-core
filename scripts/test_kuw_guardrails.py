#!/usr/bin/env python3
import tempfile
import unittest
from pathlib import Path

from kuw_guardrails import KuwGuardrails


class KuwGuardrailsTest(unittest.TestCase):
    def test_detects_storybook_box_leak(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            page = root / "storybook/src/pages/sample.rs"
            page.parent.mkdir(parents=True)
            page.write_text("fn page() { let _ = Box::leak(Box::new(\"x\")); }\n", encoding="utf-8")

            failures = KuwGuardrails(root).storybook_leak_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("Box::leak", failures[0])

    def test_detects_missing_openspec_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            task = root / "openspec/changes/sample/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text("- [x] 1.1 `storybook/src/pages/sample.rs` を追加\n", encoding="utf-8")

            failures = KuwGuardrails(root).openspec_evidence_failures()

            self.assertEqual(2, len(failures))

    def test_detects_runtime_api_gated_by_test_cfg(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            ops = root / "crates/katana-ui-widget/src/layout/split/ops.rs"
            ops.parent.mkdir(parents=True)
            ops.write_text(
                "#[cfg(test)]\npub(super) fn drag_ratio() -> f32 { 1.0 }\n",
                encoding="utf-8",
            )

            failures = KuwGuardrails(root).runtime_api_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("drag_ratio", failures[0])

    def test_detects_missing_interactive_callback(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            types = root / "crates/katana-ui-widget/src/composite/selector/toggle/types.rs"
            types.parent.mkdir(parents=True)
            types.write_text("pub struct ToggleProps { pub value: bool }\n", encoding="utf-8")

            failures = KuwGuardrails(root).callback_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("on_change", failures[0])

    def test_detects_file_length_without_review_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source = root / "crates/katana-ui-widget/src/layout/card/types.rs"
            source.parent.mkdir(parents=True)
            source.write_text("pub struct X;\n" * 260, encoding="utf-8")
            task = root / "openspec/changes/sample/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text(
                "- [x] 1.1 file-length 対応で `crates/katana-ui-widget/src/layout/card/types.rs` を追加\n",
                encoding="utf-8",
            )

            failures = KuwGuardrails(root).file_length_review_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("ops.rs", failures[0])


if __name__ == "__main__":
    unittest.main()
