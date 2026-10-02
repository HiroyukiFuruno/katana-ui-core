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
    @staticmethod
    def write_valid_archives(root, *, revision="a" * 40, version="0.4.1"):
        names = ["katana-ui-core-native-ime-evidence", "kuc-native-ime-verify"]
        artifacts = root / "artifacts"
        artifacts.mkdir()
        for platform_name in ("macos", "windows", "linux"):
            suffix = ".exe" if platform_name == "windows" else ""
            binary_data = {name + suffix: name.encode() for name in names}
            manifest = {
                "version": version,
                "revision": revision,
                "platform": platform_name,
                "architecture": "x86_64",
                "native_ime_measured": False,
                "binary_sha256": {name: hashlib.sha256(data).hexdigest() for name, data in binary_data.items()},
            }
            archive = artifacts / f"kuc-native-ime-tools-v{version}-{platform_name}-x86_64.zip"
            with zipfile.ZipFile(archive, "w") as target:
                for name, data in binary_data.items():
                    target.writestr(name, data)
                target.writestr("manifest.json", json.dumps(manifest))
        return artifacts

    @staticmethod
    def verify(verifier_script, artifacts, *, version="0.4.1", revision="a" * 40):
        return subprocess.run(
            [sys.executable, str(verifier_script.resolve()), "--directory", str(artifacts), "--version", version, "--revision", revision],
            capture_output=True,
            text=True,
        )

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

    def test_archive_verifier_rejects_tampered_binary(self):
        verifier_script = Path(__file__).parent / "release/verify-native-ime-archives.py"
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            names = ["katana-ui-core-native-ime-evidence", "kuc-native-ime-verify"]
            artifacts = self.write_valid_archives(root)
            tampered = artifacts / "kuc-native-ime-tools-v0.4.1-linux-x86_64.zip"
            with zipfile.ZipFile(tampered) as source:
                members = {name: source.read(name) for name in source.namelist()}
            members[names[0]] = b"tampered"
            with zipfile.ZipFile(tampered, "w") as target:
                for name, data in members.items():
                    target.writestr(name, data)
            verified = self.verify(verifier_script, artifacts)
            self.assertNotEqual(verified.returncode, 0)
            self.assertIn("SHA-256 mismatch", verified.stderr)

    def test_archive_verifier_rejects_wrong_revision_and_version(self):
        verifier_script = Path(__file__).parent / "release/verify-native-ime-archives.py"
        with tempfile.TemporaryDirectory() as temporary:
            artifacts = self.write_valid_archives(Path(temporary))
            self.assertNotEqual(self.verify(verifier_script, artifacts, revision="b" * 40).returncode, 0)
            self.assertNotEqual(self.verify(verifier_script, artifacts, version="0.4.2").returncode, 0)

    def test_archive_verifier_rejects_missing_or_duplicate_platform(self):
        verifier_script = Path(__file__).parent / "release/verify-native-ime-archives.py"
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            artifacts = self.write_valid_archives(root)
            (artifacts / "kuc-native-ime-tools-v0.4.1-linux-x86_64.zip").unlink()
            self.assertNotEqual(self.verify(verifier_script, artifacts).returncode, 0)

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            artifacts = self.write_valid_archives(root)
            original = artifacts / "kuc-native-ime-tools-v0.4.1-linux-x86_64.zip"
            duplicate = artifacts / "kuc-native-ime-tools-v0.4.1-macos-x86_64-linux.zip"
            with zipfile.ZipFile(original) as source:
                members = {name: source.read(name) for name in source.namelist()}
            manifest = json.loads(members["manifest.json"])
            manifest["platform"] = "macos"
            manifest["architecture"] = "x86_64-linux"
            members["manifest.json"] = json.dumps(manifest).encode()
            original.unlink()
            with zipfile.ZipFile(duplicate, "w") as target:
                for name, data in members.items():
                    target.writestr(name, data)
            self.assertNotEqual(self.verify(verifier_script, artifacts).returncode, 0)

    def test_archive_verifier_rejects_filename_mismatch_and_duplicate_binary(self):
        verifier_script = Path(__file__).parent / "release/verify-native-ime-archives.py"
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            artifacts = self.write_valid_archives(root)
            original = artifacts / "kuc-native-ime-tools-v0.4.1-linux-x86_64.zip"
            original.rename(artifacts / "wrong-name.zip")
            self.assertNotEqual(self.verify(verifier_script, artifacts).returncode, 0)

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            artifacts = self.write_valid_archives(root)
            duplicate = artifacts / "kuc-native-ime-tools-v0.4.1-linux-x86_64.zip"
            with zipfile.ZipFile(duplicate) as source:
                members = [(info.filename, source.read(info.filename)) for info in source.infolist()]
            with zipfile.ZipFile(duplicate, "w") as target:
                for name, data in members:
                    target.writestr(name, data)
                target.writestr("katana-ui-core-native-ime-evidence", b"duplicate")
            self.assertNotEqual(self.verify(verifier_script, artifacts).returncode, 0)


if __name__ == "__main__":
    unittest.main()
