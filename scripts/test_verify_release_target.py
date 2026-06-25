#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import os
import sys
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).parent / "release" / "verify-release-target.py"
SPEC = importlib.util.spec_from_file_location("verify_release_target", MODULE_PATH)
assert SPEC is not None
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)
StableVersion = MODULE.StableVersion
verify = MODULE.verify


def write_text(path: Path, source: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


def corrective_note(version: str) -> str:
    return (
        f"# KUC {version} corrective release\n\n"
        "The corrective release reason is documented.\n\n"
        "```bash\n"
        f"KUC_RELEASE_ALLOW_VERSION_LINE_OVERRIDE=1 just VERSION={version} release-check\n"
        "```\n"
    )


class VerifyReleaseTargetTest(unittest.TestCase):
    def setUp(self) -> None:
        self.previous_override = os.environ.pop("KUC_RELEASE_ALLOW_VERSION_LINE_OVERRIDE", None)

    def tearDown(self) -> None:
        if self.previous_override is not None:
            os.environ["KUC_RELEASE_ALLOW_VERSION_LINE_OVERRIDE"] = self.previous_override

    def test_rejects_non_consecutive_patch_without_corrective_note(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            status = verify(StableVersion.parse("v0.1.3"), StableVersion.parse("v0.1.1"), Path(tmp))

            self.assertEqual(1, status)

    def test_accepts_documented_corrective_patch_release(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(root / "docs/release/v0.1.3.md", corrective_note("v0.1.3"))

            status = verify(StableVersion.parse("v0.1.3"), StableVersion.parse("v0.1.1"), root)

            self.assertEqual(0, status)

    def test_rejects_incomplete_corrective_note(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(root / "docs/release/v0.1.3.md", "# KUC v0.1.3\n\nreason only\n")

            status = verify(StableVersion.parse("v0.1.3"), StableVersion.parse("v0.1.1"), root)

            self.assertEqual(1, status)

    def test_documented_note_does_not_allow_releasing_existing_version(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(root / "docs/release/v0.1.1.md", corrective_note("v0.1.1"))

            status = verify(StableVersion.parse("v0.1.1"), StableVersion.parse("v0.1.1"), root)

            self.assertEqual(1, status)


if __name__ == "__main__":
    unittest.main()
