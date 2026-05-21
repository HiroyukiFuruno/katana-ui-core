use super::{Panel, PanelRegion};
use crate::atom::Text;
use crate::render_model::{
    UiNodeKind, UiRect, UiScrollbarModel, UiScrollbarPlacement, UiScrollbarVisibility, UiTree,
};
use crate::theme::ThemeSnapshot;

#[test]
fn panel_carries_theme_setting_to_render_model() {
    let tree = UiTree::new(
        Panel::new("Preview", PanelRegion::Preview, ThemeSnapshot::dark())
            .child(Text::new("Story")),
    );

    assert_eq!(UiNodeKind::Panel, tree.root().kind());
    assert_eq!("dark", tree.root().props().theme_id);
    assert_eq!(1, tree.root().children().len());
}

#[test]
fn nested_panels_keep_independent_vertical_scroll_state() {
    let tree = UiTree::new(
        Panel::new("Parent", PanelRegion::Root, ThemeSnapshot::dark())
            .vertical_scroll(120, 600, 1800, true)
            .child(
                Panel::new("Left", PanelRegion::Navigation, ThemeSnapshot::dark())
                    .vertical_scroll(24, 320, 900, true),
            )
            .child(
                Panel::new("Right", PanelRegion::Details, ThemeSnapshot::dark())
                    .vertical_scroll(80, 320, 1200, true),
            ),
    );
    let left = &tree.root().children()[0];
    let right = &tree.root().children()[1];

    assert_eq!(120, tree.root().props().panel.scroll_y);
    assert_eq!(24, left.props().panel.scroll_y);
    assert_eq!(80, right.props().panel.scroll_y);
    assert_ne!(left.props().state_id, right.props().state_id);
}

#[test]
fn panel_scrollbar_model_carries_bounds_visibility_and_drag_state() {
    let scrollbar = UiScrollbarModel::new(
        UiScrollbarVisibility::Always,
        UiScrollbarPlacement::Overlay,
        UiRect::new(280, 0, 8, 320),
        UiRect::new(280, 32, 8, 96),
        48,
    )
    .dragging(7, 32);
    let tree = UiTree::new(
        Panel::new("Preview", PanelRegion::Preview, ThemeSnapshot::dark())
            .vertical_scroll(0, 320, 1280, false)
            .scrollbar(scrollbar),
    );
    let panel = &tree.root().props().panel;

    assert!(panel.vertical_scrollbar_visible);
    assert_eq!(48, panel.scroll_y);
    assert_eq!(
        UiScrollbarVisibility::Always,
        panel.vertical_scrollbar.visibility
    );
    assert_eq!(
        UiScrollbarPlacement::Overlay,
        panel.vertical_scrollbar.placement
    );
    assert_eq!(
        UiRect::new(280, 0, 8, 320),
        panel.vertical_scrollbar.track_bounds
    );
    assert_eq!(
        UiRect::new(280, 32, 8, 96),
        panel.vertical_scrollbar.thumb_bounds
    );
    assert!(panel.vertical_scrollbar.drag_state.dragging);
    assert_eq!(Some(7), panel.vertical_scrollbar.drag_state.pointer_id);
}

#[test]
fn panel_carries_independent_horizontal_scrollbar_model() {
    let horizontal = UiScrollbarModel::new(
        UiScrollbarVisibility::Always,
        UiScrollbarPlacement::Overlay,
        UiRect::new(0, 300, 420, 8),
        UiRect::new(96, 300, 120, 8),
        96,
    );
    let tree = UiTree::new(
        Panel::new("Menu", PanelRegion::Navigation, ThemeSnapshot::dark())
            .horizontal_scroll(96, 420, 960, true)
            .horizontal_scrollbar(horizontal),
    );
    let panel = &tree.root().props().panel;

    assert_eq!(96, panel.scroll_x);
    assert_eq!(420, panel.viewport_width);
    assert_eq!(960, panel.content_width);
    assert!(panel.horizontal_scrollbar_visible);
    assert_eq!(
        UiScrollbarPlacement::Overlay,
        panel.horizontal_scrollbar.placement
    );
    assert_eq!(
        UiRect::new(96, 300, 120, 8),
        panel.horizontal_scrollbar.thumb_bounds
    );
}

#[test]
fn panel_does_not_scroll_or_show_scrollbars_without_overflow() {
    let tree = UiTree::new(
        Panel::new("Preview", PanelRegion::Preview, ThemeSnapshot::dark())
            .vertical_scroll(80, 320, 320, true)
            .horizontal_scroll(96, 420, 420, true),
    );
    let panel = &tree.root().props().panel;

    assert_eq!(0, panel.scroll_y);
    assert_eq!(0, panel.scroll_x);
    assert!(!panel.vertical_scrollbar_visible);
    assert!(!panel.horizontal_scrollbar_visible);
    assert_eq!(
        UiScrollbarVisibility::Hidden,
        panel.vertical_scrollbar.visibility
    );
    assert_eq!(
        UiScrollbarVisibility::Hidden,
        panel.horizontal_scrollbar.visibility
    );
}
