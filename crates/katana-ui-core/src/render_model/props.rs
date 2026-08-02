use super::UiStateId;
use super::{
    UiButtonProps, UiColorPickerProps, UiColorSwatchProps, UiCommandResultProps, UiCommonProps,
    UiContextMenuProps, UiDisclosureProps, UiDragHandleProps, UiDragPreviewProps,
    UiDropIndicatorProps, UiFormFieldProps, UiGridProps, UiIconProps, UiImageSurfaceProps,
    UiLoadingProps, UiModalProps, UiPanelProps, UiPopoverProps, UiScrollAreaProps,
    UiSearchControlProps, UiShortcutProps, UiSkeletonProps, UiSplitPaneProps, UiStatusProps,
    UiTextAreaProps, UiTextEntryProps, UiTextProps, UiTreeProps,
};
use crate::facade::DEFAULT_FONT_ROLE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiVisualRole {
    Content,
    Icon,
    Shortcut,
    Control,
    Input,
    Status,
    Separator,
    Loading,
    Progress,
    MediaFrame,
    ExportMediaFrame,
    HoverSurface,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiVariant {
    Plain,
    Filled,
    Text,
    Icon,
    IconText,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTone {
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSize {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiInteractionState {
    pub open: bool,
    pub has_selection: bool,
    pub selected_index: usize,
    pub item_count: usize,
    pub value: String,
    #[serde(default)]
    pub surface_control_target_id: String,
    pub hovered: bool,
    pub active: bool,
    pub focused: bool,
    pub dragging: bool,
    pub reduced_motion: bool,
    pub animation_phase: u16,
    pub cursor: usize,
    pub selection_start: usize,
    pub selection_end: usize,
    pub dismiss_reason: String,
}

impl UiInteractionState {
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "open={} selected={} index={} count={} value={} hover={} active={} focus={} dragging={} reduced_motion={} phase={} cursor={} selection={}:{} dismiss={}",
            self.open,
            self.has_selection,
            self.selected_index,
            self.item_count,
            self.value,
            self.hovered,
            self.active,
            self.focused,
            self.dragging,
            self.reduced_motion,
            self.animation_phase,
            self.cursor,
            self.selection_start,
            self.selection_end,
            self.dismiss_reason
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiProps {
    pub label: String,
    pub state_id: UiStateId,
    pub common: UiCommonProps,
    pub disabled: bool,
    pub focusable: bool,
    pub accessibility_label: String,
    pub interaction: UiInteractionState,
    pub theme_id: String,
    pub font_role: String,
    pub style_classes: Vec<String>,
    pub visual_role: UiVisualRole,
    pub variant: UiVariant,
    pub tone: UiTone,
    pub size: UiSize,
    pub loading: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub placeholder: String,
    pub checked: bool,
    pub determinate: bool,
    pub progress_percent: u8,
    pub severity: UiTone,
    pub text: UiTextProps,
    pub button: UiButtonProps,
    pub color_swatch: UiColorSwatchProps,
    pub color_picker: UiColorPickerProps,
    pub command_result: UiCommandResultProps,
    pub form_field: UiFormFieldProps,
    pub shortcut: UiShortcutProps,
    pub image_surface: UiImageSurfaceProps,
    pub search_control: UiSearchControlProps,
    pub text_entry: UiTextEntryProps,
    pub text_area: UiTextAreaProps,
    pub status: UiStatusProps,
    pub loading_indicator: UiLoadingProps,
    pub skeleton: UiSkeletonProps,
    pub disclosure: UiDisclosureProps,
    pub icon: UiIconProps,
    pub drag_handle: UiDragHandleProps,
    pub drop_indicator: UiDropIndicatorProps,
    pub drag_preview: UiDragPreviewProps,
    pub panel: UiPanelProps,
    pub tree: UiTreeProps,
    #[serde(default)]
    pub grid: UiGridProps,
    pub context_menu: UiContextMenuProps,
    pub scroll_area: UiScrollAreaProps,
    pub split_pane: UiSplitPaneProps,
    pub modal: UiModalProps,
    pub popover: UiPopoverProps,
}

impl UiProps {
    #[must_use]
    pub fn new(label: impl Into<String>, state_id: UiStateId) -> Self {
        Self {
            label: label.into(),
            state_id,
            common: UiCommonProps::default(),
            disabled: false,
            focusable: false,
            accessibility_label: String::new(),
            interaction: UiInteractionState::default(),
            theme_id: String::new(),
            font_role: DEFAULT_FONT_ROLE.to_string(),
            style_classes: Vec::new(),
            visual_role: UiVisualRole::Content,
            variant: UiVariant::Plain,
            tone: UiTone::Neutral,
            size: UiSize::Medium,
            loading: false,
            readonly: false,
            invalid: false,
            placeholder: String::new(),
            checked: false,
            determinate: false,
            progress_percent: 0,
            severity: UiTone::Neutral,
            text: UiTextProps::default(),
            button: UiButtonProps::default(),
            color_swatch: UiColorSwatchProps::default(),
            color_picker: UiColorPickerProps::default(),
            command_result: UiCommandResultProps::default(),
            form_field: UiFormFieldProps::default(),
            shortcut: UiShortcutProps::default(),
            image_surface: UiImageSurfaceProps::default(),
            search_control: UiSearchControlProps::default(),
            text_entry: UiTextEntryProps::default(),
            text_area: UiTextAreaProps::default(),
            status: UiStatusProps::default(),
            loading_indicator: UiLoadingProps::default(),
            skeleton: UiSkeletonProps::default(),
            disclosure: UiDisclosureProps::default(),
            icon: UiIconProps::default(),
            drag_handle: UiDragHandleProps::default(),
            drop_indicator: UiDropIndicatorProps::default(),
            drag_preview: UiDragPreviewProps::default(),
            panel: UiPanelProps::default(),
            tree: UiTreeProps::default(),
            grid: UiGridProps::default(),
            context_menu: UiContextMenuProps::default(),
            scroll_area: UiScrollAreaProps::default(),
            split_pane: UiSplitPaneProps::default(),
            modal: UiModalProps::default(),
            popover: UiPopoverProps::default(),
        }
    }
}
