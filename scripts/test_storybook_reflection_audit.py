#!/usr/bin/env python3
import tempfile
import unittest
from pathlib import Path

from storybook_reflection_audit import StorybookReflectionAudit


def write_text(path: Path, source: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


class StorybookReflectionAuditTest(unittest.TestCase):
    def test_detects_required_page_missing_dedicated_surface(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(root, dedicated='"button" => draw_button(),')

            findings = StorybookReflectionAudit(root).findings()

            self.assertEqual(1, len(findings))
            self.assertEqual("missing-dedicated-surface", findings[0].code)
            self.assertEqual("tabs", findings[0].page)

    def test_accepts_required_pages_with_surface_preset_and_spec(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_minimal_repo(
                root,
                dedicated='"button" => draw_button(),\n"tabs" => draw_tabs(),',
            )

            findings = StorybookReflectionAudit(root).findings()

            self.assertEqual([], findings)


def write_minimal_repo(root: Path, dedicated: str) -> None:
    write_text(
        root / "crates/katana-ui-core-storybook/src/requirements.rs",
        'const REQUIRED_PAGES: &[&str] = &["button", "tabs"];\n'
        "const MIN_SINGLE_NODE: usize = 1;\n",
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/visual/dedicated.rs",
        f"fn draw_page(page: &str) {{ match page {{ {dedicated} _ => fallback(), }} }}\n",
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
        'fn for_page(page: &str) { match page { "button" => &["default"], "tabs" => &["default"], _ => &[] } }\n',
    )
    write_text(
        root / "crates/katana-ui-core-storybook/src/visual/interaction_spec_atoms.rs",
        'fn for_page(page: &str) { match page { "button" => Some(spec()), "tabs" => Some(spec()), _ => None } }\n',
    )


if __name__ == "__main__":
    unittest.main()
