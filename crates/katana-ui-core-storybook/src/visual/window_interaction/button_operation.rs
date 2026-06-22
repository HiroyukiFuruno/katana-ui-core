use super::StorybookWindowState;
use crate::visual::button_options::{StorybookButtonOptionControl, is_button_page};
use crate::visual::panel_screen_state::{PanelChildKey, PanelOptionControl};
use crate::visual::screen_state_search_control::SearchControlScreenAction;
use crate::visual::screen_state_segmented_toggle::SegmentedToggleScreenAction;
use crate::visual::screen_state_side_menu::SideMenuScreenAction;
use crate::visual::screen_state_tabs::TabsScreenAction;
use crate::visual::selection_screen_state::SelectionScreenAction;
use crate::visual::storybook_ui_option_contract::StorybookUiOptionContract;
use crate::visual::window_interaction::collapsible_panel_state::CollapsiblePanelStoryAction;
use crate::visual::window_interaction::color_picker_operation::{self, ColorPickerAction};
use crate::visual::window_interaction::command_palette_state::CommandPaletteStoryAction;
use crate::visual::window_interaction::diagnostics_list_operation::{
    self, DiagnosticsListStoryAction,
};
use crate::visual::window_interaction::drag_and_drop_operation::{self, DragAndDropAction};
use crate::visual::window_interaction::dynamic_array_editor_operation::{
    self, DynamicArrayEditorAction,
};
use crate::visual::window_interaction::layout_operation::{self, LayoutStoryAction};
use crate::visual::window_interaction::scroll_area_operation::{self, ScrollAreaStoryAction};
use crate::visual::window_interaction::settings_list_operation::{self, SettingsListStoryAction};
use crate::visual::window_interaction::split_pane_operation::{self, SplitPaneStoryAction};
use crate::visual::window_interaction::theme_tokens_operation::{self, ThemeTokensStoryAction};
use crate::visual::window_interaction::virtualization_state::VirtualizationStoryAction;
use crate::visual::{dedicated_breadcrumb, preview, preview_detail};

#[path = "button_operation/apply_operation.rs"]
mod apply_operation;
#[path = "button_operation/binary_choice_operation.rs"]
mod binary_choice_operation;
#[path = "button_operation/breadcrumb_operation.rs"]
mod breadcrumb_operation;
#[path = "button_operation/common_operation.rs"]
mod common_operation;
#[path = "button_operation/menu_operation.rs"]
mod menu_operation;
#[path = "button_operation/selection_operation.rs"]
mod selection_operation;
#[path = "button_operation/settings_operation.rs"]
mod settings_operation;
#[path = "button_operation/status_bar_operation.rs"]
mod status_bar_operation;
#[path = "button_operation/tabs_operation.rs"]
mod tabs_operation;
#[path = "button_operation/text_area_operation.rs"]
mod text_area_operation;
#[path = "button_operation/text_input_operation.rs"]
mod text_input_operation;
#[path = "button_operation/toolbar_operation.rs"]
mod toolbar_operation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum StorybookButtonOperation {
    LightTheme,
    DarkTheme,
    Preset(usize),
    PreviewButton,
    PreviewComponent,
    ButtonOption(StorybookButtonOptionControl),
    PanelOption(PanelOptionControl),
    PanelChild(PanelChildKey),
    ColorPicker(ColorPickerAction),
    DiagnosticsList(DiagnosticsListStoryAction),
    DragAndDrop(DragAndDropAction),
    DynamicArrayEditor(DynamicArrayEditorAction),
    Layout(LayoutStoryAction),
    PanelResize,
    ScrollArea(ScrollAreaStoryAction),
    SettingsList(SettingsListStoryAction),
    SplitPane(SplitPaneStoryAction),
    ThemeTokens(ThemeTokensStoryAction),
    SettingsOption {
        option: StorybookUiOptionContract,
        preset_index: Option<usize>,
    },
    SelectionControl(SelectionScreenAction),
    CheckboxStateRead,
    CheckboxToggle(usize),
    CheckboxToggleFocused,
    CheckboxReset,
    RadioStateRead,
    RadioSelect,
    RadioSelectIndex(usize),
    RadioReset,
    ComboStateRead,
    ComboFilter,
    ComboSelect,
    ComboReset,
    SearchStateRead,
    SearchTypeQuery,
    SearchSubmit,
    SearchClear,
    SearchCaseToggle,
    SearchRegexToggle,
    MenuOpen,
    MenuClose,
    MenuSelect(usize),
    MenuDisabledItem,
    MenuShortcutActivation,
    MenuButtonOpen,
    MenuButtonClose,
    MenuButtonSelect(usize),
    MenuButtonDisabledTrigger,
    StatusBarSegment(usize),
    ToolbarActionButton(usize),
    TabsControl(TabsScreenAction),
    TabsPinIcon {
        tab_id: String,
    },
    CloseableTabStripSelect {
        tab_id: String,
    },
    TreeViewPointer {
        pointer_x: usize,
        pointer_y: usize,
    },
    BreadcrumbSelection(usize),
    TextInputFocus {
        initial_value: &'static str,
        readonly: bool,
    },
    TextInputClearAction {
        initial_value: &'static str,
        readonly: bool,
    },
    TextInputIconButton,
    TextAreaFocus {
        readonly: bool,
        disabled: bool,
    },
    TextAreaClearAction {
        readonly: bool,
        disabled: bool,
    },
    TextAreaIconButton,
}

