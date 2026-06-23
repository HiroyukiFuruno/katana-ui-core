use std::collections::BTreeSet;

use katana_ui_core::render_model::{UiNode, UiNodeKind, UiPanelProps};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{StoryCatalog, StorybookPanel};

const EXPECTED_PANEL_COUNT: usize = 4;
const EXPECTED_ROOT_CHILD_PANELS: usize = 3;

#[test]
fn storybook_panels_have_independent_vertical_scroll_contract() {
    let examples = StoryCatalog.examples();
    let tree = StorybookPanel::new(ThemeSnapshot::dark()).build_selected(&examples, "button");
    let panels = collect_panels(tree.root());

    assert_eq!(EXPECTED_PANEL_COUNT, panels.len());
    assert_eq!(
        EXPECTED_ROOT_CHILD_PANELS,
        tree.root()
            .children()
            .iter()
            .filter(|it| it.kind() == UiNodeKind::Panel)
            .count()
    );
    assert_expected_panel(&panels, "katana-ui-core Storybook");
    assert_expected_panel(&panels, "Navigation");
    assert_expected_panel(&panels, "Preview");
    assert_expected_panel(&panels, "Details");
    assert_independent_scroll_states(&panels);
}

fn collect_panels(root: &UiNode) -> Vec<&UiNode> {
    let mut panels = Vec::new();
    collect_panels_into(root, &mut panels);
    panels
}

fn collect_panels_into<'a>(node: &'a UiNode, panels: &mut Vec<&'a UiNode>) {
    if node.kind() == UiNodeKind::Panel {
        panels.push(node);
    }
    for child in node.children() {
        collect_panels_into(child, panels);
    }
}

fn assert_expected_panel(panels: &[&UiNode], label: &str) {
    let panel = panels.iter().find(|it| it.props().label == label);
    assert!(panel.is_some(), "{label} panel is missing");
    let Some(panel) = panel else {
        return;
    };
    assert_panel_scroll(label, &panel.props().panel);
}

fn assert_panel_scroll(label: &str, scroll: &UiPanelProps) {
    assert!(
        scroll.vertical_scrollbar_visible,
        "{label} panel must expose its own vertical scrollbar"
    );
    assert!(
        scroll.content_height > scroll.viewport_height,
        "{label} panel content must overflow its own viewport"
    );
}

fn assert_independent_scroll_states(panels: &[&UiNode]) {
    let state_ids = panels
        .iter()
        .map(|it| it.props().state_id.as_str())
        .collect::<BTreeSet<_>>();
    let scroll_shapes = panels
        .iter()
        .map(|it| {
            let panel = &it.props().panel;
            (panel.viewport_height, panel.content_height)
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(panels.len(), state_ids.len());
    assert_eq!(panels.len(), scroll_shapes.len());
}
