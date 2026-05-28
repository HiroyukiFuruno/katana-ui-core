use katana_ui_core::atom::{
    TextArea, TextAreaAction, TextAreaEvent, TextAreaValidationError, TextAreaWrapPolicy,
};
use katana_ui_core::render_model::UiNode;

#[test]
fn text_area_kuc_behavior_options_use_expected_defaults() {
    let node = UiNode::from(TextArea::new("Composer"));
    let text_area = &node.props().text_area;

    assert_eq!(TextAreaWrapPolicy::Soft, text_area.wrap_policy);
    assert!(!text_area.resize_enabled);
    assert!(!text_area.vertical_scroll_enabled);
    assert!(!text_area.horizontal_scroll_enabled);
    assert!(!text_area.vertical_scrollbar_visible);
    assert!(!text_area.horizontal_scrollbar_visible);
}

#[test]
fn text_area_kuc_behavior_options_can_be_enabled_explicitly() {
    let node = UiNode::from(
        TextArea::new("Composer")
            .wrap_policy(TextAreaWrapPolicy::None)
            .resize_enabled(true)
            .vertical_scroll_enabled(true)
            .horizontal_scroll_enabled(true)
            .vertical_scrollbar_visible(true)
            .horizontal_scrollbar_visible(true),
    );
    let text_area = &node.props().text_area;

    assert_eq!(TextAreaWrapPolicy::None, text_area.wrap_policy);
    assert!(text_area.resize_enabled);
    assert!(text_area.vertical_scroll_enabled);
    assert!(text_area.horizontal_scroll_enabled);
    assert!(text_area.vertical_scrollbar_visible);
    assert!(text_area.horizontal_scrollbar_visible);
}

#[test]
fn text_area_scrollbar_visibility_requires_enabled_scroll_axis() {
    let vertical = TextArea::new("Composer").vertical_scrollbar_visible(true);
    let horizontal = TextArea::new("Composer").horizontal_scrollbar_visible(true);

    assert_eq!(
        Err(TextAreaValidationError::VerticalScrollbarRequiresVerticalScroll),
        vertical.validate()
    );
    assert_eq!(
        Err(TextAreaValidationError::HorizontalScrollbarRequiresHorizontalScroll),
        horizontal.validate()
    );
    assert_eq!(
        Ok(()),
        TextArea::new("Composer")
            .vertical_scroll_enabled(true)
            .horizontal_scroll_enabled(true)
            .vertical_scrollbar_visible(true)
            .horizontal_scrollbar_visible(true)
            .validate()
    );
}

#[test]
fn auto_grow_rows_resize_and_scroll_without_truncating_value() {
    let mut text_area = TextArea::new("Long")
        .min_rows(2)
        .max_rows(4)
        .auto_grow(true);

    let resize = text_area.apply_text_area_action(TextAreaAction::Type("1\n2\n3".to_string()));
    let overflow = text_area.apply_text_area_action(TextAreaAction::Type("\n4\n5".to_string()));

    assert_eq!(3, resize.state.measured_rows);
    let resized = resize
        .events
        .iter()
        .any(|event| matches!(event, TextAreaEvent::Resize(resize) if resize.rows == 3));
    assert!(resized);
    assert_eq!(4, overflow.state.measured_rows);
    assert!(overflow.state.internal_scroll);
    assert_eq!("1\n2\n3\n4\n5", text_area.state().value);
}
