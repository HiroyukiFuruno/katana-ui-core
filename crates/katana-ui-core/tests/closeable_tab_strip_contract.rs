use katana_ui_core::render_model::UiNode;
use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabDropRules, CloseableTabId, CloseableTabStrip,
    CloseableTabStripAction, CloseableTabStripEvent, TabGroup, TabGroupTarget,
};

#[test]
fn closeable_tab_strip_public_api_is_domain_free() {
    let mut strip = CloseableTabStrip::new("tabs")
        .group(TabGroup::new("docs", "Docs"))
        .tab(CloseableTab::new("pinned", "Pinned").pinned(true))
        .tab(CloseableTab::new("draft", "Draft").dirty(true))
        .active_tab_id("draft");

    assert!(!CloseableTabDropRules::can_accept(
        &strip.options().tabs,
        &CloseableTabId::new("draft"),
        0
    ));

    let request = strip.apply_action(CloseableTabStripAction::CloseTab {
        tab_id: CloseableTabId::new("draft"),
    });
    assert_eq!(
        vec![CloseableTabStripEvent::TabCloseRequested {
            tab_id: CloseableTabId::new("draft")
        }],
        request
    );

    strip.apply_action(CloseableTabStripAction::MoveToGroup {
        tab_id: CloseableTabId::new("draft"),
        target: TabGroupTarget::Existing("docs".into()),
    });
    let node = UiNode::from(strip);

    assert_eq!("tabs", node.props().label);
    assert!(
        node.children()
            .iter()
            .any(|it| it.props().label == "Pinned")
    );
}
