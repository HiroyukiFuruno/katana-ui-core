use katana_ui_core::atom::{Button, Checkbox, ImageSurface, Input, Text};
use katana_ui_core::layout::{Column, Row, Stack};
use katana_ui_core::molecule::{
    Accordion, ContextMenu, ContextMenuItem, ContextMenuItemKind, Toolbar,
};
use katana_ui_core::render_model::{
    UI_DISCLOSURE_TOGGLE_ACTION_ID, UI_IMAGE_HIGHLIGHT_ACTION_ID, UI_LINK_OPEN_ACTION_ID,
    UI_SETTINGS_FIELD_ACTIVATE_ACTION_ID, UI_SETTINGS_SECTION_TOGGLE_ACTION_ID,
    UI_TASK_TOGGLE_ACTION_ID, UI_TREE_ROW_ACTION_ID, UiContextMenuItem, UiContextMenuItemKind,
    UiContextMenuProps, UiHostActionKind, UiHostActionPlan, UiHostActionSpec,
    UiImageSurfaceHighlight, UiNode, UiNodeKind, UiRect, UiSlotActionSpec, UiSlotPlacement,
    UiSlotSpec, UiTaskMarker, UiTextEntryProps, UiTextSpan, UiTextSpanAction, UiTextSpanStyle,
    UiTree, UiTreeRowActionKind,
};

const APP_TOOLBAR_COPY: &str = "app.toolbar.copy";
const APP_SEARCH_CLEAR: &str = "app.search.clear";
const UI_SURFACE_PRIMARY: &str = "ui.surface.primary";
const UI_SURFACE_FIT: &str = "ui.surface.fit";
const UI_SURFACE_ZOOM_IN: &str = "ui.surface.zoom_in";
const UI_SURFACE_SECONDARY_FIT: &str = "ui.surface.secondary_fit";
const UI_SURFACE_SECONDARY_ZOOM: &str = "ui.surface.secondary_zoom";
const UI_SURFACE_FULLSCREEN: &str = "ui.surface.fullscreen";
const MENU_COPY: &str = "menu.copy";
const MENU_TABLE: &str = "menu.table";
const MENU_WRAP: &str = "menu.wrap";
const MENU_DISABLED: &str = "menu.disabled";

#[test]
fn generic_host_action_plan_collects_action_ids_and_enabled_state() -> Result<(), String> {
    let tree = UiTree::new(
        Column::new()
            .child(Toolbar::new("Media").child(Button::new("Copy").command(APP_TOOLBAR_COPY)))
            .child(Input::new("Search").trailing_svg_icon_button(
                "Clear",
                "<svg />",
                APP_SEARCH_CLEAR,
            ))
            .child(Text::new("Link").text_spans(vec![link_span()]))
            .child(Accordion::new("Details").open(true))
            .child(image_surface()?)
            .child(surface_controls_node())
            .child(surface_zoom_node())
            .child(surface_fit_node())
            .child(secondary_surface_fit_node())
            .child(secondary_surface_zoom_node())
            .child(surface_fullscreen_node()),
    );

    let actions = UiHostActionPlan::collect_from_tree(&tree);

    assert!(has_enabled(&actions, APP_TOOLBAR_COPY));
    assert!(has_enabled(&actions, APP_SEARCH_CLEAR));
    assert!(has_enabled(&actions, UI_LINK_OPEN_ACTION_ID));
    assert!(has_enabled(&actions, UI_DISCLOSURE_TOGGLE_ACTION_ID));
    assert!(has_enabled(&actions, UI_IMAGE_HIGHLIGHT_ACTION_ID));
    assert!(has_disabled(&actions, UI_SURFACE_PRIMARY));
    assert!(has_enabled_kind(
        &actions,
        UI_SURFACE_ZOOM_IN,
        UiHostActionKind::SurfaceControl
    ));
    assert!(has_enabled_kind(
        &actions,
        UI_SURFACE_FIT,
        UiHostActionKind::SurfaceControl
    ));
    assert!(has_enabled_kind(
        &actions,
        UI_SURFACE_SECONDARY_FIT,
        UiHostActionKind::SurfaceControl
    ));
    assert!(has_enabled_kind(
        &actions,
        UI_SURFACE_SECONDARY_ZOOM,
        UiHostActionKind::SurfaceControl
    ));
    assert!(has_enabled_kind(
        &actions,
        UI_SURFACE_FULLSCREEN,
        UiHostActionKind::SurfaceControl
    ));
    Ok(())
}

