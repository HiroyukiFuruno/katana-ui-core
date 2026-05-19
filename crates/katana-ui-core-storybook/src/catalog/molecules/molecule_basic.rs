use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

const TOOLBAR_AVAILABLE_WIDTH: u32 = 110;
const TOOLBAR_OVERFLOW_TRIGGER_WIDTH: u32 = 10;
const TOOLBAR_MEASURED_ACTION_WIDTH: u32 = 40;
const TOOLBAR_PRIMARY_PRIORITY: i32 = 100;
const TOOLBAR_SECONDARY_PRIORITY: i32 = 10;
const TOOLBAR_UTILITY_PRIORITY: i32 = 50;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        card_story(),
        StoryCatalog::story(
            "list",
            molecule::List::new("List")
                .child(atom::Text::new("Row 1"))
                .child(atom::Text::new("Row 2")),
        ),
        StoryCatalog::story(
            "tabs",
            molecule::Tabs::new("Tabs")
                .child(atom::Text::new("Tab"))
                .child(atom::Text::new("Panel")),
        ),
        toolbar_story(),
        StoryCatalog::story(
            "form-field",
            molecule::FormField::new("Form field")
                .child(atom::Text::new("Label"))
                .child(atom::Input::new("Value")),
        ),
        StoryCatalog::story(
            "breadcrumb",
            molecule::Breadcrumb::new("Breadcrumb")
                .child(atom::Text::new("Root"))
                .child(atom::Text::new("Leaf")),
        ),
        search_box_story(),
        StoryCatalog::story(
            "selection-list",
            molecule::SelectionList::new("Selection list")
                .child(atom::Text::new("First"))
                .child(atom::Text::new("Second")),
        ),
        StoryCatalog::story(
            "side-menu",
            molecule::SideMenu::new("Side menu")
                .child(atom::Button::new("Files"))
                .child(atom::Button::new("Settings")),
        ),
        StoryCatalog::story(
            "status-bar",
            molecule::StatusBar::new("Status bar")
                .child(atom::Badge::new("Ready"))
                .child(atom::Text::new("Ln 1")),
        ),
    ]
}

fn toolbar_story() -> StoryExample {
    let actions = toolbar_actions();
    let input = molecule::toolbar::ToolbarOverflowInput::new(
        TOOLBAR_AVAILABLE_WIDTH,
        TOOLBAR_OVERFLOW_TRIGGER_WIDTH,
        molecule::toolbar::ToolbarStrategy::Menu,
        vec![
            molecule::toolbar::MeasuredToolbarAction::new(
                "save",
                TOOLBAR_MEASURED_ACTION_WIDTH,
                molecule::toolbar::ToolbarPriority::new(TOOLBAR_PRIMARY_PRIORITY),
            ),
            molecule::toolbar::MeasuredToolbarAction::new(
                "search",
                TOOLBAR_MEASURED_ACTION_WIDTH,
                molecule::toolbar::ToolbarPriority::new(TOOLBAR_SECONDARY_PRIORITY),
            ),
            molecule::toolbar::MeasuredToolbarAction::new(
                "export",
                TOOLBAR_MEASURED_ACTION_WIDTH,
                molecule::toolbar::ToolbarPriority::new(TOOLBAR_SECONDARY_PRIORITY),
            ),
            molecule::toolbar::MeasuredToolbarAction::new(
                "settings",
                TOOLBAR_MEASURED_ACTION_WIDTH,
                molecule::toolbar::ToolbarPriority::new(TOOLBAR_UTILITY_PRIORITY),
            ),
        ],
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

fn card_story() -> StoryExample {
    let mut card = molecule::Card::new("Card")
        .interactive(true)
        .header(atom::Text::new("Header"))
        .child(atom::Text::new("Body"))
        .footer(atom::Button::new("Open"));
    let target = card.state_id().clone();
    let result = card.apply_action(&UiAction::click(target));
    StoryCatalog::interactive_story("card", card, result.callback_log)
}

fn search_box_story() -> StoryExample {
    let mut search = molecule::SearchBox::new("Search box")
        .placeholder("Search")
        .value("query")
        .clear_action("Clear")
        .submit_on_enter(true)
        .child(atom::Input::new("Query"))
        .child(atom::Button::new("Clear"));
    let target = search.state_id().clone();
    let result = search.apply_action(&UiAction::search_submitted(target));
    StoryCatalog::interactive_story("search-box", search, result.callback_log)
}
