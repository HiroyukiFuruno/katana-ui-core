use katana_ui_core::render_model::{
    UiEmojiTextSegment, UiEmojiTextSegments, UiPlatformEmojiFontFamily, UiTextSpan, UiTextSpanStyle,
};

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
fn emoji_text_segments_split_star_and_text_with_variation_selector() {
    let segments = UiEmojiTextSegments::split("Star ⭐️ mark");

    assert_eq!(
        segments
            .iter()
            .map(UiTextSegmentExpectation::from)
            .collect::<Vec<_>>(),
        vec![
            UiTextSegmentExpectation {
                text: "Star ".to_string(),
                emoji: false
            },
            UiTextSegmentExpectation {
                text: "⭐️".to_string(),
                emoji: true
            },
            UiTextSegmentExpectation {
                text: " mark".to_string(),
                emoji: false
            },
        ]
    );
}

#[test]
fn emoji_text_segments_mark_emoji_run_with_variation_selector_and_text() {
    let segments = UiEmojiTextSegments::split("Emoji: 🦀 text ⚠️");

    assert_eq!(
        segments
            .iter()
            .map(UiTextSegmentExpectation::from)
            .collect::<Vec<_>>(),
        vec![
            UiTextSegmentExpectation {
                text: "Emoji: ".to_string(),
                emoji: false
            },
            UiTextSegmentExpectation {
                text: "🦀".to_string(),
                emoji: true
            },
            UiTextSegmentExpectation {
                text: " text ".to_string(),
                emoji: false
            },
            UiTextSegmentExpectation {
                text: "⚠️".to_string(),
                emoji: true
            },
        ]
    );
}

#[test]
fn emoji_text_segments_treat_keycap_sequences_as_single_emoji_segments() {
    let segments = UiEmojiTextSegments::split("1️⃣ and *️⃣");

    assert_eq!(
        segments
            .iter()
            .map(UiTextSegmentExpectation::from)
            .collect::<Vec<_>>(),
        vec![
            UiTextSegmentExpectation {
                text: "1️⃣".to_string(),
                emoji: true
            },
            UiTextSegmentExpectation {
                text: " and ".to_string(),
                emoji: false
            },
            UiTextSegmentExpectation {
                text: "*️⃣".to_string(),
                emoji: true
            },
        ]
    );
}

#[test]
fn platform_emoji_font_family_requires_text_raster_catalog_resolution() {
    let font_family = UiPlatformEmojiFontFamily::default();

    assert_eq!(UiPlatformEmojiFontFamily::None, font_family);
    assert_eq!(None, font_family.as_str());
}

#[test]
fn emoji_marked_spans_sets_only_emoji_segments_to_emoji_style() {
    let style = UiTextSpanStyle {
        bold: true,
        underline: true,
        ..UiTextSpanStyle::default()
    };
    let segments = UiTextSpan::emoji_marked_spans("A 🧪 B", style);

    assert_eq!(3, segments.len());
    assert_eq!("A ", segments[0].text);
    assert_eq!("🧪", segments[1].text);
    assert_eq!(" B", segments[2].text);
    assert!(!segments[0].style.emoji);
    assert!(segments[1].style.emoji);
    assert!(!segments[2].style.emoji);
    assert!(segments[1].style.bold);
    assert!(segments[1].style.underline);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct UiTextSegmentExpectation {
    text: String,
    emoji: bool,
}

impl From<&UiEmojiTextSegment> for UiTextSegmentExpectation {
    fn from(segment: &UiEmojiTextSegment) -> Self {
        Self {
            text: segment.text.clone(),
            emoji: segment.emoji,
        }
    }
}