impl StorybookButtonOperation {
    pub(super) fn apply(self, state: &mut StorybookWindowState) -> bool {
        apply_operation::apply(self, state)
    }
}

pub(super) fn button_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    common_operation::theme_operation_at(x, y)
        .or_else(|| common_operation::preset_operation_at(state, x, y))
        .or_else(|| selection_operation::SelectionOperation::operation_at(state, x, y))
        .or_else(|| menu_operation::operation_at(state, x, y))
        .or_else(|| binary_choice_operation::operation_at(state.selected_page, x, y))
        .or_else(|| common_operation::panel_operation_at(state.selected_page, x, y))
        .or_else(|| {
            color_picker_operation::operation_at(state, x, y)
                .map(StorybookButtonOperation::ColorPicker)
        })
        .or_else(|| {
            diagnostics_list_operation::operation_at(state, x, y)
                .map(StorybookButtonOperation::DiagnosticsList)
        })
        .or_else(|| {
            drag_and_drop_operation::operation_at(state, x, y)
                .map(StorybookButtonOperation::DragAndDrop)
        })
        .or_else(|| {
            dynamic_array_editor_operation::operation_at(state, x, y)
                .map(StorybookButtonOperation::DynamicArrayEditor)
        })
        .or_else(|| {
            layout_operation::operation_at(state, x, y).map(StorybookButtonOperation::Layout)
        })
        .or_else(|| {
            scroll_area_operation::operation_at(state, x, y)
                .map(StorybookButtonOperation::ScrollArea)
        })
        .or_else(|| {
            settings_list_operation::operation_at(state, x, y)
                .map(StorybookButtonOperation::SettingsList)
        })
        .or_else(|| {
            split_pane_operation::operation_at(state, x, y).map(StorybookButtonOperation::SplitPane)
        })
        .or_else(|| {
            theme_tokens_operation::operation_at(state, x, y)
                .map(StorybookButtonOperation::ThemeTokens)
        })
        .or_else(|| breadcrumb_operation::operation_at(state, x, y))
        .or_else(|| text_input_operation::operation_at(state, x, y))
        .or_else(|| text_area_operation::operation_at(state, x, y))
        .or_else(|| tabs_operation::operation_at(state, x, y))
        .or_else(|| status_bar_operation::operation_at(state.selected_page, x, y))
        .or_else(|| toolbar_operation::operation_at(state, x, y))
        .or_else(|| common_operation::preview_operation_at(state.selected_page, x, y))
        .or_else(|| settings_operation::operation_at(state.selected_page, x, y))
}

#[path = "button_operation/hover_operation.rs"]
mod hover_operation;
pub(in crate::visual) use hover_operation::apply_hover_at;

pub(super) fn uses_clickable_preview_cursor(page: &str) -> bool {
    is_button_page(page) || page == "menu-button"
}
