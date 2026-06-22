use super::{StoryCatalog, StoryDetailContent, StoryPresetLabels, StorybookPanelInteractionReport};

#[test]
fn split_pane_story_exposes_typed_contract_surface() -> Result<(), String> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "split-pane")
        .ok_or_else(|| "split-pane story must exist".to_string())?;
    let detail = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "axis vertical",
            "wide gap",
            "center alignment",
            "overflow scroll",
            "ratio percent",
            "min percent clamp",
            "max percent clamp",
            "reset percent",
            "wide handle",
            "keyboard resize mode",
        ],
        StoryPresetLabels::for_page("split-pane")
    );
    for token in [
        "settings: axis gap alignment overflow ratio_percent min_percent max_percent reset_percent handle_width_px resize_mode",
        "state: ratio=50 dragging=false focused_handle=false last_event=RatioChanged",
        "event: ResizeStarted RatioChanged ResizeEnded ResizeRejected",
        "action: split_pane_set_ratio split_pane_resize_by split_pane_reset_ratio",
        "quality: clamp event_order public_api_guard",
    ] {
        assert!(
            detail.settings.contains(token)
                || story
                    .tree
                    .root()
                    .children()
                    .iter()
                    .any(|it| it.props().label.contains(token)),
            "split-pane story lacks {token}"
        );
    }
    for action in [
        "split_pane_start_resize",
        "split_pane_set_ratio",
        "split_pane_resize_by",
        "split_pane_reset_ratio",
        "split_pane_end_resize",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "split-pane callback log lacks {action}"
        );
    }
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.after.contains("ResizeRejected")),
        "split-pane callback log lacks ResizeRejected event"
    );
    Ok(())
}

#[test]
fn split_pane_settings_are_switchable_from_storybook_report() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    for option in [
        "split_pane.axis",
        "split_pane.gap",
        "split_pane.alignment",
        "split_pane.overflow",
        "split_pane.ratio_percent",
        "split_pane.min_percent",
        "split_pane.max_percent",
        "split_pane.reset_percent",
        "split_pane.handle_width_px",
        "split_pane.resize_mode",
    ] {
        assert!(
            report.settings_mutations.iter().any(|it| {
                it.page == "split-pane"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "split_pane_settings_changed"
            }),
            "missing split-pane setting mutation for {option}"
        );
    }
}
