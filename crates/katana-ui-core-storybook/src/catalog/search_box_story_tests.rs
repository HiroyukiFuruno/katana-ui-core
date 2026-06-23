use super::{StoryCatalog, StoryPresetLabels};

const SEARCH_BOX_PAGE: &str = "search-box";

#[test]
fn search_box_story_exposes_consumer_harness_controls_and_contract_lines()
-> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = search_box_story(&examples)?;
    let callback_actions = story
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>();
    let labels = story
        .tree
        .root()
        .children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect::<Vec<_>>();

    for expected in [
        "state read",
        "type query",
        "submit",
        "clear",
        "case toggle",
        "regex toggle",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(expected)),
            "search-box preview lacks {expected}"
        );
    }

    assert!(
        labels
            .iter()
            .any(|it| { it.contains("state: value=query case=false regex=false") })
    );
    assert!(labels.iter().any(|it| {
        it.contains("event: search_value_read input_value search_submitted clear_value")
    }));
    assert!(labels.iter().any(|it| {
        it.contains(
            "action: search_state_read search_type_query search_submit search_clear search_case_toggle search_regex_toggle",
        )
    }));
    assert!(labels
        .iter()
        .any(|it| it.contains("quality: typed state action event submit_on_enter clear_action")));
    for expected in [
        "search_state_read",
        "search_type_query",
        "search_submit",
        "search_clear",
        "search_case_toggle",
        "search_regex_toggle",
    ] {
        assert!(
            callback_actions.iter().any(|it| it == &expected),
            "search-box callback log lacks {expected}"
        );
    }
    Ok(())
}

#[test]
fn search_box_story_root_props_match_first_state_read_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = search_box_story(&examples)?;
    let root = story.tree.root();
    let first = story
        .callback_logs
        .first()
        .ok_or("search-box callback log missing")?;

    assert_eq!("query", root.props().interaction.value);
    assert!(!root.props().interaction.open);
    assert!(!root.props().interaction.has_selection);
    assert!(root.props().text_entry.submit_on_enter);
    assert_eq!(
        Some("Clear"),
        root.props()
            .text_entry
            .clear_action
            .as_ref()
            .map(|it| it.label.as_str())
    );
    assert_eq!("search_state_read", first.action);
    assert_eq!("value=query case=false regex=false", first.before);
    assert_eq!("value=query case=false regex=false", first.after);
    Ok(())
}

#[test]
fn search_box_story_has_component_specific_presets() {
    assert_eq!(
        &["search icon", "submit action", "regex case", "theme clear"],
        StoryPresetLabels::for_page(SEARCH_BOX_PAGE)
    );
}

fn search_box_story(
    examples: &[super::StoryExample],
) -> Result<&super::StoryExample, &'static str> {
    examples
        .iter()
        .find(|it| it.page == SEARCH_BOX_PAGE)
        .ok_or("search-box page missing")
}
