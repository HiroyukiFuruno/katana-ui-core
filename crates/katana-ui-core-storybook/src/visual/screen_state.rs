use super::button_options::StorybookButtonOptions;
use super::interaction_spec::StorybookInteractionSpec;
use super::list_screen_state::ListScreenState;
use super::panel_screen_state::PanelScreenState;
use super::screen_state_search_control::SearchControlScreenState;
use super::screen_state_segmented_toggle::SegmentedToggleScreenState;
use super::screen_state_setting_semantics::semantic_setting_state;
use super::screen_state_settings::{format_setting_action, format_setting_event};
use super::screen_state_side_menu::{SideMenuScreenAction, SideMenuScreenState};
use super::screen_state_tabs::TabsScreenState;
use super::search_box_screen_state::SearchBoxScreenState;
use super::selection_screen_state::SelectionScreenState;
use super::storybook_ui_option_contract::StorybookUiOptionContract;
use super::text_area_screen_state::TextAreaStateStore;
use super::text_input_screen_state::TextInputStateStore;
use super::window_interaction::collapsible_panel_state::{
    CollapsiblePanelScreenState, CollapsiblePanelStoryAction,
};
use super::window_interaction::color_picker_state::ColorPickerScreenState;
use super::window_interaction::command_palette_state::CommandPaletteScreenState;
use super::window_interaction::diagnostics_list_state::DiagnosticsListScreenState;
use super::window_interaction::drag_and_drop_operation::DragAndDropScreenState;
use super::window_interaction::dynamic_array_editor_operation::DynamicArrayEditorScreenState;
use super::window_interaction::layout_operation::{self, LayoutStoryState};
use super::window_interaction::runtime_structured_state::RuntimeStructuredScreenState;
use super::window_interaction::scroll_area_operation::ScrollAreaStoryState;
use super::window_interaction::settings_list_state::SettingsListScreenState;
use super::window_interaction::shortcut_cheatsheet_state::ShortcutCheatsheetScreenState;
use super::window_interaction::split_pane_operation::SplitPaneStoryState;
use super::window_interaction::theme_tokens_operation::{
    ThemeTokensStoryAction, ThemeTokensStoryState,
};
use super::window_interaction::virtualization_state::{
    VirtualizationScreenState, VirtualizationStoryAction,
};
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{EmptyState, EmptyStateAction, EmptyStateActionId, EmptyStateEvent};
use katana_ui_core::molecule::{
    FileTree, FileTreeAction, FileTreeHitTestInput, FileTreeItem, FileTreeState,
};
use katana_ui_core::state::UiComponentState;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct StorybookScreenState {
    pub(super) action_count: usize,
    pub(super) settings_revision: usize,
    pub(super) last_action: &'static str,
    pub(super) last_event: &'static str,
    pub(super) last_setting: &'static str,
    pub(super) last_setting_value: &'static str,
    pub(super) state_label: &'static str,
    pub(super) button_options: StorybookButtonOptions,
    pub(super) breadcrumb_selected_index: usize,
    pub(super) button_pressed: bool,
    pub(super) button_focused: bool,
    pub(super) preview_hovered: bool,
    pub(super) hovered_text_input_icon_button_index: Option<usize>,
    pub(super) hovered_text_input_clear_action: bool,
    pub(super) hovered_text_area_icon_button_index: Option<usize>,
    pub(super) hovered_text_area_clear_action: bool,
    pub(super) hovered_toolbar_action_index: Option<usize>,
    pub(super) hovered_summary_index: Option<usize>,
    pub(super) status_bar_open_segment_index: Option<usize>,
    pub(super) status_bar_focused_segment_index: Option<usize>,
    pub(super) status_bar_hovered_segment_index: Option<usize>,
    pub(super) modal_open: bool,
    pub(super) list: ListScreenState,
    pub(super) selection: SelectionScreenState,
    pub(super) search_box: SearchBoxScreenState,
    pub(super) search_control: SearchControlScreenState,
    pub(super) segmented_toggle: SegmentedToggleScreenState,
    pub(super) side_menu: SideMenuScreenState,
    pub(super) tabs: TabsScreenState,
    pub(super) panel: PanelScreenState,
    pub(super) checkbox_state: UiComponentState,
    pub(super) checkbox_secondary_state: UiComponentState,
    pub(super) checkbox_focused_index: usize,
    pub(super) checkbox_hovered_index: Option<usize>,
    pub(super) radio_state: UiComponentState,
    pub(super) toggle_checked: bool,
    pub(super) toggle_checked_overridden: bool,
    pub(super) progress_percent: u8,
    pub(super) progress_changed: bool,
    pub(super) progress_elapsed_ms: u16,
    pub(super) tree_view_scroll_offset: u32,
    pub(super) tree_view_selected_id: &'static str,
    pub(super) tree_view_focused_id: &'static str,
    pub(super) text_inputs: TextInputStateStore,
    pub(super) text_areas: TextAreaStateStore,
    pub(super) collapsible_panel: CollapsiblePanelScreenState,
    pub(super) command_palette: CommandPaletteScreenState,
    pub(super) color_picker: ColorPickerScreenState,
    pub(super) diagnostics_list: DiagnosticsListScreenState,
    pub(super) settings_list: SettingsListScreenState,
    pub(super) shortcut_cheatsheet: ShortcutCheatsheetScreenState,
    pub(super) drag_and_drop: DragAndDropScreenState,
    pub(super) dynamic_array_editor: DynamicArrayEditorScreenState,
    pub(super) scroll_area: ScrollAreaStoryState,
    pub(super) split_pane: SplitPaneStoryState,
    pub(super) theme_tokens: ThemeTokensStoryState,
    pub(super) layout: LayoutStoryState,
    pub(super) runtime_structured: RuntimeStructuredScreenState,
    pub(super) virtualization: VirtualizationScreenState,
}

#[path = "screen_state_empty_tree_actions.rs"]
mod screen_state_empty_tree_actions;
#[path = "screen_state_preview_action.rs"]
mod screen_state_preview_action;
#[path = "screen_state_progress.rs"]
mod screen_state_progress;
#[path = "screen_state_runtime_actions.rs"]
mod screen_state_runtime_actions;
#[path = "screen_state_selection_actions.rs"]
mod screen_state_selection_actions;
#[path = "screen_state_settings_contract.rs"]
mod screen_state_settings_contract;
#[path = "screen_state_tree_ids.rs"]
mod screen_state_tree_ids;
