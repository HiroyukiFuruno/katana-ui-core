use super::screen_state::StorybookScreenState;

const TEXT_AREA_VISIBLE_LINES: usize = 4;
const TEXT_AREA_SCROLL_STEP_LINES: usize = 1;
const TEXT_AREA_SCROLL_STEP_X: usize = 12;

impl StorybookScreenState {
    pub(super) fn scroll_text_area_vertical(
        &mut self,
        delta_y: f32,
        enabled: bool,
        max_offset: usize,
    ) -> bool {
        if !enabled {
            return false;
        }
        let previous = self.text_area_scroll_offset.min(max_offset);
        let next = if delta_y > 0.0 {
            previous.saturating_add(TEXT_AREA_SCROLL_STEP_LINES)
        } else {
            previous.saturating_sub(TEXT_AREA_SCROLL_STEP_LINES)
        }
        .min(max_offset);
        if next == previous {
            return false;
        }
        self.action_count += 1;
        self.text_area_scroll_offset = next;
        self.last_action = "text_area_scroll_y";
        self.last_event = "text_area_scroll_changed";
        self.last_setting = "text_area.vertical_scroll";
        self.last_setting_value = "wheel";
        self.state_label = "scroll_y=changed";
        true
    }

    pub(super) fn scroll_text_area_horizontal(
        &mut self,
        delta_x: f32,
        enabled: bool,
        max_offset: usize,
    ) -> bool {
        if !enabled {
            return false;
        }
        let previous = self.text_area_scroll_x_offset.min(max_offset);
        let next = if delta_x > 0.0 {
            previous.saturating_add(TEXT_AREA_SCROLL_STEP_X)
        } else {
            previous.saturating_sub(TEXT_AREA_SCROLL_STEP_X)
        }
        .min(max_offset);
        if next == previous {
            return false;
        }
        self.action_count += 1;
        self.text_area_scroll_x_offset = next;
        self.last_action = "text_area_scroll_x";
        self.last_event = "text_area_scroll_changed";
        self.last_setting = "text_area.horizontal_scroll";
        self.last_setting_value = "wheel";
        self.state_label = "scroll_x=changed";
        true
    }

    pub(super) fn text_area_max_scroll_offset(&self) -> usize {
        text_area_line_count(self.text_area_value()).saturating_sub(TEXT_AREA_VISIBLE_LINES)
    }
}

fn text_area_line_count(value: &str) -> usize {
    value.split('\n').count().max(1)
}
