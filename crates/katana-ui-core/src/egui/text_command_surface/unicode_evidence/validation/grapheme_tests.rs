use super::super::super::model::KucGraphemeArtifact;
use super::*;

#[test]
fn artifacts_preserve_unicode_byte_ranges_and_scalar_sequences() {
    let text = "a⭐️👩‍💻";
    let values = artifacts(text);
    assert_eq!(values.len(), 3);
    assert_eq!(
        text.get(values[1].byte_start..values[1].byte_end),
        Some("⭐️")
    );
    assert_eq!(values[2].scalar_sequence, scalars("👩‍💻"));
}

#[test]
fn required_range_fails_closed_for_missing_and_mismatched_boundaries() {
    let text = "⭐️";
    let ranges = artifacts(text);
    assert!(matches!(
        required_range(text, "☆", &ranges),
        Err(KucUnicodeColorGlyphEvidenceError::RequiredGraphemeMissing { target }) if target == "☆"
    ));
    assert!(matches!(
        validate_scalar_sequence(&[1], "⭐️", &scalars("⭐️")),
        Err(KucUnicodeColorGlyphEvidenceError::ExpectedScalarSequenceChanged { .. })
    ));
    assert_eq!(scalars(""), Vec::<u32>::new());
}

#[test]
fn required_range_rejects_scalar_sequence_change_with_matching_byte_range() {
    let ranges = vec![KucGraphemeArtifact {
        byte_start: 0,
        byte_end: 2,
        scalar_sequence: vec![97],
    }];
    assert!(matches!(
        required_range("ab", "a", &ranges),
        Err(
            KucUnicodeColorGlyphEvidenceError::ExpectedScalarSequenceChanged {
                target,
                expected,
                actual
            }
        ) if target == "a" && expected == vec![97] && actual == vec![97, 98]
    ));
}

#[test]
fn validate_scalar_sequence_accepts_exact_expected_sequence() {
    let target = "👩‍💻";
    assert!(matches!(
        validate_scalar_sequence(&scalars(target), target, &scalars(target)),
        Ok(())
    ));
}
