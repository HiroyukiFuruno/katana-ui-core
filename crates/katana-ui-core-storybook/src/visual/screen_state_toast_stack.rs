use super::screen_state::StorybookScreenState;
use katana_ui_core::atom::Button;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::NotificationToast;
use katana_ui_core::render_model::{UiDismissAction, UiTone, UiVariant};
use katana_ui_core::widget::molecules::{
    ToastPayload, ToastStackAction, ToastStackEvent, ToastStackManager, ToastStackOptions,
};

const STORYBOOK_TOAST_ID: &str = "save";

impl StorybookScreenState {
    pub(in crate::visual) fn register_notification_toast_dismiss(&mut self) {
        let result = notification_toast_action_result(UiAction::dismiss);
        assert!(
            result.handled && !result.after.open,
            "the toast dismiss action must close the fixture"
        );
        self.action_count += 1;
        self.last_action = "toast_dismiss";
        self.last_event = "toast_dismissed";
        self.last_setting = "interaction.open";
        self.last_setting_value = "false";
        self.state_label = "visible=false";
    }

    pub(in crate::visual) fn register_notification_toast_hover(&mut self) {
        let result = notification_toast_action_result(|target| UiAction::hover(target, true));
        assert!(
            result.handled && result.after.hovered,
            "the toast hover action must update hover state"
        );
        self.action_count += 1;
        self.preview_hovered = true;
        self.last_action = "toast_hover";
        self.last_event = "toast_hovered";
        self.last_setting = "interaction.hovered";
        self.last_setting_value = "true";
        self.state_label = "hover=true";
    }

    pub(in crate::visual) fn register_notification_toast_focus(&mut self) {
        let result = notification_toast_action_result(UiAction::focus);
        assert!(
            result.handled && result.after.focused,
            "the toast focus action must update focus state"
        );
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "toast_focus";
        self.last_event = "toast_focused";
        self.last_setting = "interaction.focused";
        self.last_setting_value = "true";
        self.state_label = "focus=true";
    }

    pub(in crate::visual) fn register_notification_toast_keyboard_dismiss(&mut self) {
        if !self.button_focused {
            self.last_action = "toast_keyboard_without_focus";
            self.last_event = "toast_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        let result = notification_toast_action_result(UiAction::dismiss);
        assert!(
            result.handled && !result.after.open,
            "the focused toast dismiss action must close the fixture"
        );
        self.action_count += 1;
        self.last_action = "toast_keyboard_dismiss";
        self.last_event = "toast_dismissed";
        self.last_setting = "interaction.open";
        self.last_setting_value = "false";
        self.state_label = "visible=false";
    }

    pub(in crate::visual) fn register_toast_stack_hover_pause(&mut self) {
        self.register_toast_stack_pause(
            "toast_stack_hover_pause",
            "toast_stack.hover",
            ToastStackAction::PauseHover(true),
        );
    }

    pub(in crate::visual) fn register_toast_stack_focus_pause(&mut self) {
        self.register_toast_stack_pause(
            "toast_stack_focus_pause",
            "toast_stack.focus",
            ToastStackAction::FocusInside(true),
        );
    }

    fn register_toast_stack_pause(
        &mut self,
        action: &'static str,
        setting: &'static str,
        toast_action: ToastStackAction,
    ) {
        let events = toast_stack_pause_events(toast_action);
        self.action_count += 1;
        self.button_focused = action == "toast_stack_focus_pause";
        self.last_action = action;
        self.last_event = toast_stack_event_name(events.first());
        self.last_setting = setting;
        self.last_setting_value = "true";
        self.state_label = "toast_stack.paused=true";
    }
}

fn notification_toast_action_result(
    action_builder: impl FnOnce(katana_ui_core::render_model::UiStateId) -> UiAction,
) -> katana_ui_core::interaction::UiActionResult {
    let mut toast = notification_toast_fixture();
    let target = toast.state_id().clone();
    toast.apply_action(&action_builder(target))
}

fn notification_toast_fixture() -> NotificationToast {
    NotificationToast::new("Saved")
        .open(true)
        .severity(UiTone::Success)
        .variant(UiVariant::Filled)
        .dismiss_action(UiDismissAction::Available)
        .child(Button::new("Undo"))
}

fn toast_stack_pause_events(action: ToastStackAction) -> Vec<ToastStackEvent> {
    let mut stack = ToastStackManager::new().options(ToastStackOptions {
        max_visible: 1,
        pause_on_hover: true,
        ..ToastStackOptions::default()
    });
    let _ = stack.apply_action(ToastStackAction::Enqueue(ToastPayload::new(
        STORYBOOK_TOAST_ID,
        "Saved",
    )));
    stack.apply_action(action)
}

fn toast_stack_event_name(event: Option<&ToastStackEvent>) -> &'static str {
    match event {
        Some(ToastStackEvent::ToastPaused) => "toast_paused",
        Some(ToastStackEvent::ToastResumed) => "toast_resumed",
        _ => "toast_noop",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_keyboard_and_stack_event_boundaries_are_explicit() {
        let mut state = StorybookScreenState::default();
        state.register_notification_toast_keyboard_dismiss();
        assert_eq!("toast_keyboard_ignored", state.last_event);
        state.register_notification_toast_focus();
        state.register_notification_toast_keyboard_dismiss();
        assert_eq!("toast_dismissed", state.last_event);

        assert_eq!(
            "toast_resumed",
            toast_stack_event_name(Some(&ToastStackEvent::ToastResumed))
        );
        assert_eq!("toast_noop", toast_stack_event_name(None));
    }
}
