#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_final_gate import (
    APPROVAL_LOG_PATH,
    EVIDENCE_PATH,
    manual_acceptance_final_gate_failures,
)


class StorybookManualAcceptanceFinalGateTest(unittest.TestCase):
    def test_rejects_manifest_with_manual_acceptance_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = Path(tmp) / "manifest.json"
            write_manifest(
                manifest,
                '"ui":[{'
                '"page":"checkbox",'
                '"audit_status":"partial",'
                '"manual_acceptance_order":20,'
                '"dependency_layer":"binary-choice-state-display",'
                '"depends_on":["text"],'
                '"required_operations":["pointer","keyboard"],'
                '"minimum_observation_frames":1,'
                '"acceptance_checks":["checkbox_pointer_checks_both_rows"],'
                '"acceptance_observations":["both checked"],'
                '"gaps":["manual_acceptance_pending: user confirmation"]'
                "}]",
            )

            failures = manual_acceptance_final_gate_failures(manifest)

            self.assertEqual(
                ["checkbox: manual acceptance is still pending"],
                failures,
            )

    def test_rejects_invalid_manual_acceptance_queue_contract_as_failure(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = Path(tmp) / "manifest.json"
            write_manifest(
                manifest,
                '"ui":[{'
                '"page":"text",'
                '"audit_status":"partial",'
                '"manual_acceptance_order":10,'
                '"dependency_layer":"foundation-text-selection",'
                '"depends_on":[],'
                '"required_operations":["pointer","drag","keyboard"],'
                '"minimum_observation_frames":0,'
                '"acceptance_checks":["text_drag_selection"],'
                '"acceptance_observations":["drag text"],'
                '"gaps":["manual_acceptance_pending: user confirmation"]'
                "}]",
            )

            failures = manual_acceptance_final_gate_failures(manifest)

            self.assertEqual(
                [
                    "manifest manual acceptance queue invalid: text: minimum_observation_frames must be a positive integer"
                ],
                failures,
            )

    def test_accepts_manifest_without_manual_acceptance_pending_pages(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = Path(tmp) / "manifest.json"
            write_manifest(
                manifest,
                '"ui":[{'
                '"page":"checkbox",'
                '"audit_status":"verified",'
                '"gaps":[]'
                "}]",
            )

            self.assertEqual([], manual_acceptance_final_gate_failures(manifest))

    def test_default_release_gate_skips_stale_manual_approval_artifacts_when_queue_is_clear(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            ledger = root / "ledger.md"
            write_manifest(
                manifest,
                '"ui":[{'
                '"page":"checkbox",'
                '"audit_status":"verified",'
                '"gaps":[]'
                "}]",
            )
            ledger.write_text(
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |\n"
                "| 10 | checkbox | headless 証跡で検証済み。 | ok | ok | 実証済み |\n",
                encoding="utf-8",
            )

            self.assertEqual(
                [],
                manual_acceptance_final_gate_failures(
                    manifest,
                    ledger,
                    APPROVAL_LOG_PATH,
                    EVIDENCE_PATH,
                ),
            )

    def test_rejects_ledger_pending_even_when_manifest_has_no_pending_gap(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            ledger = root / "ledger.md"
            write_manifest(
                manifest,
                '"ui":[{'
                '"page":"checkbox",'
                '"audit_status":"verified",'
                '"gaps":[]'
                "}]",
            )
            ledger.write_text(
                "\n".join(
                    [
                        "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |",
                        "| ---: | --- | --- | --- | --- | --- |",
                        "| 10 | checkbox | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                        "",
                    ]
                ),
                encoding="utf-8",
            )

            failures = manual_acceptance_final_gate_failures(manifest, ledger)

            self.assertEqual(
                ["checkbox: ledger manual acceptance is still pending at line 3"],
                failures,
            )

    def test_rejects_priority_order_marking_manual_pending_page_done(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            priority = root / "priority.md"
            write_manifest(
                manifest,
                '"ui":[{'
                '"page":"text",'
                '"audit_status":"partial",'
                '"manual_acceptance_order":10,'
                '"dependency_layer":"foundation-text-selection",'
                '"depends_on":[],'
                '"required_operations":["pointer","drag","keyboard"],'
                '"minimum_observation_frames":1,'
                '"acceptance_checks":["text_drag_selection"],'
                '"acceptance_observations":["drag text"],'
                '"gaps":["manual_acceptance_pending: user confirmation"]'
                "}]",
            )
            priority.write_text(
                "| priority | menu page | leaf change | 実装状況 | DoD 状況 | 次アクション | 並べ替え理由 |\n"
                "| --- | --- | --- | --- | --- | --- | --- |\n"
                "| SB-010 | `text` | `storybook-page-text` | page別描画あり | 完了 | 完了 | text |\n",
                encoding="utf-8",
            )

            failures = manual_acceptance_final_gate_failures(
                manifest,
                priority_order_path=priority,
            )

            self.assertIn(
                "text: priority order DoD status must not be 完了 while manual acceptance is pending",
                failures,
            )
            self.assertIn(
                "text: priority order next action must not be 完了 while manual acceptance is pending",
                failures,
            )

    def test_rejects_required_ledger_row_without_verified_status_after_pending_is_cleared(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            ledger_source = ledger.read_text(encoding="utf-8")
            ledger.write_text(
                ledger_source.replace(
                    "| 2 | checkbox | Storybook ユーザー確認済み。 | ok | ok | 実証済み |",
                    "| 2 | checkbox | Storybook ユーザー確認済み。 | ok | ok | partial |",
                ),
                encoding="utf-8",
            )
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: ledger manual acceptance must be 実証済み at line 3",
                failures,
            )

    def test_rejects_required_ledger_main_row_missing_even_when_follow_up_exists(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            ledger_source = ledger.read_text(encoding="utf-8")
            ledger.write_text(
                ledger_source.replace(
                    "| 2 | checkbox | Storybook ユーザー確認済み。 | ok | ok | 実証済み |\n",
                    "",
                )
                + "| 99 | checkbox follow-up: glyph | Storybook ユーザー確認済み。 | ok | ok | 実証済み |\n",
                encoding="utf-8",
            )
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: ledger manual acceptance row is missing",
                failures,
            )

    def test_accepts_when_manifest_and_ledger_have_no_pending_gap(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            ledger = root / "ledger.md"
            write_manifest(
                manifest,
                '"ui":[{'
                '"page":"checkbox",'
                '"audit_status":"verified",'
                '"gaps":[]'
                "}]",
            )
            ledger.write_text(
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |\n"
                "| 10 | checkbox | Storybook ユーザー確認済み。 | ok | ok | 実証済み |\n",
                encoding="utf-8",
            )

            self.assertEqual([], manual_acceptance_final_gate_failures(manifest, ledger))

    def test_rejects_missing_final_approval_after_pending_is_cleared(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval = root / "missing-approval.json"
            evidence = root / "missing-evidence.json"

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(f"{approval}: approval log is missing", failures)
            self.assertIn(f"{evidence}: manual acceptance evidence is missing", failures)
            self.assertIn("checkbox: final approval entry is missing", failures)

    def test_rejects_required_manual_acceptance_target_missing_from_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root, pages=("checkbox",))
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: required manual acceptance target is missing from manifest",
                failures,
            )

    def test_rejects_required_manual_acceptance_target_without_checks_or_observations(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in payload["ui"] if entry["page"] == "checkbox")
            checkbox["acceptance_checks"] = []
            checkbox["acceptance_observations"] = []
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn("checkbox: manifest acceptance_checks must not be empty", failures)
            self.assertIn(
                "checkbox: manifest acceptance_observations must not be empty",
                failures,
            )

    def test_rejects_required_manual_acceptance_target_without_verified_status(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in payload["ui"] if entry["page"] == "checkbox")
            checkbox["audit_status"] = "partial"
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: manifest audit_status must be verified after manual acceptance",
                failures,
            )

    def test_rejects_required_manual_acceptance_target_without_dependency_order_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in payload["ui"] if entry["page"] == "checkbox")
            checkbox["manual_acceptance_order"] = 5
            checkbox["depends_on"] = []
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: manifest manual_acceptance_order must be 20",
                failures,
            )
            self.assertIn(
                "checkbox: manifest depends_on must match manual acceptance dependency order: text",
                failures,
            )

    def test_rejects_required_manual_acceptance_target_without_operation_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            progress = next(entry for entry in payload["ui"] if entry["page"] == "progress-bar")
            progress["dependency_layer"] = ""
            progress["required_operations"] = ["pointer"]
            progress["minimum_observation_frames"] = 1
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "progress-bar: manifest dependency_layer must be feedback-motion-meter",
                failures,
            )
            self.assertIn(
                "progress-bar: manifest required_operations must match manual acceptance contract: pointer, timed_tick",
                failures,
            )
            self.assertIn(
                "progress-bar: manifest minimum_observation_frames must be 48",
                failures,
            )

    def test_rejects_required_manual_acceptance_target_without_commands(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in payload["ui"] if entry["page"] == "text")
            text.pop("command")
            text.pop("smoke_command")
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn("text: manifest command must match manual acceptance contract", failures)
            self.assertIn(
                "text: manifest smoke_command must match manual acceptance contract",
                failures,
            )

    def test_rejects_required_manual_acceptance_target_with_reduced_acceptance_checks(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            progress = next(entry for entry in payload["ui"] if entry["page"] == "progress-bar")
            progress["acceptance_checks"] = [
                "progress_preview_click",
                "progress_timed_tick",
                "progress_timed_cycle",
            ]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "progress-bar: manifest acceptance_checks must match manual acceptance contract: progress_preview_click, progress_timed_tick, progress_timed_cycle, progress_indeterminate_segment_motion",
                failures,
            )

    def test_rejects_text_manual_acceptance_without_copy_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in payload["ui"] if entry["page"] == "text")
            text["acceptance_checks"] = ["text_drag_selection"]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: manifest acceptance_checks must match manual acceptance contract: text_drag_selection, text_keyboard_copy, text_keyboard_paste, text_zero_distance_drag_no_selection",
                failures,
            )

    def test_rejects_text_manual_acceptance_without_zero_distance_drag_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in payload["ui"] if entry["page"] == "text")
            text["acceptance_checks"] = [
                "text_drag_selection",
                "text_keyboard_copy",
            ]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: manifest acceptance_checks must match manual acceptance contract: text_drag_selection, text_keyboard_copy, text_keyboard_paste, text_zero_distance_drag_no_selection",
                failures,
            )

    def test_rejects_text_manual_acceptance_without_zero_distance_drag_observation(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in payload["ui"] if entry["page"] == "text")
            text["acceptance_observations"] = [
                "Drag creates a visible text selection highlight",
                "Copy exports selected text",
            ]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: manifest acceptance_observations must match manual acceptance contract: Drag creates a visible text selection highlight; Copy exports selected text; Zero-distance drag does not create a selection action, highlight, or copy payload",
                failures,
            )

    def test_rejects_text_manual_acceptance_without_evidence_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            text = next(entry for entry in payload["ui"] if entry["page"] == "text")
            text["acceptance_evidence_contract"] = []
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: manifest acceptance_evidence_contract must match manual acceptance contract",
                failures,
            )

    def test_rejects_progress_bar_manual_acceptance_without_indeterminate_observation(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            progress = next(entry for entry in payload["ui"] if entry["page"] == "progress-bar")
            progress["acceptance_observations"] = [
                "preview click advances meter from 65% to 82%",
                "meter advances from 65% to 82%",
                "meter cycles back to 0% after max",
            ]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "progress-bar: manifest acceptance_observations must match manual acceptance contract: preview click advances meter from 65% to 82%; meter advances from 65% to 82%; meter cycles back to 0% after max; indeterminate segment visibly moves on timed tick",
                failures,
            )

    def test_rejects_tooltip_manual_acceptance_without_bubble_geometry_observation(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            tooltip = next(entry for entry in payload["ui"] if entry["page"] == "tooltip")
            tooltip["acceptance_observations"] = acceptance_observations_for("tooltip")[:-1]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "tooltip: manifest acceptance_observations must match manual acceptance contract: preview trigger opens the tooltip surface; hover opens the tooltip surface without repeated event spam; hover leave closes the tooltip surface without a click-like replacement event; focus opens the tooltip surface through the core focus path; window-level hover clear closes an open tooltip when the pointer leaves the window; hover bubble remains inside the preview component and visually covers the anchor center",
                failures,
            )

    def test_rejects_modal_manual_acceptance_without_focus_trap_observation(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            modal = next(entry for entry in payload["ui"] if entry["page"] == "modal")
            modal["acceptance_observations"] = acceptance_observations_for("modal")[:-1]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "modal: manifest acceptance_observations must match manual acceptance contract: preview action changes the modal open/closed surface; Escape closes the modal through the core modal action; closed modal state removes backdrop/dialog/native/close surfaces from the preview; Escape after a closed modal is ignored without emitting another close event; focus operation enters the modal focus trap",
                failures,
            )

    def test_rejects_tree_view_manual_acceptance_without_scroll_retention_observation(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            tree = next(entry for entry in payload["ui"] if entry["page"] == "tree-view")
            tree["acceptance_observations"] = acceptance_observations_for("tree-view")[:-1]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "tree-view: manifest acceptance_observations must match manual acceptance contract: row click toggles or selects a tree item; keyboard selection updates the selected item; focus targets the tree row; hover targets the tree row without repeated event spam; context menu opens on a tree row; clicking after scroll keeps the visible tree offset instead of jumping to the top",
                failures,
            )

    def test_rejects_tooltip_manual_acceptance_without_hover_geometry_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            tooltip = next(entry for entry in payload["ui"] if entry["page"] == "tooltip")
            tooltip["acceptance_checks"] = [
                "preview_click",
                "tooltip_anchor_hover_open",
                "tooltip_hover_idempotent",
                "tooltip_hover_leave_close",
                "tooltip_idle_bubble_hidden_until_hover",
                "tooltip_focus_open",
                "tooltip_window_hover_clear_close",
            ]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "tooltip: manifest acceptance_checks must match manual acceptance contract: preview_click, tooltip_anchor_hover_open, tooltip_hover_idempotent, tooltip_hover_leave_close, tooltip_idle_bubble_hidden_until_hover, tooltip_focus_open, tooltip_window_hover_clear_close, tooltip_hover_bubble_geometry",
                failures,
            )

    def test_rejects_checkbox_manual_acceptance_without_snapshot_state_consistency_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in payload["ui"] if entry["page"] == "checkbox")
            checkbox["acceptance_checks"] = acceptance_checks_for("checkbox")[:-1]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: manifest acceptance_checks must match manual acceptance contract: row_click, checkbox_pointer_checks_both_rows, checkbox_keyboard_toggle, checkbox_keyboard_toggle_off, checkbox_keyboard_focused_secondary_row, checkbox_control_toggle_reset, checkbox_focus, preview_hover, checkbox_hover_no_click_event, checkbox_hover_secondary_row, disabled_focus_keyboard_block, checkbox_disabled_pointer_block, checkbox_no_runtime_overlay_over_controls, checkbox_controls_bottom_padding, checkbox_disabled_snapshot_click_block, checkbox_disabled_controls_are_muted, checkbox_disabled_hover_is_muted, checkbox_checked_preset_state_consistency, checkbox_disabled_preset_state_consistency, checkbox_focus_preset_state_consistency, checkbox_checked_state_read_preserves_checked_state_metadata, checkbox_disabled_state_read_control_is_blocked, checkbox_focus_state_read_preserves_focus_state_metadata, checkbox_initial_snapshot_state_consistency, checkbox_focus_labels_visible, checkbox_focus_single_active_row, checkbox_inspector_options_are_labeled, checkbox_modern_spacing, checkbox_snapshot_state_consistency",
                failures,
            )

    def test_rejects_checkbox_manual_acceptance_without_glyph_state_observation(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in payload["ui"] if entry["page"] == "checkbox")
            checkbox["acceptance_observations"] = acceptance_observations_for("checkbox")[:-1]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: manifest acceptance_observations must match manual acceptance contract: row click toggles the checked mark and state together; row 0 and row 1 pointer clicks can leave both checkbox rows checked at the same time; keyboard activation toggles the checked mark and state together; second keyboard activation toggles the checked mark and state back off together; keyboard activation mutates the focused secondary row without changing the primary row; toggle and reset controls mutate checked state and rendered mark through the same public action path; focus renders a visible focus state; hover renders without repeatedly firing click events; hover does not increment action_count, emit checkbox_toggle, or mutate checked state; hover feedback follows the actual checkbox row under the pointer; disabled preset blocks focus and keyboard checked mutation while preserving the mark; disabled preset blocks pointer checked mutation while preserving the mark; Storybook runtime overlay does not draw clicked labels over core checkbox controls; checkbox control row keeps bottom padding inside the component frame; disabled clicked snapshot path does not bypass window interaction disabled blocking; disabled preset mutes checkbox control button labels instead of presenting enabled controls; disabled preset does not show enabled hover feedback; checked preset reports current checked state in preview and Inspector instead of idle/false state; disabled preset reports current disabled state in preview and Inspector instead of idle state; disabled focus and keyboard block preserve disabled=true state metadata; focus preset reports current focus state in preview and Inspector instead of idle/false state; state read preserves checked=true, disabled=true, and focused=true current public state metadata instead of replacing it with before/after history labels; initial snapshot keeps idle state visible and does not render a no-op before/after transition as operation history; focus preset keeps checkbox row labels visibly rendered; focus preset renders a focus ring on the active row only; Inspector settings rows label binary-choice mutations as option values instead of current state values; checkbox mark, row, and status spacing meet the modern binary-choice layout contract; checked glyph uses the core accent-foreground theme token through VisualPalette instead of a Storybook-only fixed literal; clicked snapshot keeps preview status and Inspector state/action/event consistent",
                failures,
            )

    def test_rejects_modal_manual_acceptance_without_focus_trap_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            modal = next(entry for entry in payload["ui"] if entry["page"] == "modal")
            modal["acceptance_checks"] = [
                "preview_click",
                "modal_keyboard_escape",
                "modal_escape_removes_surface",
                "modal_escape_after_close_idempotent",
            ]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "modal: manifest acceptance_checks must match manual acceptance contract: preview_click, modal_keyboard_escape, modal_escape_removes_surface, modal_escape_after_close_idempotent, modal_focus_trap",
                failures,
            )

    def test_rejects_tree_view_manual_acceptance_without_scroll_retention_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            tree = next(entry for entry in payload["ui"] if entry["page"] == "tree-view")
            tree["acceptance_checks"] = [
                "preview_click",
                "tree_keyboard_select",
                "tree_focus_item",
                "tree_hover_item",
                "tree_view_context_menu",
            ]
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "tree-view: manifest acceptance_checks must match manual acceptance contract: preview_click, tree_keyboard_select, tree_focus_item, tree_hover_item, tree_view_context_menu, tree_scroll_retained",
                failures,
            )

    def test_rejects_unexpected_manual_acceptance_target_in_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(
                root,
                pages=(
                    "text",
                    "checkbox",
                    "progress-bar",
                    "tooltip",
                    "modal",
                    "tree-view",
                    "button",
                ),
            )
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "button: manifest has unexpected manual acceptance target",
                failures,
            )

    def test_ignores_non_manual_acceptance_manifest_pages_after_pending_is_cleared(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            payload["ui"].append(
                {
                    "page": "button",
                    "audit_status": "verified",
                    "gaps": [],
                }
            )
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            approval, evidence = write_final_approval_and_evidence(root)

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertNotIn(
                "button: manifest has unexpected manual acceptance target",
                failures,
            )
            self.assertEqual([], failures)

    def test_rejects_final_approval_that_does_not_match_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in approvals if entry["page"] == "checkbox")
            checkbox["smoke_command"] = "old smoke"
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final approval smoke_command does not match evidence",
                failures,
            )

    def test_rejects_final_approval_and_evidence_commands_that_do_not_match_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            approval_text = next(entry for entry in approvals if entry["page"] == "text")
            evidence_text = next(entry for entry in evidence_entries if entry["page"] == "text")
            approval_text["command"] = "rtk cargo run -- --open-window wrong"
            approval_text["smoke_command"] = "rtk cargo run -- --open-window 1 wrong"
            evidence_text["command"] = "rtk cargo run -- --open-window wrong"
            evidence_text["smoke_command"] = "rtk cargo run -- --open-window 1 wrong"
            approval.write_text(json.dumps(approvals), encoding="utf-8")
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn("text: final approval command must match manifest", failures)
            self.assertIn("text: final evidence command must match manifest", failures)
            self.assertIn("text: final approval smoke_command must match manifest", failures)
            self.assertIn("text: final evidence smoke_command must match manifest", failures)

    def test_rejects_final_approval_that_omits_manifest_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in approvals if entry["page"] == "checkbox")
            checkbox["acceptance_checks"] = []
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final approval acceptance_checks do not match manifest",
                failures,
            )

    def test_rejects_final_approval_that_omits_manifest_evidence_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            text = next(entry for entry in approvals if entry["page"] == "text")
            text["acceptance_evidence_contract"] = []
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: final approval acceptance_evidence_contract does not match manifest",
                failures,
            )

    def test_rejects_final_approval_that_changes_manual_gate(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            text = next(entry for entry in approvals if entry["page"] == "text")
            text["manual_gate"] = "approved without user confirmation"
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: final approval manual_gate must match manual acceptance contract",
                failures,
            )

    def test_rejects_final_approval_and_evidence_with_reduced_minimum_frames(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            approval_progress = next(
                entry for entry in approvals if entry["page"] == "progress-bar"
            )
            evidence_progress = next(
                entry for entry in evidence_entries if entry["page"] == "progress-bar"
            )
            approval_progress["minimum_observation_frames"] = 1
            evidence_progress["minimum_observation_frames"] = 1
            approval.write_text(json.dumps(approvals), encoding="utf-8")
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "progress-bar: final approval minimum_observation_frames must match manifest",
                failures,
            )
            self.assertIn(
                "progress-bar: final evidence minimum_observation_frames must match manifest",
                failures,
            )

    def test_rejects_final_evidence_that_omits_manifest_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in evidence_entries if entry["page"] == "checkbox")
            checkbox["audit_evidence"] = []
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final evidence missing check row_click",
                failures,
            )

    def test_rejects_final_text_drag_evidence_without_selection_action_or_event(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            text = next(entry for entry in evidence_entries if entry["page"] == "text")
            drag = next(
                item
                for item in text["audit_evidence"]
                if item["check"] == "text_drag_selection"
            )
            drag["action"] = "none"
            drag["event"] = "none"
            drag["state"] = "idle"
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: final evidence text_drag_selection must include action select_text",
                failures,
            )
            self.assertIn(
                "text: final evidence text_drag_selection must include event text_selection_changed",
                failures,
            )
            self.assertIn(
                "text: final evidence text_drag_selection must include state selection=active",
                failures,
            )

    def test_rejects_final_text_keyboard_copy_without_copy_action_or_event(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            text = next(entry for entry in evidence_entries if entry["page"] == "text")
            copy = next(
                item
                for item in text["audit_evidence"]
                if item["check"] == "text_keyboard_copy"
            )
            copy["action"] = "none"
            copy["event"] = "none"
            copy["state"] = "idle"
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: final evidence text_keyboard_copy must include action copy_selection",
                failures,
            )
            self.assertIn(
                "text: final evidence text_keyboard_copy must include event clipboard_copy",
                failures,
            )
            self.assertIn(
                "text: final evidence text_keyboard_copy must include state clipboard=selected_text",
                failures,
            )

    def test_rejects_final_evidence_with_duplicate_manifest_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in evidence_entries if entry["page"] == "checkbox")
            checkbox["audit_evidence"].append(
                {
                    "check": "row_click",
                    "passed": True,
                }
            )
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final evidence check row_click is duplicated",
                failures,
            )

    def test_rejects_final_evidence_with_unexpected_check(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in evidence_entries if entry["page"] == "checkbox")
            checkbox["audit_evidence"].append(
                {
                    "check": "checkbox_extra_check",
                    "passed": True,
                }
            )
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final evidence has unexpected check checkbox_extra_check",
                failures,
            )

    def test_rejects_checkbox_final_evidence_without_pointer_state_payload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in evidence_entries if entry["page"] == "checkbox")
            for item in checkbox["audit_evidence"]:
                if item["check"] in ("row_click", "checkbox_pointer_checks_both_rows"):
                    item.pop("operation_kind", None)
                    item.pop("action", None)
                    item.pop("event", None)
                    item.pop("body_pixel_diff", None)
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final evidence row_click must include operation_kind pointer",
                failures,
            )
            self.assertIn(
                "checkbox: final evidence row_click must include action checkbox_toggle",
                failures,
            )
            self.assertIn(
                "checkbox: final evidence row_click must include event checked_changed",
                failures,
            )
            self.assertIn(
                "checkbox: final evidence row_click must include positive body_pixel_diff",
                failures,
            )
            self.assertIn(
                "checkbox: final evidence checkbox_pointer_checks_both_rows must include operation_kind pointer",
                failures,
            )

    def test_rejects_text_final_evidence_without_selection_or_clipboard_payload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            text = next(entry for entry in evidence_entries if entry["page"] == "text")
            for item in text["audit_evidence"]:
                item.pop("body_pixel_diff", None)
                item.pop("clipboard_text_len", None)
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: final evidence text_drag_selection must include positive body_pixel_diff",
                failures,
            )
            self.assertIn(
                "text: final evidence text_keyboard_copy must include positive clipboard_text_len",
                failures,
            )

    def test_rejects_text_final_evidence_without_operation_kind(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            text = next(entry for entry in evidence_entries if entry["page"] == "text")
            for item in text["audit_evidence"]:
                item.pop("operation_kind", None)
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: final evidence text_drag_selection must include operation_kind drag",
                failures,
            )
            self.assertIn(
                "text: final evidence text_keyboard_copy must include operation_kind keyboard",
                failures,
            )

    def test_rejects_text_zero_distance_final_evidence_with_selection_action(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            text = next(entry for entry in evidence_entries if entry["page"] == "text")
            zero = next(
                item
                for item in text["audit_evidence"]
                if item["check"] == "text_zero_distance_drag_no_selection"
            )
            zero["action"] = "select_text"
            zero["event"] = "text_selection_changed"
            zero["state"] = "selecting"
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "text: final evidence text_zero_distance_drag_no_selection must include action none",
                failures,
            )
            self.assertIn(
                "text: final evidence text_zero_distance_drag_no_selection must include event none",
                failures,
            )
            self.assertIn(
                "text: final evidence text_zero_distance_drag_no_selection must include state idle",
                failures,
            )

    def test_rejects_final_approval_and_evidence_that_omit_manifest_observation(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            approval_checkbox = next(entry for entry in approvals if entry["page"] == "checkbox")
            evidence_checkbox = next(
                entry for entry in evidence_entries if entry["page"] == "checkbox"
            )
            approval_checkbox["acceptance_observations"] = []
            evidence_checkbox["acceptance_observations"] = []
            approval.write_text(json.dumps(approvals), encoding="utf-8")
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final approval observations do not match manifest",
                failures,
            )
            self.assertIn(
                "checkbox: final evidence observations do not match manifest",
                failures,
            )

    def test_rejects_final_approval_with_timezone_less_approved_at(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            checkbox = next(entry for entry in approvals if entry["page"] == "checkbox")
            checkbox["approved_at"] = "2026-06-15T12:00:00"
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "checkbox: final approval approved_at must include timezone",
                failures,
            )

    def test_rejects_unexpected_final_approval_or_evidence_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            approvals.append(approval_entry("button"))
            evidence_entries.append(evidence_entry("button"))
            approval.write_text(json.dumps(approvals), encoding="utf-8")
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn("button: final approval has unexpected page", failures)
            self.assertIn("button: final evidence has unexpected page", failures)

    def test_rejects_duplicate_final_approval_or_evidence_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            approvals.append(approval_entry("checkbox"))
            evidence_entries.append(evidence_entry("checkbox"))
            approval.write_text(json.dumps(approvals), encoding="utf-8")
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn("checkbox: final approval entry is duplicated", failures)
            self.assertIn("checkbox: final evidence entry is duplicated", failures)

    def test_rejects_final_approval_outside_dependency_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            approvals = list(reversed(approvals))
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "final approval order must match manual acceptance dependency order: text, checkbox, progress-bar, tooltip, modal, tree-view",
                failures,
            )

    def test_rejects_final_evidence_outside_dependency_order(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)
            evidence_entries = json.loads(evidence.read_text(encoding="utf-8"))
            evidence_entries = list(reversed(evidence_entries))
            evidence.write_text(json.dumps(evidence_entries), encoding="utf-8")

            failures = manual_acceptance_final_gate_failures(
                manifest,
                ledger,
                approval,
                evidence,
            )

            self.assertIn(
                "final evidence order must match manual acceptance dependency order: text, checkbox, progress-bar, tooltip, modal, tree-view",
                failures,
            )

    def test_accepts_final_approval_and_evidence_for_required_pages(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, ledger = write_verified_manifest_and_ledger(root)
            approval, evidence = write_final_approval_and_evidence(root)

            self.assertEqual(
                [],
                manual_acceptance_final_gate_failures(manifest, ledger, approval, evidence),
            )

    def test_rejects_invalid_manifest_ui_shape(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = Path(tmp) / "manifest.json"
            write_manifest(manifest, '"ui":{}')

            failures = manual_acceptance_final_gate_failures(manifest)

            self.assertEqual([f"{manifest}: ui must be a list"], failures)

    def test_rejects_invalid_page_and_gap_shape(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            manifest = Path(tmp) / "manifest.json"
            write_manifest(manifest, '"ui":[{"page":"","gaps":{}}]')

            failures = manual_acceptance_final_gate_failures(manifest)

            self.assertEqual(
                [
                    f"{manifest}: ui[0].page must be a non-empty string",
                    f"{manifest}: ui[0].gaps must be a list",
                ],
                failures,
            )


def write_manifest(path: Path, body: str) -> None:
    path.write_text("{" + body + "}", encoding="utf-8")


def write_verified_manifest_and_ledger(
    root: Path,
    pages: tuple[str, ...] = (
        "text",
        "checkbox",
        "progress-bar",
        "tooltip",
        "modal",
        "tree-view",
    ),
) -> tuple[Path, Path]:
    manifest = root / "manifest.json"
    ledger = root / "ledger.md"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
                    {
                        "page": page,
                        "audit_status": "verified",
                        "manual_acceptance_order": manual_acceptance_order_for(page),
                        "depends_on": depends_on_for(page),
                        "dependency_layer": dependency_layer_for(page),
                        "required_operations": required_operations_for(page),
                        "command": command_for(page),
                        "smoke_command": smoke_command_for(page),
                        "minimum_observation_frames": minimum_observation_frames_for(page),
                        "acceptance_checks": acceptance_checks_for(page),
                        "acceptance_observations": acceptance_observations_for(page),
                        "acceptance_evidence_contract": evidence_contract_for(page),
                        "gaps": [],
                    }
                    for page in pages
                ]
            }
        ),
        encoding="utf-8",
    )
    ledger.write_text(
        "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |\n"
        + "\n".join(
            f"| {index} | {page} | Storybook ユーザー確認済み。 | ok | ok | 実証済み |"
            for index, page in enumerate(pages, start=1)
        )
        + "\n",
        encoding="utf-8",
    )
    return manifest, ledger


