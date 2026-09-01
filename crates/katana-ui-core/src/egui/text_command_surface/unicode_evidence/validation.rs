mod catalog;
mod crop;
mod grapheme;
mod serialization;

use super::constants::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, STAR_TEXT, UNICODE_EVIDENCE_SCHEMA,
    UNICODE_EVIDENCE_SCHEMA_VERSION, ZWJ_TEXT,
};
use super::model::{
    KucCaretArtifact, KucRgbaCropArtifact, KucUnicodeColorGlyphEvidence,
    KucUnicodeColorGlyphEvidenceInput, KucUnicodeColorGlyphEvidenceProfileArtifact,
};
use super::types::KucUnicodeColorGlyphEvidenceError;

#[derive(Debug, Default, Clone, Copy)]
pub struct KucUnicodeColorGlyphEvidenceBuilder;

impl KucUnicodeColorGlyphEvidenceBuilder {
    pub fn build(
        input: KucUnicodeColorGlyphEvidenceInput,
    ) -> Result<KucUnicodeColorGlyphEvidence, KucUnicodeColorGlyphEvidenceError> {
        catalog::validate(&input)?;
        let ranges = grapheme::artifacts(&input.final_text);
        let star_range = grapheme::required_range(&input.final_text, STAR_TEXT, &ranges)?;
        let control_range =
            grapheme::required_range(&input.final_text, CONTROL_STAR_TEXT, &ranges)?;
        let zwj_range = grapheme::required_range(&input.final_text, ZWJ_TEXT, &ranges)?;
        grapheme::validate_scalar_sequence(
            &input.ime.preedit_scalars,
            IME_PREEDIT_TEXT,
            &grapheme::scalars(IME_PREEDIT_TEXT),
        )?;
        grapheme::validate_scalar_sequence(
            &input.ime.commit_scalars,
            IME_COMMIT_TEXT,
            &grapheme::scalars(IME_COMMIT_TEXT),
        )?;
        validate_ime_events(&input)?;
        validate_caret(&input)?;
        validate_accesskit_node(&input)?;
        crop::validate_hit_test(&input.hit_tests, "star", star_range)?;
        crop::validate_hit_test(&input.hit_tests, "control_star", control_range)?;
        crop::validate_hit_test(&input.hit_tests, "zwj", zwj_range)?;
        let star = crop::artifact("star", &input.star_crop, true)?;
        let control_star = crop::artifact("control_star", &input.control_crop, false)?;
        let chromatic_pixel_delta = chromatic_pixel_delta(&star, &control_star)?;
        validate_hashes(&input)?;

        let profile_id = input.profile.as_str().to_string();
        let catalog_fingerprint = input.catalog_policy.fingerprint().to_hex();
        let face = catalog::face_artifact(&input, &profile_id, &catalog_fingerprint)?;
        let accesskit_text_input = accesskit_artifact(&input)?;
        let mut artifact = KucUnicodeColorGlyphEvidence {
            schema: UNICODE_EVIDENCE_SCHEMA.to_string(),
            schema_version: UNICODE_EVIDENCE_SCHEMA_VERSION,
            profile: KucUnicodeColorGlyphEvidenceProfileArtifact {
                profile_id,
                catalog_fingerprint,
            },
            catalog_face: face,
            graphemes: ranges,
            ime: super::model::KucImeArtifact {
                preedit_scalar_sequence: input.ime.preedit_scalars,
                commit_scalar_sequence: input.ime.commit_scalars,
                preedit_event_seen: input.ime.preedit_event_seen,
                commit_event_seen: input.ime.commit_event_seen,
            },
            caret: KucCaretArtifact {
                bounds: input.caret.bounds,
            },
            hit_tests: input.hit_tests.into_iter().map(hit_test_artifact).collect(),
            star,
            control_star,
            accesskit_text_input,
            chromatic_pixel_delta,
            accesskit_text_snapshot_hash: input.accesskit_text_snapshot_hash,
            root_frame_hash: input.root_frame_hash,
            root_record_hash: input.root_record_hash,
            root_rgba_hash: input.root_rgba_hash,
            artifact_sha256: String::new(),
        };
        artifact.artifact_sha256 = serialization::canonical_hash(&artifact)?;
        Ok(artifact)
    }
}