#[test]
fn text_entry_host_actions_ignore_missing_and_empty_callbacks() {
    let no_action = UiSlotSpec::new(UiSlotPlacement::Trailing, "plain");
    let mut empty_callback = UiSlotSpec::new(UiSlotPlacement::Trailing, "empty");
    empty_callback.action = Some(UiSlotActionSpec::new("empty", ""));
    let node = UiNode::new(UiNodeKind::Input, "input").text_entry(UiTextEntryProps {
        trailing_icon_buttons: vec![no_action, empty_callback],
        ..UiTextEntryProps::default()
    });

    assert!(UiHostActionPlan::collect_from_node(&node).is_empty());
}

#[test]
fn disabled_nodes_keep_host_action_ids_but_mark_them_disabled() {
    let tree = UiTree::new(Button::new("Copy").command(APP_TOOLBAR_COPY).disabled(true));

    let actions = UiHostActionPlan::collect_from_tree(&tree);

    assert!(has_disabled(&actions, APP_TOOLBAR_COPY));
}

#[test]
fn context_menu_items_are_collected_as_host_actions() {
    let tree = UiTree::new(
        ContextMenu::new("Editor menu")
            .item(ContextMenuItem::action(MENU_COPY, "Copy"))
            .item(
                ContextMenuItem::new("insert", "Insert", ContextMenuItemKind::Submenu)
                    .child(ContextMenuItem::action(MENU_TABLE, "Table")),
            )
            .item(
                ContextMenuItem::new(MENU_WRAP, "Wrap lines", ContextMenuItemKind::Toggle)
                    .checked(true),
            )
            .item(ContextMenuItem::action(MENU_DISABLED, "Disabled").disabled(true)),
    );

    let actions = UiHostActionPlan::collect_from_tree(&tree);

    assert!(has_enabled_kind(
        &actions,
        MENU_COPY,
        UiHostActionKind::Command
    ));
    assert!(has_enabled_kind(
        &actions,
        MENU_TABLE,
        UiHostActionKind::Command
    ));
    assert!(has_enabled_kind(
        &actions,
        MENU_WRAP,
        UiHostActionKind::Custom
    ));
    assert!(has_disabled(&actions, MENU_DISABLED));
    assert!(has_payload(&actions, MENU_TABLE, "path=1/0 kind=action"));
}

#[test]
fn task_control_action_is_typed_from_host_action_plan() {
    let plan = UiHostActionPlan::new(
        "checkbox".into(),
        UiHostActionSpec::task_control("Toggle task", "list-node", 2),
    );

    assert!(plan.payload.is_empty());
    let target = plan.task_control_target();
    assert!(
        target.is_some(),
        "task host action should expose typed target"
    );
    let Some(target) = target else {
        return;
    };
    assert_eq!("list-node", target.node_id);
    assert_eq!(2, target.row_index);
    assert_eq!("ui-task-state:list-node:2", target.state_id);

    let action = plan.task_control_action("[/]");
    assert!(
        action.is_some(),
        "task host action should become typed task action"
    );
    let Some(action) = action else {
        return;
    };

    assert_eq!("list-node", action.node_id);
    assert_eq!(2, action.row_index);
    assert_eq!(UiTaskMarker::Progress, action.current_marker);
    assert_eq!("ui-task-state:list-node:2", action.state_id);
    assert_eq!(
        "ui.task.state.progress",
        action.current_marker.context_menu_item_id()
    );
    assert_eq!(
        Some(UiTaskMarker::Progress),
        UiTaskMarker::from_context_menu_item_id("ui.task.state.progress")
    );
    assert!(action.menu_items.is_empty());
}

