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
fn badge_story_remains_passive_and_points_to_chip_for_dismiss() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let badge = examples
        .iter()
        .find(|it| it.page == "badge")
        .ok_or("badge page missing")?;
    let details = super::StoryDetailContent::from_example(badge);

    assert!(badge.callback_logs.is_empty());
    assert!(details.settings.contains("passive"));
    assert!(details.settings.contains("Chip"));
    Ok(())
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

#[test]
fn context_menu_story_exposes_detail_settings_and_callback_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let context_menu = examples
        .iter()
        .find(|it| it.page == "context-menu")
        .ok_or("context-menu page missing")?;
    let labels = page_children(&examples, "context-menu").ok_or("context-menu page missing")?;
    let details = super::StoryDetailContent::from_example(context_menu);

    for preset in StoryPresetLabels::for_page("context-menu") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "context-menu preview lacks preset {preset}"
        );
    }
    for setting in ["anchor=", "placement=", "item_kind=", "callback_log="] {
        assert!(
            details.settings.contains(setting),
            "context-menu settings inspector lacks {setting}"
        );
    }
    for action in [
        "context_menu_open",
        "context_menu_highlight",
        "context_menu_submenu",
        "context_menu_select",
    ] {
        assert!(
            context_menu
                .callback_logs
                .iter()
                .any(|it| it.action == action),
            "context-menu callback log lacks {action}"
        );
    }
    Ok(())
}

#[test]
fn closeable_tab_strip_story_exposes_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "closeable-tab-strip")
        .ok_or("closeable-tab-strip page missing")?;
    let labels = page_children(&examples, "closeable-tab-strip")
        .ok_or("closeable-tab-strip page missing")?;
    let details = super::StoryDetailContent::from_example(story);

    for preset in [
        "default", "overflow", "pinned", "groups", "dirty", "dragging",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "closeable-tab-strip preview lacks preset {preset}"
        );
        assert!(
            details.preset.contains(preset),
            "closeable-tab-strip details lack preset {preset}"
        );
    }
    for action in [
        "add_tab",
        "delete_tab",
        "pin_tab",
        "dirty_toggle",
        "group_toggle",
        "drag_tab",
        "overflow_open",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "closeable-tab-strip callback log lacks action {action}"
        );
        assert!(
            details.settings.contains(action),
            "closeable-tab-strip settings inspector lacks action {action}"
        );
    }
    for event in [
        "tab_added",
        "closeable_tab_closed",
        "closeable_tab_reordered",
        "closeable_tab_overflow_opened",
        "tab_dirty_changed",
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "closeable-tab-strip callback log lacks event {event}"
        );
    }
    Ok(())
}

#[test]
fn toolbar_story_exposes_overflow_split_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "toolbar")
        .ok_or("toolbar page missing")?;
    let details = super::StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "overflow menu",
            "split action",
            "display mode",
            "density",
            "accelerator"
        ],
        StoryPresetLabels::for_page("toolbar")
    );
    for setting in ["action", "priority", "overflow", "display", "density"] {
        assert!(
            details.settings.contains(setting),
            "toolbar settings inspector lacks {setting}"
        );
    }
    for action in ["toolbar_overflow_plan", "toolbar_split_open"] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "toolbar callback log lacks {action}"
        );
    }
    Ok(())
}

#[test]
fn chip_attachment_stories_expose_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let chip = examples
        .iter()
        .find(|it| it.page == "chip")
        .ok_or("chip page missing")?;
    let attachment = examples
        .iter()
        .find(|it| it.page == "attachment-chip")
        .ok_or("attachment-chip page missing")?;
    let group = examples
        .iter()
        .find(|it| it.page == "chip-group")
        .ok_or("chip-group page missing")?;
    let chip_details = super::StoryDetailContent::from_example(chip);
    let attachment_details = super::StoryDetailContent::from_example(attachment);
    let group_details = super::StoryDetailContent::from_example(group);

    assert_eq!(
        &["filter tag", "dismiss", "selected", "tone matrix"],
        StoryPresetLabels::for_page("chip")
    );
    assert_eq!(
        &[
            "file attachment",
            "image attachment",
            "url attachment",
            "uploading",
            "error retry"
        ],
        StoryPresetLabels::for_page("attachment-chip")
    );
    assert_eq!(
        &["wrap", "overflow menu", "horizontal scroll", "reorder"],
        StoryPresetLabels::for_page("chip-group")
    );
    for setting in ["variant", "tone", "size"] {
        assert!(
            chip_details.settings.contains(setting),
            "chip settings inspector lacks {setting}"
        );
    }
    for setting in ["status", "progress"] {
        assert!(
            attachment_details.settings.contains(setting),
            "attachment-chip settings inspector lacks {setting}"
        );
    }
    assert!(
        group_details.settings.contains("overflow"),
        "chip-group settings inspector lacks overflow"
    );
    assert!(
        chip.callback_logs
            .iter()
            .any(|it| it.action == "chip_dismiss")
    );
    assert!(
        attachment
            .callback_logs
            .iter()
            .any(|it| it.action == "attachment_status")
    );
    assert!(
        group
            .callback_logs
            .iter()
            .any(|it| it.action == "chip_group_overflow")
    );
    Ok(())
}

