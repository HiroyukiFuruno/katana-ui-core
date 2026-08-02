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

#[test]
fn plain_and_inline_math_spans_keep_typed_style_contracts() {
    let plain = UiTextSpan::plain("plain");
    let math = UiTextSpanStyle::default().inline_math();

    assert_eq!("plain", plain.text);
    assert_eq!(UiTextSpanStyle::default(), plain.style);
    assert!(plain.link_target.is_empty());
    assert!(math.inline_math);
}