#[test]
fn task_control_action_rejects_legacy_string_payload_contract() {
    let plan = UiHostActionPlan::new(
        "checkbox".into(),
        UiHostActionSpec::command(UI_TASK_TOGGLE_ACTION_ID, "Toggle task").payload("list-node:2"),
    );

    assert!(plan.task_control_target().is_none());
    assert!(plan.task_control_action("[/]").is_none());
}

#[test]
fn task_control_action_from_root_includes_current_marker_and_menu_items() {
    let tree = UiTree::new(
        UiNode::from(Checkbox::new("Doing").checked(true).value("[/]"))
            .context_menu(task_context_menu())
            .host_action(UiHostActionSpec::task_control(
                "Toggle task",
                "list-node",
                2,
            )),
    );
    let plan = UiHostActionPlan::collect_from_tree(&tree)
        .into_iter()
        .find(|action| action.action_id == UI_TASK_TOGGLE_ACTION_ID);
    assert!(plan.is_some(), "task host action");
    let Some(plan) = plan else {
        return;
    };

    let action = plan.task_control_action_from_root(tree.root());
    assert!(action.is_some(), "task action should use KUC root contract");
    let Some(action) = action else {
        return;
    };

    assert_eq!("list-node", action.node_id);
    assert_eq!(2, action.row_index);
    assert_eq!(UiTaskMarker::Progress, action.current_marker);
    assert_eq!(4, action.menu_items.len());
    assert_eq!("ui.task.state.progress", action.menu_items[2].item_id);
    assert_eq!(UiTaskMarker::Progress, action.menu_items[2].marker);
    assert!(action.menu_items[2].checked);
}

#[test]
fn task_control_action_from_root_uses_child_checkbox_contract_for_row_actions() {
    let checkbox = UiNode::from(Checkbox::new("").checked(false).value("[ ]"))
        .context_menu(task_context_menu());
    let tree = UiTree::new(
        UiNode::from(Row::new().child(checkbox).child(Text::new("Nested task"))).host_action(
            UiHostActionSpec::task_control("Toggle task", "list-node", 4),
        ),
    );
    let plan = UiHostActionPlan::collect_from_tree(&tree)
        .into_iter()
        .find(|action| action.action_id == UI_TASK_TOGGLE_ACTION_ID);
    assert!(plan.is_some(), "task host action");
    let Some(plan) = plan else {
        return;
    };

    let action = plan.task_control_action_from_root(tree.root());
    assert!(
        action.is_some(),
        "row action should use child checkbox KUC contract"
    );
    let Some(action) = action else {
        return;
    };

    assert_eq!("list-node", action.node_id);
    assert_eq!(4, action.row_index);
    assert_eq!(UiTaskMarker::Empty, action.current_marker);
    assert_eq!(4, action.menu_items.len());
}

#[test]
fn text_span_action_is_typed_from_host_action_plan() {
    let tree = UiTree::new(
        Column::new()
            .child(Text::new("Link").text_spans(vec![link_span()]))
            .child(Accordion::new("Details").open(true))
            .child(
                UiNode::from(Stack::new().child(Text::new("code"))).host_action(
                    UiHostActionSpec::surface_control("ui.code.copy", "Copy").payload("code-node"),
                ),
            ),
    );

    let actions = UiHostActionPlan::collect_from_tree(&tree)
        .into_iter()
        .filter_map(|action| action.text_span_action())
        .collect::<Vec<_>>();

    assert!(actions.contains(&UiTextSpanAction::OpenLink {
        target: "https://example.test/release".to_string(),
    }));
    assert!(actions.iter().any(|action| {
        matches!(
            action,
            UiTextSpanAction::ToggleAccordion { node_id, open: true }
                if !node_id.is_empty()
        )
    }));
    assert!(actions.iter().any(|action| {
        action
            .accordion_toggle_action()
            .is_some_and(|toggle| !toggle.node_id.is_empty() && !toggle.requested_open)
    }));
    assert!(actions.contains(&UiTextSpanAction::CopyCode {
        node_id: "code-node".to_string(),
    }));
}

