use katana_ui_core::atom::Text;
use katana_ui_core::molecule::structured::collapsible_panel::{
    CollapsiblePanel, CollapsiblePanelAction, CollapsiblePanelEvent, PanelMode, ResizableWidth,
};
use katana_ui_core::render_model::{UiDimension, UiNodeKind, UiTree, UiZIndex};

const WIDTH_MIN: u16 = 180;
const WIDTH_MAX: u16 = 420;
const WIDTH_DEFAULT: u16 = 240;
const WIDTH_CURRENT: u16 = 260;
const WIDTH_TOO_NARROW: u16 = 40;
const WIDTH_TOO_WIDE: u16 = 999;
const CONTAINER_WIDTH: u16 = 1200;

#[test]
fn mode_changes_emit_typed_events() {
    let mut panel = panel().mode(PanelMode::Expanded);
    let modes = [
        PanelMode::IconOnly,
        PanelMode::Collapsed,
        PanelMode::FloatingOverlay,
        PanelMode::Expanded,
    ];
    let mut previous = PanelMode::Expanded;

    for mode in modes {
        let events = panel.apply_action(CollapsiblePanelAction::SetMode(mode));
        assert_eq!(mode, panel.state().mode);
        assert_eq!(
            Some(&CollapsiblePanelEvent::ModeChanged {
                from: previous,
                to: mode
            }),
            events.first()
        );
        previous = mode;
    }
}

#[test]
fn resize_clamps_and_reports_persist_id() {
    let mut panel = panel();
    let max_events = panel.apply_action(CollapsiblePanelAction::Resize(WIDTH_TOO_WIDE));
    let min_events = panel.apply_action(CollapsiblePanelAction::Resize(WIDTH_TOO_NARROW));

    assert_eq!(WIDTH_MIN, panel.state().width.current);
    assert!(matches!(
        max_events.as_slice(),
        [CollapsiblePanelEvent::WidthChanged { width, persist_id }]
            if *width == WIDTH_MAX && persist_id.as_deref() == Some("workspace.sidebar.width")
    ));
    assert!(matches!(
        min_events.as_slice(),
        [CollapsiblePanelEvent::WidthChanged { width, .. }] if *width == WIDTH_MIN
    ));
}

#[test]
fn reset_width_returns_to_default() {
    let mut panel = panel();
    panel.apply_action(CollapsiblePanelAction::Resize(WIDTH_TOO_WIDE));
    let events = panel.apply_action(CollapsiblePanelAction::ResetWidth);

    assert_eq!(WIDTH_DEFAULT, panel.state().width.current);
    assert!(matches!(
        events.as_slice(),
        [CollapsiblePanelEvent::WidthChanged { width, .. }] if *width == WIDTH_DEFAULT
    ));
}

#[test]
fn unpinned_hover_temporarily_expands_and_restores_mode() {
    let mut panel = panel()
        .mode(PanelMode::IconOnly)
        .pinned(false)
        .expand_on_hover(true);

    panel.apply_action(CollapsiblePanelAction::HoverTrigger);
    assert_eq!(PanelMode::Expanded, panel.rendered_mode());
    assert!(panel.state().hover_open);

    panel.apply_action(CollapsiblePanelAction::LeaveTrigger);
    assert_eq!(PanelMode::IconOnly, panel.rendered_mode());
    assert!(!panel.state().hover_open);
}

#[test]
fn pinned_panel_ignores_hover_trigger() {
    let mut panel = panel().mode(PanelMode::IconOnly).expand_on_hover(true);

    let events = panel.apply_action(CollapsiblePanelAction::HoverTrigger);

    assert!(events.is_empty());
    assert_eq!(PanelMode::IconOnly, panel.rendered_mode());
    assert!(!panel.state().hover_open);
}

#[test]
fn toggle_expand_matches_accelerator_action_contract() {
    let mut panel = panel().mode(PanelMode::IconOnly);
    let events = panel.apply_action(CollapsiblePanelAction::ToggleExpand);

    assert_eq!(PanelMode::Expanded, panel.state().mode);
    assert!(matches!(
        events.as_slice(),
        [CollapsiblePanelEvent::ModeChanged {
            from: PanelMode::IconOnly,
            to: PanelMode::Expanded
        }]
    ));
}

#[test]
fn floating_overlay_keeps_main_width_and_uses_overlay_z_index() {
    let panel = panel().mode(PanelMode::FloatingOverlay);
    let tree = UiTree::new(panel.clone());
    let root = tree.root();

    assert_eq!(UiNodeKind::CollapsiblePanel, root.kind());
    assert_eq!(UiDimension::Px(WIDTH_CURRENT), root.props().common.width);
    assert_eq!(
        UiZIndex::Value(CollapsiblePanel::OVERLAY_Z_INDEX),
        root.props().common.z_index
    );
    assert_eq!(CONTAINER_WIDTH, panel.main_available_width(CONTAINER_WIDTH));
}

#[test]
fn rendered_width_matches_mode_without_image_regression() {
    let cases = [
        (PanelMode::Expanded, WIDTH_CURRENT, true),
        (PanelMode::IconOnly, 56, true),
        (PanelMode::Collapsed, 0, false),
        (PanelMode::FloatingOverlay, WIDTH_CURRENT, true),
    ];

    for (mode, expected_width, expected_visible) in cases {
        let tree = UiTree::new(panel().mode(mode));
        let root = tree.root();

        assert_eq!(UiDimension::Px(expected_width), root.props().common.width);
        assert_eq!(expected_visible, root.props().common.visible);
    }
}

#[test]
fn content_slot_keeps_child_state_separate() {
    let tree = UiTree::new(panel().content(Text::new("Explorer")));
    let root = tree.root();

    assert_eq!(1, root.children().len());
    assert_ne!(root.props().state_id, root.children()[0].props().state_id);
}

fn panel() -> CollapsiblePanel {
    CollapsiblePanel::new(
        "Sidebar",
        ResizableWidth::new(
            WIDTH_MIN,
            WIDTH_MAX,
            WIDTH_DEFAULT,
            WIDTH_CURRENT,
            Some("workspace.sidebar.width"),
        ),
    )
    .resize_handle(true)
}
