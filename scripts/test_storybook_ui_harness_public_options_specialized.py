#!/usr/bin/env python3
import unittest

from test_storybook_ui_harness import StorybookUiHarness, write_text
from test_storybook_ui_harness_public_options import (
    TemporaryStorybookRepo,
    add_option_page,
)


class StorybookUiHarnessSpecializedPublicOptionsTest(unittest.TestCase):
    def test_rejects_closeable_tab_active_option_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/bar.rs",
                "impl WorkspaceTabBar { pub fn active_tab_id(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "closeable-tab-strip: public option `pub fn active_tab_id` "
                "missing Storybook Inspector option `active_tab_id`",
                failures,
            )

    def test_rejects_color_picker_public_option_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/color/picker.rs",
                "impl ColorPicker {\n"
                "pub fn panel_scale_percent(mut self) -> Self { self }\n"
                "pub fn trigger_size(mut self) -> Self { self }\n"
                "pub fn readonly(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            for public_option, inspector_option in [
                ("pub fn panel_scale_percent", "color_picker.panel_scale_percent"),
                ("pub fn trigger_size", "color_picker.trigger_size"),
                ("pub fn readonly", "color_picker.readonly"),
            ]:
                self.assertIn(
                    f"color-picker-rgba: public option `{public_option}` "
                    f"missing Storybook Inspector option `{inspector_option}`",
                    failures,
                )

    def test_accepts_color_picker_public_option_when_storybook_option_exists(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/color/picker.rs",
                "impl ColorPicker { pub fn panel_scale_percent(mut self) -> Self { self } }\n",
            )
            add_option_page(
                root,
                "color-picker-rgba",
                "COLOR_PICKER_OPTIONS",
                'StorybookUiOptionContract::new("color_picker.panel_scale_percent", "75", "100"),',
            )

            failures = StorybookUiHarness(root).failures()

            self.assertNotIn(
                "color-picker-rgba: public option `pub fn panel_scale_percent` "
                "missing Storybook Inspector option `color_picker.panel_scale_percent`",
                failures,
            )

    def test_rejects_hover_card_public_option_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/disclosure/hover_card.rs",
                "impl HoverCard { pub fn close_delay_ms(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "hover-card: public option `pub fn close_delay_ms` "
                "missing Storybook Inspector option `hover_card.close_delay_ms`",
                failures,
            )

    def test_rejects_motion_public_field_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/interaction/motion_tokens.rs",
                "pub struct MotionSpec { pub policy: ReducedMotionPolicy }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "motion: public option `pub policy:` "
                "missing Storybook Inspector option `motion.reduced_policy`",
                failures,
            )

    def test_rejects_loading_speed_option_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "impl LoadingDots { pub fn speed_ms(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "loading-dots: public option `pub fn speed_ms` "
                "missing Storybook Inspector option `loading.speed_ms`",
                failures,
            )

    def test_rejects_layout_axis_and_overflow_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/layout/containers.rs",
                "impl Row { pub fn axis(mut self) -> Self { self }\n"
                "pub fn overflow(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "row: public option `pub fn axis` "
                "missing Storybook Inspector option `axis`",
                failures,
            )
            self.assertIn(
                "row: public option `pub fn overflow` "
                "missing Storybook Inspector option `overflow`",
                failures,
            )

    def test_rejects_scroll_area_gap_and_alignment_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/layout/scroll_area.rs",
                "impl ScrollArea { pub fn gap(mut self) -> Self { self }\n"
                "pub fn align(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "scroll-area: public option `pub fn gap` "
                "missing Storybook Inspector option `gap`",
                failures,
            )
            self.assertIn(
                "scroll-area: public option `pub fn align` "
                "missing Storybook Inspector option `alignment`",
                failures,
            )

    def test_rejects_split_pane_overflow_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/layout/split_pane.rs",
                "impl SplitPane { pub fn overflow(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "split-pane: public option `pub fn overflow` "
                "missing Storybook Inspector option `overflow`",
                failures,
            )

    def test_rejects_text_wrap_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/options.rs",
                "impl Text { pub fn wrap(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "text: public option `pub fn wrap` "
                "missing Storybook Inspector option `text.wrap`",
                failures,
            )

    def test_rejects_primitive_theme_slot_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/mod.rs",
                "impl Divider { pub fn theme_slot(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "divider: public option `pub fn theme_slot` "
                "missing Storybook Inspector option `theme.slot`",
                failures,
            )

    def test_rejects_combo_box_validation_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/choice.rs",
                "impl ComboBox { pub fn invalid(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "combo-box: public option `pub fn invalid` "
                "missing Storybook Inspector option `validation`",
                failures,
            )

    def test_rejects_combo_box_choice_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/selection/choice.rs",
                "impl ComboBox {\n"
                "pub fn disabled(mut self) -> Self { self }\n"
                "pub fn open(mut self) -> Self { self }\n"
                "pub fn long_list(mut self) -> Self { self }\n"
                "pub fn select_action(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "combo-box: public option `pub fn disabled` "
                "missing Storybook Inspector option `disabled`",
                failures,
            )
            self.assertIn(
                "combo-box: public option `pub fn open` "
                "missing Storybook Inspector option `interaction.open`",
                failures,
            )
            self.assertIn(
                "combo-box: public option `pub fn long_list` "
                "missing Storybook Inspector option `combo.long_list`",
                failures,
            )
            self.assertIn(
                "combo-box: public option `pub fn select_action` "
                "missing Storybook Inspector option `combo.select_action`",
                failures,
            )

    def test_rejects_tree_view_line_display_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/structured/options.rs",
                "impl TreeView { pub fn line_display(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "tree-view: public option `pub fn line_display` "
                "missing Storybook Inspector option `line`",
                failures,
            )

    def test_rejects_basic_composite_selection_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/basic.rs",
                "impl Menu { pub fn selected_index(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "menu: public option `pub fn selected_index` "
                "missing Storybook Inspector option `interaction.selected_index`",
                failures,
            )

    def test_rejects_list_empty_selection_and_theme_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/basic_list.rs",
                "impl List {\n"
                "pub fn selected_index(mut self) -> Self { self }\n"
                "pub fn empty_state(mut self) -> Self { self }\n"
                "pub fn row_theme_slot(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "list: public option `pub fn selected_index` "
                "missing Storybook Inspector option `list.selection`",
                failures,
            )
            self.assertIn(
                "list: public option `pub fn empty_state` "
                "missing Storybook Inspector option `list.empty_state`",
                failures,
            )
            self.assertIn(
                "list: public option `pub fn row_theme_slot` "
                "missing Storybook Inspector option `list.theme_row`",
                failures,
            )

    def test_rejects_card_content_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/card.rs",
                "impl Card {\n"
                "pub fn new(label: impl Into<String>) -> Self { todo!() }\n"
                "pub fn header(mut self) -> Self { self }\n"
                "pub fn footer(mut self) -> Self { self }\n"
                "pub fn padding(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            for public_option, inspector_option in [
                ("pub fn new", "card.label"),
                ("pub fn header", "card.header"),
                ("pub fn footer", "card.footer"),
                ("pub fn padding", "card.padding"),
            ]:
                self.assertIn(
                    f"card: public option `{public_option}` "
                    f"missing Storybook Inspector option `{inspector_option}`",
                    failures,
                )

    def test_rejects_panel_active_panel_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/panel/mod.rs",
                "impl Panel { pub fn active_panel(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "panel: public option `pub fn active_panel` "
                "missing Storybook Inspector option `active_panel`",
                failures,
            )

    def test_rejects_badge_leading_icon_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "impl Badge { pub fn leading_icon(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "badge: public option `pub fn leading_icon` "
                "missing Storybook Inspector option `badge.leading_icon`",
                failures,
            )

    def test_rejects_badge_passive_policy_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/action_policy.rs",
                "impl AtomActionPolicy { fn is_passive_status_action() -> bool { true } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "badge: public option `fn is_passive_status_action` "
                "missing Storybook Inspector option `badge.passive`",
                failures,
            )

    def test_rejects_banner_density_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/disclosure/banner.rs",
                "impl Banner { pub fn density(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "banner: public option `pub fn density` "
                "missing Storybook Inspector option `density`",
                failures,
            )

    def test_rejects_banner_title_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/disclosure/banner.rs",
                "impl Banner { pub fn title(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "banner: public option `pub fn title` "
                "missing Storybook Inspector option `banner.title`",
                failures,
            )

    def test_rejects_button_label_constructor_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/mod.rs",
                "impl Button { pub fn new(label: impl Into<String>) -> Self { todo!() } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "button: public option `pub fn new` "
                "missing Storybook Inspector option `label`",
                failures,
            )

    def test_rejects_button_border_common_option_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/render_model/common.rs",
                "impl UiCommonProps { pub fn border(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "button: public option `pub fn border` "
                "missing Storybook Inspector option `border`",
                failures,
            )

    def test_rejects_selection_focus_action_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/interaction/action_builders.rs",
                "impl UiAction { pub fn focus(target: UiStateId) -> Self { todo!() } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "checkbox: public option `pub fn focus` "
                "missing Storybook Inspector option `focus`",
                failures,
            )

    def test_rejects_command_palette_provider_group_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root
                / "crates/katana-ui-core/src/molecule/structured/command_launcher_results/row.rs",
                "impl CommandResultRow { pub fn provider_id(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "command-palette: public option `pub fn provider_id` "
                "missing Storybook Inspector option `command_palette.provider_group`",
                failures,
            )

    def test_rejects_settings_control_kind_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/app_primitives/settings/control.rs",
                "impl SettingsControl { pub const fn kind(&self) -> SettingsControlKind { todo!() } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "settings-list: public option `pub const fn kind` "
                "missing Storybook Inspector option `settings_list.control_kind`",
                failures,
            )

    def test_rejects_settings_reset_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/app_primitives/settings/field.rs",
                "impl SettingsField { pub fn reset_to_default(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "settings-list: public option `pub fn reset_to_default` "
                "missing Storybook Inspector option `settings_list.reset`",
                failures,
            )

    def test_rejects_settings_list_content_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/app_primitives/settings/mod.rs",
                "impl SettingsList {\n"
                "pub fn new(label: impl Into<String>) -> Self { todo!() }\n"
                "pub fn section(mut self) -> Self { self }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/app_primitives/settings/field.rs",
                "impl SettingsSection {\n"
                "pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self { todo!() }\n"
                "pub fn description(mut self) -> Self { self }\n"
                "pub fn icon(mut self) -> Self { self }\n"
                "pub fn field(mut self) -> Self { self }\n"
                "pub fn footer(mut self) -> Self { self }\n"
                "pub const fn collapsible(mut self) -> Self { self }\n"
                "pub const fn default_collapsed(mut self) -> Self { self }\n"
                "}\n"
                "impl SettingsField {\n"
                "pub fn new(id: impl Into<String>, label: impl Into<String>, control: SettingsControl) -> Self { todo!() }\n"
                "pub fn description(mut self) -> Self { self }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/app_primitives/settings/control.rs",
                "impl SettingsControlOption {\n"
                "pub fn new(label: impl Into<String>, value: SettingsValue) -> Self { todo!() }\n"
                "}\n"
                "impl SettingsControl {\n"
                "pub fn custom(node: UiNode) -> Self { todo!() }\n"
                "pub fn set_value(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            for public_option, inspector_option in [
                ("pub fn new", "settings_list.label"),
                ("pub fn section", "settings_list.sections"),
                ("pub fn new", "settings_list.section_label"),
                ("pub fn description", "settings_list.section_description"),
                ("pub fn icon", "settings_list.section_icon"),
                ("pub fn field", "settings_list.field_count"),
                ("pub fn footer", "settings_list.section_footer"),
                ("pub const fn collapsible", "settings_list.section_collapsible"),
                ("pub const fn default_collapsed", "settings_list.default_collapsed"),
                ("pub fn new", "settings_list.field_label"),
                ("pub fn description", "settings_list.field_description"),
                ("pub fn new", "settings_list.control_options"),
                ("pub fn custom", "settings_list.custom_control"),
                ("pub fn set_value", "settings_list.set_value"),
            ]:
                self.assertIn(
                    f"settings-list: public option `{public_option}` "
                    f"missing Storybook Inspector option `{inspector_option}`",
                    failures,
                )

    def test_rejects_shortcut_cheatsheet_selected_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/shortcut_cheatsheet.rs",
                "enum ShortcutCheatsheetAction { SelectShortcut(String) }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "shortcut-cheatsheet: public option `SelectShortcut` "
                "missing Storybook Inspector option `shortcut_cheatsheet.selected`",
                failures,
            )

    def test_rejects_shortcut_cheatsheet_result_count_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/shortcut_cheatsheet.rs",
                "impl ShortcutCheatsheet { pub fn visible_items(&self) -> Vec<()> { Vec::new() } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "shortcut-cheatsheet: public option `pub fn visible_items` "
                "missing Storybook Inspector option `shortcut_cheatsheet.result_count`",
                failures,
            )

    def test_rejects_shortcut_cheatsheet_content_options_without_storybook_options(
        self,
    ) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/shortcut_cheatsheet.rs",
                "impl ShortcutCheatsheet {\n"
                "pub fn new(label: impl Into<String>) -> Self { todo!() }\n"
                "pub fn group(mut self) -> Self { self }\n"
                "}\n"
                "impl ShortcutCheatsheetGroup {\n"
                "pub fn new(title: impl Into<String>) -> Self { todo!() }\n"
                "pub fn item(mut self) -> Self { self }\n"
                "}\n"
                "impl ShortcutCheatsheetItem {\n"
                "pub fn new(id: impl Into<String>, label: impl Into<String>, combo: KeyCombo) -> Self { todo!() }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "shortcut-cheatsheet: public option `pub fn new(label: impl Into<String>)` "
                "missing Storybook Inspector option `shortcut_cheatsheet.label`",
                failures,
            )
            self.assertIn(
                "shortcut-cheatsheet: public option `pub fn group` "
                "missing Storybook Inspector option `shortcut_cheatsheet.groups`",
                failures,
            )
            self.assertIn(
                "shortcut-cheatsheet: public option `pub fn new(title: impl Into<String>)` "
                "missing Storybook Inspector option `shortcut_cheatsheet.group_title`",
                failures,
            )
            self.assertIn(
                "shortcut-cheatsheet: public option `pub fn item` "
                "missing Storybook Inspector option `shortcut_cheatsheet.items`",
                failures,
            )
            self.assertIn(
                "shortcut-cheatsheet: public option `pub fn new(id: impl Into<String>, label: impl Into<String>, combo: KeyCombo)` "
                "missing Storybook Inspector option `shortcut_cheatsheet.item_combo`",
                failures,
            )

    def test_rejects_shortcut_combo_accessibility_label_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/shortcut_combo.rs",
                "impl ShortcutCombo { pub fn accessibility_label(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "shortcut-combo: public option `pub fn accessibility_label` "
                "missing Storybook Inspector option `shortcut_combo.a11y_label`",
                failures,
            )

    def test_rejects_status_bar_message_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/status_bar.rs",
                "impl StatusBar { pub fn message(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "status-bar: public option `pub fn message` "
                "missing Storybook Inspector option `status_bar.message`",
                failures,
            )

    def test_rejects_empty_state_content_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/empty_state/mod.rs",
                "impl EmptyState {\n"
                "pub fn new(heading: impl Into<String>) -> Self { todo!() }\n"
                "pub fn body(mut self) -> Self { self }\n"
                "pub fn icon(mut self) -> Self { self }\n"
                "pub fn illustration(mut self) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "empty-state: public option `pub fn new` "
                "missing Storybook Inspector option `empty_state.heading`",
                failures,
            )
            self.assertIn(
                "empty-state: public option `pub fn body` "
                "missing Storybook Inspector option `empty_state.body`",
                failures,
            )
            self.assertIn(
                "empty-state: public option `pub fn icon` "
                "missing Storybook Inspector option `empty_state.icon`",
                failures,
            )
            self.assertIn(
                "empty-state: public option `pub fn illustration` "
                "missing Storybook Inspector option `empty_state.illustration`",
                failures,
            )

    def test_rejects_toolbar_nested_public_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/toolbar/action_model.rs",
                "impl ToolbarAction {\n"
                "pub fn tooltip(mut self) -> Self { self }\n"
                "pub fn accessibility_label(mut self) -> Self { self }\n"
                "}\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/toolbar/group_model.rs",
                "impl ToolbarGroup { pub fn label(mut self) -> Self { self } }\n",
            )
            write_text(
                root / "crates/katana-ui-core/src/molecule/toolbar/split_model.rs",
                "impl SplitActionPart { pub fn disabled(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "toolbar: public option `pub fn tooltip` "
                "missing Storybook Inspector option `toolbar.action_tooltip`",
                failures,
            )
            self.assertIn(
                "toolbar: public option `pub fn accessibility_label` "
                "missing Storybook Inspector option `toolbar.action_a11y`",
                failures,
            )
            self.assertIn(
                "toolbar: public option `pub fn label` "
                "missing Storybook Inspector option `toolbar.group_label`",
                failures,
            )
            self.assertIn(
                "toolbar: public option `pub fn disabled` "
                "missing Storybook Inspector option `toolbar.split_disabled`",
                failures,
            )

    def test_rejects_diagnostics_bulk_action_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/structured/diagnostics_list/actions.rs",
                "enum DiagnosticsListAction { OpenBulkPreview }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "diagnostics-list: public option `OpenBulkPreview` "
                "missing Storybook Inspector option `diagnostics.bulk_action`",
                failures,
            )

    def test_rejects_diagnostics_fix_preview_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/structured/diagnostics_list/types.rs",
                "impl DiagnosticItem { pub fn fix_preview(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "diagnostics-list: public option `pub fn fix_preview` "
                "missing Storybook Inspector option `diagnostics.fix_preview`",
                failures,
            )

    def test_rejects_modal_overlay_placement_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/disclosure/modal_overlay.rs",
                "impl ModalOverlay { pub fn placement(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "modal-overlay: public option `pub fn placement` "
                "missing Storybook Inspector option `placement`",
                failures,
            )

    def test_rejects_workspace_tab_actions_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/actions.rs",
                "enum WorkspaceTabBarAction { AddTab, CloseTab, MoveTab, OpenOverflow }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "tabs: public option `AddTab` "
                "missing Storybook Inspector option `tabs.add`",
                failures,
            )
            self.assertIn(
                "tabs: public option `CloseTab` "
                "missing Storybook Inspector option `tabs.close`",
                failures,
            )
            self.assertIn(
                "tabs: public option `MoveTab` "
                "missing Storybook Inspector option `tabs.move`",
                failures,
            )
            self.assertIn(
                "tabs: public option `OpenOverflow` "
                "missing Storybook Inspector option `tabs.overflow`",
                failures,
            )

    def test_rejects_workspace_tab_active_scroll_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/scroll.rs",
                "impl WorkspaceTabScrollPlanner { pub fn follow_active() {} }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "tabs: public option `pub fn follow_active` "
                "missing Storybook Inspector option `tabs.active_scroll`",
                failures,
            )

    def test_rejects_skeleton_public_option_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/skeleton/types.rs",
                "pub enum SkeletonShape { Line { thickness: f32 } }\n"
                "impl Skeleton { pub fn aspect_ratio(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "skeleton: public option `pub fn aspect_ratio` "
                "missing Storybook Inspector option `skeleton.aspect_ratio`",
                failures,
            )

    def test_rejects_skeleton_cluster_reduced_motion_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/skeleton_cluster.rs",
                "impl SkeletonCluster { pub fn reduced_motion(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "skeleton-cluster: public option `pub fn reduced_motion` "
                "missing Storybook Inspector option `skeleton_cluster.reduced_motion`",
                failures,
            )

    def test_rejects_attachment_kind_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/attachment_chip/types.rs",
                "pub enum AttachmentKind { File, Image }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "attachment-chip: public option `pub enum AttachmentKind` "
                "missing Storybook Inspector option `attachment.kind`",
                failures,
            )

    def test_rejects_attachment_content_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/attachment_chip/model.rs",
                "impl AttachmentChip {\n"
                "pub fn new(kind: AttachmentKind, name: impl Into<String>) -> Self { todo!() }\n"
                "pub fn meta(mut self, value: AttachmentMeta) -> Self { self }\n"
                "pub fn thumbnail(mut self, value: AttachmentThumbnail) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            for public_option, inspector_option in [
                ("pub fn new(kind: AttachmentKind, name:", "attachment.name"),
                ("pub fn meta", "attachment.meta"),
                ("pub fn thumbnail", "attachment.thumbnail"),
            ]:
                self.assertIn(
                    f"attachment-chip: public option `{public_option}` "
                    f"missing Storybook Inspector option `{inspector_option}`",
                    failures,
                )

    def test_rejects_chip_group_layout_options_without_storybook_options(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/chip_group/model.rs",
                "impl ChipGroup {\n"
                "pub fn new(label: impl Into<String>) -> Self { todo!() }\n"
                "pub fn chip(mut self) -> Self { self }\n"
                "pub const fn gap(mut self, value: u16) -> Self { self }\n"
                "pub const fn available_width(mut self, value: u16) -> Self { self }\n"
                "pub const fn overflow_trigger_width(mut self, value: u16) -> Self { self }\n"
                "}\n",
            )

            failures = StorybookUiHarness(root).failures()

            for public_option, inspector_option in [
                ("pub fn new", "chip_group.label"),
                ("pub fn chip", "chip_group.chip_count"),
                ("pub const fn gap", "chip_group.gap"),
                ("pub const fn available_width", "chip_group.available_width"),
                (
                    "pub const fn overflow_trigger_width",
                    "chip_group.overflow_trigger_width",
                ),
            ]:
                self.assertIn(
                    f"chip-group: public option `{public_option}` "
                    f"missing Storybook Inspector option `{inspector_option}`",
                    failures,
                )

    def test_rejects_notification_toast_action_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/disclosure/model.rs",
                "impl NotificationToast { pub fn child(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "notification-toast: public option `pub fn child` "
                "missing Storybook Inspector option `action`",
                failures,
            )

    def test_rejects_drag_drop_indicator_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/interaction/drag_and_drop/drop_target.rs",
                "impl DropAcceptance { pub fn indicator_kind(&self) -> Option<()> { None } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "drag-and-drop: public option `pub fn indicator_kind` "
                "missing Storybook Inspector option `drag.drop_indicator`",
                failures,
            )

    def test_rejects_code_diff_language_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/molecule/diff/model.rs",
                "impl CodeDiff { pub fn language(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "code-diff: public option `pub fn language` "
                "missing Storybook Inspector option `code_diff.language`",
                failures,
            )

    def test_rejects_text_area_leading_icon_slot_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/text_area/builders.rs",
                "impl TextArea { pub fn leading_icon_slot(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "text-area: public option `pub fn leading_icon_slot` "
                "missing Storybook Inspector option `text_area.leading_slot.icon`",
                failures,
            )

    def test_rejects_text_input_background_token_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/atom/typed.rs",
                "impl Input { pub fn input_background_token(mut self) -> Self { self } }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "text-input: public option `pub fn input_background_token` "
                "missing Storybook Inspector option `theme.input_bg`",
                failures,
            )

    def test_rejects_theme_token_public_field_without_storybook_option(self) -> None:
        with TemporaryStorybookRepo() as root:
            write_text(
                root / "crates/katana-ui-core/src/theme/mod.rs",
                "pub struct ThemeSnapshot { pub id: ThemeId }\n",
            )

            failures = StorybookUiHarness(root).failures()

            self.assertIn(
                "theme-tokens: public option `pub id:` "
                "missing Storybook Inspector option `theme.id`",
                failures,
            )


if __name__ == "__main__":
    unittest.main()
