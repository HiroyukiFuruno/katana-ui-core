#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys
from pathlib import Path

from storybook_ui_harness_assertions import StorybookUiHarness

ROOT = Path(__file__).resolve().parents[1]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=ROOT)
    return parser.parse_args()


def main() -> int:
    failures = StorybookUiHarness(parse_args().root).failures()
    if not failures:
        print("storybook ui harness passed")
        return 0
    print("storybook ui harness failed", file=sys.stderr)
    for failure in failures:
        print(f"- {failure}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