def write_final_approval_and_evidence(root: Path) -> tuple[Path, Path]:
    pages = ["text", "checkbox", "progress-bar", "tooltip", "modal", "tree-view"]
    approval = root / "approval.json"
    evidence = root / "evidence.json"
    approval.write_text(
        json.dumps([approval_entry(page) for page in pages]),
        encoding="utf-8",
    )
    evidence.write_text(
        json.dumps([evidence_entry(page) for page in pages]),
        encoding="utf-8",
    )
    return approval, evidence


def approval_entry(page: str) -> dict[str, object]:
    return {
        "page": page,
        "approved": True,
        "approved_by": "user",
        "approved_at": "2026-06-15T00:00:00+09:00",
        "command": command_for(page),
        "smoke_command": smoke_command_for(page),
        "minimum_observation_frames": minimum_observation_frames_for(page),
        "acceptance_checks": acceptance_checks_for(page),
        "acceptance_observations": acceptance_observations_for(page),
        "acceptance_evidence_contract": evidence_contract_for(page),
        "manual_gate": "do not proceed to the next UI until this page is approved",
    }


def evidence_entry(page: str) -> dict[str, object]:
    return {
        "page": page,
        "command": command_for(page),
        "smoke_command": smoke_command_for(page),
        "minimum_observation_frames": minimum_observation_frames_for(page),
        "acceptance_observations": acceptance_observations_for(page),
        "audit_evidence": [
            evidence_item_for(page, check)
            for check in acceptance_checks_for(page)
        ],
    }


