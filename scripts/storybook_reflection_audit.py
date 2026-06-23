#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PAGE_TOKEN = re.compile(r'"([a-z0-9-]+)"')
ARM_TOKEN = re.compile(r'((?:"[a-z0-9-]+"(?:\s*\|\s*)?)+)\s*=>')
SPEC_TOKEN = re.compile(r'"([a-z0-9-]+)"\s*=>\s*Some\(spec')


@dataclass(frozen=True)
class AuditFinding:
    code: str
    page: str
    message: str

    def format(self) -> str:
        return f"{self.code}: {self.page}: {self.message}"


class StorybookReflectionAudit:
    def __init__(self, root: Path = ROOT) -> None:
        self.root = root

    def findings(self) -> list[AuditFinding]:
        required = self.required_pages()
        dedicated = self.page_specific_surfaces()
        presets = self.page_specific_presets()
        specs = self.explicit_interaction_specs()
        findings: list[AuditFinding] = []
        findings.extend(self.missing(required, dedicated, "missing-dedicated-surface"))
        findings.extend(self.missing(required, presets, "missing-page-preset"))
        findings.extend(self.missing(required, specs, "missing-interaction-spec"))
        return findings

    def required_pages(self) -> list[str]:
        source = self.read("crates/katana-ui-core-storybook/src/requirements.rs")
        required_block = source.split("const MIN_SINGLE_NODE", 1)[0]
        return PAGE_TOKEN.findall(required_block)

    def page_specific_surfaces(self) -> set[str]:
        source = self.read("crates/katana-ui-core-storybook/src/visual/dedicated.rs")
        match_block = self.first_match_block(source)
        return self.pages_from_match_arms(match_block)

    def page_specific_presets(self) -> set[str]:
        presets: set[str] = set()
        for path in self.preset_label_paths():
            source = path.read_text(encoding="utf-8")
            match_block = self.first_match_block(source)
            presets.update(self.pages_from_match_arms(match_block))
        return presets

    def preset_label_paths(self) -> tuple[Path, ...]:
        base = self.root / "crates/katana-ui-core-storybook/src/catalog"
        return tuple(sorted(base.glob("preset_label*.rs")))

    def explicit_interaction_specs(self) -> set[str]:
        specs: set[str] = set()
        for path in self.interaction_spec_paths():
            specs.update(SPEC_TOKEN.findall(path.read_text(encoding="utf-8")))
        return specs

    def interaction_spec_paths(self) -> tuple[Path, ...]:
        base = self.root / "crates/katana-ui-core-storybook/src/visual"
        return tuple(sorted(base.glob("interaction_spec_*.rs")))

    def read(self, relative: str) -> str:
        return (self.root / relative).read_text(encoding="utf-8")

    @staticmethod
    def first_match_block(source: str) -> str:
        if "match page {" not in source:
            return ""
        return source.split("match page {", 1)[1].split("_ =>", 1)[0]

    @staticmethod
    def pages_from_match_arms(source: str) -> set[str]:
        pages: set[str] = set()
        for arm in ARM_TOKEN.finditer(source):
            pages.update(PAGE_TOKEN.findall(arm.group(1)))
        return pages

    @staticmethod
    def missing(required: list[str], actual: set[str], code: str) -> list[AuditFinding]:
        return [
            AuditFinding(code, page, StorybookReflectionAudit.message_for(code))
            for page in required
            if page not in actual
        ]

    @staticmethod
    def message_for(code: str) -> str:
        messages = {
            "missing-dedicated-surface": "required page is still handled by the generic Storybook renderer",
            "missing-page-preset": "required page has no explicit Storybook preset labels",
            "missing-interaction-spec": "required page has no explicit option/action/event/state spec",
        }
        return messages[code]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--strict", action="store_true")
    parser.add_argument("--root", type=Path, default=ROOT)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    findings = StorybookReflectionAudit(args.root).findings()
    if not findings:
        print("storybook reflection audit passed")
        return 0
    print("storybook reflection audit found missing implementation")
    for finding in findings:
        print(f"- {finding.format()}")
    return 1 if args.strict else 0


if __name__ == "__main__":
    raise SystemExit(main())
