#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from storybook_manual_acceptance_mark_approved import mark_manual_acceptance_approved
from storybook_manual_acceptance_approve import approve_manual_acceptance


class StorybookManualAcceptanceApproveTest(unittest.TestCase):
    def test_approves_only_next_pending_page_in_manifest_and_ledger(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)

            output = approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

            payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(item for item in payload["ui"] if item["page"] == "checkbox")
            text = next(item for item in payload["ui"] if item["page"] == "text")
            self.assertEqual("partial", checkbox["audit_status"])
            self.assertIn("manual_acceptance_pending: checkbox pending", checkbox["gaps"])
            self.assertEqual("verified", text["audit_status"])
            self.assertEqual([], text["gaps"])
            ledger_source = ledger.read_text(encoding="utf-8")
            self.assertIn("| 01 | text | Storybook ユーザー確認済み。 | ok | ok | 実証済み |", ledger_source)
            self.assertIn("| 01a | text follow-up | Storybook ユーザー確認済み。 | ok | ok | 実証済み |", ledger_source)
            self.assertIn("| 10 | checkbox | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |", ledger_source)
            self.assertIn("| 10a | checkbox follow-up | detail | ok | ok | manual_acceptance_pending |", ledger_source)
            self.assertIn("text: manifest partial->verified", output)
            self.assertIn("text: ledger 2 row(s)", output)

    def test_dry_run_reports_without_writing_files(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)
            before_manifest = manifest.read_text(encoding="utf-8")
            before_ledger = ledger.read_text(encoding="utf-8")

            output = approve_manual_acceptance(
                manifest,
                ledger,
                evidence,
                approval,
                {"text"},
                dry_run=True,
            )

            self.assertEqual(before_manifest, manifest.read_text(encoding="utf-8"))
            self.assertEqual(before_ledger, ledger.read_text(encoding="utf-8"))
            self.assertIn("dry-run: no files changed", output)

    def test_marked_approval_dry_run_closes_text_and_follow_up_ledger_rows(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "approval.json"

            mark_output = mark_manual_acceptance_approved(
                manifest,
                approval,
                {"text"},
                "user",
                "2026-06-15T00:00:00+09:00",
            )
            before_manifest = manifest.read_text(encoding="utf-8")
            before_ledger = ledger.read_text(encoding="utf-8")

            output = approve_manual_acceptance(
                manifest,
                ledger,
                evidence,
                approval,
                {"text"},
                dry_run=True,
            )

            self.assertEqual(["text: marked approved"], mark_output)
            self.assertEqual(before_manifest, manifest.read_text(encoding="utf-8"))
            self.assertEqual(before_ledger, ledger.read_text(encoding="utf-8"))
            self.assertIn("text: manifest partial->verified", output)
            self.assertIn("text: ledger 2 row(s)", output)
            self.assertIn("dry-run: no files changed", output)

    def test_allows_previous_page_approval_when_approving_next_pending_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest_with_text_already_verified(root)
            ledger = write_ledger_with_text_already_verified(root)
            evidence = write_evidence(root, pages=("text", "checkbox"))
            approval = write_approval(root, pages=("text", "checkbox"))

            output = approve_manual_acceptance(
                manifest,
                ledger,
                evidence,
                approval,
                {"checkbox"},
            )

            payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(item for item in payload["ui"] if item["page"] == "checkbox")
            self.assertEqual("verified", checkbox["audit_status"])
            self.assertEqual([], checkbox["gaps"])
            self.assertIn("checkbox: manifest partial->verified", output)

    def test_rejects_page_that_is_not_pending_in_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "button"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"button"})

    def test_rejects_approval_without_current_evidence_report(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = root / "missing-evidence.json"
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "evidence"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_without_user_approval_log(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "missing-approval.json"

            with self.assertRaisesRegex(ValueError, "approval"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_log_entry_without_approval_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "approval.json"
            approval.write_text(
                json.dumps([{"page": "text", "approved": False}]),
                encoding="utf-8",
            )

            with self.assertRaisesRegex(ValueError, "approved=true"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_log_entry_with_stale_checklist(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = root / "approval.json"
            approval.write_text(
                json.dumps(
                    [
                        {
                            "page": "text",
                            "approved": True,
                            "approved_by": "user",
                            "approved_at": "2026-06-15T00:00:00+09:00",
                            "acceptance_checks": ["old_check"],
                            "acceptance_observations": ["old observation"],
                        }
                    ]
                ),
                encoding="utf-8",
            )

            with self.assertRaisesRegex(ValueError, "acceptance_checks"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_log_entry_with_stale_evidence_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            text = next(entry for entry in approvals if entry["page"] == "text")
            text["acceptance_evidence_contract"] = [
                {"check": "text_drag_selection", "state": "idle"}
            ]
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "acceptance_evidence_contract"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_log_entry_with_stale_manual_gate(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            text = next(entry for entry in approvals if entry["page"] == "text")
            text["manual_gate"] = "approved without user confirmation"
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "manual_gate"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_log_entry_with_stale_command(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root, pages=("text", "checkbox"))
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            text = next(entry for entry in approvals if entry["page"] == "text")
            text["command"] = "old command"
            text["smoke_command"] = "old smoke"
            text["minimum_observation_frames"] = 99
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "command"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_log_with_duplicate_page_before_manifest_update(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            approvals.append(approvals[0])
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "text: user approval entry is duplicated"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text_entry = next(item for item in manifest_payload["ui"] if item["page"] == "text")
            self.assertEqual("partial", text_entry["audit_status"])

    def test_rejects_approval_log_with_unexpected_page_before_manifest_update(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            approvals.append({"page": "button", "approved": True})
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "button: user approval has unexpected page"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text_entry = next(item for item in manifest_payload["ui"] if item["page"] == "text")
            self.assertEqual("partial", text_entry["audit_status"])

    def test_rejects_future_pending_approval_before_manifest_update(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root, pages=("text", "checkbox"))

            with self.assertRaisesRegex(ValueError, "checkbox: user approval has future pending page"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text_entry = next(item for item in manifest_payload["ui"] if item["page"] == "text")
            self.assertEqual("partial", text_entry["audit_status"])

    def test_rejects_future_pending_evidence_before_manifest_update(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root, pages=("text", "checkbox"))
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "checkbox: manual acceptance evidence has future pending page"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text_entry = next(item for item in manifest_payload["ui"] if item["page"] == "text")
            self.assertEqual("partial", text_entry["audit_status"])

    def test_rejects_timezone_less_approved_at_before_manifest_update(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)
            approvals = json.loads(approval.read_text(encoding="utf-8"))
            text = next(entry for entry in approvals if entry["page"] == "text")
            text["approved_at"] = "2026-06-15T12:00:00"
            approval.write_text(json.dumps(approvals), encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "approved_at must include timezone"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            text_entry = next(item for item in manifest_payload["ui"] if item["page"] == "text")
            self.assertEqual("partial", text_entry["audit_status"])

    def test_approve_script_does_not_expose_skip_evidence_escape_hatch(self) -> None:
        source = Path("scripts/storybook_manual_acceptance_approve.py").read_text(
            encoding="utf-8"
        )

        self.assertNotIn("skip-evidence", source)
        self.assertNotIn("skip_evidence", source)

    def test_rejects_approval_when_ledger_has_no_pending_row_for_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
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
            evidence = write_evidence(root)
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "ledger row"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_approval_when_only_follow_up_ledger_row_is_pending(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            ledger.write_text(
                "\n".join(
                    [
                        "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |",
                        "| ---: | --- | --- | --- | --- | --- |",
                        "| 01a | text follow-up | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                        "| 10 | checkbox | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                        "",
                    ]
                ),
                encoding="utf-8",
            )
            evidence = write_evidence(root)
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "ledger row"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text"})

    def test_rejects_out_of_order_page(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "next pending page is text"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"checkbox"})

    def test_rejects_next_page_with_pending_dependency_before_manifest_update(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_blocked_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "checkbox depends on pending pages: text"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"checkbox"})

            manifest_payload = json.loads(manifest.read_text(encoding="utf-8"))
            checkbox = next(item for item in manifest_payload["ui"] if item["page"] == "checkbox")
            self.assertEqual("partial", checkbox["audit_status"])

    def test_rejects_multiple_pages_at_once(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = write_manifest(root)
            ledger = write_ledger(root)
            evidence = write_evidence(root)
            approval = write_approval(root)

            with self.assertRaisesRegex(ValueError, "one page at a time"):
                approve_manual_acceptance(manifest, ledger, evidence, approval, {"text", "checkbox"})


def write_manifest(root: Path) -> Path:
    manifest = root / "manifest.json"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
                    {
                        "page": "text",
                        "audit_status": "partial",
                        "manual_acceptance_order": 10,
                        "dependency_layer": "foundation-text-selection",
                        "depends_on": [],
                        "required_operations": ["pointer", "drag", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": [
                            "text_drag_selection",
                            "text_keyboard_copy",
                            "text_zero_distance_drag_no_selection",
                        ],
                        "acceptance_observations": [
                            "drag text",
                            "copy text",
                            "zero distance drag no-op",
                        ],
                        "acceptance_evidence_contract": text_evidence_contract(),
                        "gaps": ["manual_acceptance_pending: text pending"],
                    },
                    {
                        "page": "checkbox",
                        "audit_status": "partial",
                        "manual_acceptance_order": 20,
                        "dependency_layer": "binary-choice-state-display",
                        "depends_on": ["text"],
                        "required_operations": ["pointer", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": ["checkbox_pointer_checks_both_rows"],
                        "acceptance_observations": ["both checked"],
                        "gaps": ["manual_acceptance_pending: checkbox pending"],
                    },
                    {
                        "page": "button",
                        "audit_status": "verified",
                        "gaps": [],
                    },
                ]
            }
        ),
        encoding="utf-8",
    )
    return manifest


def write_blocked_manifest(root: Path) -> Path:
    manifest = root / "manifest.json"
    manifest.write_text(
        json.dumps(
            {
                "ui": [
                    {
                        "page": "checkbox",
                        "audit_status": "partial",
                        "manual_acceptance_order": 10,
                        "dependency_layer": "binary-choice-state-display",
                        "depends_on": ["text"],
                        "required_operations": ["pointer", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": ["checkbox_pointer_checks_both_rows"],
                        "acceptance_observations": ["both checked"],
                        "gaps": ["manual_acceptance_pending: checkbox pending"],
                    },
                    {
                        "page": "text",
                        "audit_status": "partial",
                        "manual_acceptance_order": 20,
                        "dependency_layer": "foundation-text-selection",
                        "depends_on": [],
                        "required_operations": ["pointer", "drag", "keyboard"],
                        "minimum_observation_frames": 1,
                        "acceptance_checks": [
                            "text_drag_selection",
                            "text_keyboard_copy",
                            "text_zero_distance_drag_no_selection",
                        ],
                        "acceptance_observations": [
                            "drag text",
                            "copy text",
                            "zero distance drag no-op",
                        ],
                        "acceptance_evidence_contract": text_evidence_contract(),
                        "gaps": ["manual_acceptance_pending: text pending"],
                    },
                ]
            }
        ),
        encoding="utf-8",
    )
    return manifest


def write_manifest_with_text_already_verified(root: Path) -> Path:
    manifest = write_manifest(root)
    payload = json.loads(manifest.read_text(encoding="utf-8"))
    text = next(item for item in payload["ui"] if item["page"] == "text")
    text["audit_status"] = "verified"
    text["gaps"] = []
    manifest.write_text(json.dumps(payload), encoding="utf-8")
    return manifest


def write_evidence(root: Path, pages: tuple[str, ...] = ("text",)) -> Path:
    evidence = root / "evidence.json"
    evidence.write_text(
        json.dumps(
            [
                {
                    "page": page,
                    "command": command_for(page),
                    "smoke_command": smoke_command_for(page),
                    "minimum_observation_frames": 1,
                    "acceptance_observations": acceptance_observations_for(page),
                    "audit_evidence": [
                        evidence_item_for(page, check)
                        for check in acceptance_checks_for(page)
                    ],
                }
                for page in pages
            ]
        ),
        encoding="utf-8",
    )
    return evidence


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
    if page == "text" and check == "text_zero_distance_drag_no_selection":
        item["operation_kind"] = "drag"
        item["state"] = "idle"
        item["body_pixel_diff"] = 0
        item["clipboard_text_len"] = 0
        item["action"] = "none"
        item["event"] = "none"
    if page == "checkbox":
        item["operation_kind"] = "pointer"
        item["action"] = "checkbox_toggle"
        item["event"] = "checked_changed"
        item["body_pixel_diff"] = 12
    return item


def write_approval(root: Path, pages: tuple[str, ...] = ("text",)) -> Path:
    approval = root / "approval.json"
    approval.write_text(
        json.dumps(
            [
                {
                    "page": page,
                    "approved": True,
                    "approved_by": "user",
                    "approved_at": "2026-06-15T00:00:00+09:00",
                    "command": command_for(page),
                    "smoke_command": smoke_command_for(page),
                    "minimum_observation_frames": 1,
            "acceptance_checks": acceptance_checks_for(page),
            "acceptance_observations": acceptance_observations_for(page),
                    "acceptance_evidence_contract": text_evidence_contract()
                    if page == "text"
                    else [],
                    "manual_gate": "do not proceed to the next UI until this page is approved",
                }
                for page in pages
            ]
        ),
        encoding="utf-8",
    )
    return approval


def acceptance_checks_for(page: str) -> list[str]:
    if page == "checkbox":
        return ["checkbox_pointer_checks_both_rows"]
    return [
        "text_drag_selection",
        "text_keyboard_copy",
        "text_zero_distance_drag_no_selection",
    ]


def acceptance_observations_for(page: str) -> list[str]:
    if page == "checkbox":
        return ["both checked"]
    return [
        "drag text",
        "copy text",
        "zero distance drag no-op",
    ]


def text_evidence_contract() -> list[dict[str, str]]:
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
            "check": "text_zero_distance_drag_no_selection",
            "operation_kind": "drag",
            "state": "idle",
            "action": "none",
            "event": "none",
        },
    ]
    return ["drag text", "copy text", "zero distance drag no-op"]


def command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        f"katana-ui-core-storybook --locked -- --open-window {page}"
    )


def smoke_command_for(page: str) -> str:
    return (
        "rtk cargo run --release -p katana-ui-core-storybook --bin "
        f"katana-ui-core-storybook --locked -- --open-window 1 {page}"
    )


def write_ledger(root: Path) -> Path:
    ledger = root / "ledger.md"
    ledger.write_text(
        "\n".join(
            [
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |",
                "| ---: | --- | --- | --- | --- | --- |",
                "| 01 | text | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                "| 01a | text follow-up | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                "| 10 | checkbox | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                "| 10a | checkbox follow-up | detail | ok | ok | manual_acceptance_pending |",
                "| 04 | button | done | ok | ok | 実証済み |",
                "",
            ]
        ),
        encoding="utf-8",
    )
    return ledger


def write_ledger_with_text_already_verified(root: Path) -> Path:
    ledger = root / "ledger.md"
    ledger.write_text(
        "\n".join(
            [
                "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |",
                "| ---: | --- | --- | --- | --- | --- |",
                "| 01 | text | Storybook ユーザー確認済み。 | ok | ok | 実証済み |",
                "| 10 | checkbox | Storybook ユーザー確認は未完了。 | ok | ok | manual_acceptance_pending |",
                "| 10a | checkbox follow-up | detail | ok | ok | manual_acceptance_pending |",
                "",
            ]
        ),
        encoding="utf-8",
    )
    return ledger


if __name__ == "__main__":
    unittest.main()
