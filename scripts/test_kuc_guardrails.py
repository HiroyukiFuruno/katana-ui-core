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

    def test_requires_kuc_guardrails_to_run_release_readiness(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "kuc-guardrails:\n"
                "    python3 scripts/test_kuc_guardrails.py\n"
                "    python3 scripts/assert-kuc-guardrails.py\n",
            )

            failures = KucGuardrails(root).release_readiness_recipe_failures()

            self.assertEqual(2, len(failures))

    def test_requires_release_readiness_runtime_check_not_only_self_test(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "kuc-guardrails:\n"
                "    python3 scripts/test_kuc_guardrails.py\n"
                "    python3 scripts/assert-kuc-release-readiness.py --self-test\n"
                "    python3 scripts/assert-kuc-guardrails.py\n",
            )

            failures = KucGuardrails(root).release_readiness_recipe_failures()

            self.assertEqual(1, len(failures))
            self.assertIn("runtime check", failures[0])

    def test_accepts_kuc_guardrails_release_readiness_recipe(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "Justfile",
                "kuc-guardrails:\n"
                "    python3 scripts/test_kuc_guardrails.py\n"
                "    python3 scripts/assert-kuc-release-readiness.py --self-test\n"
                "    python3 scripts/assert-kuc-release-readiness.py\n"
                "    python3 scripts/assert-kuc-guardrails.py\n",
            )

            failures = KucGuardrails(root).release_readiness_recipe_failures()

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

    def test_checks_storybook_reflection_audit_policy(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).storybook_reflection_audit_policy_failures()

            self.assertEqual(1, len(failures))
            write_text(
                root / "Justfile",
                "kuc-guardrails:\n"
                "    python3 scripts/test_storybook_reflection_audit.py\n"
                "storybook-reflection-audit:\n"
                "    python3 scripts/assert-storybook-reflection-audit.py --strict\n",
            )
            write_text(
                root / "docs/architecture/ui-separation/ui-core-parity-gap.md",
                "just storybook-reflection-audit missing-* page 固有 surface\n",
            )

            failures = KucGuardrails(root).storybook_reflection_audit_policy_failures()

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

    def test_requires_component_state_ownership_handle_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            failures = KucGuardrails(root).component_state_ownership_failures()

            self.assertGreaterEqual(len(failures), 8)

    def test_accepts_component_state_ownership_handle_contract(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/state.rs",
                "pub struct UiStateHandle<T>(T);\n"
                "pub struct UiComponentState;\n"
                "impl<T> UiStateHandle<T> { pub fn update(&self) {} }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/component.rs",
                "pub trait ComponentStateBinding {}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/atom/mod.rs",
                "pub fn state_snapshot() {}\npub fn sync_state() {}\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/state_store.rs",
                "struct Key { component_id: &'static str }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/window_interaction.rs",
                "selected_component_presets\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/navigation_tests.rs",
                "fn preset_tab_selection_is_owned_by_component() {}\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/interaction_contract.rs",
                "fn action_targets_only_the_matching_component_state() {}\n"
                "fn complex_ui_state_is_owned_by_the_component_model() {}\n"
                "fn app_global_state_updates_component_owned_state_via_handle() {}\n"
                "fn state_handle_supports_react_like_get_set_and_update_without_global_store() {}\n",
            )
            write_text(
                root
                / "openspec/changes/establish-kuc-atoms-molecules-catalog/core-foundation-contract.md",
                "UiStateHandle set/update global state component-owned state\n",
            )

            failures = KucGuardrails(root).component_state_ownership_failures()

            self.assertEqual([], failures)

    def test_rejects_page_owned_storybook_component_state(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(root / "crates/katana-ui-core/src/state.rs", "UiStateHandle UiComponentState\n")
            write_text(root / "crates/katana-ui-core/src/component.rs", "ComponentStateBinding\n")
            write_text(root / "crates/katana-ui-core/src/atom/mod.rs", "state_snapshot sync_state\n")
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/state_store.rs",
                "struct Key { page: &'static str, component_id: &'static str }\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/visual/window_interaction.rs",
                "selected_component_presets selected_presets\n",
            )
            write_text(
                root
                / "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/navigation_tests.rs",
                "preset_tab_selection_is_owned_by_component\n",
            )
            write_text(
                root / "crates/katana-ui-core/tests/interaction_contract.rs",
                "action_targets_only_the_matching_component_state\n"
                "complex_ui_state_is_owned_by_the_component_model\n"
                "app_global_state_updates_component_owned_state_via_handle\n"
                "state_handle_supports_react_like_get_set_and_update_without_global_store\n",
            )
            write_text(
                root
                / "openspec/changes/establish-kuc-atoms-molecules-catalog/core-foundation-contract.md",
                "set/update\n",
            )

            failures = KucGuardrails(root).component_state_ownership_failures()

            self.assertIn("storybook state key must not be page-owned", failures)
            self.assertIn("storybook preset state must be component-owned", failures)

    def test_rejects_public_app_shell_api(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/mod.rs",
                "pub use app_primitives::{AppShell, CollapsiblePanel};\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/widget/molecules.rs",
                "pub use crate::molecule::{AppShellSlot, CollapsiblePanel};\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
                '"app-shell" => &["shell"],\n',
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/kind.rs",
                "pub enum UiNodeKind { AppShell, CollapsiblePanel }\n",
            )

            failures = KucGuardrails(root).public_app_shell_failures()

            self.assertEqual(4, len(failures))

    def test_accepts_collapsible_panel_without_public_app_shell(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_text(
                root / "crates/katana-ui-core/src/molecule/mod.rs",
                "pub use structured::CollapsiblePanel;\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/widget/molecules.rs",
                "pub use crate::molecule::CollapsiblePanel;\n",
            )
            write_text(
                root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
                '"collapsible-panel" => &["Explorer panel"],\n',
            )
            write_text(
                root / "crates/katana-ui-core/src/render_model/kind.rs",
                "pub enum UiNodeKind { CollapsiblePanel }\n",
            )

            failures = KucGuardrails(root).public_app_shell_failures()

            self.assertEqual([], failures)


if __name__ == "__main__":
    unittest.main()
