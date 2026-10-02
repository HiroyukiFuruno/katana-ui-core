use super::types::{
    NativeFrameObservations, NativeImeEvidenceArtifact, NativeImeVerificationError,
    NativeImeVerificationExpectations, RgbaCropObservation,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{error::Error, fmt};

pub const SCHEMA_VERSION: &str = "kuc.native-ime-evidence.v1";
pub const EXPECTED_SCALARS: [u32; 2] = [0x2b50, 0xfe0f];
const SHA256_HEX_LEN: usize = 64;
const ALPHA_CHANNEL: usize = 3;

impl RgbaCropObservation {
    pub fn seal(mut self) -> Result<Self, serde_json::Error> {
        self.has_colored_pixels = self
            .pixels
            .iter()
            .any(|p| p[ALPHA_CHANNEL] > 0 && (p[0] != p[1] || p[1] != p[2]));
        self.rgba_sha256 = digest_json(&(self.width, self.height, &self.pixels))?;
        self.control_rgba_sha256 = digest_json(&(
            self.control_width,
            self.control_height,
            &self.control_pixels,
        ))?;
        Ok(self)
    }
}
impl NativeFrameObservations {
    pub fn observation_hashes(
        &self,
    ) -> Result<(String, String, String, String), serde_json::Error> {
        Ok((
            digest_json(&(self.measurement_width, self.measurement_height))?,
            digest_json(&self.caret)?,
            digest_json(&self.hit_tests)?,
            digest_json(&self.accesskit)?,
        ))
    }
}
impl fmt::Display for NativeImeVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "native IME evidence verification failed: {self:?}")
    }
}
impl Error for NativeImeVerificationError {}

