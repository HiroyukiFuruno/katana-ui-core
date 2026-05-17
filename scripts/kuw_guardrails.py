#!/usr/bin/env python3
from pathlib import Path
import re
import sys

from kuw_openspec_guardrails import KuwOpenSpecGuardrails

RUNTIME_KEYWORDS = (
    "drag",
    "reset",
    "resize",
    "clamp",
    "ratio",
    "move",
    "compute",
    "position",
    "bounds",
    "cursor",
    "scroll",
)
TEST_HELPER_SUFFIXES = ("_for_test",)
LONG_HELPER_ONLY_VIEW_LINES = 220

CALLBACK_RULES = (
    (("composite", "selector", "toggle"), ("on_change",)),
    (("composite", "selector", "segmented"), ("on_change",)),
    (("composite", "selector", "select"), ("on_change", "on_select")),
    (("composite", "selector", "color"), ("on_change", "on_click", "on_input")),
    (("composite", "selector", "color_picker"), ("on_change",)),
    (("composite", "combo_box"), ("on_change", "on_select")),
    (("composite", "menu_button"), ("on_open", "on_close")),
    (("composite", "input", "text"), ("on_input", "on_change", "on_submit")),
    (("composite", "input", "search"), ("on_submit", "on_change", "on_input")),
    (("layout", "accordion"), ("on_toggle",)),
)

class KuwGuardrails:
    def __init__(self, root: Path) -> None:
        self.root = root

    def run(self) -> list[str]:
        failures: list[str] = []
        failures.extend(self.runtime_api_failures())
        failures.extend(self.callback_failures())
        failures.extend(self.storybook_leak_failures())
        failures.extend(self.helper_only_view_failures())
        failures.extend(self.openspec_evidence_failures())
        failures.extend(self.file_length_review_failures())
        return failures

    def rust_files(self, base: Path) -> list[Path]:
        if not base.exists():
            return []
        return sorted(base.rglob("*.rs"))

    def relative(self, path: Path) -> str:
        return path.relative_to(self.root).as_posix()

    def read(self, path: Path) -> str:
        return path.read_text(encoding="utf-8")

    def runtime_api_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "crates"):
            if path.name not in {"ops.rs", "view.rs"}:
                continue
            pending_cfg_test = False
            for line_number, line in enumerate(self.read(path).splitlines(), start=1):
                stripped = line.strip()
                if stripped.startswith("#[cfg(test)]"):
                    pending_cfg_test = True
                    continue
                if pending_cfg_test and not stripped:
                    continue
                if pending_cfg_test:
                    name = self.public_fn_name(stripped)
                    if name and self.is_test_only_runtime_name(name):
                        failures.append(
                            f"{self.relative(path)}:{line_number}: runtime API `{name}` is test-only"
                        )
                    pending_cfg_test = False
        return failures

    def public_fn_name(self, line: str) -> str | None:
        pattern = r"\bpub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([A-Za-z0-9_]+)"
        match = re.search(pattern, line)
        if match:
            return match.group(1)
        return None

    def is_test_only_runtime_name(self, name: str) -> bool:
        lowered = name.lower()
        if lowered.endswith(TEST_HELPER_SUFFIXES):
            return False
        return any(keyword in lowered for keyword in RUNTIME_KEYWORDS)

    def callback_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "crates"):
            if path.name != "types.rs":
                continue
            required = self.required_callbacks(path.as_posix())
            if not required:
                continue
            source = self.read(path)
            if "Props" not in source:
                continue
            if not any(token in source for token in required):
                failures.append(
                    f"{self.relative(path)}: interactive Props missing callback: {'/'.join(required)}"
                )
        return failures

    def required_callbacks(self, path_text: str) -> tuple[str, ...]:
        for segments, callbacks in CALLBACK_RULES:
            if all(segment in path_text for segment in segments):
                return callbacks
        return ()

    def storybook_leak_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "storybook" / "src"):
            lines = self.read(path).splitlines()
            for index, line in enumerate(lines):
                if "Box::leak" not in line:
                    continue
                start = max(0, index - 2)
                end = min(len(lines), index + 3)
                allowed = any(
                    "WHY: allow(storybook_box_leak)" in candidate
                    for candidate in lines[start:end]
                )
                if not allowed:
                    failures.append(f"{self.relative(path)}:{index + 1}: Box::leak is not allowed")
        return failures

    def helper_only_view_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "crates"):
            if path.name != "view.rs":
                continue
            source = self.production_source(self.read(path))
            lines = source.splitlines()
            if len(lines) <= LONG_HELPER_ONLY_VIEW_LINES:
                continue
            if re.search(r"\b(struct|enum|impl|type)\b", source):
                continue
            if "fn " not in source and "const " not in source:
                continue
            view_pattern = r"->\s+(impl\s+IntoView|impl\s+View|Box<dyn\s+View>|[A-Za-z0-9_]*View)\b"
            if re.search(view_pattern, source):
                continue
            failures.append(f"{self.relative(path)}: view.rs is helper-only")
        return failures

    def production_source(self, source: str) -> str:
        marker = "\n#[cfg(test)]\nmod tests"
        if marker in source:
            return source.split(marker, maxsplit=1)[0]
        return source

    def openspec_evidence_failures(self) -> list[str]:
        return KuwOpenSpecGuardrails(self.root).evidence_failures()

    def file_length_review_failures(self) -> list[str]:
        return KuwOpenSpecGuardrails(self.root).file_length_review_failures()


def main() -> int:
    failures = KuwGuardrails(Path.cwd()).run()
    if failures:
        print("katana-ui-core guardrails failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
