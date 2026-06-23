use katana_ui_core::component::{ComponentAction, ComponentTree};
use katana_ui_core::interaction::UiAction;
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{UiNode, UiNodeId, UiNodeKind, UiStateId};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core::widget::atoms::{Button, Input, TextArea, TextAreaAction, TextAreaEvent};
use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabGroup, CloseableTabId, CloseableTabScrollConfig,
    CloseableTabScrollPlanner, CloseableTabStrip, CloseableTabStripAction, CloseableTabStripEvent,
    MeasuredCloseableTab,
};

#[path = "generic_rust_app_contract/support.rs"]
mod support;

#[test]
fn generic_rust_app_can_compose_shell_from_public_kuc_api() {
    let tree = generic_app_tree();
    let children = tree.root().children();

    assert_eq!(UiNodeKind::Panel, tree.root().kind());
    assert_eq!(4, children.len());
    assert_eq!(UiNodeKind::CloseableTabStrip, children[0].kind());
    assert_eq!(UiNodeKind::Input, children[1].kind());
    assert_eq!(UiNodeKind::TextArea, children[2].kind());
    assert_eq!(UiNodeKind::Button, children[3].kind());

    support::assert_search_input_contract(&children[1]);
    support::assert_text_area_contract(&children[2]);
    support::assert_workspace_tab_contract(&children[0]);
}

#[test]
fn generic_app_inputs_keep_internal_state_per_instance() {
    let mut search = Input::new("Search").value("initial");
    let title = Input::new("Title").value("title");
    let action = UiAction::input_value(search.state_id().clone(), "query");

    let result = search.apply_action(&action);

    assert!(result.handled);
    assert_eq!("query", search.state_snapshot().interaction.value);
    assert_eq!("title", title.state_snapshot().interaction.value);
    assert_ne!(search.state_id(), title.state_id());
}

#[test]
fn generic_app_can_rebuild_input_with_stable_state_identity() {
    let initial = Input::new("Search")
        .stable_state_id("generic.search.input")
        .value("src");
    let action = UiAction::input_value(initial.state_id().clone(), "query");
    let mut rebuilt = Input::new("Search")
        .stable_state_id("generic.search.input")
        .value("src");
    let mut other = Input::new("Other")
        .stable_state_id("generic.other.input")
        .value("untouched");

    let accepted = rebuilt.apply_action(&action);
    let ignored = other.apply_action(&action);

    assert!(accepted.handled);
    assert!(!ignored.handled);
    assert_eq!("query", rebuilt.state_snapshot().interaction.value);
    assert_eq!("untouched", other.state_snapshot().interaction.value);
    assert_eq!("generic.search.input", rebuilt.state_id().as_str());
}

#[test]
fn generic_app_can_rebuild_text_area_with_stable_state_identity() {
    let initial = TextArea::new("Notes")
        .stable_state_id(UiStateId::new("generic.notes.text-area"))
        .value("line 1");
    let action = UiAction::input_value(initial.state_id().clone(), "line 2");
    let mut rebuilt = TextArea::new("Notes")
        .stable_state_id("generic.notes.text-area")
        .value("line 1");

    let accepted = rebuilt.apply_action(&action);

    assert!(accepted.handled);
    assert_eq!("line 2", rebuilt.state().value);
    assert_eq!("generic.notes.text-area", rebuilt.state_id().as_str());
}

#[test]
fn generic_app_can_assign_stable_render_node_identity() {
    let node = UiNode::from(Input::new("Search"))
        .stable_node_id(UiNodeId::new("generic.search.node"))
        .stable_state_id("generic.search.state");

    assert_eq!("generic.search.node", node.id().as_str());
    assert_eq!("generic.search.state", node.props().state_id.as_str());
}

