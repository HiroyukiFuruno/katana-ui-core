"""Package native tools; binary distribution is not an IME measurement."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import tomllib
import zipfile


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--sha", required=True)
    args = parser.parse_args()
    version = tomllib.loads(Path("Cargo.toml").read_text())["workspace"]["package"]["version"]
    native_platform = {"Darwin": "macos", "Windows": "windows", "Linux": "linux"}[platform.system()]
    target = native_platform + "-" + platform.machine().lower()
    suffix = ".exe" if platform.system() == "Windows" else ""
    names = ["katana-ui-core-native-ime-evidence", "kuc-native-ime-verify"]
    binaries = [(name + suffix, Path("target/release") / (name + suffix)) for name in names]
    manifest = {"version": version, "revision": args.sha, "platform": native_platform, "architecture": platform.machine().lower(),
                "native_ime_measured": False,
                "binary_sha256": {name: hashlib.sha256(path.read_bytes()).hexdigest() for name, path in binaries}}
    args.output.mkdir(parents=True, exist_ok=True)
    archive = args.output / f"kuc-native-ime-tools-v{version}-{target}.zip"
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as output:
        for name, path in binaries:
            output.write(path, name)
        output.writestr("manifest.json", json.dumps(manifest, indent=2) + "\n")
        output.write("docs/native-ime-evidence.md", "README.md")
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as output:
            output.write(f"tag=v{version}\narchive={archive}\n")
    print(archive)


if __name__ == "__main__":
    main()
