#!/usr/bin/env python3
import unittest

from test_storybook_ui_harness import StorybookUiHarness, write_text
from test_storybook_ui_harness_public_options import (
    TemporaryStorybookRepo,
    add_option_page,
)


class StorybookUiHarnessFeedbackPublicOptionsTest(unittest.TestCase):
    def test_rejects_chip_state_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/chip/model.rs",
                "impl Chip {\n"
                "pub fn leading_icon(mut self) -> Self { self }\n"
                "pub fn selected(mut self) -> Self { self }\n"
                "pub fn accessibility_label(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            for public_option, inspector_option in [
                ("pub fn leading_icon", "chip.leading_icon"),
                ("pub fn selected", "chip.selected"),
                ("pub fn accessibility_label", "chip.a11y_label"),
            ]:
                self.assertIn(
                    f"chip: public option `{public_option}` "
                    f"missing Storybook Inspector option `{inspector_option}`",
                    failures,
                )

    def test_accepts_chip_public_option_when_storybook_option_exists(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/chip/model.rs",
                "impl Chip { pub fn leading_icon(mut self) -> Self { self } }\n",
            )
            add_option_page(
                root,
                "chip",
                "CHIP_OPTIONS",
                'StorybookUiOptionContract::new("chip.leading_icon", "filter", "tag"),',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "chip: public option `pub fn leading_icon` "
                "missing Storybook Inspector option `chip.leading_icon`",
                failures,
            )

    def test_rejects_split_pane_public_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/layout/split_pane.rs",
                "impl SplitPane {\n"
                "pub fn ratio_percent(mut self) -> Self { self }\n"
                "pub fn handle_width_px(mut self) -> Self { self }\n"
                "pub fn resize_mode(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            for public_option, inspector_option in [
                ("pub fn ratio_percent", "split_pane.ratio_percent"),
                ("pub fn handle_width_px", "split_pane.handle_width_px"),
                ("pub fn resize_mode", "split_pane.resize_mode"),
            ]:
                self.assertIn(
                    f"split-pane: public option `{public_option}` "
                    f"missing Storybook Inspector option `{inspector_option}`",
                    failures,
                )

    def test_accepts_split_pane_public_option_when_storybook_option_exists(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/layout/split_pane.rs",
                "impl SplitPane { pub fn handle_width_px(mut self) -> Self { self } }\n",
            )
            add_option_page(
                root,
                "split-pane",
                "SPLIT_PANE_OPTIONS",
                'StorybookUiOptionContract::new("split_pane.handle_width_px", "8", "10"),',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "split-pane: public option `pub fn handle_width_px` "
                "missing Storybook Inspector option `split_pane.handle_width_px`",
                failures,
            )


if __name__ == "__main__":
    unittest.main()
