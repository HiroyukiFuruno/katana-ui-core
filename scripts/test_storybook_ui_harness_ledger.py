#!/usr/bin/env python3
from __future__ import annotations

import tempfile
import unittest
import json
from pathlib import Path

from storybook_ui_harness_ledger import StorybookUiHarnessLedger


class StorybookUiHarnessLedgerTest(unittest.TestCase):
    def test_rejects_progress_bar_ledger_without_live_tick_and_native_matrix(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            ledger = root / "docs" / "storybook-77ui-deep-audit-ledger.md"
            ledger.parent.mkdir()
            ledger.write_text(
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |\n"
                "| 19 | progress-bar | percent 証跡あり。 | sync | core public API | 実証済み |\n",
                encoding="utf-8",
            )

            failures = StorybookUiHarnessLedger(root).failures()

            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include target/manual-ui-probe/native-matrix-expanded-v3/summary.json",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_preview_click",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_timed_tick_advances_via_core_progress_action",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_timed_tick_cycles_after_reaching_maximum",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_live_audit_reports_timed_tick_progress_contract",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_live_audit_reports_timed_cycle_after_maximum",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_live_audit_reports_indeterminate_segment_motion",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_indeterminate_segment_moves_on_runtime_tick",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_window_runtime_tick_repaints_meter_body",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_window_runtime_tick_cycles_after_maximum",
                failures,
            )
            self.assertIn(
                "docs/storybook-77ui-deep-audit-ledger.md: progress-bar ledger must include progress_bar_dedicated_render_uses_core_progress_bar_public_api",
                failures,
            )

    def test_accepts_progress_bar_ledger_with_live_tick_core_and_native_matrix(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            ledger = root / "docs" / "storybook-77ui-deep-audit-ledger.md"
            ledger.parent.mkdir()
            ledger.write_text(
                "## UI: progress-bar\n"
                "- Existing ledger verdict: 旧台帳の `実証済み` は新 DoD では無効。\n"
                "- Current status: 実証済み\n"
                "- Native matrix: target/manual-ui-probe/native-matrix-expanded-v3/summary.json\n"
                "- Matrix check: progress_preview_click\n"
                "- Matrix check: progress_timed_tick\n"
                "- Matrix check: progress_timed_cycle\n"
                "- Matrix check: progress_indeterminate_segment_motion\n"
                "- Evidence: progress_bar_timed_tick_advances_via_core_progress_action\n"
                "- Evidence: progress_bar_timed_tick_cycles_after_reaching_maximum\n"
                "- Evidence: progress_bar_live_audit_reports_timed_tick_progress_contract\n"
                "- Evidence: progress_bar_live_audit_reports_timed_cycle_after_maximum\n"
                "- Evidence: progress_bar_live_audit_reports_indeterminate_segment_motion\n"
                "- Evidence: progress_bar_indeterminate_segment_moves_on_runtime_tick\n"
                "- Evidence: progress_bar_window_runtime_tick_repaints_meter_body\n"
                "- Evidence: progress_bar_window_runtime_tick_cycles_after_maximum\n"
                "- Evidence: progress_bar_dedicated_render_uses_core_progress_bar_public_api\n",
                encoding="utf-8",
            )

            self.assertEqual([], StorybookUiHarnessLedger(root).failures())

    def test_rejects_manual_pending_manifest_page_without_entrypoint(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            docs = root / "docs"
            docs.mkdir()
            (docs / "storybook-77ui-deep-audit-ledger.md").write_text(
                "## UI: progress-bar\n"
                "- Current status: manual_acceptance_pending\n"
                "- Evidence: progress_bar_timed_tick_advances_via_core_progress_action\n"
                "- Evidence: progress_bar_timed_tick_cycles_after_reaching_maximum\n"
                "- Evidence: progress_bar_live_audit_reports_timed_tick_progress_contract\n"
                "- Evidence: progress_bar_live_audit_reports_timed_cycle_after_maximum\n"
                "- Evidence: progress_bar_live_audit_reports_indeterminate_segment_motion\n"
                "- Evidence: progress_bar_indeterminate_segment_moves_on_runtime_tick\n"
                "- Evidence: progress_bar_window_runtime_tick_repaints_meter_body\n"
                "- Evidence: progress_bar_window_runtime_tick_cycles_after_maximum\n"
                "- Evidence: progress_bar_dedicated_render_uses_core_progress_bar_public_api\n",
                encoding="utf-8",
            )
            (docs / "storybook-77ui-interaction-manifest.json").write_text(
                json.dumps(
                    {
                        "ui": [
                            {
                                "page": "text",
                                "audit_status": "partial",
                                "gaps": [
                                    "manual_acceptance_pending: user confirmation is required"
                                ],
                            },
                            {
                                "page": "progress-bar",
                                "audit_status": "partial",
                                "gaps": [
                                    "manual_acceptance_pending: user confirmation is required"
                                ],
                            },
                        ]
                    }
                ),
                encoding="utf-8",
            )
            (docs / "storybook-77ui-repair-plan.md").write_text(
                "## UI: text\n"
                "- Manual confirmation entrypoint: missing\n"
                "- Manual confirmation smoke: missing\n"
                "\n"
                "## UI: progress-bar\n"
                "- Manual confirmation entrypoint:\n"
                "  - `rtk cargo run --release -p katana-ui-core-storybook "
                "--bin katana-ui-core-storybook --locked -- --open-window progress-bar`\n"
                "- Manual confirmation smoke:\n"
                "  - `rtk just storybook-manual-acceptance-smoke`\n"
                "  - `rtk cargo run --release -p katana-ui-core-storybook "
                "--bin katana-ui-core-storybook --locked -- --open-window 48 progress-bar`\n",
                encoding="utf-8",
            )

            failures = StorybookUiHarnessLedger(root).failures()

            self.assertIn(
                "docs/storybook-77ui-repair-plan.md: text manual confirmation entrypoint must include `--open-window text`",
                failures,
            )


if __name__ == "__main__":
    unittest.main()
