use super::super::model::KucGraphemeArtifact;
use super::super::types::KucUnicodeColorGlyphEvidenceError;
use katana_ui_core_text_raster::PlatformTextGraphemeRange;

pub(super) fn artifacts(text: &str) -> Vec<KucGraphemeArtifact> {
    PlatformTextGraphemeRange::ranges(text)
        .into_iter()
        .filter_map(|range| {
            text.get(range.byte_start..range.byte_end)
                .map(|grapheme| KucGraphemeArtifact {
                    byte_start: range.byte_start,
                    byte_end: range.byte_end,
                    scalar_sequence: scalars(grapheme),
                })
        })
        .collect()
}

pub(super) fn required_range(
    text: &str,
    target: &str,
    ranges: &[KucGraphemeArtifact],
) -> Result<(usize, usize), KucUnicodeColorGlyphEvidenceError> {
    let expected = scalars(target);
    let range = ranges
        .iter()
        .find(|range| range.scalar_sequence == expected)
        .map(|range| (range.byte_start, range.byte_end))
        .ok_or_else(
            || KucUnicodeColorGlyphEvidenceError::RequiredGraphemeMissing {
                target: target.to_string(),
            },
        )?;
    let Some(actual_text) = text.get(range.0..range.1) else {
        return Err(KucUnicodeColorGlyphEvidenceError::RequiredGraphemeMissing {
            target: target.to_string(),
        });
    };
    if actual_text != target {
        return Err(
            KucUnicodeColorGlyphEvidenceError::ExpectedScalarSequenceChanged {
                target: target.to_string(),
                expected,
                actual: scalars(actual_text),
            },
        );
    }
    Ok(range)
}

pub(super) fn validate_scalar_sequence(
    actual: &[u32],
    target: &str,
    expected: &[u32],
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    if actual == expected {
        return Ok(());
    }
    Err(
        KucUnicodeColorGlyphEvidenceError::ExpectedScalarSequenceChanged {
            target: target.to_string(),
            expected: expected.to_vec(),
            actual: actual.to_vec(),
        },
    )
}

pub(super) fn scalars(text: &str) -> Vec<u32> {
    text.chars().map(u32::from).collect()
}

#[cfg(test)]
#[cfg(test)]
#[path = "grapheme_tests.rs"]
mod tests;
