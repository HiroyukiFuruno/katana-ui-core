#!/usr/bin/env python3
from pathlib import Path


class WorkspaceTabGuardrails:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.base = (
            root
            / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar"
        )

    def failures(self) -> list[str]:
        core_targets = (
            self.base / "ordering.rs",
            self.base / "bulk_close.rs",
            self.base / "actions.rs",
            self.base / "tests/state_action_contract.rs",
            self.base / "tests/api_contract.rs",
        )
        storybook_context = (
            self.root
            / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_context_close.rs"
        )
        storybook_context_id_targets = (
            self.root
            / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_context_menu_types.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs_context_menu.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_context_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_context_group_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_closeable_tab_strip_context_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_closeable_tab_strip_context_no_group_tests.rs",
        )
        storybook_tabs = (
            self.root / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs.rs"
        )
        group_move_targets = (
            self.base / "actions.rs",
            self.base / "bar.rs",
            self.base / "apply_action.rs",
            self.base / "context_commands.rs",
            self.base / "events.rs",
            self.base / "group_mutations.rs",
            self.base / "tests/state_action_contract.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_group_context.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_group_move_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_context_group_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_closeable_tab_strip_group_context_tests.rs",
        )
        storybook_drag_targets = (
            self.root
            / "crates/katana-ui-core-storybook/src/visual/window_interaction/tabs_drag.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/window_interaction.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_drag.rs",
        )
        storybook_keyboard_targets = (
            self.base / "keyboard.rs",
            self.base / "tests/keyboard_contract.rs",
            self.root / "crates/katana-ui-core-storybook/src/visual/window_keyboard.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/window_interaction/tabs_keyboard.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_keyboard.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_bridge.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_keyboard_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_closeable_tab_strip_keyboard_tests.rs",
        )
        storybook_order_targets = (
            self.base / "scroll.rs",
            self.base / "tests/overflow_contract.rs",
            self.root / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/dedicated_closeable_tab_strip.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/dedicated_tabs_scroll.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_order_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_scroll_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_closeable_tab_strip_scroll_tests.rs",
        )
        storybook_option_targets = (
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_options_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tabs_state_tests.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_bridge.rs",
        )

        failures: list[str] = []
        if any(path.exists() for path in core_targets):
            failures.extend(self.ordering_failures(core_targets[0]))
            failures.extend(self.bulk_close_failures(core_targets[1]))
            failures.extend(self.drop_rule_failures(core_targets[2]))
            failures.extend(self.state_action_test_failures(core_targets[3]))
            failures.extend(self.api_contract_test_failures(core_targets[4]))
        failures.extend(self.storybook_context_order_failures(storybook_context))
        context_bridge_trigger_paths = (
            storybook_context_id_targets[0],
            storybook_context_id_targets[1],
            storybook_context_id_targets[2],
            storybook_context_id_targets[4],
            storybook_context_id_targets[5],
        )
        if any(path.exists() for path in context_bridge_trigger_paths):
            failures.extend(
                self.storybook_context_item_id_bridge_failures(storybook_context_id_targets)
            )
        failures.extend(self.storybook_move_order_failures(storybook_tabs))
        if any(path.exists() for path in group_move_targets[:-2]):
            failures.extend(self.group_move_failures(group_move_targets))
        if any(path.exists() for path in storybook_drag_targets):
            failures.extend(self.storybook_drag_order_failures(storybook_drag_targets))
        keyboard_trigger_paths = (
            storybook_keyboard_targets[0],
            storybook_keyboard_targets[1],
            storybook_keyboard_targets[2],
            storybook_keyboard_targets[3],
            storybook_keyboard_targets[4],
            storybook_keyboard_targets[6],
            storybook_keyboard_targets[7],
        )
        if any(path.exists() for path in keyboard_trigger_paths):
            failures.extend(self.storybook_keyboard_failures(storybook_keyboard_targets))
        if any(path.exists() for path in storybook_order_targets):
            failures.extend(self.storybook_order_contract_failures(storybook_order_targets))
        if any(path.exists() for path in storybook_option_targets[:2]):
            failures.extend(self.storybook_option_regression_failures(storybook_option_targets))
        return failures

    def ordering_failures(self, path: Path) -> list[str]:
        if not path.exists():
            return [f"{self.relative(path)}: workspace tab visual order guard source is missing"]
        source = self.read(path)
        order = (
            ("pinned tabs", "push_pinned_tabs"),
            ("declared group tabs", "push_group_tabs"),
            ("unknown group tabs", "push_unknown_group_tabs"),
            ("ungrouped tabs", "filter(|tab| !tab.pinned && tab.group_id.is_none())"),
        )
        required = (
            "groups: &[WorkspaceTabGroup]",
            "for group in groups",
            "tab.group_id.as_ref() == Some(&group.id)",
        )
        positions = [(name, source.find(token)) for name, token in order]
        missing = [name for name, position in positions if position < 0]
        missing.extend(token for token in required if token not in source)
        if missing:
            return [
                f"{self.relative(path)}: workspace tab visual order missing {', '.join(missing)}"
            ]
        if not positions[0][1] < positions[1][1] < positions[2][1] < positions[3][1]:
            return [
                f"{self.relative(path)}: workspace tab visual order must be pinned tabs -> declared groups -> unknown groups -> ungrouped tabs"
            ]
        return []

    def bulk_close_failures(self, path: Path) -> list[str]:
        required = (
            "use super::ordering::ordered_tabs;",
            "fn visual_tab_ids(&self) -> Vec<WorkspaceTabId>",
            "ordered_tabs(&self.options.tabs, &self.options.groups)",
            "close_tabs_to_right",
            "close_tabs_to_left",
        )
        return self.missing_token_failures(
            path,
            required,
            "bulk close must use the shared visual tab order",
        )

    def drop_rule_failures(self, path: Path) -> list[str]:
        required = (
            "grouped_without_dragged",
            "grouped_start",
            "ungrouped_start",
            "if dragged.pinned",
            "if dragged.group_id.is_some()",
        )
        return self.missing_token_failures(
            path,
            required,
            "drop rules must keep grouped, pinned, and ungrouped regions distinct",
        )

    def state_action_test_failures(self, path: Path) -> list[str]:
        required = (
            "pinned_tabs_are_before_grouped_tabs_and_bulk_close_uses_that_visual_order",
            "visual_tabs_keep_pinned_before_declared_group_order",
            "close_to_right_after_pin_uses_pinned_before_group_visual_order",
            "pinning_grouped_tab_removes_group_membership_and_moves_to_fixed_region",
            "move_to_group_rejects_pinned_and_ungroupable_tabs",
            "closed_tab_history_restores_last_closed_tab_through_typed_action",
            "drop_rules_keep_grouped_prefix_and_pinned_region_distinct",
            "WorkspaceTabBarAction::CloseToLeft",
            "WorkspaceTabBarAction::RestoreClosedTab",
            "WorkspaceTabBarEvent::TabGroupChanged",
            "WorkspaceTabBarEvent::TabRestored",
            "WorkspaceTabDropRules::can_accept",
        )
        return self.missing_token_failures(
            path,
            required,
            "state/action tests must cover visual order and action order together",
        )

    def api_contract_test_failures(self, path: Path) -> list[str]:
        required = (
            "all_tab_context_command_ids_round_trip_to_public_actions",
            "WorkspaceTabContextCommand::from_id(command.id())",
            "WorkspaceTabContextCommand::CloseToLeft",
            "WorkspaceTabContextCommand::RestoreClosed",
            "WorkspaceTabContextCommand::Unpin",
            "WorkspaceTabContextCommand::MoveToNewGroup",
            "WorkspaceTabContextCommand::MoveToGroup",
        )
        return self.missing_token_failures(
            path,
            required,
            "workspace tab API tests must cover context command id round-trips",
        )

    def storybook_context_order_failures(self, path: Path) -> list[str]:
        if not path.exists():
            return []
        source = self.read(path)
        if "self.core_visual_tab_ids()" not in source:
            return [
                f"{self.relative(path)}: Storybook context close order must use core visual tab order"
            ]
        return []

    def storybook_context_item_id_bridge_failures(self, paths: tuple[Path, ...]) -> list[str]:
        types, menu, tabs_tests, tabs_group_tests, closeable_tests, closeable_no_group_tests = paths
        failures: list[str] = []
        failures.extend(
            self.missing_token_failures(
                types,
                (
                    "CloseableTabContextCommand::from_id",
                    "CloseableTabGroupContextCommand::from_id",
                    "from_item_id",
                    "move_to_group_id_from_item_id",
                    "MoveToExistingGroup",
                    "to_context_menu_items",
                    "group_submenu_item",
                    "Self::NewGroup.id()",
                ),
                "Storybook tabs context menu must parse rendered item ids through core commands",
            )
        )
        failures.extend(
            self.missing_token_failures(
                menu,
                (
                    "items.get(index)",
                    "TabsContextMenuCommand::from_item_id",
                    "visible_items(menu)",
                    "push_visible_item",
                ),
                "Storybook tabs context menu click must use rendered item id instead of parallel command index",
            )
        )
        failures.extend(
            self.missing_token_failures(
                tabs_tests,
                (
                    "tabs_context_menu_click_uses_rendered_item_id_not_parallel_index",
                    "items[0]",
                    "id = \"pin\"",
                    "tabs_context_menu_restores_last_closed_tab_through_core_action",
                    "RESTORE_CLOSED_INDEX",
                    "closeable_tab_restored",
                    "tabs_context_menu_moves_to_selected_existing_group_not_fixed_default",
                    "move-to-group:docs",
                    "\"Review\"",
                    "\"グループに追加\"",
                    "tabs_context_menu_without_existing_groups_uses_direct_new_group_action",
                    "!pinned_labels.contains(&\"新しいグループを作成\")",
                    "tabs_context_menu_hides_group_commands_for_ungroupable_tab",
                    "scratch.groupable = false",
                ),
                "Storybook tabs tests must reject menu item id and command index drift",
            )
        )
        failures.extend(
            self.missing_token_failures(
                tabs_group_tests,
                (
                    "tabs_group_header_context_menu_toggles_collapse_through_core_action",
                    "items[1]",
                    "id = \"rename\"",
                    "group_context_rename",
                ),
                "Storybook group context tests must reject menu item id and command index drift",
            )
        )
        failures.extend(
            self.missing_token_failures(
                closeable_tests,
                (
                    "closeable_tab_strip_context_menu_click_uses_rendered_item_id",
                    "items[0]",
                    "id = \"pin\"",
                    "closeable_tab_pin_changed",
                    "closeable_tab_strip_context_menu_restores_last_closed_tab",
                    "closeable_tab_restored",
                    "closeable_tab_strip_context_menu_moves_to_selected_existing_group",
                    "\"Review\"",
                    "\"グループに追加\"",
                    "!pinned_labels.contains(&\"新しいグループを作成\")",
                    "closeable_tab_strip_context_menu_hides_group_commands_for_ungroupable_tab",
                    "scratch.groupable = false",
                ),
                    "Storybook closeable-tab-strip tests must reject menu item id and command index drift",
            )
        )
        failures.extend(
            self.missing_token_failures(
                closeable_no_group_tests,
                (
                    "closeable_tab_strip_context_menu_without_existing_groups_uses_direct_new_group_action",
                    "\"新しいグループを作成\"",
                    "!labels.contains(&\"グループに追加\")",
                    "tab_context_new_group",
                    "closeable_tab_group_changed",
                    "context-group",
                ),
                "Storybook closeable-tab-strip tests must cover direct new group without existing groups",
            )
        )
        return failures

    def storybook_move_order_failures(self, path: Path) -> list[str]:
        if not path.exists():
            return []
        required = (
            "fn move_active_right",
            "let visual_ids = self.core_visual_tab_ids();",
            "to_visual_index: from + 1",
        )
        return self.missing_token_failures(
            path,
            required,
            "Storybook tab move must use the shared visual tab order",
        )

    def storybook_drag_order_failures(self, paths: tuple[Path, Path, Path]) -> list[str]:
        drag_path, interaction_path, state_path = paths
        failures: list[str] = []
        failures.extend(
            self.missing_token_failures(
                drag_path,
                (
                    "dedicated_tabs::tab_hit_at",
                    "drop_visual_index",
                    "register_tabs_drag_start",
                    "register_tabs_drag_move",
                    "register_tabs_drag_end",
                ),
                "Storybook tab drag must use hit-tested visual order and core drag actions",
            )
        )
        failures.extend(
            self.missing_token_failures(
                interaction_path,
                (
                    "tabs_drag::start_at",
                    "tabs_drag::apply_drag_at",
                    "tabs_drag::release",
                ),
                "Storybook window interaction must route tab drag lifecycle",
            )
        )
        failures.extend(
            self.missing_token_failures(
                state_path,
                (
                    "CloseableTabStripAction::StartDrag",
                    "CloseableTabStripAction::MoveTab",
                    "apply_core_tab_drag_end",
                ),
                "Storybook tab drag state must bridge to core tab actions",
            )
        )
        return failures

    def storybook_keyboard_failures(self, paths: tuple[Path, ...]) -> list[str]:
        core, core_tests, window_keyboard, route, state, bridge, tests, closeable_tests = paths
        failures: list[str] = []
        failures.extend(
            self.missing_token_failures(
                core,
                (
                    "WorkspaceTabKeyboardInput::NextTab",
                    "WorkspaceTabKeyboardInput::PreviousTab",
                    "WorkspaceTabKeyboardInput::SelectLastVisible",
                ),
                "core workspace tab keyboard contract must expose relative and last-visible inputs",
            )
        )
        failures.extend(
            self.missing_token_failures(
                core_tests,
                (
                    "keyboard_ctrl_tab_cycles_visible_tabs",
                    "keyboard_shift_ctrl_tab_cycles_backwards",
                    "keyboard_digit_zero_selects_last_visible_tab",
                ),
                "core workspace tab keyboard tests must cover relative and last-visible selection",
            )
        )
        failures.extend(
            self.missing_token_failures(
                window_keyboard,
                (
                    "tabs_keyboard_shortcut",
                    "apply_tabs_keyboard_shortcut",
                    "command_or_control",
                    "Key::LeftSuper",
                ),
                "Storybook tabs keyboard route must map window shortcuts",
            )
        )
        failures.extend(
            self.missing_token_failures(
                route,
                (
                    "is_tab_story_page",
                    "page == \"closeable-tab-strip\"",
                    "CloseableTabKeyboardInput::from_shortcut",
                    "register_tabs_keyboard_input",
                    "tabs_drag_target.take()",
                    "register_tabs_drag_end(&target.tab_id, false)",
                ),
                "Storybook tabs keyboard route must bridge shortcuts to real tab state",
            )
        )
        failures.extend(
            self.missing_token_failures(
                state,
                (
                    "apply_core_tab_keyboard_input",
                    "CloseableTabKeyboardInput::CloseActiveTab",
                    "tab_keyboard_select_visible",
                    "tab_keyboard_close",
                ),
                "Storybook tabs keyboard state must use core tab keyboard input",
            )
        )
        failures.extend(
            self.missing_token_failures(
                bridge,
                ("register_tabs_keyboard_input", "apply_keyboard_input(input)"),
                "Storybook screen state must expose tabs keyboard input registration",
            )
        )
        failures.extend(
            self.missing_token_failures(
                tests,
                (
                    "tabs_keyboard_shortcuts_route_through_storybook_window_interaction",
                    "CloseableTabKeyboardShortcut",
                    "CloseableTabKey::Digit(2)",
                    "closeable_tab_close_requested",
                    "CloseableTabKey::Escape",
                ),
                "Storybook tests must cover tabs keyboard shortcut routing",
            )
        )
        failures.extend(
            self.missing_token_failures(
                closeable_tests,
                (
                    "closeable_tab_strip_keyboard_shortcuts_route_through_storybook_window_interaction",
                    "CloseableTabKeyboardShortcut",
                    "CloseableTabKey::Digit(2)",
                    "CloseableTabKey::Tab",
                    "closeable_tab_close_requested",
                ),
                "Storybook tests must cover closeable-tab-strip keyboard shortcut routing",
            )
        )
        return failures

    def storybook_order_contract_failures(self, paths: tuple[Path, ...]) -> list[str]:
        core_scroll, core_scroll_tests, dedicated, closeable, scroll, order_tests, scroll_tests, closeable_tests = paths
        failures: list[str] = []
        failures.extend(
            self.missing_token_failures(
                core_scroll,
                ("impl WorkspaceTabScrollPlanner", "pub fn follow_active"),
                "core workspace tab scroll planner must expose active follow",
            )
        )
        failures.extend(
            self.missing_token_failures(
                core_scroll_tests,
                (
                    "scroll_planner_follows_active_tab_when_external_selection_moves_right",
                    "scroll_planner_follows_active_tab_when_external_selection_moves_left",
                    "WorkspaceTabScrollPlanner::follow_active",
                ),
                "core workspace tab scroll tests must cover external active follow",
            )
        )
        failures.extend(
            self.missing_token_failures(
                dedicated,
                ("layout_item_ids_for_test", "format!(\"group:{}\", group.id)"),
                "Storybook tabs layout order must be test-addressable",
            )
        )
        failures.extend(
            self.missing_token_failures(
                closeable,
                ("scroll_x_for_test", "strip_rect_for_test"),
                "Storybook closeable-tab-strip scroll follow must be test-addressable",
            )
        )
        failures.extend(
            self.missing_token_failures(
                scroll,
                ("measured_item_ids_for_test", "measured_items(state)"),
                "Storybook tabs scroll measurements must be test-addressable",
            )
        )
        failures.extend(
            self.missing_token_failures(
                order_tests,
                (
                    "tabs_storybook_layout_order_matches_core_visual_order_for_declared_unknown_pinned_ungrouped",
                    "core_visual_tab_ids()",
                    "dedicated_tabs::tab_ids_for_test",
                ),
                "Storybook tabs render order must match core visual order",
            )
        )
        failures.extend(
            self.missing_token_failures(
                scroll_tests,
                (
                    "tabs_scroll_measured_order_matches_render_layout_order",
                    "layout_item_ids_for_test",
                    "measured_item_ids_for_test",
                    "tabs_active_follow_preset_scrolls_current_tab_into_strip",
                ),
                "Storybook tabs scroll measured order must match render layout order",
            )
        )
        failures.extend(
            self.missing_token_failures(
                closeable_tests,
                (
                    "closeable_tab_strip_active_follow_preset_scrolls_current_tab_into_strip",
                    "dedicated_closeable_tab_strip::scroll_x_for_test",
                    "dedicated_closeable_tab_strip::strip_rect_for_test",
                ),
                "Storybook closeable-tab-strip tests must cover active follow scroll",
            )
        )
        return failures

    def storybook_option_regression_failures(self, paths: tuple[Path, ...]) -> list[str]:
        options, state, bridge = paths
        failures: list[str] = []
        failures.extend(
            self.missing_token_failures(
                options,
                (
                    "tabs_inspector_options_mutate_tab_model_state",
                    "tabs.overflow_width",
                    "overflow_trigger_width",
                    "tabs.group_auto_expand",
                    "collapsed_group_auto_expand_ms",
                ),
                "Storybook tabs Inspector options must mutate semantic tab model state",
            )
        )
        failures.extend(
            self.missing_token_failures(
                state,
                ("tabs_window_interaction_keeps_instance_state_isolated",),
                "Storybook tabs instances must keep local state isolated",
            )
        )
        failures.extend(
            self.missing_token_failures(
                bridge,
                (
                    "option.setting != \"active_tab_id\"",
                    "CloseableTabStripAction::SelectTab",
                    "tabs.active_scroll",
                ),
                "Storybook tabs options must route active tab changes through core actions",
            )
        )
        return failures

    def group_move_failures(self, paths: tuple[Path, ...]) -> list[str]:
        (
            actions,
            bar,
            apply_action,
            commands,
            events,
            groups,
            tests,
            storybook,
            storybook_move_tests,
            storybook_group_tests,
            closeable_group_tests,
        ) = paths
        failures: list[str] = []
        failures.extend(
            self.missing_token_failures(
                actions,
                (
                    "MoveGroup",
                    "SetGroupColor",
                    "Ungroup",
                    "CloseGroup",
                    "group_id: WorkspaceTabGroupId",
                    "to_index: usize",
                ),
                "workspace tab groups must expose typed group actions",
            )
        )
        failures.extend(
            self.missing_token_failures(
                apply_action if apply_action.exists() else bar,
                (
                    "WorkspaceTabBarAction::MoveGroup",
                    "self.move_group(group_id, to_index)",
                    "WorkspaceTabBarAction::SetGroupColor",
                    "self.set_group_color(group_id, color)",
                    "WorkspaceTabBarAction::Ungroup",
                    "self.ungroup(group_id)",
                    "WorkspaceTabBarAction::CloseGroup",
                    "self.close_group(group_id)",
                ),
                "workspace tab bar must apply typed group actions",
            )
        )
        failures.extend(
            self.missing_token_failures(
                commands,
                (
                    "pub fn move_group_action",
                    "pub fn set_group_color_action",
                    "WorkspaceTabBarAction::MoveGroup",
                    "WorkspaceTabBarAction::SetGroupColor",
                    "WorkspaceTabBarAction::Ungroup",
                    "WorkspaceTabBarAction::CloseGroup",
                ),
                "group context commands must map group commands to public typed actions",
            )
        )
        failures.extend(
            self.missing_token_failures(
                events,
                (
                    "GroupReordered",
                    "GroupColorChanged",
                    "GroupRemoved",
                    "closeable_tab_group_reordered",
                    "closeable_tab_group_color_changed",
                    "closeable_tab_group_removed",
                ),
                "workspace tab groups must emit typed group events",
            )
        )
        failures.extend(
            self.missing_token_failures(
                groups,
                (
                    "fn move_group",
                    "fn set_group_color",
                    "fn ungroup",
                    "fn close_group",
                    "WorkspaceTabBarEvent::GroupReordered",
                    "WorkspaceTabBarEvent::GroupColorChanged",
                    "WorkspaceTabBarEvent::GroupRemoved",
                ),
                "workspace tab group actions must mutate declared group state",
            )
        )
        failures.extend(
            self.missing_token_failures(
                tests,
                (
                    "move_group_reorders_declared_groups_and_visual_tabs",
                    "move_group_clamps_out_of_range_target_index_to_last_declared_group",
                    "group_color_ungroup_and_close_group_emit_typed_events",
                    "WorkspaceTabBarAction::MoveGroup",
                    "WorkspaceTabBarAction::SetGroupColor",
                    "WorkspaceTabBarAction::Ungroup",
                    "WorkspaceTabBarAction::CloseGroup",
                    "WorkspaceTabBarEvent::GroupReordered",
                    "WorkspaceTabBarEvent::GroupColorChanged",
                    "WorkspaceTabBarEvent::GroupRemoved",
                ),
                "state/action tests must cover group actions and visual order",
            )
        )
        failures.extend(
            self.missing_token_failures(
                storybook,
                (
                    "move_group_from_context",
                    "set_group_color_from_context",
                    "ungroup_from_context",
                    "close_group_from_context",
                    "CloseableTabGroupContextCommand::move_group_action",
                    "CloseableTabGroupContextCommand::set_group_color_action",
                    "CloseableTabGroupContextCommand::Ungroup",
                    "CloseableTabGroupContextCommand::Close",
                ),
                "Storybook group context commands must bridge to core typed actions",
            )
        )
        failures.extend(
            self.missing_token_failures(
                storybook_move_tests,
                (
                    "tabs_group_header_context_menu_move_reorders_groups_through_core_action",
                    "tabs_group_header_context_menu_move_wraps_last_group_to_first_through_core_action",
                    "target_index=0",
                    "target_index=1",
                    "closeable_tab_group_reordered",
                ),
                "Storybook tests must prove group context move reorders real tabs",
            )
        )
        failures.extend(
            self.missing_token_failures(
                storybook_group_tests,
                (
                    "tabs_group_header_context_menu_applies_color_ungroup_and_close",
                    "closeable_tab_group_color_changed",
                    "closeable_tab_group_removed",
                ),
                "Storybook tests must prove group context commands mutate real tabs",
            )
        )
        failures.extend(
            self.missing_token_failures(
                closeable_group_tests,
                (
                    "closeable_tab_strip_group_header_context_menu_moves_ungroups_and_uses_rendered_item_ids",
                    "items[1]",
                    "id = \"rename\"",
                    "GROUP_MOVE_INDEX",
                    "GROUP_UNGROUP_INDEX",
                    "closeable_tab_group_reordered",
                    "closeable_tab_group_removed",
                ),
                "Closeable-tab-strip tests must prove group header commands mutate real tabs",
            )
        )
        return failures

    def missing_token_failures(
        self,
        path: Path,
        required: tuple[str, ...],
        message: str,
    ) -> list[str]:
        if not path.exists():
            return [f"{self.relative(path)}: {message}; source is missing"]
        source = self.read(path)
        return [
            f"{self.relative(path)}: {message}; missing token `{token}`"
            for token in required
            if token not in source
        ]

    def read(self, path: Path) -> str:
        return path.read_text(encoding="utf-8")

    def relative(self, path: Path) -> str:
        return path.relative_to(self.root).as_posix()
