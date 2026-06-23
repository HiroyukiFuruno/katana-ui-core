use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    ArrayEditorItem, CommandItem, CommandPalette, DisclosureTriggerArea, DynamicArrayEditor,
    TreeLineStyle, TreeNode, TreeNodeKind, TreeView,
};
use katana_ui_core::render_model::UiTree;
use katana_ui_core::render_model::{UiTreeLineStyle, UiTreeNodeKind, UiTreeToggleTriggerArea};

const ITEM_COUNT: usize = 1;

#[test]
fn tree_command_palette_and_array_editor_have_dedicated_item_models() {
    let tree = TreeView::new("Tree")
        .active("root")
        .line_display(true)
        .line_style(TreeLineStyle::Dashed)
        .line_width(2)
        .icons_visible(true)
        .directory_icon("<svg data-icon=\"folder\"/>")
        .file_icon("<svg data-icon=\"file\"/>")
        .tree_font_role("body")
        .tree_theme_id("katana-dark")
        .empty_area_context_menu(true)
        .default_open(true)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .toggle_trigger_area(DisclosureTriggerArea::IconAndText)
        .item(
            TreeNode::new("root", "Root", 0)
                .directory()
                .expanded(true)
                .selected(true)
                .active(true),
        );
    let palette = CommandPalette::new("Commands")
        .query("format")
        .filtered_action(CommandItem::new("format", "Format Document"))
        .keyboard_action("enter")
        .item(CommandItem::new("format", "Format Document").shortcut("Cmd Shift F"));
    let editor = DynamicArrayEditor::new("Rows")
        .add_action("append")
        .delete_action("delete-row")
        .reorder_action("move-row")
        .edit_action("edit-value")
        .empty_state("No rows")
        .item(ArrayEditorItem::new("row-1", "Row 1"));

    assert_eq!("root", tree.items()[0].id);
    assert_eq!(TreeNodeKind::Directory, tree.items()[0].kind);
    assert!(tree.items()[0].expanded);
    assert!(tree.items()[0].selected);
    assert!(tree.items()[0].active);
    assert_eq!("root", tree.active_model());
    assert!(tree.line_display_model());
    assert_eq!(TreeLineStyle::Dashed, tree.line_style_model());
    assert_eq!(2, tree.line_width_model());
    assert!(tree.icons_visible_model());
    assert_eq!("<svg data-icon=\"folder\"/>", tree.directory_icon_model());
    assert_eq!("<svg data-icon=\"file\"/>", tree.file_icon_model());
    assert_eq!("body", tree.tree_font_role_model());
    assert_eq!("katana-dark", tree.tree_theme_id_model());
    assert!(tree.empty_area_context_menu_model());
    assert!(tree.default_open_model());
    assert_eq!("<svg data-icon=\"chevron\"/>", tree.toggle_icon_model());
    assert_eq!(
        DisclosureTriggerArea::IconAndText,
        tree.toggle_trigger_area_model()
    );
    assert_eq!("format", palette.query_model());
    assert_eq!("format", palette.filtered_actions()[0].id);
    assert_eq!("enter", palette.keyboard_action_model());
    assert_eq!("Cmd Shift F", palette.items()[0].shortcut);
    assert_eq!("row-1", editor.items()[0].id);
    assert_eq!("append", editor.add_action_model());
    assert_eq!("delete-row", editor.delete_action_model());
    assert_eq!("move-row", editor.reorder_action_model());
    assert_eq!("edit-value", editor.edit_action_model());
    assert_eq!("No rows", editor.empty_state_model());

    let tree_model = UiTree::new(tree.clone());
    let tree_props = &tree_model.root().props().tree;

    assert_eq!(ITEM_COUNT, tree_model.root().props().interaction.item_count);
    assert!(UiTree::new(tree).root().props().interaction.open);
    assert_eq!("root", tree_props.active_id);
    assert!(tree_props.line_display);
    assert_eq!(UiTreeLineStyle::Dashed, tree_props.line_style);
    assert_eq!(2, tree_props.line_width);
    assert!(tree_props.icons_visible);
    assert_eq!("<svg data-icon=\"folder\"/>", tree_props.directory_icon);
    assert_eq!("<svg data-icon=\"file\"/>", tree_props.file_icon);
    assert_eq!("body", tree_props.font_role);
    assert_eq!("katana-dark", tree_props.theme_id);
    assert!(tree_props.empty_area_context_menu);
    assert!(tree_props.default_open);
    assert_eq!("<svg data-icon=\"chevron\"/>", tree_props.toggle_icon);
    assert_eq!(
        UiTreeToggleTriggerArea::IconAndText,
        tree_props.toggle_trigger_area
    );
    assert_eq!(ITEM_COUNT, tree_props.nodes.len());
    assert_eq!("root", tree_props.nodes[0].id);
    assert_eq!(UiTreeNodeKind::Directory, tree_props.nodes[0].kind);
    assert!(tree_props.nodes[0].expanded);
    assert!(tree_props.nodes[0].selected);
    assert!(tree_props.nodes[0].active);
    assert_eq!(
        ITEM_COUNT,
        UiTree::new(palette).root().props().interaction.item_count
    );
    assert_eq!(
        ITEM_COUNT,
        UiTree::new(editor).root().props().interaction.item_count
    );
}

#[test]
fn tree_view_click_toggles_directory_open_state_and_logs_click() {
    let mut tree = TreeView::new("Tree")
        .default_open(true)
        .toggle_trigger_area(DisclosureTriggerArea::IconAndText)
        .item(TreeNode::new("root", "Root", 0).directory().expanded(true));
    let target = tree.state_id().clone();

    let close_result = tree.apply_action(&UiAction::click(target.clone()));
    let open_result = tree.apply_action(&UiAction::click(target));

    assert!(close_result.handled);
    assert!(close_result.before.open);
    assert!(!close_result.after.open);
    assert_eq!("click", close_result.callback_log[0].action);
    assert!(open_result.handled);
    assert!(!open_result.before.open);
    assert!(open_result.after.open);
    assert_eq!(
        DisclosureTriggerArea::IconAndText,
        tree.toggle_trigger_area_model()
    );
}
