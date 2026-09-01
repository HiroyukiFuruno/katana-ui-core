use super::*;

#[test]
fn explicit_spans_segments_plain_text() {
    let spans = explicit_spans("abc");
    assert_eq!(
        spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<Vec<_>>(),
        ["a", "b", "c"]
    );
}

#[test]
fn explicit_spans_preserves_plain_and_emoji_roles_in_order() {
    let spans = explicit_spans("a⭐️b👩‍💻c");
    assert_eq!(spans.len(), 5);
    assert!(spans[1].style.emoji);
    assert!(spans[3].style.emoji);
    assert_eq!(spans[0].text, "a");
    assert_eq!(spans[4].text, "c");
}