def evidence_item_for(page: str, check: str) -> dict[str, object]:
    item: dict[str, object] = {
        "check": check,
        "passed": True,
    }
    if page == "text" and check == "text_drag_selection":
        item["operation_kind"] = "drag"
        item["state"] = "selection=active"
        item["action"] = "select_text"
        item["event"] = "text_selection_changed"
        item["body_pixel_diff"] = 12
    if page == "text" and check == "text_keyboard_copy":
        item["operation_kind"] = "keyboard"
        item["state"] = "clipboard=selected_text"
        item["action"] = "copy_selection"
        item["event"] = "clipboard_copy"
        item["clipboard_text_len"] = 8
    if page == "text" and check == "text_keyboard_paste":
        item["operation_kind"] = "keyboard"
        item["state"] = "idle"
        item["action"] = "none"
        item["event"] = "none"
    if page == "text" and check == "text_zero_distance_drag_no_selection":
        item["operation_kind"] = "drag"
        item["state"] = "idle"
        item["body_pixel_diff"] = 0
        item["clipboard_text_len"] = 0
        item["action"] = "none"
        item["event"] = "none"
    if page == "checkbox" and check in ("row_click", "checkbox_pointer_checks_both_rows"):
        item["operation_kind"] = "pointer"
        item["action"] = "checkbox_toggle"
        item["event"] = "checked_changed"
        item["body_pixel_diff"] = 18
    if page == "progress-bar":
        item["operation_kind"] = (
            "pointer" if check == "progress_preview_click" else "timed_tick"
        )
        item["state"] = {
            "progress_preview_click": "percent=82",
            "progress_timed_tick": "percent=82",
            "progress_timed_cycle": "percent=0",
            "progress_indeterminate_segment_motion": "percent=82",
        }.get(check, "percent=82")
        item["action"] = (
            "progress_change"
            if check == "progress_preview_click"
            else "progress_tick"
        )
        item["event"] = "progress_changed"
        item["body_pixel_diff"] = 20
    return item


