use super::screen_state::StorybookScreenState;

const TEXT_AREA_CARET_BLINK_FRAMES: usize = 30;

pub(super) enum TextAreaInputKey {
    Character(char),
    Backspace,
    Newline,
    Submit,
}

impl StorybookScreenState {
    pub(super) fn register_text_area_focus(&mut self) {
        self.action_count += 1;
        self.text_area_focused = true;
        self.text_area_uses_live_value = true;
        self.text_area_caret_visible = true;
        self.last_action = "text_area_focus";
        self.last_event = "text_area_focused";
        self.last_setting = "text_area.value";
        self.last_setting_value = "focus";
        self.state_label = "focused=true";
    }

    pub(super) fn register_text_area_key(&mut self, key: TextAreaInputKey) -> bool {
        if !self.text_area_focused {
            return false;
        }
        match key {
            TextAreaInputKey::Character(value) => self.register_text_area_character(value),
            TextAreaInputKey::Backspace => self.register_text_area_backspace(),
            TextAreaInputKey::Newline => self.register_text_area_newline(),
            TextAreaInputKey::Submit => self.register_text_area_submit(),
        }
    }

    pub(super) fn text_area_value(&self) -> &str {
        self.text_area_value.as_str()
    }

    pub(super) const fn text_area_focused(&self) -> bool {
        self.text_area_focused
    }

    pub(super) const fn text_area_uses_live_value(&self) -> bool {
        self.text_area_uses_live_value
    }

    pub(super) const fn text_area_caret_visible(&self) -> bool {
        self.text_area_caret_visible
    }

    pub(super) const fn text_area_wrap_enabled(&self) -> bool {
        self.text_area_wrap_enabled
    }

    pub(super) const fn text_area_resize_enabled(&self) -> bool {
        self.text_area_resize_enabled
    }

    pub(super) const fn text_area_vertical_scroll_enabled(&self) -> bool {
        self.text_area_vertical_scroll_enabled
    }

    pub(super) const fn text_area_horizontal_scroll_enabled(&self) -> bool {
        self.text_area_horizontal_scroll_enabled
    }

    pub(super) const fn text_area_vertical_scrollbar_visible(&self) -> bool {
        self.text_area_vertical_scrollbar_visible
    }

    pub(super) const fn text_area_horizontal_scrollbar_visible(&self) -> bool {
        self.text_area_horizontal_scrollbar_visible
    }

    pub(super) const fn text_area_scroll_offset(&self) -> usize {
        self.text_area_scroll_offset
    }

    pub(super) const fn text_area_scroll_x_offset(&self) -> usize {
        self.text_area_scroll_x_offset
    }

    pub(super) const fn text_area_resize_width_delta(&self) -> usize {
        self.text_area_resize_width_delta
    }

    pub(super) const fn text_area_resize_height_delta(&self) -> usize {
        self.text_area_resize_height_delta
    }

    pub(super) fn show_text_area_caret(&mut self) -> bool {
        self.set_text_area_caret_visibility(true)
    }

    pub(super) fn update_text_area_caret_visibility(&mut self, elapsed_frames: usize) -> bool {
        if !self.text_area_focused {
            return self.set_text_area_caret_visibility(false);
        }
        let blink_index = elapsed_frames / TEXT_AREA_CARET_BLINK_FRAMES;
        self.set_text_area_caret_visibility(blink_index.is_multiple_of(2))
    }

    pub(super) fn register_text_area_resize_toggle(&mut self) {
        self.settings_revision += 1;
        self.text_area_resize_enabled = !self.text_area_resize_enabled;
        self.last_action = "set_text_area.resize_enabled";
        self.last_event = "text_area_settings_changed";
        self.last_setting = "text_area.resize_enabled";
        self.last_setting_value = if self.text_area_resize_enabled {
            "true"
        } else {
            "false"
        };
        self.state_label = if self.text_area_resize_enabled {
            "resize=true"
        } else {
            "resize=false"
        };
    }

    pub(super) fn register_text_area_resize_drag(
        &mut self,
        width_delta: usize,
        height_delta: usize,
    ) -> bool {
        if self.text_area_resize_width_delta == width_delta
            && self.text_area_resize_height_delta == height_delta
        {
            return false;
        }
        self.action_count += 1;
        self.text_area_resize_width_delta = width_delta;
        self.text_area_resize_height_delta = height_delta;
        self.last_action = "text_area_resize_drag";
        self.last_event = "text_area_resized";
        self.last_setting = "text_area.resize_enabled";
        self.last_setting_value = "drag";
        self.state_label = "size=changed";
        true
    }

    fn register_text_area_character(&mut self, value: char) -> bool {
        self.text_area_value.push(value);
        self.apply_text_area_value("text_area_type", "text_area_changed", "value=typing");
        true
    }

    fn register_text_area_backspace(&mut self) -> bool {
        if self.text_area_value.pop().is_none() {
            return false;
        }
        self.apply_text_area_value(
            "text_area_delete_backward",
            "text_area_changed",
            "value=typing",
        );
        true
    }

    fn register_text_area_newline(&mut self) -> bool {
        self.text_area_value.push('\n');
        self.apply_text_area_value("text_area_newline", "text_area_changed", "newline=inserted");
        true
    }

    fn register_text_area_submit(&mut self) -> bool {
        self.apply_text_area_value("text_area_submit", "text_area_submitted", "value=typed");
        true
    }

    fn apply_text_area_value(
        &mut self,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) {
        self.action_count += 1;
        self.text_area_uses_live_value = true;
        self.text_area_caret_visible = true;
        self.text_area_scroll_offset = if self.text_area_vertical_scroll_enabled {
            self.text_area_max_scroll_offset()
        } else {
            0
        };
        self.last_action = action;
        self.last_event = event;
        self.last_setting = "text_area.value";
        self.last_setting_value = "keyboard";
        self.state_label = state;
    }

    fn set_text_area_caret_visibility(&mut self, visible: bool) -> bool {
        if self.text_area_caret_visible == visible {
            return false;
        }
        self.text_area_caret_visible = visible;
        true
    }
}
