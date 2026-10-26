use super::super::super::accesskit_evidence::AccessKitTargetClass;
use super::super::super::root::EguiTextCommandSurfaceRootOutput;
use super::{
    KucAccessKitNodeObservation, KucBounds, KucCaretObservation, KucHitTestObservation,
    KucLiveUnicodeObservation, KucRgbaCropEvidence, KucUnicodeColorGlyphEvidenceError, trace,
};
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
#[path = "capture_tests.rs"]
mod capture_tests;

impl KucLiveUnicodeObservation {
    pub(crate) fn observe(
        output: &EguiTextCommandSurfaceRootOutput,
        range: (usize, usize),
        frame_serial: u64,
        font_identity: &(Option<String>, String),
    ) -> Result<KucLiveUnicodeObservation, KucUnicodeColorGlyphEvidenceError> {
        if output.evidence_text.record.frame.preedit.is_some() {
            return Err(trace("IME composition has not committed document text"));
        }
        let raster = &output.evidence_text.raster;
        validate_range(&raster.text, range)?;
        let glyphs = raster
            .grapheme_bounds
            .iter()
            .filter(|glyph| glyph.byte_start >= range.0 && glyph.byte_end <= range.1)
            .collect::<Vec<_>>();
        let first = glyphs.first().ok_or_else(|| {
            trace(format!(
                "observation glyph range {range:?} is absent (text bytes {}, raster end {:?})",
                raster.text.len(),
                raster.grapheme_bounds.last().map(|glyph| glyph.byte_end)
            ))
        })?;
        let min_x = glyphs
            .iter()
            .map(|glyph| glyph.x)
            .fold(f32::INFINITY, f32::min);
        let min_y = glyphs
            .iter()
            .map(|glyph| glyph.y)
            .fold(f32::INFINITY, f32::min);
        let max_x = glyphs
            .iter()
            .map(|glyph| glyph.x + glyph.width)
            .fold(f32::NEG_INFINITY, f32::max);
        let max_y = glyphs
            .iter()
            .map(|glyph| glyph.y + glyph.height)
            .fold(f32::NEG_INFINITY, f32::max);
        let origin = output.evidence_text.record.texture_bounds;
        let canvas = output.evidence_composite.canvas.ui_rect();
        let x = origin
            .x
            .saturating_add(min_x.floor() as i32)
            .saturating_sub(canvas.x);
        let y = origin
            .y
            .saturating_add(min_y.floor() as i32)
            .saturating_sub(canvas.y);
        let bounds = KucBounds::new(
            u32::try_from(x).map_err(|_| trace("glyph crop is outside the frame"))?,
            u32::try_from(y).map_err(|_| trace("glyph crop is outside the frame"))?,
            (max_x.ceil() - min_x.floor()).max(1.0) as u32,
            (max_y.ceil() - min_y.floor()).max(1.0) as u32,
        );
        let crop = crop_rgba(&output.evidence_composite.rgba_pixels, canvas, bounds)?;
        let local_x = first.x + first.width / 2.0;
        let local_y = first.y + first.height / 2.0;
        let hit = raster
            .hit_test(local_x, local_y)
            .ok_or_else(|| trace("glyph hit test missed"))?;
        if hit.byte_start < range.0 || hit.byte_end > range.1 {
            return Err(trace("glyph hit test range mismatch"));
        }
        let surface = output.evidence_text.record.frame.surface_bounds;
        let accesskit_node_id = output
            .accesskit_evidence
            .iter()
            .find(|entry| {
                entry.target_class == AccessKitTargetClass::TextSurfaceContextTarget
                    && entry.bounds == surface
            })
            .map(|entry| entry.response_id.value());
        let accesskit = output
            .accesskit_text_input_nodes
            .iter()
            .find(|node| {
                node.is_text_input()
                    && node.value.as_deref() == Some(raster.text.as_str())
                    && node.bounds.is_some_and(|bounds| {
                        f64::from_bits(bounds.x0_bits) == f64::from(surface.x)
                            && f64::from_bits(bounds.y0_bits) == f64::from(surface.y)
                    })
            })
            .and_then(|node| {
                let bounds = node.bounds?;
                Some(KucAccessKitNodeObservation {
                    node_id: accesskit_node_id?.to_string(),
                    role: format!("{:?}", node.role),
                    value: node.value.clone()?,
                    scalar_sequence: node.scalar_sequence.clone(),
                    bounds: KucBounds::new(
                        (f64::from_bits(bounds.x0_bits) - f64::from(canvas.x)).max(0.0) as u32,
                        (f64::from_bits(bounds.y0_bits) - f64::from(canvas.y)).max(0.0) as u32,
                        (f64::from_bits(bounds.x1_bits) - f64::from_bits(bounds.x0_bits)).max(0.0)
                            as u32,
                        (f64::from_bits(bounds.y1_bits) - f64::from_bits(bounds.y0_bits)).max(0.0)
                            as u32,
                    ),
                })
            });
        let frame = output.frame();
        let caret = output.evidence_text.record.frame.selection.caret;
        let caret = UiRect::new(
            caret.x.saturating_sub(canvas.x),
            caret.y.saturating_sub(canvas.y),
            caret.width,
            caret.height,
        );
        Ok(KucLiveUnicodeObservation {
            final_text: raster.text.clone(),
            observed_range: range,
            scalars: raster.text[range.0..range.1]
                .chars()
                .map(u32::from)
                .collect(),
            measurement: bounds,
            caret: KucCaretObservation::from_ui_rect(caret),
            hit_test: KucHitTestObservation {
                target: raster.text[hit.byte_start..hit.byte_end].to_owned(),
                query_x: (origin.x as f32 + local_x - canvas.x as f32).max(0.0) as u32,
                query_y: (origin.y as f32 + local_y - canvas.y as f32).max(0.0) as u32,
                byte_start: hit.byte_start,
                byte_end: hit.byte_end,
            },
            crop,
            accesskit,
            accesskit_node_id,
            root_id: frame.identity().to_owned(),
            frame_id: format!("{}:{frame_serial}", frame.identity()),
            root_frame_hash: frame.record_hash().to_owned(),
            root_rgba_hash: frame.rgba_hash().to_owned(),
            root_record_hash: frame.record_hash().to_owned(),
            font_identity: font_identity.clone(),
            frame_serial,
            state_revision: frame.state_revision(),
            event_correlation: output
                .events()
                .current_context()
                .correlation_fingerprint()
                .to_owned(),
            canvas,
        })
    }
}

