use super::capture::validate_range;
use super::{KucBounds, KucLiveUnicodeObservation, KucUnicodeColorGlyphEvidenceError, trace};
use crate::egui::text_command_surface::EguiTextCommandSurfaceRootEventForwardingReceipt;
use crate::native_ime_evidence::{
    AccessKitObservation, Bounds, HitTestObservation, NativeFrameObservations,
    NativeObservationHashes, RgbaCropObservation,
};
use sha2::{Digest, Sha256};

#[cfg(test)]
#[path = "native_tests.rs"]
mod native_tests;

const COLOR_GLYPH_SCALARS: [u32; 2] = [0x2b50, 0xfe0f];
const CONTROL_GLYPH_SCALARS: [u32; 1] = [0x2606];
const SHA256_HEX_LENGTH: usize = 64;

type NativeObservations = (
    NativeFrameObservations,
    RgbaCropObservation,
    NativeObservationHashes,
);

impl KucLiveUnicodeObservation {
    /// 同一frameの色付きglyph・対照glyphと実forwarding receiptを既存形式へ接続する。
    /// originの認定は行わない。OS由来のtranscriptの検証は既存verifierが担当する。
    pub fn native_observations(
        &self,
        control: &Self,
        receipt: &EguiTextCommandSurfaceRootEventForwardingReceipt,
        committed_range: (usize, usize),
    ) -> Result<NativeObservations, KucUnicodeColorGlyphEvidenceError> {
        self.validate_binding(
            control.root_id(),
            control.frame_id(),
            control.root_record_hash(),
            control.root_rgba_hash(),
        )?;
        self.validate_receipt(receipt)?;
        validate_range(&self.final_text, committed_range)?;
        if self.scalars != COLOR_GLYPH_SCALARS || control.scalars != CONTROL_GLYPH_SCALARS {
            return Err(trace(
                "color and control observations require distinct exact scalar ranges",
            ));
        }
        let font_file = self
            .font_identity
            .0
            .as_ref()
            .ok_or_else(|| trace("loaded font identity missing"))?;
        if !valid_hash(font_file) || !valid_hash(&self.font_identity.1) {
            return Err(trace("font identity is invalid"));
        }
        let accesskit = self
            .accesskit
            .as_ref()
            .ok_or_else(|| trace("AccessKit observation missing"))?;
        let node_id = self
            .accesskit_node_id
            .ok_or_else(|| trace("AccessKit node identity missing"))?;
        let frame = NativeFrameObservations {
            final_text: self.final_text.clone(),
            committed_text: self.final_text[committed_range.0..committed_range.1].to_owned(),
            committed_range_start: checked_u32(committed_range.0)?,
            committed_range_end: checked_u32(committed_range.1)?,
            measurement_width: self.canvas.width,
            measurement_height: self.canvas.height,
            caret: bounds(self.caret.bounds),
            hit_tests: vec![hit(self)?, hit(control)?],
            accesskit: AccessKitObservation {
                role: accesskit.role.clone(),
                value: accesskit.value.clone(),
                bounds: bounds(accesskit.bounds),
                node_id,
            },
            root_id: self.root_id.clone(),
            frame_id: self.frame_id.clone(),
        };
        let crop = RgbaCropObservation {
            width: self.crop.bounds.width,
            height: self.crop.bounds.height,
            pixels: self.crop.pixels.clone(),
            control_width: control.crop.bounds.width,
            control_height: control.crop.bounds.height,
            control_pixels: control.crop.pixels.clone(),
            has_colored_pixels: false,
            rgba_sha256: String::new(),
            control_rgba_sha256: String::new(),
        }
        .seal()
        .map_err(trace)?;
        let (measurement_sha256, caret_sha256, hit_test_sha256, accesskit_sha256) =
            frame.observation_hashes().map_err(trace)?;
        let receipt_bytes = serde_json::to_vec(&(
            receipt.root_identity(),
            receipt.state_revision(),
            receipt.frame_serial(),
            receipt.correlation_fingerprint(),
            receipt.event_batch_fingerprint(),
            receipt.consumed_once(),
            receipt.event_cardinality(),
        ))
        .map_err(trace)?;
        let hashes = NativeObservationHashes {
            measurement_sha256,
            caret_sha256,
            hit_test_sha256,
            accesskit_sha256,
            frame_sha256: self.root_frame_hash.clone(),
            root_sha256: self.root_rgba_hash.clone(),
            receipt_sha256: hex::encode(Sha256::digest(receipt_bytes)),
            font_file_sha256: font_file.clone(),
            font_catalog_sha256: self.font_identity.1.clone(),
            root_id: self.root_id.clone(),
            frame_id: self.frame_id.clone(),
        };
        Ok((frame, crop, hashes))
    }

    fn validate_receipt(
        &self,
        receipt: &EguiTextCommandSurfaceRootEventForwardingReceipt,
    ) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
        if receipt.root_identity() != self.root_id
            || receipt.frame_serial() != self.frame_serial
            || receipt.state_revision() != self.state_revision
            || receipt.correlation_fingerprint() != self.event_correlation
            || !receipt.consumed_once()
        {
            return Err(trace("Unicode forwarding receipt binding mismatch"));
        }
        Ok(())
    }
}

fn bounds(value: KucBounds) -> Bounds {
    Bounds {
        x: value.x,
        y: value.y,
        width: value.width,
        height: value.height,
    }
}

fn hit(
    value: &KucLiveUnicodeObservation,
) -> Result<HitTestObservation, KucUnicodeColorGlyphEvidenceError> {
    Ok(HitTestObservation {
        target: value.hit_test.target.clone(),
        target_bounds: bounds(value.crop.bounds),
        range_start: checked_u32(value.hit_test.byte_start)?,
        range_end: checked_u32(value.hit_test.byte_end)?,
        query_x: value.hit_test.query_x,
        query_y: value.hit_test.query_y,
    })
}

fn checked_u32(value: usize) -> Result<u32, KucUnicodeColorGlyphEvidenceError> {
    value
        .try_into()
        .map_err(|_| trace("UTF-8 observation range exceeds the native format"))
}

fn valid_hash(value: &str) -> bool {
    value.len() == SHA256_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
