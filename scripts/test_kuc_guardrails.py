#!/usr/bin/env python3
import tempfile
import unittest
from pathlib import Path

from kuc_guardrails import KucGuardrails


def write_text(path: Path, source: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


def write_repo_policy(root: Path, spec_extra: str = "") -> None:
    baseline = "KUC repo `scripts/` `kal` 側\n"
    spec = (
        "KUC-specific UI ownership and Storybook rules MUST be implemented inside this repository\n"
        "no `kal` repository changes are required\n"
        f"{spec_extra}"
    )
    paths = (
        "docs/architecture/ui-separation/owned-ui-task-map.md",
        "openspec/changes/ui-core-interaction-visual-parity/tasks.md",
        "tmp/reports/2026-05-17-overnight-residual-scope.md",
    )
    for path in paths:
        write_text(root / path, baseline)
    write_text(
        root
        / "openspec/changes/ui-core-interaction-visual-parity/specs/ui-core-interaction-visual-parity/spec.md",
        spec,
    )


class KucGuardrailsTest(unittest.TestCase):
    def test_detects_storybook_box_leak(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            page = root / "storybook/src/pages/sample.rs"
            page.parent.mkdir(parents=True)
            page.write_text("fn page() { let _ = Box::leak(Box::new(\"x\")); }\n", encoding="utf-8")

            failures = KucGuardrails(root).storybook_leak_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("Box::leak", failures[0])

    def test_detects_missing_openspec_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            task = root / "openspec/changes/ui-core-root-plan/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text("- [x] 1.1 `storybook/src/pages/sample.rs` を追加\n", encoding="utf-8")

            failures = KucGuardrails(root).openspec_evidence_failures()

            self.assertEqual(2, len(failures))

    def test_detects_runtime_api_gated_by_test_cfg(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            ops = root / "crates/katana-ui-core/src/layout/split/ops.rs"
            ops.parent.mkdir(parents=True)
            ops.write_text(
                "#[cfg(test)]\npub(super) fn drag_ratio() -> f32 { 1.0 }\n",
                encoding="utf-8",
            )

            failures = KucGuardrails(root).runtime_api_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("drag_ratio", failures[0])

    def test_detects_missing_interactive_callback(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            types = root / "crates/katana-ui-core/src/composite/selector/toggle/types.rs"
            types.parent.mkdir(parents=True)
            types.write_text("pub struct ToggleProps { pub value: bool }\n", encoding="utf-8")

            failures = KucGuardrails(root).callback_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("on_change", failures[0])

    def test_detects_file_length_without_review_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source = root / "crates/katana-ui-core/src/layout/card/types.rs"
            source.parent.mkdir(parents=True)
            source.write_text("pub struct X;\n" * 260, encoding="utf-8")
            task = root / "openspec/changes/ui-core-root-plan/tasks.md"
            task.parent.mkdir(parents=True)
            task.write_text(
                "- [x] 1.1 file-length 対応で `crates/katana-ui-core/src/layout/card/types.rs` を追加\n",
                encoding="utf-8",
            )

            failures = KucGuardrails(root).file_length_review_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("ops.rs", failures[0])

    def test_requires_repo_local_guardrail_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).repo_local_guardrail_policy_failures()

            self.assertEqual(4, len(failures))

    def test_accepts_repo_local_guardrail_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_repo_policy(root)

            failures = KucGuardrails(root).repo_local_guardrail_policy_failures()

            self.assertEqual([], failures)

    def test_rejects_kal_side_guardrail_dependency(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_repo_policy(root, "../kal\n")

            failures = KucGuardrails(root).repo_local_guardrail_policy_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("kal-side edits", failures[0])

    def test_requires_agent_stop_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).agent_stop_policy_failures()

            self.assertEqual(1, len(failures))

    def test_accepts_agent_stop_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "AGENTS.md",
                "## runner 停止条件\n"
                "v0.1.0 release readiness が未達\n"
                "ローカル保存（commit）\n"
                "停止理由にしない\n"
                "push confirmation required\n"
                "release confirmation required\n"
                "destructive operation confirmation required\n"
                "次の未完了タスク\n",
            )

            failures = KucGuardrails(root).agent_stop_policy_failures()

            self.assertEqual([], failures)

    def test_requires_agent_stop_hook_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).agent_hook_policy_failures()

            self.assertEqual(4, len(failures))

    def test_accepts_agent_stop_hook_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / ".githooks/pre-commit",
                "just kuc-guardrails\n"
                "fix-and-continue\n"
                "push confirmation required\n"
                "release confirmation required\n"
                "destructive operation confirmation required\n"
                "ユーザー確認で止まらず\n",
            )
            write_text(
                root / ".githooks/pre-push",
                "KUC_PUSH_CONFIRMED\n"
                "push confirmation required\n"
                "release confirmation required\n",
            )
            write_text(
                root / "scripts/install-git-hooks.sh",
                "git config core.hooksPath .githooks\n",
            )
            write_text(root / "AGENTS.md", "repository hook\n")

            failures = KucGuardrails(root).agent_hook_policy_failures()

            self.assertEqual([], failures)

    def test_rejects_commit_confirmation_as_stop_reason(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / ".githooks/pre-commit",
                "just kuc-guardrails\n"
                "fix-and-continue\n"
                "push confirmation required\n"
                "release confirmation required\n"
                "destructive operation confirmation required\n"
                "ユーザー確認で止まらず\n",
            )
            write_text(
                root / ".githooks/pre-push",
                "KUC_PUSH_CONFIRMED\n"
                "push confirmation required\n"
                "release confirmation required\n",
            )
            write_text(
                root / "scripts/install-git-hooks.sh",
                "git config core.hooksPath .githooks\n",
            )
            write_text(root / "AGENTS.md", "commit confirmation required\n")

            failures = KucGuardrails(root).agent_hook_policy_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("local commit must not be a stop reason", failures[0])

    def test_checks_storybook_panel_evidence_markers(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).storybook_panel_evidence_failures()

            self.assertEqual(7, len(failures))
            docs = root / "docs/architecture/ui-separation/ui-core-parity-gap.md"
            write_text(
                docs,
                "storybook-panel-interaction-report.json story_selection theme_switch "
                "operation_sequence callback log target state id before / after summary\n",
            )

            failures = KucGuardrails(root).storybook_panel_evidence_failures()

            self.assertEqual([], failures)

    def test_checks_visual_fallback_policy_markers(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).visual_fallback_policy_failures()

            self.assertEqual(3, len(failures))
            docs = root / "docs/architecture/ui-separation/ui-core-parity-gap.md"
            write_text(
                docs,
                "required_ui_fallbacks=0 generic `node` fallback は完了根拠にしない\n",
            )

            failures = KucGuardrails(root).visual_fallback_policy_failures()

            self.assertEqual([], failures)

    def test_requires_typed_action_model(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).typed_action_model_failures()

            self.assertEqual(3, len(failures))

    def test_accepts_typed_action_model_without_external_store(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            interaction = root / "crates/katana-ui-core/src/interaction/mod.rs"
            component = root / "crates/katana-ui-core/src/component.rs"
            contract = root / "crates/katana-ui-core/tests/interaction_contract.rs"
            write_text(
                interaction,
                "pub enum UiAction {}\npub struct UiActionResult {}\npub struct UiCallbackLog {}\n",
            )
            write_text(
                component,
                "pub trait ComponentAction { fn apply_action(&mut self); }\n",
            )
            write_text(
                contract,
                "fn action_targets_only_the_matching_component_state() {}\n"
                "fn action_result_is_serializable_snapshot() {}\n",
            )

            failures = KucGuardrails(root).typed_action_model_failures()

            self.assertEqual([], failures)


if __name__ == "__main__":
    unittest.main()