fn validate_accesskit_node(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    let node = input
        .accesskit_text_input
        .as_ref()
        .ok_or(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode)?;
    if node.node_id.is_empty()
        || node.value.is_empty()
        || node.bounds.width == 0
        || node.bounds.height == 0
    {
        return Err(KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode {
            reason: "node id, value, and positive bounds are required",
        });
    }
    let expected_role = "MultilineTextInput";
    if node.role != expected_role {
        return Err(KucUnicodeColorGlyphEvidenceError::AccessKitRoleMismatch {
            expected: expected_role.to_string(),
            actual: node.role.clone(),
        });
    }
    if node.value != input.final_text {
        return Err(KucUnicodeColorGlyphEvidenceError::AccessKitValueMismatch);
    }
    let expected = node.value.chars().map(u32::from).collect::<Vec<_>>();
    if node.scalar_sequence != expected {
        return Err(
            KucUnicodeColorGlyphEvidenceError::AccessKitScalarSequenceMismatch {
                expected,
                actual: node.scalar_sequence.clone(),
            },
        );
    }
    Ok(())
}

fn accesskit_artifact(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<super::model::KucAccessKitNodeArtifact, KucUnicodeColorGlyphEvidenceError> {
    let node = input
        .accesskit_text_input
        .as_ref()
        .ok_or(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode)?;
    Ok(super::model::KucAccessKitNodeArtifact {
        node_id: node.node_id.clone(),
        role: node.role.clone(),
        value: node.value.clone(),
        scalar_sequence: node.scalar_sequence.clone(),
        bounds: node.bounds,
    })
}

fn validate_ime_events(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    if !input.ime.preedit_event_seen {
        return Err(KucUnicodeColorGlyphEvidenceError::MissingImeEvent { kind: "preedit" });
    }
    if !input.ime.commit_event_seen {
        return Err(KucUnicodeColorGlyphEvidenceError::MissingImeEvent { kind: "commit" });
    }
    Ok(())
}

fn validate_caret(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    if input.caret.bounds.width == 0 || input.caret.bounds.height == 0 {
        Err(KucUnicodeColorGlyphEvidenceError::InvalidCaret)
    } else {
        Ok(())
    }
}

fn chromatic_pixel_delta(
    star: &KucRgbaCropArtifact,
    control_star: &KucRgbaCropArtifact,
) -> Result<i64, KucUnicodeColorGlyphEvidenceError> {
    if star == control_star {
        return Err(KucUnicodeColorGlyphEvidenceError::IndistinguishableCrops);
    }
    let delta = star
        .chromatic_pixel_count
        .saturating_sub(control_star.chromatic_pixel_count);
    if delta == 0 {
        return Err(KucUnicodeColorGlyphEvidenceError::IndistinguishableCrops);
    }
    i64::try_from(delta).map_err(|_| {
        KucUnicodeColorGlyphEvidenceError::Serialization(
            "chromatic pixel delta exceeds i64".to_string(),
        )
    })
}

fn validate_hashes(
    input: &KucUnicodeColorGlyphEvidenceInput,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    [
        (
            "accesskit_text_snapshot_hash",
            input.accesskit_text_snapshot_hash.as_str(),
        ),
        ("root_frame_hash", input.root_frame_hash.as_str()),
        ("root_record_hash", input.root_record_hash.as_str()),
        ("root_rgba_hash", input.root_rgba_hash.as_str()),
    ]
    .into_iter()
    .find_map(|(field, value)| value.is_empty().then_some(field))
    .map_or(Ok(()), |field| {
        Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash { field })
    })
}

fn hit_test_artifact(hit: super::model::KucHitTestObservation) -> super::model::KucHitTestArtifact {
    super::model::KucHitTestArtifact {
        target: hit.target,
        query_x: hit.query_x,
        query_y: hit.query_y,
        byte_start: hit.byte_start,
        byte_end: hit.byte_end,
    }
}

#[cfg(test)]
#[path = "validation_tests.rs"]
mod tests;
