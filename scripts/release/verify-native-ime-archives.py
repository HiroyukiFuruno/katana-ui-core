#!/usr/bin/env python3
"""Fail-closed verification for the three native tool release archives."""

import argparse
import hashlib
import json
from pathlib import Path
import zipfile


PLATFORMS = {"macos", "windows", "linux"}
BINARY_BASE_NAMES = (
    "katana-ui-core-native-ime-evidence",
    "kuc-native-ime-verify",
)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fail(message: str) -> None:
    raise SystemExit(message)


def verify_archive(path: Path, version: str, revision: str) -> str:
    if not path.name.startswith(f"kuc-native-ime-tools-v{version}-"):
        fail(f"unexpected archive name: {path.name}")
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        manifest_names = [name for name in names if name == "manifest.json"]
        if manifest_names != ["manifest.json"]:
            fail(f"{path.name}: expected exactly one manifest.json")
        try:
            manifest = json.loads(archive.read("manifest.json"))
        except (KeyError, json.JSONDecodeError) as error:
            fail(f"{path.name}: invalid manifest.json: {error}")
        platform_name = manifest.get("platform")
        if platform_name not in PLATFORMS:
            fail(f"{path.name}: unsupported platform {platform_name!r}")
        architecture = manifest.get("architecture")
        if not isinstance(architecture, str) or not architecture.strip():
            fail(f"{path.name}: architecture must be non-empty")
        expected_name = f"kuc-native-ime-tools-v{version}-{platform_name}-{architecture}.zip"
        if path.name != expected_name:
            fail(f"{path.name}: filename does not match platform/architecture manifest")
        expected_suffix = ".exe" if platform_name == "windows" else ""
        expected_binaries = [name + expected_suffix for name in BINARY_BASE_NAMES]
        for binary in expected_binaries:
            if names.count(binary) != 1:
                fail(f"{path.name}: expected exactly one ZIP entry for {binary}")
        if manifest.get("version") != version:
            fail(f"{path.name}: version does not match {version}")
        if manifest.get("revision") != revision:
            fail(f"{path.name}: revision does not match {revision}")
        if manifest.get("native_ime_measured") is not False:
            fail(f"{path.name}: native_ime_measured must be false")
        binary_hashes = manifest.get("binary_sha256")
        if not isinstance(binary_hashes, dict) or set(binary_hashes) != set(expected_binaries):
            fail(f"{path.name}: binary_sha256 must contain exactly the required binaries")
        for binary in expected_binaries:
            digest = binary_hashes.get(binary)
            if not isinstance(digest, str) or len(digest) != 64 or any(c not in "0123456789abcdef" for c in digest):
                fail(f"{path.name}: invalid SHA-256 for {binary}")
            if digest != sha256(archive.read(binary)):
                fail(f"{path.name}: SHA-256 mismatch for {binary}")
        return platform_name


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--directory", type=Path, required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--revision", required=True)
    args = parser.parse_args()
    version = args.version.removeprefix("v")
    archives = sorted(args.directory.glob("*.zip"))
    if len(archives) != 3:
        fail(f"expected exactly three native tool archives, found {len(archives)}")
    platforms = [verify_archive(path, version, args.revision) for path in archives]
    if set(platforms) != PLATFORMS or len(set(platforms)) != 3:
        fail(f"expected exactly one archive for each platform, found {platforms}")
    print(f"verified native tool archives: {', '.join(sorted(platforms))}")


if __name__ == "__main__":
    main()
