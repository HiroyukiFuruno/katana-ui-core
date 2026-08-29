use super::constants::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, INITIAL_TEXT, STAR_TEXT, TRACE_POINTER_X,
    TRACE_POINTER_Y, ZWJ_TEXT,
};
use super::crop_observation;
use super::model::{
    KucCaretObservation, KucImeTraceEvidence, KucUnicodeColorGlyphEvidence,
    KucUnicodeColorGlyphEvidenceInput, KucUnicodeColorGlyphEvidenceOptions,
};
use super::runner;
use super::surface;
use super::types::KucUnicodeColorGlyphEvidenceError;
use crate::text_command_surface::{EguiTextCommandSurface, EguiTextCommandSurfaceRoot};
use katana_ui_core_text_raster::{
    PlatformColorEmojiAvailability, PlatformFontCatalog, PlatformFontCatalogPolicy,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct KucUnicodeColorGlyphEvidenceCapture;

impl KucUnicodeColorGlyphEvidenceCapture {
    pub fn capture(
        options: KucUnicodeColorGlyphEvidenceOptions,
    ) -> Result<KucUnicodeColorGlyphEvidence, KucUnicodeColorGlyphEvidenceError> {
        let policy = options.config.catalog_policy();
        let style = surface::trace_style();
        let mut root = EguiTextCommandSurfaceRoot::with_text_raster_config(
            options.root_identity,
            EguiTextCommandSurface::new(surface::evidence_surface()),
            options.config.clone(),
        )
        .map_err(|error| KucUnicodeColorGlyphEvidenceError::RootTrace(error.to_string()))?;
        let context = egui::Context::default();
        ensure_face_is_resolved_and_pinned(root.evidence_catalog(), &policy)?;
        let _initial = runner::run_frame(&context, &mut root, &style, Vec::new())?;
        let _press = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![runner::pointer_button(
                egui::pos2(TRACE_POINTER_X, TRACE_POINTER_Y),
                true,
            )],
        )?;
        let _release = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![runner::pointer_button(
                egui::pos2(TRACE_POINTER_X, TRACE_POINTER_Y),
                false,
            )],
        )?;
        let preedit = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![egui::Event::Ime(egui::ImeEvent::Preedit(
                IME_PREEDIT_TEXT.to_string(),
            ))],
        )?;
        let committed = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![egui::Event::Ime(egui::ImeEvent::Commit(
                IME_COMMIT_TEXT.to_string(),
            ))],
        )?;
        if committed.output.evidence_composite.rgba_pixels.is_empty() {
            return Err(KucUnicodeColorGlyphEvidenceError::RootTrace(
                "retained root produced an empty RGBA composite".to_string(),
            ));
        }

        let final_text = format!("{INITIAL_TEXT}{IME_COMMIT_TEXT}");
        let star_range = required_range(&final_text, STAR_TEXT)?;
        let control_range = required_range(&final_text, CONTROL_STAR_TEXT)?;
        let zwj_range = required_range(&final_text, ZWJ_TEXT)?;
        let raster = &committed.output.evidence_text.raster;
        let texture_bounds = committed.output.evidence_text.record.texture_bounds;
        let offset_bounds = |range| {
            let mut bounds = crop_observation::bounds_for_range(raster, range)?;
            bounds.x = bounds.x.saturating_add(texture_bounds.x.max(0) as u32);
            bounds.y = bounds.y.saturating_add(texture_bounds.y.max(0) as u32);
            Ok(bounds)
        };
        let star_bounds = offset_bounds(star_range)?;
        let control_bounds = offset_bounds(control_range)?;
        let zwj_bounds = offset_bounds(zwj_range)?;
        let canvas_width = committed.output.evidence_composite.canvas.ui_rect().width;
        let star_crop = crop_observation::crop_for_composite(
            &committed.output.evidence_composite.rgba_pixels,
            canvas_width,
            star_bounds,
        )?;
        let control_crop = crop_observation::crop_for_composite(
            &committed.output.evidence_composite.rgba_pixels,
            canvas_width,
            control_bounds,
        )?;
        let hit_tests = vec![
            crop_observation::hit_test_observation("star", raster, star_bounds)?,
            crop_observation::hit_test_observation("control_star", raster, control_bounds)?,
            crop_observation::hit_test_observation("zwj", raster, zwj_bounds)?,
        ];
        let preedit_event_seen = preedit.output.evidence_text.events.iter().any(|event| {
            matches!(
                event,
                katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                    katana_ui_core::atom::TextAreaEvent::ImeComposition(value)
                ) if value.preedit == IME_PREEDIT_TEXT
            )
        });
        let commit_event_seen = committed.output.evidence_text.events.iter().any(|event| {
            matches!(
                event,
                katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                    katana_ui_core::atom::TextAreaEvent::ImeCommit(value)
                ) if value == IME_COMMIT_TEXT
            )
        });
        let accesskit_text_input = extract_accesskit_text_input(&committed.accesskit_update)?;
        let final_frame = committed.output.frame();
        let input = KucUnicodeColorGlyphEvidenceInput {
            profile: policy.platform_profile,
            catalog_policy: policy,
            face: root.evidence_catalog().emoji_face().clone(),
            final_text,
            ime: KucImeTraceEvidence {
                preedit_scalars: IME_PREEDIT_TEXT.chars().map(u32::from).collect(),
                commit_scalars: IME_COMMIT_TEXT.chars().map(u32::from).collect(),
                preedit_event_seen,
                commit_event_seen,
            },
            caret: KucCaretObservation::from_ui_rect(
                committed.output.evidence_text.record.frame.selection.caret,
            ),
            hit_tests,
            star_crop,
            control_crop,
            accesskit_text_input: Some(accesskit_text_input),
            accesskit_text_snapshot_hash: final_frame.accessibility().snapshot_hash().to_string(),
            root_frame_hash: final_frame.record_hash().to_string(),
            root_record_hash: committed
                .output
                .evidence_text
                .artifact
                .frame_record_hash
                .clone(),
            root_rgba_hash: final_frame.rgba_hash().to_string(),
        };
        super::validation::KucUnicodeColorGlyphEvidenceBuilder::build(input)
    }
}

