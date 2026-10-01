"""Unit contract for release tool archives, not native IME evidence."""
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import tempfile
import unittest
import zipfile


class NativeToolPackageTest(unittest.TestCase):
    def test_archive_pins_binary_bytes_and_does_not_claim_measurement(self):
        script = Path(__file__).parent / "release/package-native-ime-tools.py"
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "Cargo.toml").write_text('[workspace.package]\nversion = "0.4.1"\n')
            (root / "docs").mkdir()
            (root / "docs/native-ime-evidence.md").write_text("Unmeasured native implementation")
            (root / "target/release").mkdir(parents=True)
            suffix = ".exe" if platform.system() == "Windows" else ""
            names = ["katana-ui-core-native-ime-evidence", "kuc-native-ime-verify"]
            for name in names:
                (root / "target/release" / (name + suffix)).write_bytes(name.encode())
            output = root / "github-output"
            result = subprocess.run([sys.executable, str(script.resolve()), "--output", "artifacts", "--sha", "a" * 40], cwd=root, env={**os.environ, "GITHUB_OUTPUT": str(output)}, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            archives = list((root / "artifacts").glob("*.zip"))
            self.assertEqual(len(archives), 1)
            with zipfile.ZipFile(archives[0]) as archive:
                manifest = json.loads(archive.read("manifest.json"))
                self.assertFalse(manifest["native_ime_measured"])
                self.assertEqual(manifest["revision"], "a" * 40)
                self.assertEqual(manifest["version"], "0.4.1")
                self.assertIn(manifest["platform"], ["macos", "windows", "linux"])
                for name in names:
                    key = name + suffix
                    self.assertEqual(manifest["binary_sha256"][key], hashlib.sha256(archive.read(key)).hexdigest())
            self.assertIn("tag=v0.4.1", output.read_text())


if __name__ == "__main__":
    unittest.main()