#[test]
fn typed_host_action_accessors_reject_wrong_ids_payloads_and_text_contracts() {
    let field = UiHostActionPlan::new(
        "settings".into(),
        UiHostActionSpec::settings_field_control("Theme", "theme"),
    );
    assert_eq!(
        Some("theme"),
        field
            .settings_field_control_target()
            .as_ref()
            .map(|target| target.field_id.as_str())
    );
    let mut wrong_field_id = field.clone();
    wrong_field_id.action_id = "settings.wrong".to_string();
    assert!(wrong_field_id.settings_field_control_target().is_none());
    let wrong_field_payload = UiHostActionPlan::new(
        "settings".into(),
        UiHostActionSpec::command(UI_SETTINGS_FIELD_ACTIVATE_ACTION_ID, "Theme"),
    );
    assert!(
        wrong_field_payload
            .settings_field_control_target()
            .is_none()
    );

    let section = UiHostActionPlan::new(
        "settings".into(),
        UiHostActionSpec::settings_section_toggle("Appearance", "appearance"),
    );
    assert_eq!(
        Some("appearance"),
        section
            .settings_section_toggle_target()
            .as_ref()
            .map(|target| target.section_id.as_str())
    );
    let mut wrong_section_id = section.clone();
    wrong_section_id.action_id = "settings.wrong".to_string();
    assert!(wrong_section_id.settings_section_toggle_target().is_none());
    let wrong_section_payload = UiHostActionPlan::new(
        "settings".into(),
        UiHostActionSpec::command(UI_SETTINGS_SECTION_TOGGLE_ACTION_ID, "Appearance"),
    );
    assert!(
        wrong_section_payload
            .settings_section_toggle_target()
            .is_none()
    );

    let tree_row = UiHostActionPlan::new(
        "tree".into(),
        UiHostActionSpec::tree_row("README", "readme", UiTreeRowActionKind::Select),
    );
    assert_eq!(
        Some("readme"),
        tree_row
            .tree_row_action_target()
            .as_ref()
            .map(|target| target.node_id.as_str())
    );
    let wrong_tree_payload = UiHostActionPlan::new(
        "tree".into(),
        UiHostActionSpec::command(UI_TREE_ROW_ACTION_ID, "README"),
    );
    assert!(wrong_tree_payload.tree_row_action_target().is_none());
    let mut wrong_tree_id = tree_row.clone();
    wrong_tree_id.action_id = "tree.wrong".to_string();
    assert!(wrong_tree_id.tree_row_action_target().is_none());

    let empty_link = UiHostActionPlan::new(
        "text".into(),
        UiHostActionSpec::command(UI_LINK_OPEN_ACTION_ID, "Empty link"),
    );
    assert!(empty_link.text_span_action().is_none());
    let invalid_accordion = UiHostActionPlan::new(
        "accordion".into(),
        UiHostActionSpec::command(UI_DISCLOSURE_TOGGLE_ACTION_ID, "Invalid")
            .payload("open=invalid"),
    );
    assert!(invalid_accordion.text_span_action().is_none());
    let unknown = UiHostActionPlan::new(
        "text".into(),
        UiHostActionSpec::command("ui.unknown", "Unknown"),
    );
    assert!(unknown.text_span_action().is_none());
    assert!(
        UiTextSpanAction::OpenLink {
            target: "https://example.test".to_string(),
        }
        .accordion_toggle_action()
        .is_none()
    );
}

#[test]
fn blank_text_links_and_non_dispatching_context_menu_children_are_ignored() {
    let mut blank_link = UiTextSpan::plain("blank");
    blank_link.link_target = "   ".to_string();
    let tree = UiTree::new(
        Column::new()
            .child(Text::new("Blank link").text_spans(vec![blank_link]))
            .child(
                ContextMenu::new("Menu").item(
                    ContextMenuItem::new("section", "Section", ContextMenuItemKind::Section)
                        .child(ContextMenuItem::action("hidden", "Hidden")),
                ),
            ),
    );

    let actions = UiHostActionPlan::collect_from_tree(&tree);
    assert!(
        !actions
            .iter()
            .any(|action| action.action_id == UI_LINK_OPEN_ACTION_ID)
    );
    assert!(!actions.iter().any(|action| action.action_id == "hidden"));
}

