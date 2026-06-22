use super::color_picker_operation::ColorPickerAction;
use super::color_picker_update::ColorPickerUpdate;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::molecule::{ColorPicker, RgbaColor};

const EYEDROPPER_CALLBACK: &str = "storybook-eyedropper";
const DEFAULT_HUE: u16 = 214;
const DRAG_HUE: u16 = 226;
const DEFAULT_PANEL_SCALE_PERCENT: u16 = 75;
const FULL_PANEL_SCALE_PERCENT: u16 = 100;
const DEFAULT_VALUE: RgbaActionValue = RgbaActionValue::new(64, 128, 255, 204);
const DRAG_VALUE: RgbaActionValue = RgbaActionValue::new(72, 136, 240, 188);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::visual) struct ColorPickerScreenState {
    value: RgbaActionValue,
    hue: u16,
    preview: bool,
    readonly: bool,
    disabled: bool,
    color_changed: bool,
    callback_action: &'static str,
    option_state: ColorPickerOptionState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct ColorPickerOptionState {
    pub(in crate::visual) panel_open: bool,
    pub(in crate::visual) blending_multiply: bool,
    pub(in crate::visual) color_area_visible: bool,
    pub(in crate::visual) trigger_large: bool,
    pub(in crate::visual) title_customized: bool,
    pub(in crate::visual) rgba_mode: bool,
    pub(in crate::visual) panel_scale_percent: u16,
    pub(in crate::visual) trigger_border: bool,
}

impl Default for ColorPickerOptionState {
    fn default() -> Self {
        Self {
            panel_open: false,
            blending_multiply: false,
            color_area_visible: false,
            trigger_large: false,
            title_customized: false,
            rgba_mode: true,
            panel_scale_percent: DEFAULT_PANEL_SCALE_PERCENT,
            trigger_border: true,
        }
    }
}

impl Default for ColorPickerScreenState {
    fn default() -> Self {
        Self {
            value: DEFAULT_VALUE,
            hue: DEFAULT_HUE,
            preview: true,
            readonly: false,
            disabled: false,
            color_changed: false,
            callback_action: "none",
            option_state: ColorPickerOptionState::default(),
        }
    }
}

impl ColorPickerScreenState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: ColorPickerAction,
    ) -> ColorPickerUpdate {
        match action {
            ColorPickerAction::Drag => self.apply_drag(
                DRAG_VALUE,
                DRAG_HUE,
                "color_drag",
                "rgba_changed",
                "rgba=accent",
            ),
            ColorPickerAction::HueDrag => self.apply_drag(
                DRAG_VALUE,
                DRAG_HUE,
                "color_hue_drag",
                "hue_changed",
                "color_picker.hue=226",
            ),
            ColorPickerAction::AlphaDrag => self.apply_drag(
                DRAG_VALUE,
                DEFAULT_HUE,
                "color_alpha_drag",
                "alpha_changed",
                "color_picker.alpha=188",
            ),
            ColorPickerAction::Eyedropper => self.apply_eyedropper(),
            ColorPickerAction::Focus => self.apply_focus(),
            ColorPickerAction::Hover => self.apply_hover(),
            ColorPickerAction::ReadonlyBlocked => self.apply_blocked_write(true, false),
            ColorPickerAction::DisabledBlocked => self.apply_blocked_write(false, true),
        }
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "color_picker.rgba" => self.apply_option_color(DEFAULT_VALUE, DEFAULT_HUE),
            "color_picker.value" => self.apply_option_color(DRAG_VALUE, DEFAULT_HUE),
            "color_picker.open" => self.option_state.panel_open = true,
            "color_picker.hue" => self.hue = DEFAULT_HUE,
            "color_picker.alpha" => self.value.alpha = DEFAULT_VALUE.alpha,
            "color_picker.blending" => self.option_state.blending_multiply = true,
            "color_picker.color_area" => self.option_state.color_area_visible = true,
            "color_picker.trigger_size" => self.option_state.trigger_large = true,
            "color_picker.title" => self.option_state.title_customized = true,
            "color_picker.rgba_mode" => self.option_state.rgba_mode = false,
            "color_picker.panel_scale_percent" => {
                self.option_state.panel_scale_percent = FULL_PANEL_SCALE_PERCENT;
            }
            "color_picker.trigger_border" => self.option_state.trigger_border = false,
            "color_picker.eyedropper_callback" => {
                self.callback_action = "color_eyedropper_request";
            }
            "color_picker.readonly" => self.readonly = true,
            "color_picker.disabled" => self.disabled = true,
            _ => {}
        }
    }

    #[cfg(test)]
    pub(in crate::visual) fn rgba_label(&self) -> String {
        self.value.css_rgba()
    }

    #[cfg(test)]
    pub(in crate::visual) const fn hue(&self) -> u16 {
        self.hue
    }

    #[cfg(test)]
    pub(in crate::visual) const fn alpha(&self) -> u8 {
        self.value.alpha
    }

    pub(in crate::visual) const fn callback_action(&self) -> &'static str {
        self.callback_action
    }

    pub(in crate::visual) const fn has_committed_color(&self) -> bool {
        self.color_changed
    }

    pub(in crate::visual) const fn blocks_writes(&self) -> bool {
        self.readonly
    }

    pub(in crate::visual) const fn blocks_focus(&self) -> bool {
        self.disabled
    }

    #[cfg(test)]
    pub(in crate::visual) const fn option_state(&self) -> ColorPickerOptionState {
        self.option_state
    }

    fn apply_drag(
        &mut self,
        value: RgbaActionValue,
        hue: u16,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> ColorPickerUpdate {
        let mut picker = self.core_picker(false, false);
        let result = picker.apply_action(&UiAction::color_drag(
            picker.state_id().clone(),
            value,
            hue,
            self.preview,
        ));
        assert!(result.handled, "core color picker must handle color drag");
        self.value = value;
        self.hue = picker.hue_value();
        self.preview = picker.previews_color();
        self.color_changed = true;
        self.callback_action = action;
        ColorPickerUpdate::counted(action, event, state, "color_picker.value")
    }

    fn apply_eyedropper(&mut self) -> ColorPickerUpdate {
        let mut picker = self.core_picker(false, false);
        let result = picker.apply_action(&UiAction::invoke_callback(
            picker.state_id().clone(),
            EYEDROPPER_CALLBACK,
        ));
        assert!(
            result.handled,
            "core color picker must handle eyedropper callback"
        );
        self.callback_action = "color_eyedropper_request";
        ColorPickerUpdate::counted(
            "color_eyedropper_request",
            "eyedropper_requested",
            "color_picker.eyedropper=storybook-eyedropper",
            "color_picker.eyedropper_callback",
        )
    }

    fn apply_focus(&mut self) -> ColorPickerUpdate {
        let mut picker = self.core_picker(false, false);
        let result = picker.apply_action(&UiAction::focus(picker.state_id().clone()));
        assert!(result.handled, "core color picker must handle focus");
        self.callback_action = "color_picker_focus";
        ColorPickerUpdate::counted(
            "color_picker_focus",
            "color_picker_focused",
            "focus=true",
            "color_picker.focus",
        )
    }

    fn apply_hover(&mut self) -> ColorPickerUpdate {
        let mut picker = self.core_picker(false, false);
        let result = picker.apply_action(&UiAction::hover(picker.state_id().clone(), true));
        assert!(result.handled, "core color picker must handle hover");
        self.callback_action = "color_picker_hover";
        ColorPickerUpdate::counted(
            "color_picker_hover",
            "color_picker_hovered",
            "hover=true",
            "color_picker.hover",
        )
    }

    fn apply_blocked_write(&mut self, readonly: bool, disabled: bool) -> ColorPickerUpdate {
        let mut picker = self.core_picker(readonly, disabled);
        let result = picker.apply_action(&UiAction::color_drag(
            picker.state_id().clone(),
            DRAG_VALUE,
            DRAG_HUE,
            true,
        ));
        assert!(!result.handled, "core color picker must block this write");
        if disabled {
            self.callback_action = "color_picker_disabled_blocked";
            return ColorPickerUpdate::uncounted(
                "color_picker_disabled_blocked",
                "color_picker_focus_blocked",
                "color_picker.disabled.blocks_focus",
                "color_picker.disabled",
            );
        }
        self.callback_action = "color_picker_readonly_blocked";
        ColorPickerUpdate::uncounted(
            "color_picker_readonly_blocked",
            "color_picker_write_blocked",
            "color_picker.readonly.blocks_writes",
            "color_picker.readonly",
        )
    }

    fn apply_option_color(&mut self, value: RgbaActionValue, hue: u16) {
        self.value = value;
        self.hue = hue;
        self.color_changed = true;
    }

    fn core_picker(&self, readonly: bool, disabled: bool) -> ColorPicker {
        ColorPicker::new("Storybook color picker")
            .rgba(RgbaColor::new(
                self.value.red,
                self.value.green,
                self.value.blue,
                self.value.alpha,
            ))
            .hue(self.hue)
            .eyedropper_callback(EYEDROPPER_CALLBACK)
            .readonly(readonly || self.readonly)
            .disabled(disabled || self.disabled)
    }
}