#[test]
fn generic_app_redraw_keeps_public_widget_state_ids_from_caller_contract() {
    let first = generic_app_tree_with_stable_ids();
    let second = generic_app_tree_with_stable_ids();
    let first_children = first.root().children();
    let second_children = second.root().children();

    assert_eq!("generic.shell", first.root().props().state_id.as_str());
    assert_eq!(
        first.root().props().state_id,
        second.root().props().state_id
    );
    assert_eq!("generic.tabs", first_children[0].props().state_id.as_str());
    assert_eq!(
        first_children[0].props().state_id,
        second_children[0].props().state_id
    );
    assert_eq!(
        "generic.search",
        first_children[1].props().state_id.as_str()
    );
    assert_eq!(
        first_children[1].props().state_id,
        second_children[1].props().state_id
    );
    assert_eq!("generic.notes", first_children[2].props().state_id.as_str());
    assert_eq!(
        first_children[2].props().state_id,
        second_children[2].props().state_id
    );
    assert_eq!("generic.save", first_children[3].props().state_id.as_str());
    assert_eq!(
        first_children[3].props().state_id,
        second_children[3].props().state_id
    );
}

#[test]
fn generic_app_closeable_tab_child_state_id_survives_reorder() -> Result<(), String> {
    let mut tabs = CloseableTabStrip::new("workspace")
        .stable_state_id("generic.tabs")
        .tab(CloseableTab::new("home", "Home"))
        .tab(CloseableTab::new("editor", "Editor"))
        .tab(CloseableTab::new("preview", "Preview"));
    let before = rendered_tab_state_id(&tabs, "Editor")?;

    tabs.apply_action(CloseableTabStripAction::MoveTab {
        tab_id: CloseableTabId::new("editor"),
        to_visual_index: 0,
    });
    let after = rendered_tab_state_id(&tabs, "Editor")?;

    assert_eq!("generic.tabs:closeable-tab:editor", before.as_str());
    assert_eq!(before, after);
    Ok(())
}

#[test]
fn generic_app_readonly_input_rejects_write_actions() {
    let mut readonly = Input::new("Readonly").value("locked").readonly(true);
    let action = UiAction::input_value(readonly.state_id().clone(), "changed");

    let result = readonly.apply_action(&action);

    assert!(!result.handled);
    assert_eq!("locked", readonly.state_snapshot().interaction.value);
}

#[test]
fn generic_app_readonly_input_allows_selection_without_write_mutation() {
    let mut readonly = Input::new("Readonly").value("locked").readonly(true);
    let selection = UiAction::cursor_selection(readonly.state_id().clone(), 4, 1, 4);

    let result = readonly.apply_action(&selection);

    assert!(result.handled);
    assert_eq!("locked", result.after.value);
    assert_eq!(4, result.after.cursor);
    assert_eq!(1, result.after.selection_start);
    assert_eq!(4, result.after.selection_end);
}

#[test]
fn generic_app_readonly_text_area_allows_selection_and_submit_without_write_mutation() {
    let mut readonly = TextArea::new("Readonly").value("locked").readonly(true);

    let write = readonly.apply_action(&UiAction::input_value(
        readonly.state_id().clone(),
        "changed",
    ));
    let selection = readonly.apply_action(&UiAction::cursor_selection(
        readonly.state_id().clone(),
        4,
        1,
        4,
    ));
    let submit = readonly.apply_text_area_action(TextAreaAction::Submit);

    assert!(!write.handled);
    assert!(selection.handled);
    assert!(submit.handled);
    assert_eq!("locked", readonly.state().value);
    assert_eq!(4, readonly.state().caret);
    assert_eq!(1, readonly.state().selection.start);
    assert_eq!(4, readonly.state().selection.end);
}

#[test]
fn generic_app_text_area_resize_is_typed_action() {
    let mut notes = support::notes_text_area();

    let resized = notes.apply_text_area_action(TextAreaAction::resize(32, 8));
    let node = UiNode::from(notes);

    assert!(resized.handled);
    assert_eq!(32, resized.state.resize_width_delta);
    assert_eq!(8, resized.state.resize_height_delta);
    assert!(resized.events.iter().any(|event| {
        matches!(
            event,
            TextAreaEvent::Resize(resize)
                if resize.width_delta == 32 && resize.height_delta == 8
        )
    }));
    assert_eq!(32, node.props().text_area.resize_width_delta);
    assert_eq!(8, node.props().text_area.resize_height_delta);
}

