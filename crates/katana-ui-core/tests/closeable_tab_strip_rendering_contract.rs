use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTone};
use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabStrip, CloseableTabTone, TabGroup,
};

#[test]
fn closeable_tab_strip_uses_distinct_render_kinds_and_leading_pinned_order() {
    let strip = CloseableTabStrip::new("Tabs")
        .tab(CloseableTab::new("main", "Main"))
        .tab(CloseableTab::new("pinned", "Pinned").pinned(true))
        .tab(CloseableTab::new("settings", "Settings"));
    let node = UiNode::from(strip);
    let child_labels: Vec<&str> = node
        .children()
        .iter()
        .map(|child| child.props().label.as_str())
        .collect();

    assert_eq!(UiNodeKind::CloseableTabStrip, node.kind());
    assert_eq!(UiDimension::Fill, node.props().common.width);
    assert_eq!(UiDimension::Px(40), node.props().common.height);
    assert_eq!(vec!["Pinned", "Main", "Settings"], child_labels);
    assert!(
        node.props()
            .style_classes
            .contains(&"closeable-tab-strip".to_string())
    );
}

#[test]
fn closeable_tab_children_expose_dirty_pinned_closeable_and_tone_classes() {
    let strip = CloseableTabStrip::new("Tabs")
        .group(TabGroup::new("docs", "Docs"))
        .tab(
            CloseableTab::new("pinned", "Pinned")
                .pinned(true)
                .tone(CloseableTabTone::Accent),
        )
        .tab(
            CloseableTab::new("dirty", "Dirty")
                .dirty(true)
                .tone(CloseableTabTone::Warning)
                .group_id("docs"),
        )
        .tab(CloseableTab::new("danger", "Danger").tone(CloseableTabTone::Danger));
    let node = UiNode::from(strip);
    let pinned = &node.children()[0];
    let dirty = &node.children()[1];
    let danger = &node.children()[2];

    assert_eq!(UiNodeKind::CloseableTab, pinned.kind());
    assert_eq!(UiTone::Accent, pinned.props().tone);
    assert_eq!(UiTone::Warning, dirty.props().tone);
    assert_eq!(UiTone::Danger, danger.props().tone);
    assert!(
        pinned
            .props()
            .style_classes
            .contains(&"closeable-tab-pinned".to_string())
    );
    assert!(
        !pinned
            .props()
            .style_classes
            .contains(&"closeable-tab-closeable".to_string())
    );
    assert!(
        dirty
            .props()
            .style_classes
            .contains(&"closeable-tab-dirty".to_string())
    );
    assert!(
        dirty
            .props()
            .style_classes
            .contains(&"closeable-tab-closeable".to_string())
    );
}
