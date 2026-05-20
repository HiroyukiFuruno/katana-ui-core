use super::StoryPageContract;
use super::{StoryCatalog, StoryPresetLabels};
use katana_ui_core::render_model::{UiNodeKind, UiVisualRole};
use katana_ui_core::{atom, render_model::UiTree};

#[test]
fn atom_examples_use_typed_props_without_type_classes() {
    let examples = StoryCatalog.examples();
    let atoms = examples
        .iter()
        .filter(|it| is_atom_kind(it.tree.root().kind()));

    for example in atoms {
        let props = example.tree.root().props();
        assert!(props.style_classes.is_empty(), "{}", example.page);
    }
    let key_cap = examples.iter().find(|it| it.page == "key-cap");
    assert!(key_cap.is_some(), "key-cap story is required");
    let key_cap_props = key_cap.map(|it| it.tree.root().props());
    assert_eq!(
        Some(UiVisualRole::Shortcut),
        key_cap_props.map(|it| it.visual_role)
    );
    assert_eq!(Some("code"), key_cap_props.map(|it| it.font_role.as_str()));
}

#[test]
fn interactive_atom_examples_expose_callback_logs() {
    let examples = StoryCatalog.examples();
    let log_pages: Vec<&str> = examples
        .iter()
        .filter(|it| !it.callback_logs.is_empty())
        .map(|it| it.page)
        .collect();

    assert!(log_pages.contains(&"button"));
    assert!(log_pages.contains(&"text-input"));
    assert!(log_pages.contains(&"checkbox"));
    assert!(log_pages.contains(&"toggle"));
}

#[test]
fn story_page_contract_is_derived_from_materialized_evidence() {
    let incomplete =
        StoryPageContract::from_tree("button", &UiTree::new(atom::Button::new("Button")), 99, &[]);
    let passive = StoryPageContract::from_tree(
        "divider",
        &UiTree::new(atom::Divider::new("Divider")),
        1,
        &[],
    );

    assert!(!incomplete.preview);
    assert!(!incomplete.action_history);
    assert!(!incomplete.event_history);
    assert!(!incomplete.is_complete());
    assert!(passive.action_history);
    assert!(passive.event_history);
}

#[test]
fn color_picker_and_code_diff_stories_materialize_required_controls() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let color_picker =
        page_children(&examples, "color-picker-rgba").ok_or("color picker page missing")?;
    let code_diff = page_children(&examples, "code-diff").ok_or("code diff page missing")?;

    assert!(color_picker.iter().any(|it| it.contains("trigger")));
    assert!(color_picker.iter().any(|it| it.contains("floating")));
    assert!(color_picker.iter().any(|it| it.contains("R=64")));
    assert!(code_diff.iter().any(|it| it.contains("split / inline")));
    assert!(code_diff.iter().any(|it| it.contains("collapse")));
    assert!(code_diff.iter().any(|it| it.contains("日本語")));
    Ok(())
}

#[test]
fn color_picker_and_code_diff_presets_are_dod_specific() {
    assert_eq!(
        &[
            "rgba panel",
            "color trigger",
            "size presets",
            "borderless",
            "floating panel"
        ],
        StoryPresetLabels::for_page("color-picker-rgba")
    );
    assert_eq!(
        &[
            "split left-right",
            "split top-bottom",
            "inline",
            "collapsed",
            "japanese whitespace"
        ],
        StoryPresetLabels::for_page("code-diff")
    );
}

#[test]
fn drag_and_drop_story_exposes_preset_specific_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let drag_and_drop = examples
        .iter()
        .find(|it| it.page == "drag-and-drop")
        .ok_or("drag-and-drop page missing")?;
    let labels = page_children(&examples, "drag-and-drop").ok_or("drag-and-drop page missing")?;
    let details = super::StoryDetailContent::from_example(drag_and_drop);

    assert_eq!(5, labels.len());
    for preset in StoryPresetLabels::for_page("drag-and-drop") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "drag-and-drop preview lacks preset {preset}"
        );
    }
    for setting in ["accept=", "autoscroll=", "keyboard_draggable="] {
        assert!(
            labels.iter().all(|it| it.contains(setting)),
            "drag-and-drop preview lacks setting {setting}"
        );
        assert!(
            drag_and_drop
                .callback_logs
                .iter()
                .all(|it| it.before.contains(setting) || it.after.contains(setting)),
            "drag-and-drop logs lack setting {setting}"
        );
    }
    assert!(details.settings.contains("accept="));
    assert!(details.settings.contains("autoscroll="));
    assert!(details.settings.contains("keyboard_draggable="));
    for event in [
        "DragStart",
        "DragMove",
        "DragEnter",
        "Drop",
        "DragCancel",
        "DragEnd",
    ] {
        assert!(
            drag_and_drop
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "drag-and-drop logs lack event {event}"
        );
    }
    assert!(
        drag_and_drop
            .callback_logs
            .iter()
            .any(|it| it.action == "file_drop_accept" && it.after.contains("os/file-list")),
        "file drop log must expose the OS file-list payload"
    );
    Ok(())
}

fn page_children(examples: &[super::StoryExample], page: &str) -> Option<Vec<String>> {
    examples.iter().find(|it| it.page == page).map(|it| {
        it.tree
            .root()
            .children()
            .iter()
            .map(|child| child.props().label.clone())
            .collect()
    })
}

fn is_atom_kind(kind: UiNodeKind) -> bool {
    matches!(
        kind,
        UiNodeKind::Text
            | UiNodeKind::Icon
            | UiNodeKind::Button
            | UiNodeKind::Input
            | UiNodeKind::Checkbox
            | UiNodeKind::Radio
            | UiNodeKind::Badge
            | UiNodeKind::Divider
            | UiNodeKind::Spacer
            | UiNodeKind::KeyCap
            | UiNodeKind::LoadingDots
            | UiNodeKind::Spinner
            | UiNodeKind::ProgressBar
            | UiNodeKind::ColorSwatch
            | UiNodeKind::Toggle
            | UiNodeKind::SlideControl
            | UiNodeKind::SvgButton
            | UiNodeKind::TextButton
            | UiNodeKind::IconTextButton
    )
}
