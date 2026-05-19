use super::StoryCatalog;
use super::StoryPageContract;
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
