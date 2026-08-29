use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

const AVAILABLE_WIDTH: u32 = 110;
const OVERFLOW_TRIGGER_WIDTH: u32 = 10;
const MEASURED_ACTION_WIDTH: u32 = 40;
const PRIMARY_PRIORITY: i32 = 100;
const SECONDARY_PRIORITY: i32 = 10;
const UTILITY_PRIORITY: i32 = 50;

pub(super) fn story() -> StoryExample {
    let actions = toolbar_actions();
    let input = molecule::toolbar::ToolbarOverflowInput::new(
        AVAILABLE_WIDTH,
        OVERFLOW_TRIGGER_WIDTH,
        molecule::toolbar::ToolbarStrategy::Menu,
        measured_actions(),
    );
    let plan = molecule::toolbar::ToolbarOverflowPlanner::plan(&input);
    let mut state =
        molecule::toolbar::ToolbarState::new(molecule::toolbar::ToolbarDisplayMode::IconLeading);
    let events = state.apply_action(
        &molecule::toolbar::ToolbarInteractionAction::open_split_dropdown("save-as"),
        &actions,
    );
    let root = molecule::Toolbar::new("Toolbar")
        .child(atom::Button::new("Save"))
        .child(atom::Button::new("Search"))
        .child(atom::Button::new("More"))
        .child(atom::KeyCap::new("Cmd+S"));
    let target = UiStateId::new("state:Toolbar:storybook");
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "toolbar_overflow_plan",
            "visible=all",
            format!("hidden={:?}", plan.hidden_action_ids()),
        ),
        UiCallbackLog::new(
            target,
            "toolbar_split_open",
            "split_open=false",
            format!("events={}", toolbar_event_names(&events)),
        ),
    ];
    StoryCatalog::interactive_story("toolbar", root, logs)
}

fn measured_actions() -> Vec<molecule::toolbar::MeasuredToolbarAction> {
    vec![
        measured_action("save", PRIMARY_PRIORITY),
        measured_action("search", SECONDARY_PRIORITY),
        measured_action("export", SECONDARY_PRIORITY),
        measured_action("settings", UTILITY_PRIORITY),
    ]
}

fn measured_action(id: &'static str, priority: i32) -> molecule::toolbar::MeasuredToolbarAction {
    molecule::toolbar::MeasuredToolbarAction::new(
        id,
        MEASURED_ACTION_WIDTH,
        molecule::toolbar::ToolbarPriority::new(priority),
    )
}

fn toolbar_actions() -> Vec<molecule::toolbar::ToolbarAction> {
    vec![
        molecule::toolbar::ToolbarAction::new("save-as", "Save As").split(
            molecule::toolbar::SplitAction::new(
                molecule::toolbar::SplitActionPart::new().disabled(true),
                molecule::toolbar::SplitActionPart::new()
                    .disabled(false)
                    .tooltip("More save options"),
            ),
        ),
        molecule::toolbar::ToolbarAction::new("search", "Search")
            .accelerator(molecule::toolbar::KeyCombo::command_or_control("f")),
    ]
}

fn toolbar_event_names(events: &[molecule::toolbar::ToolbarEvent]) -> String {
    events
        .iter()
        .map(|event| match event {
            molecule::toolbar::ToolbarEvent::Command { .. } => "command",
            molecule::toolbar::ToolbarEvent::OverflowOpened => "overflow_opened",
            molecule::toolbar::ToolbarEvent::SplitDropdownOpened { .. } => "split_dropdown_opened",
            molecule::toolbar::ToolbarEvent::AcceleratorTriggered { .. } => "accelerator_triggered",
            molecule::toolbar::ToolbarEvent::GroupCollapseToggled { .. } => {
                "group_collapse_toggled"
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolbar_event_names_cover_every_public_event_variant() {
        let events = [
            molecule::toolbar::ToolbarEvent::Command {
                action_id: "save".into(),
            },
            molecule::toolbar::ToolbarEvent::OverflowOpened,
            molecule::toolbar::ToolbarEvent::SplitDropdownOpened {
                action_id: "save-as".into(),
                placement: molecule::toolbar::ToolbarPlacementRequest::Menu,
            },
            molecule::toolbar::ToolbarEvent::AcceleratorTriggered {
                action_id: "search".into(),
                combo: molecule::toolbar::KeyCombo::command_or_control("f"),
            },
            molecule::toolbar::ToolbarEvent::GroupCollapseToggled {
                group_id: "editing".into(),
            },
        ];

        assert_eq!(
            "command,overflow_opened,split_dropdown_opened,accelerator_triggered,group_collapse_toggled",
            toolbar_event_names(&events)
        );
    }
}
