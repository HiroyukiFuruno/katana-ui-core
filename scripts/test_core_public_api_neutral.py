#!/usr/bin/env python3
"""Self-tests for the consumer-neutral public API guard."""

from pathlib import Path
import subprocess
import tempfile
import shutil
import re


ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/assert-core-public-api-neutral.sh"
PUBLIC_DOCS = [
    ROOT / "README.md",
    ROOT / "docs/selection-dialogs.md",
    ROOT / "docs/ui-selection-snapshots.md",
    ROOT / "docs/live-unicode-observation.md",
    ROOT / "docs/tab-strip-host-proposals.md",
    ROOT / "docs/context-menu-enabled-resolver.md",
    ROOT / "docs/ui-separation-plan.md",
    ROOT / "docs/architecture/ui-separation/owned-ui-task-map.md",
    ROOT / "docs/architecture/ui-separation/fallible-text-command-root-design.md",
    ROOT / "docs/architecture/ui-separation/root-plan-source.md",
    ROOT / "docs/architecture/ui-separation/ast-lint-remediation-plan.md",
]


def test_normal_guard_passes() -> None:
    result = subprocess.run(["bash", str(GUARD)], cwd=ROOT, text=True, capture_output=True)
    assert result.returncode == 0, result.stdout + result.stderr


def assert_consumer_neutral(path: Path) -> None:
    forbidden = ("katana-chat-ui", "katana-language-editor", "katana-document-viewer", "katana-render-runtime")
    text = path.read_text(encoding="utf-8").lower()
    assert not any(term in text for term in forbidden), path
    assert re.search(r"\b(?:kcu|kle|kdv|krr|kml|kcf|kdr)\b", text) is None, path


def test_current_public_docs_are_consumer_neutral() -> None:
    paths = list(PUBLIC_DOCS)
    for directory in (".codex/skills", ".agents/skills", ".agent/skills", ".claude/skills"):
        paths.extend((ROOT / directory).glob("*/SKILL.md"))
    for directory in (".codex/workflows", ".agent/workflows", ".claude/commands"):
        paths.extend((ROOT / directory).glob("*.md"))
    for path in paths:
        assert_consumer_neutral(path)


def make_fixture(root: Path) -> None:
    (root / "scripts").mkdir()
    shutil.copy2(GUARD, root / "scripts" / GUARD.name)
    for name in ("facade.rs", "runtime", "window", "surface", "panel", "style", "theme", "render_model", "widget", "adapter_contract", "raster_host", "egui"):
        path = root / "crates/katana-ui-core/src" / name
        if path.suffix:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("", encoding="utf-8")
        else:
            path.mkdir(parents=True, exist_ok=True)
    for path in PUBLIC_DOCS:
        destination = root / path.relative_to(ROOT)
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text("Generic KUC contract", encoding="utf-8")


def test_active_guidance_and_missing_docs_are_checked() -> None:
    with tempfile.TemporaryDirectory(prefix="kuc-guidance-") as directory:
        root = Path(directory)
        make_fixture(root)
        directories = (".codex/skills", ".agents/skills", ".agent/skills", ".claude/skills", ".codex/workflows", ".agent/workflows", ".claude/commands")
        for directory in directories:
            relative = "release/SKILL.md" if directory.endswith("skills") else "release.md"
            skill = root / directory / relative
            skill.parent.mkdir(parents=True)
            for payload in ("cd ../katana-chat-ui", "reference ../katana/crates/widgets"):
                skill.write_text(payload, encoding="utf-8")
                result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
                assert result.returncode != 0, (directory, payload, result.stdout + result.stderr)
            skill.write_text("Generic KUC release", encoding="utf-8")
            result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
            assert result.returncode == 0, (directory, result.stdout + result.stderr)
        skill.write_text("Requires KLE: host acceptance", encoding="utf-8")
        try:
            assert_consumer_neutral(skill)
        except AssertionError:
            pass
        else:
            raise AssertionError("uppercase consumer abbreviation must be rejected")
        result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
        assert result.returncode != 0, result.stdout + result.stderr
        skill.write_text("katana-ui-corex specialization", encoding="utf-8")
        result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
        assert result.returncode != 0, result.stdout + result.stderr
        skill.write_text("katana-ui-core単独のrelease", encoding="utf-8")
        result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
        assert result.returncode == 0, result.stdout + result.stderr
        document = root / "README.md"
        document.rename(root / "README.unavailable")
        result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
        assert result.returncode != 0, "missing required guidance must fail closed"


def test_external_name_is_rejected_in_production_comment() -> None:
    with tempfile.TemporaryDirectory(prefix="kuc-boundary-") as directory:
        root = Path(directory)
        make_fixture(root)
        fixture = root / "crates/katana-ui-core/src/egui/boundary.rs"
        fixture.write_text(
            "fn fixture<'a>(value: &'a str) { let _char = 'x'; let _byte = b'x';\n"
            "/* block comment starts here\n   katana-markdown-model requires this branch */\n"
            "let _ = value; }\n",
            encoding="utf-8",
        )
        result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
        assert result.returncode != 0
        assert "production API comments" in result.stdout, result.stdout + result.stderr
        fixture.write_text(
            'const text = "katana-language-editor is a literal";\n'
            'const raw = r#"/* katana-language-editor is also a literal */"#;\n'
            "fn lifetime<'a>(value: &'a str) { let _char = '\\''; let _byte = b'\\''; let _ = value; }\n"
            "/* katana-ui-core単独の契約を全てのhostへ適用する。 */\n",
            encoding="utf-8",
        )
        result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
        assert result.returncode == 0, result.stdout + result.stderr
        document = root / "docs/selection-dialogs.md"
        document.write_text("Requires katana-markdown-model internals", encoding="utf-8")
        result = subprocess.run(["bash", str(root / "scripts" / GUARD.name)], cwd=root, text=True, capture_output=True)
        assert result.returncode != 0
        assert "current public guidance" in result.stdout, result.stdout + result.stderr


if __name__ == "__main__":
    test_normal_guard_passes()
    test_current_public_docs_are_consumer_neutral()
    test_external_name_is_rejected_in_production_comment()
    test_active_guidance_and_missing_docs_are_checked()
    print("core public API neutral self-tests: 4 passed")
