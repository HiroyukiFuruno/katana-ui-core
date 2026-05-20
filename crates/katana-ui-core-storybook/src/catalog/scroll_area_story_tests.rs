use super::{StoryCatalog, StoryDetailContent};
use crate::catalog::panel_interaction::StorybookPanelInteractionReport;
use katana_ui_core::render_model::{UiScrollAreaAxis, UiScrollbarPlacement, UiScrollbarVisibility};

const SCROLL_AREA_PAGE: &str = "scroll-area";

#[test]
fn scroll_area_story_exposes_typed_props_and_contract_sections() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = scroll_area_story(&examples)?;
    let props = story.tree.root().props();
    let labels = story
        .tree
        .root()
        .children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect::<Vec<_>>();
    let details = StoryDetailContent::from_example(story);

    assert_eq!(UiScrollAreaAxis::Both, props.scroll_area.axis);
    assert_eq!(40, props.scroll_area.offset_x);
    assert_eq!(180, props.scroll_area.offset_y);
    assert_eq!(320, props.scroll_area.viewport_width);
    assert_eq!(220, props.scroll_area.viewport_height);
    assert_eq!(860, props.scroll_area.content_width);
    assert_eq!(1400, props.scroll_area.content_height);
    assert_eq!(
        UiScrollbarVisibility::Always,
        props.scroll_area.scrollbar_visibility
    );
    assert_eq!(
        UiScrollbarPlacement::Reserved,
        props.scroll_area.scrollbar_placement
    );

    for expected in [
        "settings: axis offset viewport content scrollbar visibility placement edge_threshold",
        "state: offset=40,180 viewport=320x220 content=860x1400 edge=none",
        "event: Scrolled ScrollEdgeReached ScrollCommandRejected",
        "action: scroll_to scroll_by scroll_into_view scrollbar_visibility",
        "quality: nested_state_identity clamp edge_event axis_rejection",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(expected)),
            "scroll-area preview lacks {expected}"
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
            "scroll-area details lack {expected}"
        );
    }
    Ok(())
}

#[test]
fn scroll_area_settings_report_covers_typed_options() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    for option in [
        "scroll_area.axis",
        "scroll_area.offset",
        "scroll_area.viewport",
        "scroll_area.content",
        "scroll_area.scrollbar_visibility",
        "scroll_area.scrollbar_placement",
    ] {
        assert!(
            report.settings_mutations.iter().any(|it| {
                it.page == SCROLL_AREA_PAGE
                    && it.option.name == option
                    && it.event == "scroll_area_settings_changed"
            }),
            "missing scroll-area setting mutation for {option}"
        );
    }
}

fn scroll_area_story(
    examples: &[super::StoryExample],
) -> Result<&super::StoryExample, &'static str> {
    examples
        .iter()
        .find(|it| it.page == SCROLL_AREA_PAGE)
        .ok_or("scroll-area page missing")
}
