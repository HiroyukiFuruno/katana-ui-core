#!/usr/bin/env python3
from __future__ import annotations

import unittest
from pathlib import Path

from storybook_compute_audit import (
    ROOT,
    manifest_manual_acceptance_pending,
    resolve_under_root,
)


class StorybookComputeAuditTest(unittest.TestCase):
    def test_resolves_relative_output_under_repo_root(self) -> None:
        self.assertEqual(
            ROOT / "target/storybook-compute-audit",
            resolve_under_root("target/storybook-compute-audit"),
        )

    def test_keeps_absolute_output_path(self) -> None:
        path = Path("/tmp/storybook-compute-audit")

        self.assertEqual(path, resolve_under_root(str(path)))

    def test_acceptance_checks_do_not_mean_manual_acceptance_pending(self) -> None:
        self.assertFalse(
            manifest_manual_acceptance_pending(
                {
                    "page": "text",
                    "acceptance_checks": ["drag selection"],
                    "audit_status": "verified",
                }
            )
        )

    def test_manual_acceptance_gap_means_pending(self) -> None:
        self.assertTrue(
            manifest_manual_acceptance_pending(
                {
                    "page": "text",
                    "gaps": ["manual_acceptance_pending: user confirmation required"],
                }
            )
        )


if __name__ == "__main__":
    unittest.main()
