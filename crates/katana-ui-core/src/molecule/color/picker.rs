use super::{ColorBlendingMode, RgbaColor};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind, UiSize};
use serde::{Deserialize, Serialize};

const RGBA_ALPHA_MAX: u8 = 255;
const DEFAULT_COLOR_CHANNEL: u8 = 0;
const DEFAULT_PANEL_SCALE_PERCENT: u16 = 75;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorPicker {
    pub(super) label: String,
    pub(super) state: MoleculeState,
    pub(super) value: RgbaColor,
    pub(super) hue: u16,
    pub(super) alpha: u8,
    pub(super) blending: ColorBlendingMode,
    pub(super) preview: bool,
    pub(super) color_area: String,
    pub(super) trigger_size: UiSize,
    pub(super) title: String,
    pub(super) rgba_mode: bool,
    pub(super) trigger_border: bool,
    pub(super) eyedropper_callback: String,
    pub(super) panel_scale_percent: u16,
    pub(super) children: Vec<UiNode>,
}

impl ColorPicker {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: default_state(),
            value: RgbaColor::new(
                DEFAULT_COLOR_CHANNEL,
                DEFAULT_COLOR_CHANNEL,
                DEFAULT_COLOR_CHANNEL,
                RGBA_ALPHA_MAX,
            ),
            hue: 0,
            alpha: RGBA_ALPHA_MAX,
            blending: ColorBlendingMode::Replace,
            preview: true,
            color_area: String::new(),
            trigger_size: UiSize::Medium,
            title: String::new(),
            rgba_mode: true,
            trigger_border: true,
            eyedropper_callback: String::new(),
            panel_scale_percent: DEFAULT_PANEL_SCALE_PERCENT,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn rgba(mut self, value: RgbaColor) -> Self {
        self.value = self.color_for_mode(value);
        self.alpha = self.value.alpha;
        self.state.value = self.value.css_rgba();
        self
    }

    #[must_use]
    pub fn value(self, value: impl Into<String>) -> Self {
        let mut next = self;
        next.state.value = value.into();
        next
    }

    #[must_use]
    pub fn open(mut self, value: bool) -> Self {
        self.state.open = value;
        self
    }

    #[must_use]
    pub fn hue(mut self, value: u16) -> Self {
        self.hue = value;
        self
    }

    #[must_use]
    pub fn alpha(mut self, value: u8) -> Self {
        self.alpha = self.alpha_for_mode(value);
        self.value.alpha = self.alpha;
        self.state.value = self.value.css_rgba();
        self
    }

    #[must_use]
    pub fn blending(mut self, value: ColorBlendingMode) -> Self {
        self.blending = value;
        self
    }

    #[must_use]
    pub fn color_area(mut self, value: impl Into<String>) -> Self {
        self.color_area = value.into();
        self
    }

    #[must_use]
    pub fn trigger_size(mut self, value: UiSize) -> Self {
        self.trigger_size = value;
        self
    }

    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }

    #[must_use]
    pub fn rgba_mode(mut self, value: bool) -> Self {
        self.rgba_mode = value;
        if !value {
            self.value = self.value.opaque();
            self.alpha = RGBA_ALPHA_MAX;
            self.state.value = self.value.css_rgba();
        }
        self
    }

    #[must_use]
    pub fn trigger_border(mut self, value: bool) -> Self {
        self.trigger_border = value;
        self
    }

    #[must_use]
    pub fn eyedropper_callback(mut self, value: impl Into<String>) -> Self {
        self.eyedropper_callback = value.into();
        self
    }

    #[must_use]
    pub fn panel_scale_percent(mut self, value: u16) -> Self {
        self.panel_scale_percent = value;
        self
    }

    #[must_use]
    pub fn readonly(mut self, value: bool) -> Self {
        self.state.readonly = value;
        self
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.state.disabled = value;
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    pub(super) fn color_for_mode(&self, value: RgbaColor) -> RgbaColor {
        if self.rgba_mode {
            value
        } else {
            value.opaque()
        }
    }

    const fn alpha_for_mode(&self, value: u8) -> u8 {
        if self.rgba_mode {
            value
        } else {
            RGBA_ALPHA_MAX
        }
    }
}

fn default_state() -> MoleculeState {
    let mut state = MoleculeState::new(UiNodeKind::ColorPicker);
    state.value = RgbaColor::new(
        DEFAULT_COLOR_CHANNEL,
        DEFAULT_COLOR_CHANNEL,
        DEFAULT_COLOR_CHANNEL,
        RGBA_ALPHA_MAX,
    )
    .css_rgba();
    state
}
