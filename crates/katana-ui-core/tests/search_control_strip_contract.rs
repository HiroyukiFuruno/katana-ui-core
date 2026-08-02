use katana_ui_core::atom::Text;
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
fn every_search_option_and_result_summary_branch_is_typed() {
    let mut strip = SearchControlStrip::new("Search controls");
    for option in [
        SearchOptionKind::MatchCase,
        SearchOptionKind::WholeWord,
        SearchOptionKind::UseRegex,
    ] {
        assert!(matches!(
            strip
                .apply_action(SearchControlStripAction::ToggleSearchOption(option))
                .as_slice(),
            [SearchControlStripEvent::SearchOptionChanged { enabled: true, .. }]
        ));
    }
    assert!(strip.options_model().match_case);
    assert!(strip.options_model().whole_word);
    assert!(strip.options_model().use_regex);

    assert_eq!(
        "1 / 1",
        SearchControlStrip::new("One")
            .result_position(1, None)
            .result_summary_model()
    );
    assert_eq!(
        "4 results",
        SearchControlStrip::new("Many")
            .result_position(4, None)
            .result_summary_model()
    );
    assert_eq!(
        "",
        SearchControlStrip::new("Unknown").result_summary_model()
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
fn replace_configuration_and_result_position_emit_typed_state_changes() {
    let mut strip = SearchControlStrip::new("Search controls");

    assert_eq!(
        vec![SearchControlStripEvent::ReplaceModeChanged(
            ReplaceMode::Visible
        )],
        strip.apply_action(SearchControlStripAction::SetReplaceMode(
            ReplaceMode::Visible
        ))
    );
    assert_eq!(
        vec![SearchControlStripEvent::ReplaceValueChanged(
            "replacement".to_string()
        )],
        strip.apply_action(SearchControlStripAction::SetReplaceValue(
            "replacement".to_string()
        ))
    );
    assert_eq!(
        vec![SearchControlStripEvent::SearchResultPositionChanged {
            result_count: 4,
            active_index: Some(3)
        }],
        strip.apply_action(SearchControlStripAction::SetResultPosition {
            result_count: 4,
            active_index: Some(3)
        })
    );
    assert_eq!(
        vec![SearchControlStripEvent::SearchNavigationRequested {
            direction: SearchNavigationDirection::Previous
        }],
        strip.apply_action(SearchControlStripAction::Navigate(
            SearchNavigationDirection::Previous
        ))
    );
    assert_eq!("4 / 4", strip.result_summary_model());
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
fn zero_results_disable_navigation_buttons_without_search_logic() {
    let tree = UiTree::new(SearchControlStrip::new("Search controls").result_position(0, None));
    let root = tree.root();

    for label in ["Previous result", "Next result"] {
        assert!(
            root.children()
                .iter()
                .any(|it| it.props().label == label && it.props().disabled),
            "{label} should be disabled when there are no consumer-provided results"
        );
    }
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

#[test]
fn search_control_accessors_and_custom_child_preserve_owned_state() {
    let strip = SearchControlStrip::new("Search controls")
        .query("needle")
        .replace_mode(ReplaceMode::Visible)
        .replace_value("replacement")
        .child(Text::new("Consumer status"));
    let state_id = strip.state_id().clone();

    assert_eq!("needle", strip.query_model());
    assert_eq!(ReplaceMode::Visible, strip.replace_mode_model());
    assert_eq!("replacement", strip.replace_value_model());

    let tree = UiTree::new(strip);
    assert_eq!(state_id, tree.root().props().state_id);
    assert!(
        tree.root()
            .children()
            .iter()
            .any(|child| child.props().label == "Consumer status")
    );
}