pub(super) fn validate_range(
    text: &str,
    range: (usize, usize),
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    if range.0 >= range.1 || text.get(range.0..range.1).is_none() {
        return Err(trace("invalid UTF-8 observation range"));
    }
    let boundary = |position| {
        position == text.len()
            || text
                .grapheme_indices(true)
                .any(|(start, _)| start == position)
    };
    if !boundary(range.0) || !boundary(range.1) {
        return Err(trace("observation splits a grapheme"));
    }
    Ok(())
}

fn crop_rgba(
    pixels: &[u8],
    canvas: UiRect,
    bounds: KucBounds,
) -> Result<KucRgbaCropEvidence, KucUnicodeColorGlyphEvidenceError> {
    let right = bounds
        .x
        .checked_add(bounds.width)
        .ok_or_else(|| trace("crop overflow"))?;
    let bottom = bounds
        .y
        .checked_add(bounds.height)
        .ok_or_else(|| trace("crop overflow"))?;
    if right > canvas.width || bottom > canvas.height {
        return Err(trace("crop is not fully visible"));
    }
    let mut output = Vec::new();
    for y in bounds.y..bottom {
        for x in bounds.x..right {
            let offset = (y as usize)
                .checked_mul(canvas.width as usize)
                .and_then(|row| row.checked_add(x as usize))
                .and_then(|pixel| pixel.checked_mul(RGBA_CHANNEL_COUNT))
                .ok_or_else(|| trace("crop overflow"))?;
            let pixel = pixels
                .get(offset..offset + RGBA_CHANNEL_COUNT)
                .ok_or_else(|| trace("crop pixel missing"))?;
            output.push([pixel[0], pixel[1], pixel[2], pixel[RGBA_CHANNEL_COUNT - 1]]);
        }
    }
    Ok(KucRgbaCropEvidence::new(bounds, output))
}
