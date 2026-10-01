use super::NativeApplication;
use katana_ui_core::native_ime_evidence::{
    NativeImeEvidenceArtifact, NativeImeTranscript, NativeImeVerificationExpectations,
    NativeObservationHashes, NativeRunBinding, RgbaCropObservation,
};
use std::io::Write;

impl NativeApplication {
    pub(super) fn write_artifact(
        &mut self,
        observations: &katana_ui_core::native_ime_evidence::NativeFrameObservations,
        rgba_crop: &RgbaCropObservation,
        hashes: &NativeObservationHashes,
    ) -> Result<(), String> {
        if self.artifact_written {
            return Ok(());
        }
        let artifact = NativeImeEvidenceArtifact {
            schema_version: katana_ui_core::native_ime_evidence::SCHEMA_VERSION.into(),
            revision: self.transcript.revision.clone(),
            platform: self.transcript.platform.clone(),
            run: NativeRunBinding {
                run_id: self.transcript.run_id.clone(),
                challenge: self.transcript.challenge.clone(),
                producer_id: self.transcript.producer_id.clone(),
                runner_id: self.transcript.runner_id.clone(),
                input_method: self.transcript.input_method.clone(),
                origin: "os-native".into(),
            },
            ime: NativeImeTranscript {
                preedit: self.transcript.preedit.clone(),
                commit: self.transcript.commit.clone(),
                preedit_sequence: self.transcript.preedit_sequence,
                commit_sequence: self.transcript.commit_sequence,
            },
            scalar_sequence: katana_ui_core::native_ime_evidence::EXPECTED_SCALARS.to_vec(),
            rgba_crop: rgba_crop.clone(),
            observations: hashes.clone(),
            frame_observations: observations.clone(),
            artifact_sha256: String::new(),
        }
        .seal()
        .map_err(|e| e.to_string())?;
        let expected = NativeImeVerificationExpectations {
            revision: self.transcript.revision.clone(),
            platform: self.transcript.platform.clone(),
            run_id: self.transcript.run_id.clone(),
            challenge: self.transcript.challenge.clone(),
            producer_id: self.transcript.producer_id.clone(),
            runner_id: self.transcript.runner_id.clone(),
            attested_artifact_sha256: artifact.artifact_sha256.clone(),
        };
        /* WHY: producer 内の照合は保存前の整合性検査。native 起源の信頼は、この照合ではなく所有する OS event loop と外部 runner の実行証明で確立する。 */
        artifact
            .verify(&expected)
            .map_err(|error| error.to_string())?;
        let bytes = serde_json::to_vec_pretty(&artifact).map_err(|e| e.to_string())?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.output)
            .map_err(|e| e.to_string())?;
        file.write_all(&bytes).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        let confirmation = serde_json::json!({"run_id": artifact.run.run_id, "artifact_sha256": artifact.artifact_sha256});
        println!(
            "{}",
            serde_json::to_string(&confirmation).map_err(|e| e.to_string())?
        );
        std::io::stdout().flush().map_err(|e| e.to_string())?;
        self.artifact_written = true;
        Ok(())
    }
}
