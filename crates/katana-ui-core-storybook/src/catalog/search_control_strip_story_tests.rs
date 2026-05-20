use super::{StoryCatalog, StoryDetailContent, StoryPresetLabels};
use crate::catalog::panel_interaction::StorybookPanelInteractionReport;

const SEARCH_CONTROL_PAGE: &str = "search-control-strip";

#[test]
fn search_control_story_exposes_settings_state_event_action_and_quality() -> Result<(), &'static str>
{
    let examples = StoryCatalog.examples();
    let story = search_control_story(&examples)?;
    let details = StoryDetailContent::from_example(story);
    let labels = story
        .tree
        .root()
        .children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect::<Vec<_>>();

    for expected in [
        "settings: query match_case whole_word regex replace_mode result_count active_index",
        "state: query=heading match_case=true whole_word=true regex=true replace=title result=3 / 12",
        "event: SearchQueryChanged SearchOptionChanged SearchNavigationRequested ReplaceRequested",
        "action: query option navigate replace result-position",
        "quality: typed options state_id result_count event_contract",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(expected)),
            "search-control-strip preview lacks {expected}"
        );
        assert!(
            [
                details.settings.as_str(),
                details.state.as_str(),
                details.event.as_str(),
                details.action.as_str(),
                details.quality.as_str(),
            ]
            .iter()
            .any(|it| it.contains(expected)),
            "search-control-strip details lack {expected}"
        );
    }
    Ok(())
}

#[test]
fn search_control_story_has_component_specific_presets() {
    assert_eq!(
        &[
            "workspace search",
            "editor find",
            "editor replace",
            "viewer search",
            "history search"
        ],
        StoryPresetLabels::for_page(SEARCH_CONTROL_PAGE)
    );
}

#[test]
fn search_control_settings_report_covers_all_mutable_story_options() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    for option in [
        "search_control.query",
        "search_control.match_case",
        "search_control.whole_word",
        "search_control.use_regex",
        "search_control.replace_mode",
        "search_control.result_count",
        "search_control.active_index",
    ] {
        assert!(
            report.settings_mutations.iter().any(|it| {
                it.page == SEARCH_CONTROL_PAGE
                    && it.option.name == option
                    && it.event == "search_control_strip_settings_changed"
            }),
            "missing search-control-strip setting mutation for {option}"
        );
    }
}

fn search_control_story(
    examples: &[super::StoryExample],
) -> Result<&super::StoryExample, &'static str> {
    examples
        .iter()
        .find(|it| it.page == SEARCH_CONTROL_PAGE)
        .ok_or("search-control-strip page missing")
}
