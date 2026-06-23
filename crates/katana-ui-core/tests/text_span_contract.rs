use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};

#[test]
fn emoji_span_is_explicit_render_contract() {
    let span = UiTextSpan::emoji("🙂");

    assert_eq!("🙂", span.text);
    assert!(span.style.emoji);
    assert!(!span.style.monospace);
    assert!(span.link_target.is_empty());
}

#[test]
fn emoji_style_can_be_combined_with_existing_text_styles() {
    let style = UiTextSpanStyle {
        bold: true,
        ..UiTextSpanStyle::default().emoji()
    };

    assert!(style.emoji);
    assert!(style.bold);
}
