use super::screen_state::StorybookScreenState;
use katana_ui_core::molecule::toolbar::{
    KeyCombo, ToolbarAction, ToolbarDisplayMode, ToolbarEvent, ToolbarInteractionAction,
    ToolbarKeyboardInput, ToolbarKeyboardNavigator, ToolbarState,
};

const SAVE_ACTION_INDEX: usize = 0;
const SPLIT_ACTION_INDEX: usize = 1;
const SEARCH_ACTION_INDEX: usize = 2;
const EXPORT_ACTION_INDEX: usize = 3;
const MORE_ACTION_INDEX: usize = 4;
const ACTION_COUNT: usize = 5;

impl StorybookScreenState {
    pub(in crate::visual) fn register_toolbar_action_button(&mut self, action_index: usize) {
        let events = toolbar_events_for_index(action_index);
        let Some(event) = events.first() else {
            return;
        };
        self.action_count += 1;
        self.apply_toolbar_event(event);
    }

    pub(in crate::visual) fn register_toolbar_focus(&mut self) {
        let result =
            ToolbarKeyboardNavigator::apply(None, ACTION_COUNT, ToolbarKeyboardInput::Home);
        let Some(index) = result.focused_index() else {
            return;
        };
        self.action_count += 1;
        self.button_focused = true;
        self.hovered_toolbar_action_index = Some(index);
        self.last_action = "toolbar_focus";
        self.last_event = "toolbar_focused";
        self.state_label = toolbar_focus_label(index);
    }

    pub(in crate::visual) fn register_toolbar_keyboard_activation(&mut self) {
        if !self.button_focused {
            self.last_action = "toolbar_keyboard_without_focus";
            self.last_event = "toolbar_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        let result = ToolbarKeyboardNavigator::apply(
            self.hovered_toolbar_action_index,
            ACTION_COUNT,
            ToolbarKeyboardInput::Space,
        );
        let Some(index) = result.activated_index() else {
            return;
        };
        self.register_toolbar_action_button(index);
    }

    fn apply_toolbar_event(&mut self, event: &ToolbarEvent) {
        match event {
            ToolbarEvent::Command { .. } => {
                self.last_action = "tool_toggle";
                self.last_event = "tool_changed";
                self.state_label = "active=true";
            }
            ToolbarEvent::OverflowOpened => {
                self.last_action = "toolbar_overflow_open";
                self.last_event = "toolbar_overflow_opened";
                self.state_label = "overflow=open";
            }
            ToolbarEvent::SplitDropdownOpened { .. } => {
                self.last_action = "toolbar_split_open";
                self.last_event = "toolbar_split_opened";
                self.state_label = "split=open";
            }
            ToolbarEvent::AcceleratorTriggered { .. } => {
                self.last_action = "toolbar_accelerator";
                self.last_event = "toolbar_accelerator_triggered";
                self.state_label = "accelerator=triggered";
            }
            ToolbarEvent::GroupCollapseToggled { .. } => {
                self.last_action = "toolbar_group_toggle";
                self.last_event = "toolbar_group_toggled";
                self.state_label = "group=collapsed";
            }
        }
    }
}

fn toolbar_events_for_index(action_index: usize) -> Vec<ToolbarEvent> {
    let actions = toolbar_actions();
    let mut state = ToolbarState::new(ToolbarDisplayMode::IconLeading);
    let action = match action_index {
        SPLIT_ACTION_INDEX => ToolbarInteractionAction::open_split_dropdown("save-as"),
        MORE_ACTION_INDEX => ToolbarInteractionAction::OpenOverflow,
        SEARCH_ACTION_INDEX => ToolbarInteractionAction::activate("search"),
        EXPORT_ACTION_INDEX => ToolbarInteractionAction::activate("export"),
        _ => ToolbarInteractionAction::press("save"),
    };
    state.apply_action(&action, &actions)
}

fn toolbar_actions() -> Vec<ToolbarAction> {
    vec![
        ToolbarAction::new("save", "Save"),
        ToolbarAction::new("save-as", "Save As"),
        ToolbarAction::new("search", "Search").accelerator(KeyCombo::command_or_control("f")),
        ToolbarAction::new("export", "Export"),
    ]
}

fn toolbar_focus_label(index: usize) -> &'static str {
    match index {
        SAVE_ACTION_INDEX => "focus=save",
        SPLIT_ACTION_INDEX => "focus=split",
        SEARCH_ACTION_INDEX => "focus=search",
        EXPORT_ACTION_INDEX => "focus=export",
        MORE_ACTION_INDEX => "focus=more",
        _ => "focus=toolbar",
    }
}