#[test]
fn diagnostics_list_story_exposes_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "diagnostics-list")
        .ok_or("diagnostics-list page missing")?;
    let details = super::StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "lint result",
            "editor inline",
            "tool result",
            "empty",
            "loading",
            "bulk fix"
        ],
        StoryPresetLabels::for_page("diagnostics-list")
    );
    for preset in StoryPresetLabels::for_page("diagnostics-list") {
        assert!(
            details.preset.contains(preset),
            "diagnostics-list detail preset lacks {preset}"
        );
    }
    for setting in [
        "group_by",
        "sort_by",
        "severity_filter",
        "bulk_action",
        "fix_preview",
    ] {
        assert!(
            details.settings.contains(setting),
            "diagnostics-list settings inspector lacks {setting}"
        );
    }
    for action in [
        "diagnostic_fix_preview",
        "diagnostic_bulk_preview",
        "diagnostic_select_error",
        "diagnostic_apply_fix",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "diagnostics-list callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn empty_state_story_exposes_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "empty-state")
        .ok_or("empty-state page missing")?;
    let details = super::StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "explorer empty",
            "search no result",
            "diagnostics clean",
            "history empty",
            "error fallback"
        ],
        StoryPresetLabels::for_page("empty-state")
    );
    for preset in StoryPresetLabels::for_page("empty-state") {
        assert!(
            details.preset.contains(preset),
            "empty-state detail preset lacks {preset}"
        );
    }
    for setting in ["tone", "size", "alignment", "actions"] {
        assert!(
            details.settings.contains(setting),
            "empty-state settings inspector lacks {setting}"
        );
    }
    for action in ["empty_state_primary", "empty_state_secondary"] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "empty-state callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn hover_card_story_exposes_rich_slots_and_callback_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "hover-card")
        .ok_or("hover-card page missing")?;
    let labels = page_children(&examples, "hover-card").ok_or("hover-card page missing")?;
    let details = super::StoryDetailContent::from_example(story);

    for expected in [
        "Anchor",
        "Heading: Capability",
        "Body: Shows rich hover and focus content",
        "Footer: Keeps open while the card is focused",
        "Configure",
    ] {
        assert!(
            labels.iter().any(|it| it == expected),
            "hover-card preview lacks {expected}"
        );
    }
    assert_eq!(
        &[
            "delayed open",
            "pointer follow",
            "focus trigger",
            "rich content",
            "actions"
        ],
        StoryPresetLabels::for_page("hover-card")
    );
    for action in ["hover_card_open", "hover_card_keep_open"] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "hover-card callback log lacks {action}"
        );
    }
    for setting in ["delay", "placement", "arrow", "focus", "slot"] {
        assert!(
            details.settings.contains(setting),
            "hover-card settings inspector lacks {setting}"
        );
    }
    Ok(())
}

#[test]
fn popover_story_exposes_arrow_slots_and_focus_presets() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "popover")
        .ok_or("popover page missing")?;
    let details = super::StoryDetailContent::from_example(story);

    assert_eq!(
        &["anchor", "arrow", "slots", "focus management"],
        StoryPresetLabels::for_page("popover")
    );
    for setting in ["placement", "arrow", "focus", "slot"] {
        assert!(
            details.settings.contains(setting),
            "popover settings inspector lacks {setting}"
        );
    }
    Ok(())
}

#[test]
fn catalog_contains_single_independent_context_menu_story() {
    let examples = StoryCatalog.examples();
    let count = examples
        .iter()
        .filter(|it| it.page == "context-menu")
        .count();

    assert_eq!(1, count);
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
            | UiNodeKind::Chip
            | UiNodeKind::Toggle
            | UiNodeKind::SlideControl
            | UiNodeKind::SvgButton
            | UiNodeKind::TextButton
            | UiNodeKind::IconTextButton
    )
}