def evidence_contract_for(page: str) -> list[dict[str, str]]:
    if page != "text":
        return []
    return [
        {
            "check": "text_drag_selection",
            "operation_kind": "drag",
            "state": "selection=active",
            "action": "select_text",
            "event": "text_selection_changed",
        },
        {
            "check": "text_keyboard_copy",
            "operation_kind": "keyboard",
            "state": "clipboard=selected_text",
            "action": "copy_selection",
            "event": "clipboard_copy",
        },
        {
            "check": "text_keyboard_paste",
            "operation_kind": "keyboard",
            "state": "idle",
            "action": "none",
            "event": "none",
        },
        {
            "check": "text_zero_distance_drag_no_selection",
            "operation_kind": "drag",
            "state": "idle",
            "action": "none",
            "event": "none",
        },
    ]


def command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        f"katana-ui-core-storybook --locked -- --open-window {page}"
    )


def smoke_command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        "katana-ui-core-storybook --locked -- --open-window "
        f"{minimum_observation_frames_for(page)} {page}"
    )


def manual_acceptance_order_for(page: str) -> int:
    return {
        "text": 10,
        "checkbox": 20,
        "progress-bar": 30,
        "tooltip": 40,
        "modal": 50,
        "tree-view": 60,
    }.get(page, 999)