impl NativeImeEvidenceArtifact {
    pub fn computed_sha256(&self) -> Result<String, serde_json::Error> {
        let mut c = self.clone();
        c.artifact_sha256.clear();
        digest_json(&c)
    }
    pub fn has_valid_digest(&self) -> Result<bool, serde_json::Error> {
        Ok(self.artifact_sha256 == self.computed_sha256()?)
    }
    pub fn seal(mut self) -> Result<Self, serde_json::Error> {
        self.artifact_sha256 = self.computed_sha256()?;
        Ok(self)
    }
    pub fn verify(
        &self,
        e: &NativeImeVerificationExpectations,
    ) -> Result<(), NativeImeVerificationError> {
        self.verify_binding(e)?;
        self.verify_transcript()?;
        self.verify_crop()?;
        self.verify_observations()?;
        if !is_sha256(&self.artifact_sha256) || !self.has_valid_digest().map_err(serialization)? {
            return Err(NativeImeVerificationError::InvalidHash("artifact_sha256"));
        }
        Ok(())
    }
    fn verify_binding(
        &self,
        e: &NativeImeVerificationExpectations,
    ) -> Result<(), NativeImeVerificationError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(NativeImeVerificationError::InvalidSchema);
        }
        if !matches!(self.platform.as_str(), "macos" | "windows" | "linux") {
            return Err(NativeImeVerificationError::UnsupportedPlatform);
        }
        for (n, v) in [
            ("revision", &self.revision),
            ("platform", &self.platform),
            ("run_id", &self.run.run_id),
            ("challenge", &self.run.challenge),
            ("producer_id", &self.run.producer_id),
            ("runner_id", &self.run.runner_id),
            ("input_method", &self.run.input_method),
            ("artifact_sha256", &self.artifact_sha256),
        ] {
            if v.trim().is_empty() {
                return Err(NativeImeVerificationError::MissingField(n));
            }
        }
        for (n, a, w) in [
            ("revision", &self.revision, &e.revision),
            ("platform", &self.platform, &e.platform),
            ("run_id", &self.run.run_id, &e.run_id),
            ("challenge", &self.run.challenge, &e.challenge),
            ("producer_id", &self.run.producer_id, &e.producer_id),
            ("runner_id", &self.run.runner_id, &e.runner_id),
        ] {
            if a != w {
                return Err(NativeImeVerificationError::BindingMismatch(n));
            }
        }
        if e.attested_artifact_sha256 != self.artifact_sha256 {
            return Err(NativeImeVerificationError::DigestMismatch);
        }
        Ok(())
    }
    fn verify_transcript(&self) -> Result<(), NativeImeVerificationError> {
        if self.run.origin != "os-native" {
            return Err(NativeImeVerificationError::InvalidOrigin);
        }
        if self.ime.preedit.trim().is_empty()
            || self.ime.commit.trim().is_empty()
            || !contains_japanese(&self.ime.preedit)
            || !contains_japanese(&self.ime.commit)
            || self.ime.preedit_sequence >= self.ime.commit_sequence
            || self.ime.commit != self.frame_observations.committed_text
            || self.frame_observations.committed_range_start
                >= self.frame_observations.committed_range_end
            || self.frame_observations.final_text.get(
                self.frame_observations.committed_range_start as usize
                    ..self.frame_observations.committed_range_end as usize,
            ) != Some(self.ime.commit.as_str())
        {
            return Err(NativeImeVerificationError::InvalidTranscript);
        }
        if self.scalar_sequence != EXPECTED_SCALARS
            || !self.frame_observations.final_text.contains("⭐️")
            || !self.frame_observations.final_text.contains('☆')
        {
            return Err(NativeImeVerificationError::InvalidScalars);
        }
        Ok(())
    }
    fn verify_crop(&self) -> Result<(), NativeImeVerificationError> {
        let c = &self.rgba_crop;
        let count = c
            .width
            .checked_mul(c.height)
            .ok_or(NativeImeVerificationError::InvalidCrop)? as usize;
        let control_count = c
            .control_width
            .checked_mul(c.control_height)
            .ok_or(NativeImeVerificationError::InvalidCrop)? as usize;
        if c.width == 0
            || c.height == 0
            || c.pixels.len() != count
            || c.control_width == 0
            || c.control_height == 0
            || c.control_pixels.len() != control_count
        {
            return Err(NativeImeVerificationError::InvalidCrop);
        }
        let colored = c
            .pixels
            .iter()
            .any(|p| p[ALPHA_CHANNEL] > 0 && (p[0] != p[1] || p[1] != p[2]));
        if colored != c.has_colored_pixels
            || !colored
            || c.pixels == c.control_pixels
            || digest_json(&(c.width, c.height, &c.pixels))
                .map_err(|_| NativeImeVerificationError::InvalidCrop)?
                != c.rgba_sha256
            || digest_json(&(c.control_width, c.control_height, &c.control_pixels))
                .map_err(|_| NativeImeVerificationError::InvalidCrop)?
                != c.control_rgba_sha256
        {
            return Err(NativeImeVerificationError::InvalidCrop);
        }
        Ok(())
    }
    fn verify_observations(&self) -> Result<(), NativeImeVerificationError> {
        let f = &self.frame_observations;
        if f.measurement_width == 0
            || f.measurement_height == 0
            || f.caret.width == 0
            || f.caret.height == 0
            || f.hit_tests.is_empty()
            || f.accesskit.role != "MultilineTextInput"
            || f.accesskit.value != f.final_text
            || f.accesskit.bounds.width == 0
            || f.accesskit.bounds.height == 0
            || f.root_id.trim().is_empty()
            || f.frame_id.trim().is_empty()
            || f.root_id != self.observations.root_id
            || f.frame_id != self.observations.frame_id
        {
            return Err(NativeImeVerificationError::InvalidObservation(
                "frame observations",
            ));
        }
        let text_len = f.final_text.len() as u32;
        if f.hit_tests.iter().any(|hit| {
            let target_right = hit.target_bounds.x.checked_add(hit.target_bounds.width);
            let target_bottom = hit.target_bounds.y.checked_add(hit.target_bounds.height);
            hit.range_start >= hit.range_end
                || hit.range_end > text_len
                || !f.final_text.is_char_boundary(hit.range_start as usize)
                || !f.final_text.is_char_boundary(hit.range_end as usize)
                || !matches!(hit.target.as_str(), "⭐️" | "☆")
                || f.final_text[hit.range_start as usize..hit.range_end as usize] != hit.target
                || hit.target_bounds.width == 0
                || hit.target_bounds.height == 0
                || target_right.is_none()
                || target_bottom.is_none()
                || target_right.unwrap_or(u32::MAX) > f.measurement_width
                || target_bottom.unwrap_or(u32::MAX) > f.measurement_height
                || hit.query_x >= f.measurement_width
                || hit.query_y >= f.measurement_height
                || hit.query_x < hit.target_bounds.x
                || hit.query_y < hit.target_bounds.y
                || hit.query_x >= target_right.unwrap_or(0)
                || hit.query_y >= target_bottom.unwrap_or(0)
        }) || !f.hit_tests.iter().any(|hit| hit.target == "⭐️")
            || !f.hit_tests.iter().any(|hit| hit.target == "☆")
        {
            return Err(NativeImeVerificationError::InvalidObservation("hit tests"));
        }
        let h = f.observation_hashes().map_err(serialization)?;
        for (n, a, v) in [
            (
                "measurement_sha256",
                &self.observations.measurement_sha256,
                &h.0,
            ),
            ("caret_sha256", &self.observations.caret_sha256, &h.1),
            ("hit_test_sha256", &self.observations.hit_test_sha256, &h.2),
            (
                "accesskit_sha256",
                &self.observations.accesskit_sha256,
                &h.3,
            ),
        ] {
            if !is_sha256(a) || a != v {
                return Err(NativeImeVerificationError::InvalidHash(n));
            }
        }
        for (n, v) in [
            ("frame_sha256", &self.observations.frame_sha256),
            ("root_sha256", &self.observations.root_sha256),
            ("receipt_sha256", &self.observations.receipt_sha256),
            ("font_file_sha256", &self.observations.font_file_sha256),
            (
                "font_catalog_sha256",
                &self.observations.font_catalog_sha256,
            ),
        ] {
            if !is_sha256(v) {
                return Err(NativeImeVerificationError::InvalidHash(n));
            }
        }
        Ok(())
    }
}
fn serialization(e: serde_json::Error) -> NativeImeVerificationError {
    NativeImeVerificationError::Serialization(e.to_string())
}
fn digest_json<T: Serialize>(v: &T) -> Result<String, serde_json::Error> {
    Ok(Sha256::digest(serde_json::to_vec(v)?)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}
fn is_sha256(v: &str) -> bool {
    v.len() == SHA256_HEX_LEN && v.bytes().all(|b| b.is_ascii_hexdigit())
}
fn contains_japanese(v: &str) -> bool {
    v.chars().any(|c| {
        ('\u{3040}'..='\u{30ff}').contains(&c)
            || ('\u{3400}'..='\u{4dbf}').contains(&c)
            || ('\u{4e00}'..='\u{9fff}').contains(&c)
    })
}

#[cfg(test)]
mod tests {
    use super::{digest_json, serialization};
    use serde::{Serialize, Serializer};

    struct SerializationFailure;

    impl Serialize for SerializationFailure {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom(
                "intentional serialization failure",
            ))
        }
    }

    #[test]
    fn serialization_failures_are_preserved_as_verification_errors() {
        let error = digest_json(&SerializationFailure).expect_err("fixture must fail");
        let verification_error = serialization(error);
        assert!(matches!(
            verification_error,
            super::NativeImeVerificationError::Serialization(message)
                if message.contains("intentional serialization failure")
        ));
    }
}
