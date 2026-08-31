use katana_ui_core::molecule::tab_strip_icon_catalog::TabStripIcon;
use katana_ui_core::render_model::UiSvgPaintPolicy;

#[test]
fn catalog_contains_the_complete_generic_tab_strip_set() {
    assert_eq!(7, TabStripIcon::all().len());
    assert_eq!(
        [
            TabStripIcon::Previous,
            TabStripIcon::Next,
            TabStripIcon::Overflow,
            TabStripIcon::Close,
            TabStripIcon::Pin,
            TabStripIcon::DisclosureExpanded,
            TabStripIcon::DisclosureCollapsed,
        ],
        TabStripIcon::all()
    );
}

#[test]
fn every_icon_has_a_valid_current_color_svg_contract() {
    for icon in TabStripIcon::all() {
        let props = icon.icon_props();
        let source = props.svg_source.trim();

        assert!(source.starts_with("<svg>"), "{icon:?}");
        assert!(source.ends_with("</svg>"), "{icon:?}");
        assert_eq!(1, source.matches("<path ").count(), "{icon:?}");
        assert!(source.contains("fill=\"currentColor\""), "{icon:?}");
        assert!(source.contains(" d=\""), "{icon:?}");
        assert!(!source.contains("<text"), "{icon:?}");
        assert!(!source.contains("<image"), "{icon:?}");
        assert!(!source.contains("<use"), "{icon:?}");
        assert_eq!("0 0 16 16", props.view_box);
        assert!(!props.role.trim().is_empty(), "{icon:?}");
        assert!(!props.path_summary.trim().is_empty(), "{icon:?}");
        assert_eq!(UiSvgPaintPolicy::CurrentColor, props.paint_policy);
        assert_eq!(props, icon.icon_props());
    }
}
