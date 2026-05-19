#!/usr/bin/env python3
from pathlib import Path
import re
import sys

from kuc_openspec_guardrails import KucOpenSpecGuardrails

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

class KucGuardrails:
    def __init__(self, root: Path) -> None:
        self.root = root

    def run(self) -> list[str]:
        failures: list[str] = []
        failures.extend(self.runtime_api_failures())
        failures.extend(self.callback_failures())
        failures.extend(self.storybook_leak_failures())
        failures.extend(self.helper_only_view_failures())
        failures.extend(self.typed_action_model_failures())
        failures.extend(self.storybook_panel_evidence_failures())
        failures.extend(self.visual_fallback_policy_failures())
        failures.extend(self.repo_local_guardrail_policy_failures())
        failures.extend(self.agent_stop_policy_failures())
        failures.extend(self.agent_hook_policy_failures())
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

    def read_rust_dir(self, path: Path) -> str:
        return "\n".join(self.read(source) for source in self.rust_files(path))

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
        return KucOpenSpecGuardrails(self.root).evidence_failures()

    def file_length_review_failures(self) -> list[str]:
        return KucOpenSpecGuardrails(self.root).file_length_review_failures()

    def repo_local_guardrail_policy_failures(self) -> list[str]:
        required_files = (
            self.root / "docs/architecture/ui-separation/owned-ui-task-map.md",
            self.root / "openspec/changes/ui-core-interaction-visual-parity/tasks.md",
            self.root
            / "openspec/changes/ui-core-interaction-visual-parity/specs/ui-core-interaction-visual-parity/spec.md",
            self.root / "tmp/reports/2026-05-17-overnight-residual-scope.md",
        )
        missing_files = [path for path in required_files if not path.exists()]
        if missing_files:
            return [
                f"{self.relative(path)}: KUC repo-local guardrail policy file is missing"
                for path in missing_files
            ]

        combined = "\n".join(self.read(path) for path in required_files)
        required_tokens = (
            "KUC repo",
            "`scripts/`",
            "`kal` 側",
            "no `kal` repository changes are required",
            "KUC-specific UI ownership and Storybook rules MUST be implemented inside this repository",
        )
        failures = [
            f"KUC repo-local guardrail policy missing token: {token}"
            for token in required_tokens
            if token not in combined
        ]
        forbidden_tokens = (
            "../kal",
            "../katana",
            "/works/private/katana/",
            "kal.json に追記",
            "kal repository changes are required",
        )
        failures.extend(
            f"KUC guardrail policy must not require kal-side edits: {token}"
            for token in forbidden_tokens
            if token in combined
        )
        return failures

    def agent_stop_policy_failures(self) -> list[str]:
        agents = self.root / "AGENTS.md"
        if not agents.exists():
            return ["AGENTS.md: runner stop policy is missing"]

        source = self.read(agents)
        required_tokens = (
            "## runner 停止条件",
            "v0.1.0 release readiness が未達",
            "ローカル保存（commit）",
            "停止理由にしない",
            "push confirmation required",
            "release confirmation required",
            "destructive operation confirmation required",
            "次の未完了タスク",
        )
        return [
            f"agent runner stop policy missing token: {token}"
            for token in required_tokens
            if token not in source
        ]

    def agent_hook_policy_failures(self) -> list[str]:
        hook = self.root / ".githooks/pre-commit"
        installer = self.root / "scripts/install-git-hooks.sh"
        agents = self.root / "AGENTS.md"
        missing_files = [path for path in (hook, installer, agents) if not path.exists()]
        if missing_files:
            return [
                f"{self.relative(path)}: agent stop hook policy file is missing"
                for path in missing_files
            ]

        combined = "\n".join(self.read(path) for path in (hook, installer, agents))
        required_tokens = (
            "core.hooksPath .githooks",
            "just kuc-guardrails",
            "fix-and-continue",
            "push confirmation required",
            "release confirmation required",
            "destructive operation confirmation required",
            "ユーザー確認で止まらず",
        )
        return [
            f"agent stop hook policy missing token: {token}"
            for token in required_tokens
            if token not in combined
        ]

    def storybook_panel_evidence_failures(self) -> list[str]:
        docs = self.guard_docs_source()
        required_tokens = (
            "storybook-panel-interaction-report.json",
            "story_selection",
            "theme_switch",
            "operation_sequence",
            "callback log",
            "target state id",
            "before / after summary",
        )
        return [
            f"storybook panel evidence missing token: {token}"
            for token in required_tokens
            if token not in docs
        ]

    def visual_fallback_policy_failures(self) -> list[str]:
        docs = self.guard_docs_source()
        required_tokens = (
            "required_ui_fallbacks=0",
            "generic `node` fallback",
            "完了根拠にしない",
        )
        return [
            f"visual fallback policy missing token: {token}"
            for token in required_tokens
            if token not in docs
        ]

    def guard_docs_source(self) -> str:
        paths = (
            self.root / "docs/architecture/ui-separation/ui-core-parity-gap.md",
            self.root / "docs/architecture/ui-separation/owned-ui-task-map.md",
            self.root / "tmp/reports/2026-05-17-overnight-residual-scope.md",
        )
        return "\n".join(self.read(path) for path in paths if path.exists())

    def typed_action_model_failures(self) -> list[str]:
        required_files = (
            self.root / "crates/katana-ui-core/src/interaction/mod.rs",
            self.root / "crates/katana-ui-core/src/component.rs",
            self.root / "crates/katana-ui-core/tests/interaction_contract.rs",
        )
        missing_files = [path for path in required_files if not path.exists()]
        if missing_files:
            return [
                f"{self.relative(path)}: typed action model file is missing"
                for path in missing_files
            ]

        combined = "\n".join(
            (
                self.read_rust_dir(self.root / "crates/katana-ui-core/src/interaction"),
                self.read(self.root / "crates/katana-ui-core/src/component.rs"),
                self.read(self.root / "crates/katana-ui-core/tests/interaction_contract.rs"),
            )
        )
        required_tokens = (
            "pub enum UiAction",
            "pub struct UiActionResult",
            "pub struct UiCallbackLog",
            "pub trait ComponentAction",
            "apply_action",
            "action_targets_only_the_matching_component_state",
            "action_result_is_serializable_snapshot",
        )
        failures = [
            f"typed action model missing token: {token}"
            for token in required_tokens
            if token not in combined
        ]
        forbidden_tokens = (
            "ExternalStore",
            "GlobalStore",
            "external_store",
            "OnceLock",
            "static mut",
            "lazy_static",
        )
        failures.extend(
            f"typed action model must not require external store: {token}"
            for token in forbidden_tokens
            if token in combined
        )
        return failures


def main() -> int:
    failures = KucGuardrails(Path.cwd()).run()
    if failures:
        print("katana-ui-core guardrails failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
