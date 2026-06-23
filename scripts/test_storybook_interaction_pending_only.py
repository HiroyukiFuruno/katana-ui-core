#!/usr/bin/env python3
from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from storybook_interaction_pending_only import (
    pending_only_difference_failures,
    pending_only_failures,
)


class StorybookInteractionPendingOnlyTest(unittest.TestCase):
    def test_accepts_when_manifest_smoke_has_only_non_blocking_manual_pending(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(root, status="partial")

            self.assertEqual([], pending_only_failures(manifest, audit))

    def test_rejects_extra_live_audit_failure_mixed_with_manual_pending(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(root, status="partial", include_audit=False)

            failures = pending_only_failures(manifest, audit)

            self.assertIn(
                "unexpected failure: text: live interaction audit scenario is missing",
                failures,
            )

    def test_rejects_unexpected_blocking_failure(self) -> None:
        failures = pending_only_difference_failures(
            actual={
                "text: live interaction audit scenario is missing",
            },
            expected=set(),
        )

        self.assertIn(
            "unexpected failure: text: live interaction audit scenario is missing",
            failures,
        )

    def test_accepts_when_manifest_smoke_has_no_failures_and_no_manual_pending(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest, audit = write_fixture(
                root,
                status="verified",
                gaps=(),
            )

            self.assertEqual([], pending_only_failures(manifest, audit))


def write_fixture(
    root: Path,
    status: str,
    gaps: tuple[str, ...] = ("manual_acceptance_pending: user confirmation",),
    include_audit: bool = True,
) -> tuple[Path, Path]:
    manifest = root / "manifest.json"
    audit = root / "audit.json"
    gaps_json = ",".join(f'"{gap}"' for gap in gaps)
    manifest.write_text(
        "{"
        '"operation_kinds":["drag","keyboard"],'
        '"defaults_by_engine":{"text-entry":{"required_operations":["drag","keyboard"]}},'
        '"ui":[{'
        '"page":"text",'
        '"engine":"text-entry",'
        '"audit_status":"'
        + status
        + '",'
        '"required_operations":["drag","keyboard"],'
        '"acceptance_checks":["text_drag_selection","text_keyboard_copy","text_keyboard_paste","text_zero_distance_drag_no_selection"],'
        '"gaps":['
        + gaps_json
        + "]"
        "}]"
        "}",
        encoding="utf-8",
    )
    scenarios = (
        "{"
        '"page":"text",'
        '"operation_kind":"drag",'
        '"operation":"text_drag_selection",'
        '"passed":true,'
        '"action":"selection_drag",'
        '"event":"selection_changed",'
        '"state":"selected=Markdown",'
        '"body_pixel_diff":1,'
        '"clipboard_text_len":0'
        "},"
        "{"
        '"page":"text",'
        '"operation_kind":"keyboard",'
        '"operation":"text_keyboard_copy",'
        '"passed":true,'
        '"action":"copy_selection",'
        '"event":"clipboard_copy",'
        '"state":"clipboard=Markdown",'
        '"body_pixel_diff":1,'
        '"clipboard_text_len":8'
        "}"
        ","
        "{"
        '"page":"text",'
        '"operation_kind":"drag",'
        '"operation":"text_zero_distance_drag_no_selection",'
        '"passed":true,'
        '"action":"none",'
        '"event":"none",'
        '"state":"idle",'
        '"body_pixel_diff":0,'
        '"clipboard_text_len":0'
        "}"
        ","
        "{"
        '"page":"text",'
        '"operation_kind":"keyboard",'
        '"operation":"text_keyboard_paste",'
        '"passed":true,'
        '"action":"none",'
        '"event":"none",'
        '"state":"idle",'
        '"body_pixel_diff":0,'
        '"clipboard_text_len":0'
        "}"
        if include_audit
        else ""
    )
    audit.write_text(
        '{"scenarios":[' + scenarios + "]}",
        encoding="utf-8",
    )
    return manifest, audit


if __name__ == "__main__":
    unittest.main()
