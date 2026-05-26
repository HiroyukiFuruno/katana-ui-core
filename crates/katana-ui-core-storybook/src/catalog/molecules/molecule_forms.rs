use super::super::{StoryCatalog, StoryExample};
use super::molecule_virtualization;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

const SELECTION_LIST_FOCUSED_INDEX: usize = 12;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![search_box_story(), selection_list_story()]
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

    let callback_logs = vec![
        UiCallbackLog::new(
            harness_root.state_id().clone(),
            "search_state_read",
            "value=query case=false regex=false",
            "value=query case=false regex=false",
        ),
        UiCallbackLog::new(
            harness_root.state_id().clone(),
            "search_type_query",
            "value=query case=false regex=false",
            "value=typed query case=false regex=false",
        ),
        UiCallbackLog::new(
            harness_root.state_id().clone(),
            "search_submit",
            "value=typed query case=false regex=false",
            "value=typed query submitted=true",
        ),
        UiCallbackLog::new(
            harness_root.state_id().clone(),
            "search_clear",
            "value=typed query submitted=true",
            "value=empty case=false regex=false",
        ),
        UiCallbackLog::new(
            harness_root.state_id().clone(),
            "search_case_toggle",
            "value=empty case=false regex=false",
            "value=empty case=true regex=false",
        ),
        UiCallbackLog::new(
            harness_root.state_id().clone(),
            "search_regex_toggle",
            "value=empty case=true regex=false",
            "value=empty case=true regex=true",
        ),
    ];

    StoryCatalog::interactive_story("search-box", harness_root, callback_logs)
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
        .child(molecule::VirtualizedList::new(
            "Selection virtualization",
            config.clone(),
        ))
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
