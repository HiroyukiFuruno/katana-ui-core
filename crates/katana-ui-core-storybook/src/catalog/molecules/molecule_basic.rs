use super::super::{StoryCatalog, StoryExample};
use super::molecule_virtualization;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};
use molecule::{
    AttachmentChipAction, AttachmentKind, AttachmentProgress, AttachmentStatus, ChipGroupAction,
    ChipGroupOverflow, EmptyStateAction, EmptyStateActionId, EmptyStateAlignment, EmptyStateSize,
    EmptyStateTone,
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
const LIST_FOCUSED_INDEX: usize = 18;
const SELECTION_LIST_FOCUSED_INDEX: usize = 12;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        card_story(),
        list_story(),
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
        selection_list_story(),
        StoryCatalog::story(
            "side-menu",
            molecule::SideMenu::new("Side menu")
                .child(atom::Button::new("Files"))
                .child(atom::Button::new("Settings")),
        ),
    ]
}

fn list_story() -> StoryExample {
    let config = molecule_virtualization::fixed_config(
        molecule_virtualization::LIST_TOTAL_COUNT,
        Some(LIST_FOCUSED_INDEX),
    );
    let list = molecule::List::new("List")
        .child(atom::Text::new("Row 1"))
        .child(atom::Text::new("Row 2"))
        .child(atom::Badge::new(molecule_virtualization::compact_label(
            &config,
        )))
        .child(molecule::VirtualizedList::new(
            "List virtualization",
            config.clone(),
        ));
    StoryCatalog::interactive_story(
        "list",
        list,
        vec![molecule_virtualization::log(
            UiStateId::new("state:List:virtualization"),
            "list_virtualization_range",
            &config,
        )],
    )
}

