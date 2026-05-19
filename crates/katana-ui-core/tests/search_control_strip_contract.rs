use katana_ui_core::molecule::{
    CommandPalette, CommandResultRow, ReplaceMode, SearchControlStrip, SearchControlStripAction,
    SearchControlStripEvent, SearchNavigationDirection, SearchOptionKind, SearchOptions,
    SearchReplaceScope,
};
use katana_ui_core::render_model::{UiNodeKind, UiSearchReplaceMode, UiTree};

#[test]
fn query_change_emits_typed_event_without_running_search() {
    let mut strip = SearchControlStrip::new("Search controls");
    let events = strip.apply_action(SearchControlStripAction::SetSearchQuery(
        "heading".to_string(),
    ));

    assert_eq!("heading", strip.query_model());
    assert_eq!(
        vec![SearchControlStripEvent::SearchQueryChanged(
            "heading".to_string()
        )],
        events
    );
}

#[test]
fn option_toggle_updates_typed_options_and_event() {
    let mut strip = SearchControlStrip::new("Search controls");
    let events = strip.apply_action(SearchControlStripAction::ToggleSearchOption(
        SearchOptionKind::UseRegex,
    ));

    assert!(strip.options_model().use_regex);
    assert_eq!(
        vec![SearchControlStripEvent::SearchOptionChanged {
            option: SearchOptionKind::UseRegex,
            enabled: true
        }],
        events
    );
}

#[test]
fn navigation_emits_request_without_computing_result() {
    let mut strip = SearchControlStrip::new("Search controls").result_position(10, Some(2));
    let events = strip.apply_action(SearchControlStripAction::Navigate(
        SearchNavigationDirection::Next,
    ));

    assert_eq!(
        vec![SearchControlStripEvent::SearchNavigationRequested {
            direction: SearchNavigationDirection::Next
        }],
        events
    );
    assert_eq!("3 / 10", strip.result_summary_model());
}

#[test]
fn replace_hidden_ignores_replace_action_and_visible_emits_request() {
    let mut hidden = SearchControlStrip::new("Search controls")
        .replace_mode(ReplaceMode::Hidden)
        .replace_value("new");
    assert!(
        hidden
            .apply_action(SearchControlStripAction::Replace(SearchReplaceScope::All))
            .is_empty()
    );

    let mut visible = SearchControlStrip::new("Search controls")
        .replace_mode(ReplaceMode::Visible)
        .replace_value("new");
    assert_eq!(
        vec![SearchControlStripEvent::ReplaceRequested {
            scope: SearchReplaceScope::All,
            value: "new".to_string()
        }],
        visible.apply_action(SearchControlStripAction::Replace(SearchReplaceScope::All))
    );
}

#[test]
fn render_exposes_result_summary_tooltips_and_replace_state() {
    let tree = UiTree::new(
        SearchControlStrip::new("Search controls")
            .query("katana")
            .options(SearchOptions {
                match_case: true,
                whole_word: false,
                use_regex: true,
            })
            .result_position(0, None)
            .replace_mode(ReplaceMode::Disabled)
            .replace_value("blade"),
    );
    let root = tree.root();

    assert_eq!(UiNodeKind::SearchControlStrip, root.kind());
    assert_eq!("katana", root.props().search_control.query);
    assert!(root.props().search_control.match_case);
    assert!(root.props().search_control.use_regex);
    assert_eq!("0 results", root.props().search_control.result_summary);
    assert_eq!(
        UiSearchReplaceMode::Disabled,
        root.props().search_control.replace_mode
    );
    assert!(root.children().iter().any(|it| {
        it.children()
            .iter()
            .any(|child| child.kind() == UiNodeKind::Tooltip)
    }));
    assert!(
        root.children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::Input && it.props().disabled)
    );
}

#[test]
fn search_control_and_command_palette_state_ids_are_independent() {
    let search = UiTree::new(SearchControlStrip::new("Search controls"));
    let command = UiTree::new(
        CommandPalette::new("Commands").result_row(CommandResultRow::new("open", "Open")),
    );

    assert_ne!(
        search.root().props().state_id,
        command.root().props().state_id
    );
}
