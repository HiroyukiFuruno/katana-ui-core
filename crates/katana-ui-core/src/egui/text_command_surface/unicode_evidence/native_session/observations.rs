use super::super::crop_observation;
use super::super::{CONTROL_STAR_TEXT, KucUnicodeColorGlyphEvidenceError, STAR_TEXT};
use crate::egui::text_command_surface::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootOutput, KucRootEventBatchForwarder,
};
use crate::native_ime_evidence::{
    AccessKitObservation, Bounds, HitTestObservation, NativeFrameObservations,
    NativeObservationHashes, RgbaCropObservation,
};

type Observation = (
    NativeFrameObservations,
    RgbaCropObservation,
    NativeObservationHashes,
);

pub(super) fn extract(
    root: &EguiTextCommandSurfaceRoot,
    output: &EguiTextCommandSurfaceRootOutput,
    update: &egui::accesskit::TreeUpdate,
    commit: &str,
    serial: u64,
) -> Result<Observation, KucUnicodeColorGlyphEvidenceError> {
    let raster = &output.evidence_text.raster;
    let texture = output.evidence_text.record.texture_bounds;
    let (star, star_hit) = crop_for_target(output, raster, texture, "star", STAR_TEXT)?;
    let (control, control_hit) =
        crop_for_target(output, raster, texture, "control_star", CONTROL_STAR_TEXT)?;
    let hit_tests = vec![star_hit, control_hit];
    let rgba_crop = RgbaCropObservation {
        width: star.bounds.width,
        height: star.bounds.height,
        pixels: star.pixels.clone(),
        control_width: control.bounds.width,
        control_height: control.bounds.height,
        control_pixels: control.pixels.clone(),
        has_colored_pixels: false,
        rgba_sha256: String::new(),
        control_rgba_sha256: String::new(),
    }
    .seal()
    .map_err(super::root_trace)?;
    let frame = output.frame();
    let nodes = update
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::MultilineTextInput)
        .collect::<Vec<_>>();
    let [(id, node)] = nodes.as_slice() else {
        return Err(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode);
    };
    let node_bounds = require(node.bounds(), "native AccessKit bounds missing")?;
    let value = require(node.value(), "native AccessKit text missing")?;
    let frame_observations = NativeFrameObservations {
        final_text: raster.text.clone(),
        committed_text: commit.to_owned(),
        measurement_width: checked_u32(raster.width, "measurement width overflow")?,
        measurement_height: checked_u32(raster.height, "measurement height overflow")?,
        caret: bounds(output.evidence_text.record.frame.selection.caret),
        hit_tests,
        accesskit: AccessKitObservation {
            role: format!("{:?}", node.role()),
            value: value.to_owned(),
            node_id: id.0,
            bounds: Bounds {
                x: node_bounds.x0.max(0.0) as u32,
                y: node_bounds.y0.max(0.0) as u32,
                width: (node_bounds.x1 - node_bounds.x0).max(0.0) as u32,
                height: (node_bounds.y1 - node_bounds.y0).max(0.0) as u32,
            },
        },
        root_id: frame.identity().to_owned(),
        frame_id: format!("{}/{serial}", frame.identity()),
    };
    let (measurement_sha256, caret_sha256, hit_test_sha256, accesskit_sha256) = frame_observations
        .observation_hashes()
        .map_err(super::root_trace)?;
    let face = root.evidence_catalog().emoji_face();
    let file_hash = require(face.raw_file_sha256, "native loaded font hash missing")?;
    let receipt = output
        .events()
        .forward_once(&mut EvidenceSink)
        .map_err(forwarding_error)?;
    let hashes = NativeObservationHashes {
        measurement_sha256,
        caret_sha256,
        hit_test_sha256,
        accesskit_sha256,
        frame_sha256: frame.record_hash().to_owned(),
        root_sha256: frame.rgba_hash().to_owned(),
        receipt_sha256: receipt.correlation_fingerprint().to_owned(),
        font_file_sha256: file_hash.to_hex(),
        font_catalog_sha256: root.evidence_catalog().fingerprint().to_hex(),
        root_id: frame_observations.root_id.clone(),
        frame_id: frame_observations.frame_id.clone(),
    };
    Ok((frame_observations, rgba_crop, hashes))
}

fn crop_for_target(
    output: &EguiTextCommandSurfaceRootOutput,
    raster: &crate::text_raster::PlatformTextRaster,
    texture: crate::render_model::UiRect,
    target: &str,
    text: &str,
) -> Result<
    (
        crate::egui::text_command_surface::unicode_evidence::model::KucRgbaCropEvidence,
        HitTestObservation,
    ),
    KucUnicodeColorGlyphEvidenceError,
> {
    let range = require(
        crop_observation::find_range(&raster.text, text),
        "native star/control scalar range missing",
    )?;
    let local = crop_observation::bounds_for_range(raster, range)?;
    let hit = crop_observation::hit_test_observation(target, raster, local)?;
    let hit_test = HitTestObservation {
        target: hit.target,
        range_start: checked_u32(hit.byte_start, "hit-test start overflow")?,
        range_end: checked_u32(hit.byte_end, "hit-test end overflow")?,
        query_x: hit.query_x,
        query_y: hit.query_y,
    };
    let bounds = super::super::model::KucBounds::new(
        local.x.saturating_add(texture.x.max(0) as u32),
        local.y.saturating_add(texture.y.max(0) as u32),
        local.width,
        local.height,
    );
    let crop = crop_observation::crop_for_composite(
        output.rgba_pixels(),
        output.frame().dimensions().width(),
        bounds,
    )?;
    Ok((crop, hit_test))
}

fn bounds(rect: crate::render_model::UiRect) -> Bounds {
    Bounds {
        x: rect.x.max(0) as u32,
        y: rect.y.max(0) as u32,
        width: rect.width,
        height: rect.height,
    }
}

fn require<T>(
    value: Option<T>,
    message: &'static str,
) -> Result<T, KucUnicodeColorGlyphEvidenceError> {
    value.ok_or_else(|| trace(message))
}

fn checked_u32<T>(value: T, message: &'static str) -> Result<u32, KucUnicodeColorGlyphEvidenceError>
where
    T: TryInto<u32>,
{
    value.try_into().map_err(|_| trace(message))
}

fn forwarding_error<E: std::fmt::Debug>(error: E) -> KucUnicodeColorGlyphEvidenceError {
    trace(&format!("native root forwarding failed: {error:?}"))
}

fn trace(message: &str) -> KucUnicodeColorGlyphEvidenceError {
    KucUnicodeColorGlyphEvidenceError::RootTrace(message.to_owned())
}

struct EvidenceSink;

impl KucRootEventBatchForwarder for EvidenceSink {
    type Error = std::convert::Infallible;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        /* WHY: 計測専用 host は通知を一度消費し、実際の root が発行する forwarding receipt を証跡に結合する。 */
        drop(transport);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_u32_rejects_values_outside_the_evidence_contract() {
        assert_eq!(
            checked_u32(17_usize, "unused").expect("small values fit"),
            17
        );
        assert!(matches!(
            checked_u32(u64::MAX, "width overflow"),
            Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
                if message == "width overflow"
        ));
    }
}