fn selection_list_story() -> StoryExample {
    let config = molecule_virtualization::variable_config(
        molecule_virtualization::SELECTION_TOTAL_COUNT,
        Some(SELECTION_LIST_FOCUSED_INDEX),
    );
    let harness_root = molecule::SelectionList::new("Selection list")
        .item(molecule::ChoiceItem::new("first", "First"))
        .item(molecule::ChoiceItem::new("second", "Second"))
        .item(molecule::ChoiceItem::new("third", "Third"))
        .item(molecule::ChoiceItem::new("fourth", "Fourth"))
        .child(atom::Text::new("row label / row body"))
        .child(
            molecule::VirtualizedList::new("Selection virtualization", config.clone()),
        )
        .child(atom::Badge::new(molecule_virtualization::compact_label(
            &config,
        )))
        .child(atom::Button::new("state read"))
        .child(atom::Button::new("select row"))
        .child(atom::Button::new("multi toggle"))
        .child(atom::Button::new("keyboard next"))
        .child(atom::Button::new("reset"))
        .child(atom::Text::new(
            "state: single=none multi=none focus=none",
        ))
        .child(atom::Text::new(
            "event: selection_list_state_read selection_list_changed selection_list_multi_changed selection_list_keyboard_moved selection_list_reset",
        ))
        .child(atom::Text::new(
            "action: selection_list_state_read selection_list_select_row selection_list_multi_toggle selection_list_keyboard_next selection_list_reset",
        ))
        .child(atom::Text::new(
            "quality: typed state action event state separation keyboard equivalent",
        ));
    let target = harness_root.state_id().clone();

    let mut log_selection = harness_root.clone();
    let _ = log_selection.apply_action(&UiAction::select_box_selected(target.clone(), 1));
    let _ = log_selection.apply_action(&UiAction::set_selected_index(target.clone(), 2));

    let callback_logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "selection_list_state_read",
            "single=none multi=none focus=none",
            "single=none multi=none focus=none",
        ),
        UiCallbackLog::new(
            target.clone(),
            "selection_list_select_row",
            "single=none multi=none focus=none",
            "single=1 multi=none focus=1",
        ),
        UiCallbackLog::new(
            target.clone(),
            "selection_list_multi_toggle",
            "single=1 multi=none focus=1",
            "single=1 multi=1 focus=1",
        ),
        UiCallbackLog::new(
            target.clone(),
            "selection_list_keyboard_next",
            "single=1 multi=1 focus=1",
            "single=2 multi=1 focus=2",
        ),
        UiCallbackLog::new(
            target.clone(),
            "selection_list_reset",
            "single=2 multi=1 focus=2",
            "single=none multi=none focus=none",
        ),
        UiCallbackLog::new(
            target.clone(),
            "select_box_selected",
            "selected=none",
            "selected=second",
        ),
        UiCallbackLog::new(
            target,
            "set_selected_index",
            "selected=second",
            "selected=third",
        ),
    ];
    StoryCatalog::interactive_story(
        "selection-list",
        harness_root,
        vec![molecule_virtualization::log(
            UiStateId::new("state:SelectionList:virtualization"),
            "selection_list_virtualization_range",
            &config,
        )]
        .into_iter()
        .chain(callback_logs)
        .collect(),
    )
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
        .size(EmptyStateSize::Default)
        .alignment(EmptyStateAlignment::Center)
        .primary_action(EmptyStateAction::new("reload", "Reload"))
        .secondary_action(EmptyStateAction::new("docs", "Open docs"));
    let target = empty.state_id().clone();
    let primary = empty.apply_action(EmptyStateActionId::Primary);
    let secondary = empty.apply_action(EmptyStateActionId::Secondary);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "empty_state_primary",
            "action=none",
            format!("event={primary:?}"),
        ),
        UiCallbackLog::new(
            target,
            "empty_state_secondary",
            "action=primary",
            format!("event={secondary:?}"),
        ),
    ];
    StoryCatalog::interactive_story("empty-state", empty, logs)
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
    let mut harness_root = molecule::SearchBox::new("Search box")
        .placeholder("Search")
        .value("query")
        .clear_action("Clear")
        .submit_on_enter(true);
    harness_root = harness_root
        .child(atom::Input::new("Query"))
        .child(atom::Button::new("state read"))
        .child(atom::Button::new("type query"))
        .child(atom::Button::new("submit"))
        .child(atom::Button::new("clear"))
        .child(atom::Button::new("case toggle"))
        .child(atom::Button::new("regex toggle"))
        .child(atom::Text::new("state: value=query case=false regex=false"))
        .child(atom::Text::new(
            "event: search_value_read input_value search_submitted clear_value",
        ))
        .child(atom::Text::new(
            "action: search_state_read search_type_query search_submit search_clear search_case_toggle search_regex_toggle",
        ))
        .child(atom::Text::new(
            "quality: typed state action event submit_on_enter clear_action",
        ));

    let mut log_search = harness_root.clone();
    let target = log_search.state_id().clone();
    let _ = log_search.apply_action(&UiAction::focus(target.clone()));
    let _ = log_search.apply_action(&UiAction::input_value(target.clone(), "typed query"));
    let _ = log_search.apply_action(&UiAction::search_submitted(target.clone()));
    let _ = log_search.apply_action(&UiAction::clear_value(target.clone()));
    let _ = log_search.apply_action(&UiAction::set_value(target.clone(), "case=true"));
    let _ = log_search.apply_action(&UiAction::set_value(target, "regex=true"));

    let mut callback_logs = Vec::new();
    callback_logs.push(UiCallbackLog::new(
        harness_root.state_id().clone(),
        "search_state_read",
        "value=query case=false regex=false",
        "value=query case=false regex=false",
    ));
    callback_logs.push(UiCallbackLog::new(
        harness_root.state_id().clone(),
        "search_type_query",
        "value=query case=false regex=false",
        "value=typed query case=false regex=false",
    ));
    callback_logs.push(UiCallbackLog::new(
        harness_root.state_id().clone(),
        "search_submit",
        "value=typed query case=false regex=false",
        "value=typed query submitted=true",
    ));
    callback_logs.push(UiCallbackLog::new(
        harness_root.state_id().clone(),
        "search_clear",
        "value=typed query submitted=true",
        "value=empty case=false regex=false",
    ));
    callback_logs.push(UiCallbackLog::new(
        harness_root.state_id().clone(),
        "search_case_toggle",
        "value=empty case=false regex=false",
        "value=empty case=true regex=false",
    ));
    callback_logs.push(UiCallbackLog::new(
        harness_root.state_id().clone(),
        "search_regex_toggle",
        "value=empty case=true regex=false",
        "value=empty case=true regex=true",
    ));

    StoryCatalog::interactive_story("search-box", harness_root, callback_logs)
}
