use super::StorybookScreenState;

impl StorybookScreenState {
    pub(in crate::visual) fn register_chip_group_overflow(&mut self) {
        self.action_count += 1;
        let update = self.runtime_structured.chip_group.preview_overflow();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_chip_group_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.runtime_structured.chip_group.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_chip_group_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.runtime_structured.chip_group.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_chip_group_keyboard_dismiss(&mut self) {
        if !self.button_focused {
            self.last_action = "chip_group_keyboard_without_focus";
            self.last_event = "chip_group_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.runtime_structured.chip_group.keyboard_dismiss();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_attachment_chip_error(&mut self) {
        self.action_count += 1;
        let update = self.runtime_structured.attachment_chip.preview_error();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_attachment_chip_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.runtime_structured.attachment_chip.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_attachment_chip_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.runtime_structured.attachment_chip.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_attachment_chip_keyboard_retry(&mut self) {
        if !self.button_focused {
            self.last_action = "attachment_keyboard_without_focus";
            self.last_event = "attachment_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.runtime_structured.attachment_chip.keyboard_retry();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_startup_state_error(&mut self) {
        self.action_count += 1;
        let update = self.runtime_structured.startup_state.preview_error();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_startup_state_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.runtime_structured.startup_state.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_startup_state_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.runtime_structured.startup_state.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_startup_state_keyboard_retry(&mut self) {
        if !self.button_focused {
            self.last_action = "startup_state_keyboard_without_focus";
            self.last_event = "startup_state_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.runtime_structured.startup_state.keyboard_retry();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_window_control_press(&mut self) {
        self.action_count += 1;
        let update = self.runtime_structured.window_control.press_close();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_window_control_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.runtime_structured.window_control.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_window_control_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.runtime_structured.window_control.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_window_control_keyboard_restore(&mut self) {
        if !self.button_focused {
            self.last_action = "window_control_keyboard_without_focus";
            self.last_event = "window_control_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.runtime_structured.window_control.keyboard_restore();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_motion_preview(&mut self) {
        self.action_count += 1;
        let update = self.runtime_structured.motion.preview_reduce();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_motion_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.runtime_structured.motion.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_motion_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.runtime_structured.motion.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_motion_keyboard_tick(&mut self) {
        if !self.button_focused {
            self.last_action = "motion_keyboard_without_focus";
            self.last_event = "motion_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self.runtime_structured.motion.keyboard_tick();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_skeleton_cluster_preview(&mut self) {
        self.action_count += 1;
        let update = self.runtime_structured.skeleton_cluster.preview_card();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_skeleton_cluster_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        let update = self.runtime_structured.skeleton_cluster.focus();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_skeleton_cluster_hover(&mut self) {
        self.action_count += 1;
        self.preview_hovered = true;
        let update = self.runtime_structured.skeleton_cluster.hover();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(in crate::visual) fn register_skeleton_cluster_keyboard_reduce_motion(&mut self) {
        if !self.button_focused {
            self.last_action = "skeleton_cluster_keyboard_without_focus";
            self.last_event = "skeleton_cluster_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let update = self
            .runtime_structured
            .skeleton_cluster
            .keyboard_reduce_motion();
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}

#[cfg(test)]
mod tests {
    use super::StorybookScreenState;

    #[test]
    fn runtime_keyboard_actions_reject_input_before_focus() {
        let mut state = StorybookScreenState::default();

        state.register_chip_group_keyboard_dismiss();
        assert_eq!("chip_group_keyboard_without_focus", state.last_action);
        state.register_attachment_chip_keyboard_retry();
        assert_eq!("attachment_keyboard_without_focus", state.last_action);
        state.register_startup_state_keyboard_retry();
        assert_eq!("startup_state_keyboard_without_focus", state.last_action);
        state.register_window_control_keyboard_restore();
        assert_eq!("window_control_keyboard_without_focus", state.last_action);
        state.register_motion_keyboard_tick();
        assert_eq!("motion_keyboard_without_focus", state.last_action);
        state.register_skeleton_cluster_keyboard_reduce_motion();
        assert_eq!("skeleton_cluster_keyboard_without_focus", state.last_action);
        assert_eq!(0, state.action_count);
    }
}
