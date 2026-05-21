use super::*;

#[test]
fn modal_stories_expose_window_overlay_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let modal = examples
        .iter()
        .find(|it| it.page == "modal")
        .ok_or("modal page missing")?;
    let overlay = examples
        .iter()
        .find(|it| it.page == "modal-overlay")
        .ok_or("modal-overlay page missing")?;
    let modal_labels = page_children(&examples, "modal").ok_or("modal page missing")?;
    let overlay_labels =
        page_descendant_labels(&examples, "modal-overlay").ok_or("modal-overlay page missing")?;
    let modal_details = StoryDetailContent::from_example(modal);
    let overlay_details = StoryDetailContent::from_example(overlay);

    assert_eq!(
        &[
            "native window",
            "escape close",
            "focus return",
            "parent block",
            "title footer size"
        ],
        StoryPresetLabels::for_page("modal")
    );
    assert_eq!(
        &[
            "overlay dialog",
            "backdrop close",
            "escape close",
            "focus trap",
            "dismiss disabled"
        ],
        StoryPresetLabels::for_page("modal-overlay")
    );
    for preset in StoryPresetLabels::for_page("modal") {
        assert!(
            modal_labels.iter().any(|it| it.contains(preset)),
            "modal preview lacks preset {preset}"
        );
        assert!(
            modal_details.preset.contains(preset),
            "modal details lack preset {preset}"
        );
    }
    for preset in StoryPresetLabels::for_page("modal-overlay") {
        assert!(
            overlay_labels.iter().any(|it| it.contains(preset)),
            "modal-overlay preview lacks preset {preset}"
        );
        assert!(
            overlay_details.preset.contains(preset),
            "modal-overlay details lack preset {preset}"
        );
    }
    for setting in [
        "option=",
        "action=",
        "event=",
        "state=",
        "preset=",
        "native_window_mode=true",
        "parent_interaction=Block",
        "title=Preferences",
        "footer=Cancel / Save",
        "size=medium",
    ] {
        assert!(
            modal_details.settings.contains(setting),
            "modal settings inspector lacks {setting}"
        );
    }
    for setting in [
        "option=",
        "action=",
        "event=",
        "state=",
        "preset=",
        "same_window_overlay=true",
        "backdrop_close=true",
        "escape_close=true",
        "focus_trap=true",
        "dismiss_disabled=true",
    ] {
        assert!(
            overlay_details.settings.contains(setting),
            "modal-overlay settings inspector lacks {setting}"
        );
    }
    for action in ["modal_escape", "modal_focus_return", "modal_parent_block"] {
        assert!(
            modal.callback_logs.iter().any(|it| it.action == action),
            "modal callback log lacks action {action}"
        );
    }
    for action in [
        "modal_backdrop_click",
        "modal_escape",
        "modal_focus_trap",
        "modal_focus_return",
        "modal_dismiss_disabled",
    ] {
        assert!(
            overlay.callback_logs.iter().any(|it| it.action == action),
            "modal-overlay callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn skeleton_stories_expose_presets_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let skeleton = examples
        .iter()
        .find(|it| it.page == "skeleton")
        .ok_or("skeleton page missing")?;
    let cluster = examples
        .iter()
        .find(|it| it.page == "skeleton-cluster")
        .ok_or("skeleton-cluster page missing")?;
    let skeleton_labels = page_children(&examples, "skeleton").ok_or("skeleton page missing")?;
    let cluster_labels =
        page_children(&examples, "skeleton-cluster").ok_or("skeleton-cluster page missing")?;
    let skeleton_details = StoryDetailContent::from_example(skeleton);
    let cluster_details = StoryDetailContent::from_example(cluster);

    assert_eq!(
        &[
            "text lines",
            "avatar circle",
            "rect shimmer",
            "line wave",
            "reduced motion",
            "tone/radius"
        ],
        StoryPresetLabels::for_page("skeleton")
    );
    assert_eq!(
        &[
            "list loading",
            "message loading",
            "card loading",
            "paragraph loading",
            "code block loading",
            "image card loading"
        ],
        StoryPresetLabels::for_page("skeleton-cluster")
    );
    for preset in StoryPresetLabels::for_page("skeleton") {
        assert!(
            skeleton_labels.iter().any(|it| it.contains(preset)),
            "skeleton preview lacks preset {preset}"
        );
        assert!(
            skeleton_details.preset.contains(preset),
            "skeleton detail preset lacks {preset}"
        );
    }
    for preset in StoryPresetLabels::for_page("skeleton-cluster") {
        assert!(
            cluster_labels.iter().any(|it| it.contains(preset)),
            "skeleton-cluster preview lacks preset {preset}"
        );
        assert!(
            cluster_details.preset.contains(preset),
            "skeleton-cluster detail preset lacks {preset}"
        );
    }
    for setting in [
        "shape",
        "size",
        "animation",
        "tone",
        "radius",
        "reduced_motion",
        "accessibility_label",
    ] {
        assert!(
            skeleton_details.settings.contains(setting),
            "skeleton settings inspector lacks {setting}"
        );
    }
    for setting in ["preset", "children", "live_region", "reduced_motion"] {
        assert!(
            cluster_details.settings.contains(setting),
            "skeleton-cluster settings inspector lacks {setting}"
        );
    }
    for action in ["skeleton_animation_changed", "skeleton_shape_changed"] {
        assert!(
            skeleton.callback_logs.iter().any(|it| it.action == action),
            "skeleton callback log lacks action {action}"
        );
    }
    for action in ["skeleton_cluster_preset_apply", "skeleton_cluster_changed"] {
        assert!(
            cluster.callback_logs.iter().any(|it| it.action == action),
            "skeleton-cluster callback log lacks action {action}"
        );
    }
    Ok(())
}
