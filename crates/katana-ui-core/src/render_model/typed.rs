use super::button_layout::UiButtonLayoutDto;
use super::{UiStateId, UiTone, UiVariant};
use serde::{Deserialize, Serialize};

#[path = "typed_color_picker.rs"]
mod typed_color_picker;
#[path = "typed_command.rs"]
mod typed_command;
#[path = "typed_disclosure.rs"]
mod typed_disclosure;
#[path = "typed_drag.rs"]
mod typed_drag;
#[path = "typed_grid.rs"]
mod typed_grid;
#[path = "typed_grid_types.rs"]
mod typed_grid_types;
#[path = "typed_icon.rs"]
mod typed_icon;
#[path = "typed_modal.rs"]
mod typed_modal;
#[path = "typed_panel.rs"]
mod typed_panel;
#[path = "typed_popover.rs"]
mod typed_popover;
#[path = "typed_scroll_area.rs"]
mod typed_scroll_area;
#[path = "typed_search.rs"]
mod typed_search;
#[path = "typed_split_pane.rs"]
mod typed_split_pane;
#[path = "typed_text.rs"]
mod typed_text;
#[path = "typed_text_emoji.rs"]
mod typed_text_emoji;
#[path = "typed_text_entry.rs"]
mod typed_text_entry;
pub use typed_color_picker::{UiColorBlendingMode, UiColorPickerProps, UiColorPickerTriggerKind};
pub use typed_command::UiCommandResultProps;
pub use typed_disclosure::{
    UiDisclosureIndicatorPosition, UiDisclosureProps, UiDisclosureTriggerArea,
};
pub use typed_drag::{UiDragHandleProps, UiDragPreviewProps, UiDropIndicatorProps};
pub use typed_grid::{
    UiGridCell, UiGridCellAppearance, UiGridCellSpan, UiGridCoordinate, UiGridDataBar,
    UiGridHorizontalAlignment, UiGridIcon, UiGridIndexRange, UiGridProps, UiGridRating,
    UiGridSelection, UiGridValidationError, UiGridVerticalAlignment, UiGridViewport,
    UiGridVisibleRange,
};
pub use typed_icon::{UiIconProps, UiSvgPaintPolicy};
pub use typed_modal::{
    UiModalParentInteraction, UiModalPlacement, UiModalPresentation, UiModalProps, UiModalSize,
};
pub use typed_panel::{
    UiPanelProps, UiRect, UiScrollbarDragState, UiScrollbarModel, UiScrollbarPlacement,
    UiScrollbarVisibility,
};
pub use typed_popover::{UiPopoverFocusManagement, UiPopoverPlacement, UiPopoverProps};
pub use typed_scroll_area::{UiScrollAreaAxis, UiScrollAreaProps};
pub use typed_search::{UiSearchControlProps, UiSearchReplaceMode};
pub use typed_split_pane::{
    UiSplitPaneAxis, UiSplitPaneHandleProps, UiSplitPaneProps, UiSplitPaneResizeMode,
};
pub use typed_text::{
    APPLE_COLOR_EMOJI_FONT_FAMILY, LINUX_COLOR_EMOJI_FONT_FAMILY, RGBA_CHANNEL_COUNT,
    UiEmojiTextSegment, UiEmojiTextSegments, UiPlatformEmojiFontFamily, UiTextProps, UiTextSpan,
    UiTextSpanStyle, UiTextWrapMode,
};
pub use typed_text_entry::{
    UiClearActionSpec, UiSlotActionSpec, UiSlotPlacement, UiSlotSpec, UiTextEntryProps,
};

const DEFAULT_LOADING_SPEED_MS: u16 = 900;
const DEFAULT_DOT_COUNT: u8 = 3;
const DEFAULT_SKELETON_RADIUS_PX: u16 = 4;
const DEFAULT_SKELETON_LINE_COUNT: usize = 1;
const DEFAULT_SKELETON_LAST_LINE_PERCENT: u8 = 100;
const DEFAULT_SKELETON_LINE_THICKNESS_PX: u16 = 12;
const DEFAULT_SKELETON_ASPECT_RATIO_WIDTH: u16 = 0;
const DEFAULT_SKELETON_ASPECT_RATIO_HEIGHT: u16 = 0;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiButtonProps {
    pub icon_position: String,
    pub command: String,
    pub keyboard_activation: bool,
    pub layout: UiButtonLayoutDto,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiColorSwatchProps {
    pub palette: Vec<String>,
    pub selected_color: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiShortcutProps {
    pub platform: String,
    pub combo: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiFormFieldProps {
    pub helper_text: String,
    pub required: bool,
    pub control_state_id: Option<UiStateId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiDismissAction {
    None,
    Available,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiStatusProps {
    pub severity: UiTone,
    pub variant: UiVariant,
    pub dismiss_action: UiDismissAction,
    pub leading_icon: String,
}

impl Default for UiStatusProps {
    fn default() -> Self {
        Self {
            severity: UiTone::Neutral,
            variant: UiVariant::Plain,
            dismiss_action: UiDismissAction::None,
            leading_icon: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiProgressMode {
    Determinate,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiAnimationState {
    Idle,
    Running,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiLoadingProps {
    pub mode: UiProgressMode,
    pub label: String,
    pub animation_state: UiAnimationState,
    pub speed_ms: u16,
    pub dot_count: u8,
    pub reduced_motion: bool,
}

impl Default for UiLoadingProps {
    fn default() -> Self {
        Self {
            mode: UiProgressMode::Indeterminate,
            label: String::new(),
            animation_state: UiAnimationState::Idle,
            speed_ms: DEFAULT_LOADING_SPEED_MS,
            dot_count: DEFAULT_DOT_COUNT,
            reduced_motion: false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSkeletonShape {
    #[default]
    Rect,
    Circle,
    Line,
    Text,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSkeletonAnimation {
    #[default]
    None,
    Pulse,
    Shimmer,
    Wave,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSkeletonProps {
    pub shape: UiSkeletonShape,
    pub animation: UiSkeletonAnimation,
    pub radius_px: u16,
    pub line_count: usize,
    pub last_line_percent: u8,
    pub line_thickness_px: u16,
    pub aspect_ratio_width: u16,
    pub aspect_ratio_height: u16,
}

impl Default for UiSkeletonProps {
    fn default() -> Self {
        Self {
            shape: UiSkeletonShape::Rect,
            animation: UiSkeletonAnimation::None,
            radius_px: DEFAULT_SKELETON_RADIUS_PX,
            line_count: DEFAULT_SKELETON_LINE_COUNT,
            last_line_percent: DEFAULT_SKELETON_LAST_LINE_PERCENT,
            line_thickness_px: DEFAULT_SKELETON_LINE_THICKNESS_PX,
            aspect_ratio_width: DEFAULT_SKELETON_ASPECT_RATIO_WIDTH,
            aspect_ratio_height: DEFAULT_SKELETON_ASPECT_RATIO_HEIGHT,
        }
    }
}