def depends_on_for(page: str) -> list[str]:
    return {
        "text": [],
        "checkbox": ["text"],
        "progress-bar": ["text"],
        "tooltip": ["text", "checkbox"],
        "modal": ["text", "checkbox", "tooltip"],
        "tree-view": ["text", "checkbox"],
    }.get(page, [])


def dependency_layer_for(page: str) -> str:
    return {
        "text": "foundation-text-selection",
        "checkbox": "binary-choice-state-display",
        "progress-bar": "feedback-motion-meter",
        "tooltip": "overlay-anchor-hover-focus",
        "modal": "overlay-modal-focus-dismiss",
        "tree-view": "selection-tree-scroll-context",
    }.get(page, "")


def required_operations_for(page: str) -> list[str]:
    return {
        "text": ["pointer", "drag", "keyboard"],
        "checkbox": ["pointer", "keyboard", "focus", "hover"],
        "progress-bar": ["pointer", "timed_tick"],
        "tooltip": ["pointer", "hover", "focus"],
        "modal": ["pointer", "keyboard", "focus"],
        "tree-view": ["pointer", "keyboard", "focus", "hover", "scroll", "context_menu"],
    }.get(page, [])


def acceptance_checks_for(page: str) -> list[str]:
    if page == "text":
        return [
            "text_drag_selection",
            "text_keyboard_copy",
            "text_keyboard_paste",
            "text_zero_distance_drag_no_selection",
        ]
    if page == "checkbox":
        return [
            "row_click",
            "checkbox_pointer_checks_both_rows",
            "checkbox_keyboard_toggle",
            "checkbox_keyboard_toggle_off",
            "checkbox_keyboard_focused_secondary_row",
            "checkbox_control_toggle_reset",
            "checkbox_focus",
            "preview_hover",
            "checkbox_hover_no_click_event",
            "checkbox_hover_secondary_row",
            "disabled_focus_keyboard_block",
            "checkbox_disabled_pointer_block",
            "checkbox_no_runtime_overlay_over_controls",
            "checkbox_controls_bottom_padding",
            "checkbox_disabled_snapshot_click_block",
            "checkbox_disabled_controls_are_muted",
            "checkbox_disabled_hover_is_muted",
            "checkbox_checked_preset_state_consistency",
            "checkbox_disabled_preset_state_consistency",
            "checkbox_focus_preset_state_consistency",
            "checkbox_checked_state_read_preserves_checked_state_metadata",
            "checkbox_disabled_state_read_control_is_blocked",
            "checkbox_focus_state_read_preserves_focus_state_metadata",
            "checkbox_initial_snapshot_state_consistency",
            "checkbox_focus_labels_visible",
            "checkbox_focus_single_active_row",
            "checkbox_inspector_options_are_labeled",
            "checkbox_modern_spacing",
            "checkbox_snapshot_state_consistency",
        ]
    if page == "progress-bar":
        return [
            "progress_preview_click",
            "progress_timed_tick",
            "progress_timed_cycle",
            "progress_indeterminate_segment_motion",
        ]
    if page == "tooltip":
        return [
            "preview_click",
            "tooltip_anchor_hover_open",
            "tooltip_hover_idempotent",
            "tooltip_hover_leave_close",
            "tooltip_idle_bubble_hidden_until_hover",
            "tooltip_focus_open",
            "tooltip_window_hover_clear_close",
            "tooltip_hover_bubble_geometry",
        ]
    if page == "modal":
        return [
            "preview_click",
            "modal_keyboard_escape",
            "modal_escape_removes_surface",
            "modal_escape_after_close_idempotent",
            "modal_focus_trap",
        ]
    if page == "tree-view":
        return [
            "preview_click",
            "tree_keyboard_select",
            "tree_focus_item",
            "tree_hover_item",
            "tree_view_context_menu",
            "tree_scroll_retained",
        ]
    return [f"{page}_manual_check"]


