use super::button_layout::UiButtonLayoutDto;
use super::{UiTone, UiVariant};
use serde::{Deserialize, Serialize};

#[path = "typed_panel.rs"]
mod typed_panel;
pub use typed_panel::{
    UiPanelProps, UiRect, UiScrollbarDragState, UiScrollbarModel, UiScrollbarPlacement,
    UiScrollbarVisibility,
};

const DEFAULT_LOADING_SPEED_MS: u16 = 900;
const DEFAULT_DOT_COUNT: u8 = 3;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextProps {
    pub role: String,
    pub color_token: String,
    pub line_height_px: u16,
    pub baseline_offset_px: i16,
    pub vertical_centered: bool,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSlotPlacement {
    Leading,
    Trailing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSlotSpec {
    pub placement: UiSlotPlacement,
    pub label: String,
}

impl UiSlotSpec {
    #[must_use]
    pub fn new(placement: UiSlotPlacement, label: impl Into<String>) -> Self {
        Self {
            placement,
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiClearActionSpec {
    pub label: String,
}

impl UiClearActionSpec {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextEntryProps {
    pub leading_slot: Option<UiSlotSpec>,
    pub trailing_slot: Option<UiSlotSpec>,
    pub clear_action: Option<UiClearActionSpec>,
    pub submit_on_enter: bool,
    pub ime_enabled: bool,
    pub emoji_enabled: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSvgPaintPolicy {
    #[default]
    CurrentColor,
    StrokeOnly,
    FillOnly,
    StrokeAndFill,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiIconProps {
    pub svg_source: String,
    pub view_box: String,
    pub path_summary: String,
    pub paint_policy: UiSvgPaintPolicy,
    pub role: String,
    pub color_token: String,
    pub theme_token: String,
}

impl UiIconProps {
    #[must_use]
    pub fn new(svg_source: impl Into<String>) -> Self {
        Self {
            svg_source: svg_source.into(),
            view_box: String::new(),
            path_summary: String::new(),
            paint_policy: UiSvgPaintPolicy::CurrentColor,
            role: String::new(),
            color_token: String::new(),
            theme_token: String::new(),
        }
    }

    #[must_use]
    pub fn view_box(mut self, value: impl Into<String>) -> Self {
        self.view_box = value.into();
        self
    }

    #[must_use]
    pub fn path_summary(mut self, value: impl Into<String>) -> Self {
        self.path_summary = value.into();
        self
    }

    #[must_use]
    pub fn paint_policy(mut self, value: UiSvgPaintPolicy) -> Self {
        self.paint_policy = value;
        self
    }

    #[must_use]
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = value.into();
        self
    }

    #[must_use]
    pub fn color_token(mut self, value: impl Into<String>) -> Self {
        self.color_token = value.into();
        self
    }

    #[must_use]
    pub fn theme_token(mut self, value: impl Into<String>) -> Self {
        self.theme_token = value.into();
        self
    }
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
