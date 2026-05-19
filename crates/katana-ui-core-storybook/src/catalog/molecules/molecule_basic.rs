use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};
use molecule::{
    AttachmentChipAction, AttachmentKind, AttachmentProgress, AttachmentStatus, ChipGroupAction,
    ChipGroupOverflow, EmptyStateAction, EmptyStateActionId, EmptyStateTone,
};

const TOOLBAR_AVAILABLE_WIDTH: u32 = 110;
const TOOLBAR_OVERFLOW_TRIGGER_WIDTH: u32 = 10;
const TOOLBAR_MEASURED_ACTION_WIDTH: u32 = 40;
const TOOLBAR_PRIMARY_PRIORITY: i32 = 100;
const TOOLBAR_SECONDARY_PRIORITY: i32 = 10;
const TOOLBAR_UTILITY_PRIORITY: i32 = 50;
const CHIP_GROUP_AVAILABLE_WIDTH: u16 = 88;
const CHIP_GROUP_TRIGGER_WIDTH: u16 = 24;
const CHIP_GROUP_CHIP_WIDTH: u16 = 42;
const ATTACHMENT_UPLOAD_PROGRESS: u16 = 4_200;

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
        attachment_chip_story(),
        chip_group_story(),
        empty_state_story(),
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
    ]
}

fn attachment_chip_story() -> StoryExample {
    let mut attachment = molecule::AttachmentChip::new(AttachmentKind::Image, "screenshot.png")
        .progress(AttachmentProgress::from_basis_points(
            ATTACHMENT_UPLOAD_PROGRESS,
        ))
        .status(AttachmentStatus::Uploading);
    let target = attachment.chip().state_id().clone();
    let events = attachment.apply_action(AttachmentChipAction::TransitionStatus(
        AttachmentStatus::Error,
    ));
    let log = UiCallbackLog::new(
        target,
        "attachment_status",
        "status=uploading",
        format!("events={events:?}"),
    );
    StoryCatalog::interactive_story("attachment-chip", attachment, vec![log])
}

fn chip_group_story() -> StoryExample {
    let first = atom::Chip::new("lint").dismissible(true);
    let first_id = first.state_id().clone();
    let second = atom::Chip::new("format").dismissible(true);
    let third = atom::Chip::new("docs").dismissible(true);
    let mut group = molecule::ChipGroup::new("Chip group")
        .chip(first, CHIP_GROUP_CHIP_WIDTH)
        .chip(second, CHIP_GROUP_CHIP_WIDTH)
        .chip(third, CHIP_GROUP_CHIP_WIDTH)
        .wrap(false)
        .available_width(CHIP_GROUP_AVAILABLE_WIDTH)
        .overflow_trigger_width(CHIP_GROUP_TRIGGER_WIDTH)
        .overflow(ChipGroupOverflow::Menu)
        .reorder(true);
    let events = group.apply_action(ChipGroupAction::OpenOverflow);
    let log = UiCallbackLog::new(
        first_id,
        "chip_group_overflow",
        "hidden=0",
        format!("events={events:?}"),
    );
    StoryCatalog::interactive_story("chip-group", group, vec![log])
}

fn empty_state_story() -> StoryExample {
    let empty = molecule::EmptyState::new("No diagnostics")
        .body("日本語 mixed text is centered.")
        .tone(EmptyStateTone::Accent)
        .primary_action(EmptyStateAction::new("reload", "Reload"));
    let target = empty.state_id().clone();
    let event = empty.apply_action(EmptyStateActionId::Primary);
    let log = UiCallbackLog::new(
        target,
        "empty_state_action",
        "action=none",
        format!("event={event:?}"),
    );
    StoryCatalog::interactive_story("empty-state", empty, vec![log])
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
