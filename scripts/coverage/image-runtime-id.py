#!/usr/bin/env python3
"""Derive a stable fail-closed identity for a Docker image's runtime content."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import re
import sys
from collections.abc import Mapping

RUNTIME_PREFIX = "runtime-v1:sha256:"
RUNTIME_ID_PATTERN = re.compile(r"^runtime-v1:sha256:[0-9a-f]{64}$")
LAYER_DIGEST_PATTERN = re.compile(r"^sha256:[0-9a-f]{64}$")
REQUIRED_IMAGE_FIELDS = ("Architecture", "Os", "RootFS", "Config")


class ImageRuntimeIdentityError(ValueError):
    """Raised when Docker inspect data cannot identify one runtime image."""


def required_mapping(image: Mapping[str, object], field: str) -> Mapping[str, object]:
    value = image.get(field)
    if not isinstance(value, Mapping):
        raise ImageRuntimeIdentityError(f"image field {field} must be an object")
    return value


def required_string(image: Mapping[str, object], field: str) -> str:
    value = image.get(field)
    if not isinstance(value, str) or not value:
        raise ImageRuntimeIdentityError(f"image field {field} must be a non-empty string")
    return value


def runtime_payload(inspect_payload: object) -> dict[str, object]:
    if not isinstance(inspect_payload, list) or len(inspect_payload) != 1:
        raise ImageRuntimeIdentityError("docker inspect must return exactly one image")
    image = inspect_payload[0]
    if not isinstance(image, Mapping):
        raise ImageRuntimeIdentityError("docker inspect image must be an object")

    architecture = required_string(image, "Architecture")
    operating_system = required_string(image, "Os")
    variant = image.get("Variant")
    if variant is not None and not isinstance(variant, str):
        raise ImageRuntimeIdentityError("image field Variant must be a string or null")

    rootfs = required_mapping(image, "RootFS")
    if rootfs.get("Type") != "layers":
        raise ImageRuntimeIdentityError("image RootFS.Type must be layers")
    layers = rootfs.get("Layers")
    if not isinstance(layers, list) or not layers:
        raise ImageRuntimeIdentityError("image RootFS.Layers must be a non-empty list")
    if not all(isinstance(layer, str) and LAYER_DIGEST_PATTERN.fullmatch(layer) for layer in layers):
        raise ImageRuntimeIdentityError("image RootFS.Layers contains an invalid digest")

    config = required_mapping(image, "Config")
    return {
        "schema": "kuc-coverage-runtime-v1",
        "architecture": architecture,
        "os": operating_system,
        "variant": variant,
        "rootfs": rootfs,
        "config": config,
    }


def runtime_identity(inspect_payload: object) -> str:
    canonical = json.dumps(
        runtime_payload(inspect_payload),
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    identity = RUNTIME_PREFIX + hashlib.sha256(canonical).hexdigest()
    if not RUNTIME_ID_PATTERN.fullmatch(identity):
        raise ImageRuntimeIdentityError("generated runtime identity has an invalid format")
    return identity


def expect_rejected(payload: object) -> None:
    try:
        runtime_identity(payload)
    except ImageRuntimeIdentityError:
        return
    raise ImageRuntimeIdentityError("invalid docker inspect fixture was accepted")


def self_test() -> None:
    layer_one = "sha256:" + "1" * 64
    layer_two = "sha256:" + "2" * 64
    base = [
        {
            "Id": "sha256:" + "a" * 64,
            "Created": "2026-01-01T00:00:00Z",
            "Architecture": "arm64",
            "Os": "linux",
            "Variant": None,
            "RootFS": {"Type": "layers", "Layers": [layer_one]},
            "Config": {"Env": ["PATH=/usr/bin"], "Cmd": ["bash"]},
        }
    ]
    baseline = runtime_identity(base)

    volatile_metadata_changed = copy.deepcopy(base)
    volatile_metadata_changed[0]["Id"] = "sha256:" + "b" * 64
    volatile_metadata_changed[0]["Created"] = "2026-02-01T00:00:00Z"
    if runtime_identity(volatile_metadata_changed) != baseline:
        raise ImageRuntimeIdentityError("volatile manifest metadata changed runtime identity")

    rootfs_changed = copy.deepcopy(base)
    rootfs_changed[0]["RootFS"]["Layers"] = [layer_two]
    if runtime_identity(rootfs_changed) == baseline:
        raise ImageRuntimeIdentityError("RootFS change did not change runtime identity")

    config_changed = copy.deepcopy(base)
    config_changed[0]["Config"]["Env"] = ["PATH=/opt/bin"]
    if runtime_identity(config_changed) == baseline:
        raise ImageRuntimeIdentityError("Config change did not change runtime identity")

    for field in REQUIRED_IMAGE_FIELDS:
        missing = copy.deepcopy(base)
        del missing[0][field]
        expect_rejected(missing)
    malformed_layer = copy.deepcopy(base)
    malformed_layer[0]["RootFS"]["Layers"] = ["not-a-digest"]
    expect_rejected(malformed_layer)
    expect_rejected([])
    expect_rejected([base[0], base[0]])


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    try:
        if args.self_test:
            self_test()
            return 0
        print(runtime_identity(json.load(sys.stdin)))
    except (ImageRuntimeIdentityError, json.JSONDecodeError) as error:
        print(f"coverage image runtime identity failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
