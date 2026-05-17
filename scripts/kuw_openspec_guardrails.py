#!/usr/bin/env python3
from pathlib import Path
import re

RESPONSIBILITY_EVIDENCE = (
    "types.rs",
    "ops.rs",
    "mod.rs",
    "view.rs",
    "/tests/",
    "storybook/src/pages",
)


class KuwOpenSpecGuardrails:
    def __init__(self, root: Path) -> None:
        self.root = root

    def evidence_failures(self) -> list[str]:
        failures: list[str] = []
        for task_file in self.task_files():
            items = self.checked_task_paths(task_file)
            storybook_pages: list[str] = []
            for line_number, path_tokens in items:
                for token in path_tokens:
                    if self.is_storybook_page_token(token):
                        storybook_pages.append(token)
                    if not self.resolve_path(token).exists():
                        failures.append(f"{self.relative(task_file)}:{line_number}: missing `{token}`")
            if any(not self.storybook_page_registered(token) for token in storybook_pages):
                failures.append(f"{self.relative(task_file)}: Storybook page task lacks pages/mod.rs evidence")
        return failures

    def file_length_review_failures(self) -> list[str]:
        failures: list[str] = []
        for task_file in self.task_files():
            if not self.has_checked_file_length_signal(task_file):
                continue
            paths = {token for _, tokens in self.checked_task_paths(task_file) for token in tokens}
            if not any(self.is_long_rust_file(token) for token in paths):
                continue
            missing = [item for item in RESPONSIBILITY_EVIDENCE if not self.has_evidence(paths, item)]
            if missing:
                failures.append(f"{self.relative(task_file)}: missing responsibility evidence: {', '.join(missing)}")
        return failures

    def task_files(self) -> list[Path]:
        changes = self.root / "openspec" / "changes"
        if not changes.exists():
            return []
        return [
            path
            for path in sorted(changes.rglob("tasks.md"))
            if "archive" not in path.relative_to(changes).parts
        ]

    def has_checked_file_length_signal(self, task_file: Path) -> bool:
        for line in self.read(task_file).splitlines():
            normalized = line.lower()
            if line.lstrip().startswith("- [x] ") and (
                "file-length" in normalized or "type-separation" in normalized
            ):
                return True
        return False

    def checked_task_paths(self, task_file: Path) -> list[tuple[int, list[str]]]:
        items: list[tuple[int, list[str]]] = []
        for line_number, line in enumerate(self.read(task_file).splitlines(), start=1):
            if not line.lstrip().startswith("- [x] "):
                continue
            paths = [token for token in self.backtick_tokens(line) if self.looks_like_path(token)]
            if paths:
                items.append((line_number, paths))
        return items

    def backtick_tokens(self, line: str) -> list[str]:
        return [self.normalize_token(token) for token in re.findall(r"`([^`]+)`", line)]

    def looks_like_path(self, token: str) -> bool:
        return "/" in token and (token.endswith(".rs") or token.endswith(".md"))

    def normalize_token(self, token: str) -> str:
        return token.strip().strip("'\"`").rstrip(",.;")

    def resolve_path(self, token: str) -> Path:
        path = Path(self.normalize_token(token))
        if path.is_absolute():
            return path
        for base in (self.root, self.root / "crates/katana-ui-core/src", self.root / "storybook/src"):
            candidate = base / path
            if candidate.exists():
                return candidate
        return self.root / path

    def is_long_rust_file(self, token: str) -> bool:
        path = self.resolve_path(token)
        return path.suffix == ".rs" and path.exists() and len(self.read(path).splitlines()) > 250

    def has_evidence(self, paths: set[str], required: str) -> bool:
        if required == "/tests/":
            return any("/tests/" in path or path.endswith("_test.rs") for path in paths)
        return any(path.endswith(required) or required in path for path in paths)

    def is_storybook_page_token(self, token: str) -> bool:
        path = self.resolve_path(token)
        return "storybook/src/pages/" in path.as_posix() and path.name != "mod.rs"

    def storybook_page_registered(self, token: str) -> bool:
        path = self.resolve_path(token)
        mod_file = self.root / "storybook/src/pages/mod.rs"
        if not mod_file.exists():
            return False
        module_name = path.stem
        mod_source = self.read(mod_file)
        return f"mod {module_name};" in mod_source or f"pub mod {module_name};" in mod_source

    def relative(self, path: Path) -> str:
        return path.relative_to(self.root).as_posix()

    def read(self, path: Path) -> str:
        return path.read_text(encoding="utf-8")