#[test]
fn accordion_text_action_exposes_requested_open_without_consumer_inversion() -> Result<(), String> {
    let closed = UiTree::new(Accordion::new("Closed").open(false));
    let open = UiTree::new(Accordion::new("Open").open(true));

    let closed_action = accordion_toggle_action(&closed)?;
    let open_action = accordion_toggle_action(&open)?;

    assert!(closed_action.requested_open);
    assert!(!open_action.requested_open);
    Ok(())
}

fn link_span() -> UiTextSpan {
    UiTextSpan {
        text: "release notes".to_string(),
        style: UiTextSpanStyle {
            underline: true,
            ..UiTextSpanStyle::default()
        },
        link_target: "https://example.test/release".to_string(),
    }
}

fn accordion_toggle_action(
    tree: &UiTree,
) -> Result<katana_ui_core::render_model::UiAccordionToggleAction, String> {
    UiHostActionPlan::collect_from_tree(tree)
        .into_iter()
        .filter_map(|action| action.text_span_action())
        .find_map(|action| action.accordion_toggle_action())
        .ok_or_else(|| "accordion toggle action missing".to_string())
}

fn image_surface() -> Result<ImageSurface, String> {
    ImageSurface::from_rgba("Preview", "sha", 1, 1, vec![0, 0, 0, 255])
        .map(|surface| {
            surface.highlight_rect(UiImageSurfaceHighlight::current_search_hit(
                UiRect::new(1, 2, 3, 4),
                "current search hit",
            ))
        })
        .map_err(|error| error.to_string())
}

fn surface_controls_node() -> UiNode {
    UiNode::from(Stack::new().child(Text::new("surface"))).host_action(
        UiHostActionSpec::surface_control(UI_SURFACE_PRIMARY, "Primary").enabled(false),
    )
}

fn surface_zoom_node() -> UiNode {
    UiNode::from(Stack::new().child(Text::new("surface"))).host_action(
        UiHostActionSpec::surface_control(UI_SURFACE_ZOOM_IN, "Zoom in"),
    )
}

fn surface_fit_node() -> UiNode {
    UiNode::from(Stack::new().child(Text::new("surface"))).host_action(
        UiHostActionSpec::surface_control(UI_SURFACE_FIT, "Fit surface"),
    )
}

fn secondary_surface_fit_node() -> UiNode {
    UiNode::from(Stack::new().child(Text::new("surface"))).host_action(
        UiHostActionSpec::surface_control(UI_SURFACE_SECONDARY_FIT, "Fit surface"),
    )
}

fn secondary_surface_zoom_node() -> UiNode {
    UiNode::from(Stack::new().child(Text::new("surface"))).host_action(
        UiHostActionSpec::surface_control(UI_SURFACE_SECONDARY_ZOOM, "Zoom surface"),
    )
}

fn surface_fullscreen_node() -> UiNode {
    UiNode::from(Stack::new().child(Text::new("surface"))).host_action(
        UiHostActionSpec::surface_control(UI_SURFACE_FULLSCREEN, "Fullscreen"),
    )
}

fn task_context_menu() -> UiContextMenuProps {
    UiContextMenuProps {
        items: UiTaskMarker::ALL
            .into_iter()
            .map(|marker| {
                UiContextMenuItem::new(
                    marker.context_menu_item_id(),
                    marker.marker(),
                    UiContextMenuItemKind::Radio,
                )
                .checked(marker == UiTaskMarker::Progress)
            })
            .collect(),
        ..UiContextMenuProps::default()
    }
}

fn has_enabled(actions: &[UiHostActionPlan], action_id: &str) -> bool {
    actions
        .iter()
        .any(|action| action.action_id == action_id && action.enabled)
}

fn has_disabled(actions: &[UiHostActionPlan], action_id: &str) -> bool {
    actions
        .iter()
        .any(|action| action.action_id == action_id && !action.enabled)
}

fn has_enabled_kind(actions: &[UiHostActionPlan], action_id: &str, kind: UiHostActionKind) -> bool {
    actions
        .iter()
        .any(|action| action.action_id == action_id && action.enabled && action.kind == kind)
}

fn has_payload(actions: &[UiHostActionPlan], action_id: &str, payload: &str) -> bool {
    actions
        .iter()
        .any(|action| action.action_id == action_id && action.payload == payload)
}