#[test]
fn generic_app_tabs_support_add_close_move_group_and_pin_contracts() {
    let mut tabs = CloseableTabStrip::new("workspace")
        .group(CloseableTabGroup::new("docs", "Docs"))
        .tab(CloseableTab::new("pinned", "Pinned").pinned(true))
        .tab(CloseableTab::new("draft", "Draft").group_id("docs"))
        .active_tab_id("draft");

    let added = tabs.apply_action(CloseableTabStripAction::AddTab {
        tab: CloseableTab::new("scratch", "Scratch"),
        activate: true,
    });
    tabs.apply_action(CloseableTabStripAction::MoveTab {
        tab_id: CloseableTabId::new("scratch"),
        to_visual_index: 2,
    });
    tabs.apply_action(CloseableTabStripAction::UnpinTab {
        tab_id: CloseableTabId::new("pinned"),
    });
    let closed = tabs.apply_action(CloseableTabStripAction::CloseTab {
        tab_id: CloseableTabId::new("pinned"),
    });

    assert_eq!(
        vec![CloseableTabStripEvent::TabAdded {
            tab_id: CloseableTabId::new("scratch")
        }],
        added
    );
    assert_eq!(
        vec![CloseableTabStripEvent::TabClosed {
            tab_id: CloseableTabId::new("pinned")
        }],
        closed
    );
    assert_eq!(2, tabs.options().tabs.len());
    assert_eq!(
        Some(&CloseableTabId::new("scratch")),
        tabs.state().active_tab_id.as_ref()
    );
}

#[test]
fn generic_app_tabs_can_follow_externally_selected_tab_by_scroll() {
    let measured = vec![
        MeasuredCloseableTab::new("home", 80),
        MeasuredCloseableTab::new("editor", 70),
        MeasuredCloseableTab::new("preview", 90),
        MeasuredCloseableTab::new("logs", 60),
    ];

    let plan = CloseableTabScrollPlanner::follow_active(
        CloseableTabScrollConfig::new(160, 0),
        &measured,
        Some(&CloseableTabId::new("preview")),
    );

    assert!(plan.overflow_scroll_enabled);
    assert!(plan.active_tab_visible);
    assert_eq!(80, plan.scroll_x);
}

fn generic_app_tree() -> katana_ui_core::render_model::UiTree {
    ComponentTree::new(
        Panel::new("generic app", PanelRegion::Root, ThemeSnapshot::dark())
            .child(support::generic_tabs())
            .child(support::search_input())
            .child(support::notes_text_area())
            .child(Button::new("Save").focusable(true)),
    )
    .into_tree()
}

fn generic_app_tree_with_stable_ids() -> katana_ui_core::render_model::UiTree {
    ComponentTree::new(
        Panel::new("generic app", PanelRegion::Root, ThemeSnapshot::dark())
            .stable_state_id("generic.shell")
            .child(
                CloseableTabStrip::new("workspace")
                    .stable_state_id("generic.tabs")
                    .tab(CloseableTab::new("home", "Home"))
                    .active_tab_id("home"),
            )
            .child(Input::new("Search").stable_state_id("generic.search"))
            .child(TextArea::new("Notes").stable_state_id("generic.notes"))
            .child(
                Button::new("Save")
                    .stable_state_id("generic.save")
                    .focusable(true),
            ),
    )
    .into_tree()
}

fn rendered_tab_state_id(tabs: &CloseableTabStrip, title: &str) -> Result<UiStateId, String> {
    UiNode::from(tabs.clone())
        .children()
        .iter()
        .find(|child| child.props().label == title)
        .map(|child| child.props().state_id.clone())
        .ok_or_else(|| format!("tab `{title}` must render"))
}
