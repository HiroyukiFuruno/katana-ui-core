use super::capture::validate_range;

#[test]
fn unicode_range_validation_accepts_grapheme_boundaries_and_rejects_invalid_utf8() {
    let text = "日本語⭐️";
    assert!(validate_range(text, (0, "日本語".len())).is_ok());
    assert!(validate_range(text, ("日本語".len(), text.len())).is_ok());
    assert!(validate_range(text, (1, "日本語".len())).is_err());
    assert!(validate_range(text, (0, 0)).is_err());
    assert!(validate_range(text, (0, text.len() + 1)).is_err());
}
#[test]
fn trace_preserves_the_original_serialization_cause() {
    let cause =
        serde_json::from_str::<serde_json::Value>("{").expect_err("an incomplete object must fail");
    let message = cause.to_string();
    assert!(matches!(
        super::trace(cause),
        super::KucUnicodeColorGlyphEvidenceError::RootTrace(actual) if actual == message
    ));
}