fn extract_accesskit_text_input(
    update: &egui::accesskit::TreeUpdate,
) -> Result<super::model::KucAccessKitNodeObservation, KucUnicodeColorGlyphEvidenceError> {
    let nodes = update
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::MultilineTextInput)
        .collect::<Vec<_>>();
    let [(node_id, node)] = nodes.as_slice() else {
        return Err(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode);
    };
    let value = node
        .value()
        .ok_or(KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode {
            reason: "multiline text input value is missing",
        })?;
    let bounds = node
        .bounds()
        .ok_or(KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode {
            reason: "multiline text input bounds are missing",
        })?;
    let width = (bounds.x1 - bounds.x0).max(0.0) as u32;
    let height = (bounds.y1 - bounds.y0).max(0.0) as u32;
    Ok(super::model::KucAccessKitNodeObservation {
        node_id: format!("{node_id:?}"),
        role: format!("{:?}", node.role()),
        scalar_sequence: value.chars().map(u32::from).collect(),
        value: value.to_string(),
        bounds: super::model::KucBounds::new(
            bounds.x0.max(0.0) as u32,
            bounds.y0.max(0.0) as u32,
            width,
            height,
        ),
    })
}

fn required_range(
    text: &str,
    target: &str,
) -> Result<(usize, usize), KucUnicodeColorGlyphEvidenceError> {
    crop_observation::find_range(text, target).ok_or_else(|| {
        KucUnicodeColorGlyphEvidenceError::RootTrace(format!("{target} range missing"))
    })
}

fn ensure_face_is_resolved_and_pinned(
    catalog: &PlatformFontCatalog,
    policy: &PlatformFontCatalogPolicy,
) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
    let face = catalog.emoji_face();
    match &face.availability {
        PlatformColorEmojiAvailability::Resolved => {}
        PlatformColorEmojiAvailability::Unavailable(_) => {
            return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable {
                face: Box::new(face.clone()),
            });
        }
        PlatformColorEmojiAvailability::Error(_) => {
            return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError {
                face: Box::new(face.clone()),
            });
        }
    }
    let pinned = face
        .source_file_path
        .as_ref()
        .zip(face.raw_file_sha256)
        .is_some_and(|(path, hash)| {
            policy.emoji_candidates.iter().any(|candidate| {
                candidate.source_file_path == *path
                    && candidate.expected_raw_file_sha256 == Some(hash)
            })
        });
    if pinned {
        Ok(())
    } else {
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned {
            profile_id: policy.platform_profile.as_str().to_string(),
        })
    }
}
