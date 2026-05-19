use katana_ui_core::render_model::{UiContextMenuAnchor, UiContextMenuItemKind, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{StoryCatalog, StoryDetailContent, StorybookPanel, StorybookVisual};

const PAGE: &str = "context-menu";
const MIN_CONTEXT_MENU_PIXELS: usize = 10_000;
const MIN_PRESET_DIFF_PIXELS: usize = 1_000;

#[test]
fn context_menu_story_exposes_component_specific_contract() {
    let examples = StoryCatalog.examples();
    let example = examples.iter().find(|it| it.page == PAGE);
    assert!(example.is_some(), "{PAGE} story is missing");
    let Some(example) = example else {
        return;
    };
    let root = example.tree.root();
    let context_menu = &root.props().context_menu;

    assert_eq!(UiNodeKind::ContextMenu, root.kind());
    assert_eq!(
        UiContextMenuAnchor::Pointer { x: 192, y: 128 },
        context_menu.anchor
    );
    assert!(
        context_menu
            .items
            .iter()
            .any(|it| it.kind == UiContextMenuItemKind::Section)
    );
    assert!(
        context_menu
            .items
            .iter()
            .any(|it| it.kind == UiContextMenuItemKind::Divider)
    );
    assert!(
        context_menu
            .items
            .iter()
            .any(|it| it.kind == UiContextMenuItemKind::Submenu)
    );
    assert!(
        context_menu
            .items
            .iter()
            .any(|it| it.kind == UiContextMenuItemKind::Toggle)
    );
    assert!(
        context_menu
            .items
            .iter()
            .any(|it| it.kind == UiContextMenuItemKind::Radio)
    );
    assert!(context_menu.items.iter().any(|it| it.shortcut == "Cmd+C"));
    assert!(context_menu.items.iter().any(|it| it.destructive));
    assert!(
        example
            .callback_logs
            .iter()
            .any(|it| it.action == "context_menu_open")
    );
    assert!(
        example
            .callback_logs
            .iter()
            .any(|it| it.after.contains("event=context_menu_opened"))
    );

    let detail = StoryDetailContent::from_example(example);
    assert!(detail.settings.contains("context_menu.anchor"));
    assert!(detail.event.contains("context_menu_opened"));
    assert!(detail.action.contains("context_menu_open"));
    assert!(detail.preset.contains("編集器右クリック"));
    assert!(detail.preset.contains("leading icon + shortcut"));
}

#[test]
fn context_menu_page_is_visible_in_storybook_navigation_and_visual_presets() {
    let examples = StoryCatalog.examples();
    let panel = StorybookPanel::new(ThemeSnapshot::dark()).build_selected(&examples, PAGE);
    let root = panel.root();
    let navigation = root
        .children()
        .iter()
        .find(|it| it.props().label == "Navigation");
    let preview = root
        .children()
        .iter()
        .find(|it| it.props().label == "Preview");
    assert!(navigation.is_some(), "navigation panel is missing");
    assert!(preview.is_some(), "preview panel is missing");
    let (Some(navigation), Some(preview)) = (navigation, preview) else {
        return;
    };

    assert!(
        navigation
            .children()
            .iter()
            .any(|it| it.props().label == PAGE)
    );
    assert!(
        preview
            .children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::ContextMenu)
    );

    let default = StorybookVisual.render_preset("dark", PAGE, 0, 0);
    let shortcut = StorybookVisual.render_preset("dark", PAGE, 4, 0);

    assert!(default.non_background_pixels(0x1f1f1f) > MIN_CONTEXT_MENU_PIXELS);
    assert!(pixel_diff(&default, &shortcut) > MIN_PRESET_DIFF_PIXELS);
}

fn pixel_diff(
    left: &katana_ui_core_storybook::Canvas,
    right: &katana_ui_core_storybook::Canvas,
) -> usize {
    left.pixels()
        .iter()
        .zip(right.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}
