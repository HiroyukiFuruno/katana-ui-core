use super::{
    CollapsiblePanelStoryAction, SideMenuScreenAction, StorybookInteractionSpec,
    StorybookScreenState, ThemeTokensStoryAction, VirtualizationStoryAction,
};
use crate::visual::screen_state_search_control::SearchControlScreenAction;
use crate::visual::screen_state_segmented_toggle::SegmentedToggleScreenAction;
use crate::visual::search_box_screen_state::SearchBoxScreenAction;
use crate::visual::selection_screen_state::SelectionScreenAction;

impl StorybookScreenState {
    pub(in crate::visual) fn register_preview_action(&mut self, page: &str) {
        if page == "checkbox" {
            self.register_checkbox_toggle();
            return;
        }
        if page == "radio" {
            self.register_radio_select();
            return;
        }
        if page == "toggle" {
            self.register_toggle_change();
            return;
        }
        if page == "progress-bar" {
            self.register_progress_bar_change();
            return;
        }
        if page == "panel" {
            self.action_count += 1;
            let update = self.panel.apply_preview_action();
            self.apply_panel_update(update);
            return;
        }
        if page == "tabs" {
            self.register_tabs_preview_action();
            return;
        }
        if page == "breadcrumb" {
            self.register_breadcrumb_preview_action();
            return;
        }
        if page == "breadcrumb-hover" {
            self.register_breadcrumb_hover(2);
            return;
        }
        if page == "breadcrumb-focus" {
            self.register_breadcrumb_focus(2);
            return;
        }
        if page == "breadcrumb-keyboard" {
            self.register_breadcrumb_focus(0);
            self.register_breadcrumb_keyboard_next();
            return;
        }
        if page == "accordion" {
            self.register_accordion_preview_toggle();
            return;
        }
        if page == "accordion-hover" {
            self.register_accordion_hover();
            return;
        }
        if page == "accordion-focus" {
            self.register_accordion_focus();
            return;
        }
        if page == "accordion-keyboard" {
            self.register_accordion_focus();
            self.register_accordion_keyboard_toggle();
            return;
        }
        if page == "accordion-disabled" {
            self.register_accordion_disabled_block();
            return;
        }
        if page == "accordion-group" {
            self.register_accordion_group_toggle();
            return;
        }
        if page == "code-diff" {
            self.register_code_diff_mode_switch();
            return;
        }
        if page == "code-diff-hover" {
            self.register_code_diff_hover();
            return;
        }
        if page == "code-diff-focus" {
            self.register_code_diff_focus();
            return;
        }
        if page == "code-diff-keyboard" {
            self.register_code_diff_focus();
            self.register_code_diff_keyboard_expand();
            return;
        }
        if page == "code-diff-scroll-sync" {
            self.register_code_diff_scroll_sync();
            return;
        }
        if page == "theme-tokens-hover" {
            self.register_theme_tokens_action(ThemeTokensStoryAction::Hover);
            return;
        }
        if page == "theme-tokens-focus" {
            self.register_theme_tokens_action(ThemeTokensStoryAction::Focus);
            return;
        }
        if page == "theme-tokens-keyboard" {
            self.register_theme_tokens_action(ThemeTokensStoryAction::Focus);
            self.register_theme_tokens_action(ThemeTokensStoryAction::Keyboard);
            return;
        }
        if page == "theme-tokens-resize" {
            self.register_theme_tokens_action(ThemeTokensStoryAction::Resize);
            return;
        }
        if page == "collapsible-panel" {
            self.register_collapsible_panel_action(CollapsiblePanelStoryAction::Resize);
            return;
        }
        if page == "virtualization" {
            self.register_virtualization_action(VirtualizationStoryAction::Scroll);
            return;
        }
        if page == "list" {
            self.register_list_select(1);
            return;
        }
        if page == "tree-view-hover" {
            self.register_tree_view_hover();
            return;
        }
        if page == "tree-view-focus" {
            self.register_tree_view_focus();
            return;
        }
        if page == "tree-view-keyboard" {
            self.register_tree_view_focus();
            self.register_tree_view_keyboard_select();
            return;
        }
        if page == "tree-view-scroll" {
            self.register_tree_view_scroll_retention();
            return;
        }
        if page == "menu-button" {
            self.register_menu_button_open();
            return;
        }
        if page == "modal-overlay" {
            self.register_modal_overlay_backdrop_close();
            return;
        }
        if page == "notification-toast" {
            self.register_notification_toast_dismiss();
            return;
        }
        if page == "popover" {
            self.register_popover_open();
            return;
        }
        if page == "hover-card" {
            self.register_hover_card_open();
            return;
        }
        if page == "hover-card-hover" {
            self.register_hover_card_hover();
            return;
        }
        if page == "hover-card-focus" {
            self.register_hover_card_focus();
            return;
        }
        if page == "hover-card-inner-focus" {
            self.register_hover_card_inner_focus_keep_open();
            return;
        }
        if page == "search-box" {
            self.register_search_box_action(SearchBoxScreenAction::Submit);
            return;
        }
        if page == "search-control-strip" {
            self.register_search_control_action(SearchControlScreenAction::Query);
            return;
        }
        if page == "segmented-toggle" {
            self.register_segmented_toggle_action(SegmentedToggleScreenAction::Select);
            return;
        }
        if page == "selection-list" {
            self.register_selection_action(SelectionScreenAction::SelectionListSelectRow(2));
            return;
        }
        if page == "side-menu" {
            self.register_side_menu_action(SideMenuScreenAction::Select(1));
            return;
        }
        if page == "shortcut-combo" {
            self.register_shortcut_combo_preview();
            return;
        }
        if page == "shortcut-cheatsheet" {
            self.register_shortcut_cheatsheet_preview();
            return;
        }
        if page == "skeleton-cluster" {
            self.register_skeleton_cluster_preview();
            return;
        }
        if page == "motion" {
            self.register_motion_preview();
            return;
        }
        if page == "window-control-button-group" {
            self.register_window_control_press();
            return;
        }
        if page == "startup-state-panel" {
            self.register_startup_state_error();
            return;
        }
        if page == "attachment-chip" {
            self.register_attachment_chip_error();
            return;
        }
        if page == "chip-group" {
            self.register_chip_group_overflow();
            return;
        }
        if page == "empty-state" {
            self.register_empty_state_primary_action();
            return;
        }
        if page == "empty-state-hover" {
            self.register_empty_state_hover();
            return;
        }
        if page == "empty-state-focus" {
            self.register_empty_state_focus();
            return;
        }
        if page == "empty-state-keyboard" {
            self.register_empty_state_focus();
            self.register_empty_state_keyboard_action();
            return;
        }
        self.action_count += 1;
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.state_label = spec.state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_action_routes_cover_every_specialized_and_fallback_route() {
        let routes = [
            "checkbox",
            "radio",
            "toggle",
            "progress-bar",
            "panel",
            "tabs",
            "breadcrumb",
            "breadcrumb-hover",
            "breadcrumb-focus",
            "breadcrumb-keyboard",
            "accordion",
            "accordion-hover",
            "accordion-focus",
            "accordion-keyboard",
            "accordion-disabled",
            "accordion-group",
            "code-diff",
            "code-diff-hover",
            "code-diff-focus",
            "code-diff-keyboard",
            "code-diff-scroll-sync",
            "theme-tokens-hover",
            "theme-tokens-focus",
            "theme-tokens-keyboard",
            "theme-tokens-resize",
            "collapsible-panel",
            "virtualization",
            "list",
            "tree-view-hover",
            "tree-view-focus",
            "tree-view-keyboard",
            "tree-view-scroll",
            "menu-button",
            "modal-overlay",
            "notification-toast",
            "popover",
            "hover-card",
            "hover-card-hover",
            "hover-card-focus",
            "hover-card-inner-focus",
            "search-box",
            "search-control-strip",
            "segmented-toggle",
            "selection-list",
            "side-menu",
            "shortcut-combo",
            "shortcut-cheatsheet",
            "skeleton-cluster",
            "motion",
            "window-control-button-group",
            "startup-state-panel",
            "attachment-chip",
            "chip-group",
            "empty-state",
            "empty-state-hover",
            "empty-state-focus",
            "empty-state-keyboard",
            "fallback-route",
        ];

        for route in routes {
            let mut state = StorybookScreenState::default();
            state.register_preview_action(route);
            assert_ne!(state.last_action, "none", "route {route}");
            assert_ne!(state.last_event, "none", "route {route}");
            assert_ne!(state.state_label, "idle", "route {route}");
        }
    }
}
