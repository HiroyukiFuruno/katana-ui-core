use super::*;

#[test]
fn preset_tabs_are_measured_and_do_not_overlap() {
    let container = preset_container_rect();

    for index in 0..PRESET_TAB_COUNT {
        let rect = preset_tab_rect(index);
        let active = preset_tab_visual_rect(index, true);
        let inactive = preset_tab_visual_rect(index, false);
        assert!(rect.inside_canvas());
        assert!(active.inside_canvas());
        assert!(inactive.inside_canvas());
        assert_eq!(active.y, inactive.y);
        assert_eq!(active.height, inactive.height);
        assert!(container.contains(rect.x, rect.y));
        assert!(container.contains(rect.right() - 1, rect.bottom() - 1));
        if index > 0 {
            assert_eq!(preset_tab_rect(index - 1).right(), rect.x);
        }
    }
}

#[test]
fn storybook_regions_stay_inside_canvas_without_overlap() {
    let navigation = LayoutRect::new(0, 0, NAV_WIDTH, CONTENT_HEIGHT);
    let preview = LayoutRect::new(PREVIEW_X, 0, INSPECTOR_X - PREVIEW_X, CONTENT_HEIGHT);
    let inspector = inspector_rect();

    assert!(navigation.inside_content());
    assert!(preview.inside_content());
    assert!(inspector.inside_canvas());
    assert!(!navigation.overlaps(preview));
    assert!(!preview.overlaps(inspector));
}

#[test]
fn storybook_main_content_reaches_root_scrollbar_without_right_gap() {
    assert_eq!(
        super::super::scrollbar::track_rect().x,
        storybook_content_right_edge()
    );
}

#[test]
fn navigation_header_controls_are_balanced_segmented_pairs() {
    let light = light_theme_rect();
    let dark = dark_theme_rect();
    let scrollbar_on = scrollbar_on_rect();
    let scrollbar_off = scrollbar_off_rect();

    assert_eq!(light.width, dark.width);
    assert_eq!(scrollbar_on.width, scrollbar_off.width);
    assert_eq!(light.right(), dark.x);
    assert_eq!(scrollbar_on.right(), scrollbar_off.x);
    assert_eq!(light.x, scrollbar_on.x);
    assert_eq!(dark.right(), scrollbar_off.right());
    assert!(dark.right() <= NAV_WIDTH - BRAND_X);
}