def acceptance_observations_for(page: str) -> list[str]:
    if page == "text":
        return [
            "Drag creates a visible text selection highlight",
            "Copy exports selected text",
            "Zero-distance drag does not create a selection action, highlight, or copy payload",
        ]
    if page == "checkbox":
        return [
            "row click toggles the checked mark and state together",
            "row 0 and row 1 pointer clicks can leave both checkbox rows checked at the same time",
            "keyboard activation toggles the checked mark and state together",
            "second keyboard activation toggles the checked mark and state back off together",
            "keyboard activation mutates the focused secondary row without changing the primary row",
            "toggle and reset controls mutate checked state and rendered mark through the same public action path",
            "focus renders a visible focus state",
            "hover renders without repeatedly firing click events",
            "hover does not increment action_count, emit checkbox_toggle, or mutate checked state",
            "hover feedback follows the actual checkbox row under the pointer",
            "disabled preset blocks focus and keyboard checked mutation while preserving the mark",
            "disabled preset blocks pointer checked mutation while preserving the mark",
            "Storybook runtime overlay does not draw clicked labels over core checkbox controls",
            "checkbox control row keeps bottom padding inside the component frame",
            "disabled clicked snapshot path does not bypass window interaction disabled blocking",
            "disabled preset mutes checkbox control button labels instead of presenting enabled controls",
            "disabled preset does not show enabled hover feedback",
            "checked preset reports current checked state in preview and Inspector instead of idle/false state",
            "disabled preset reports current disabled state in preview and Inspector instead of idle state",
            "disabled focus and keyboard block preserve disabled=true state metadata",
            "focus preset reports current focus state in preview and Inspector instead of idle/false state",
            "state read preserves checked=true, disabled=true, and focused=true current public state metadata instead of replacing it with before/after history labels",
            "initial snapshot keeps idle state visible and does not render a no-op before/after transition as operation history",
            "focus preset keeps checkbox row labels visibly rendered",
            "focus preset renders a focus ring on the active row only",
            "Inspector settings rows label binary-choice mutations as option values instead of current state values",
            "checkbox mark, row, and status spacing meet the modern binary-choice layout contract",
            "checked glyph uses the core accent-foreground theme token through VisualPalette instead of a Storybook-only fixed literal",
            "clicked snapshot keeps preview status and Inspector state/action/event consistent",
        ]
    if page == "progress-bar":
        return [
            "preview click advances meter from 65% to 82%",
            "meter advances from 65% to 82%",
            "meter cycles back to 0% after max",
            "indeterminate segment visibly moves on timed tick",
        ]
    if page == "tooltip":
        return [
            "preview trigger opens the tooltip surface",
            "hover opens the tooltip surface without repeated event spam",
            "hover leave closes the tooltip surface without a click-like replacement event",
            "focus opens the tooltip surface through the core focus path",
            "window-level hover clear closes an open tooltip when the pointer leaves the window",
            "hover bubble remains inside the preview component and visually covers the anchor center",
        ]
    if page == "modal":
        return [
            "preview action changes the modal open/closed surface",
            "Escape closes the modal through the core modal action",
            "closed modal state removes backdrop/dialog/native/close surfaces from the preview",
            "Escape after a closed modal is ignored without emitting another close event",
            "focus operation enters the modal focus trap",
        ]
    if page == "tree-view":
        return [
            "row click toggles or selects a tree item",
            "keyboard selection updates the selected item",
            "focus targets the tree row",
            "hover targets the tree row without repeated event spam",
            "context menu opens on a tree row",
            "clicking after scroll keeps the visible tree offset instead of jumping to the top",
        ]
    return [f"{page} manually accepted"]


def minimum_observation_frames_for(page: str) -> int:
    return {
        "progress-bar": 48,
    }.get(page, 1)


if __name__ == "__main__":
    unittest.main()
