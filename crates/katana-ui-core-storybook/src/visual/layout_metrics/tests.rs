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
