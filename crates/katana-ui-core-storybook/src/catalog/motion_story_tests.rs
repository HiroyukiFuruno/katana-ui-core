use super::{StoryCatalog, StoryDetailContent, StoryPresetLabels, StorybookPanelInteractionReport};

#[test]
fn motion_story_exposes_primitives_reduced_policy_and_overrides() {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "motion")
        .expect("motion story must exist");
    let detail = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "4 primitive",
            "reduced-motion respect",
            "force ignore",
            "per-molecule motion",
        ],
        StoryPresetLabels::for_page("motion")
    );
    for token in [
        "primitive: Fade Slide Scale Shimmer",
        "tokens: duration=Default easing=Emphasized distance=Default",
        "state: instant=false duration=200 distance=8",
        "event: reduced_motion_query override=Ignore context=Storybook",
        "action: motion_reduce motion_tick motion_force motion_ignore motion_override",
        "quality: token_resolution reduced_static override_isolated",
    ] {
        assert!(
            detail.settings.contains(token)
                || story
                    .tree
                    .root()
                    .children()
                    .iter()
                    .any(|it| it.props().label.contains(token)),
            "motion story lacks {token}"
        );
    }
    for action in [
        "motion_reduce",
        "motion_tick",
        "motion_force",
        "motion_ignore",
        "motion_override",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "motion callback log lacks {action}"
        );
    }
}

#[test]
fn motion_settings_are_switchable_from_storybook_report() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    for option in [
        "motion.primitive",
        "motion.duration",
        "motion.easing",
        "motion.distance",
        "motion.reduced_policy",
        "motion.disable_context",
    ] {
        assert!(
            report.settings_mutations.iter().any(|it| {
                it.page == "motion"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "motion_settings_changed"
            }),
            "missing motion setting mutation for {option}"
        );
    }
}
