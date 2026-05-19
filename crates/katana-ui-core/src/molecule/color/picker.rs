use super::{ColorBlendingMode, RgbaColor};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind, UiSize};
use serde::{Deserialize, Serialize};

const RGBA_ALPHA_MAX: u8 = 255;
const DEFAULT_COLOR_CHANNEL: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorPicker {
    label: String,
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
    children: Vec<UiNode>,
}

impl ColorPicker {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: MoleculeState::new(UiNodeKind::ColorPicker),
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
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn rgba(mut self, value: RgbaColor) -> Self {
        self.value = value;
        self.state.value = value.css_rgba();
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
        self.alpha = value;
        self.value.alpha = value;
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
}

impl ComponentAction for ColorPicker {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction();
        if action.target() != &self.state.state_id || self.state.disabled {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        if self.state.readonly && color_change_action(action) {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        if let UiAction::SetValue {
            color_drag: Some(drag),
            ..
        } = action
        {
            self.value = drag.value.into();
            self.hue = drag.hue;
            self.alpha = drag.value.alpha;
            self.preview = drag.preview;
            self.state.value = self.value.css_rgba();
            return UiActionResult::handled(
                self.state.state_id.clone(),
                action,
                before,
                self.state.interaction(),
            );
        }
        self.state.apply_action(action, false)
    }
}

fn color_change_action(action: &UiAction) -> bool {
    matches!(
        action,
        UiAction::SetValue {
            color_drag: Some(_),
            ..
        }
    )
}

impl From<ColorPicker> for UiNode {
    fn from(value: ColorPicker) -> Self {
        let mut node = value.state.node(UiNodeKind::ColorPicker, value.label);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}
