use super::constants::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, INITIAL_TEXT, STAR_TEXT, TRACE_POINTER_X,
    TRACE_POINTER_Y, ZWJ_TEXT,
};
use super::crop_observation;
use super::model::{
    KucCaretObservation, KucImeTraceEvidence, KucUnicodeColorGlyphEvidence,
    KucUnicodeColorGlyphEvidenceCapture, KucUnicodeColorGlyphEvidenceInput,
    KucUnicodeColorGlyphEvidenceOptions,
};
use super::runner;
use super::surface;
use super::types::KucUnicodeColorGlyphEvidenceError;
use crate::text_command_surface::{EguiTextCommandSurface, EguiTextCommandSurfaceRoot};
use katana_ui_core_text_raster::{
    PlatformColorEmojiAvailability, PlatformFontCatalog, PlatformFontCatalogPolicy,
};

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
        );
        let context = egui::Context::default();
        ensure_face_is_resolved_and_pinned(root.evidence_catalog(), &policy)?;
        let _initial = runner::run_frame(&context, &mut root, &style, Vec::new())?;
        let press = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![runner::pointer_button(
                egui::pos2(TRACE_POINTER_X, TRACE_POINTER_Y),
                true,
            )],
        );
        let _press = press?;
        let release = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![runner::pointer_button(
                egui::pos2(TRACE_POINTER_X, TRACE_POINTER_Y),
                false,
            )],
        );
        let _release = release?;
        let preedit_result = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![egui::Event::Ime(egui::ImeEvent::Preedit {
                text: IME_PREEDIT_TEXT.to_string(),
                active_range_chars: None,
            })],
        );
        let preedit = preedit_result?;
        let committed_result = runner::run_frame(
            &context,
            &mut root,
            &style,
            vec![egui::Event::Ime(egui::ImeEvent::Commit(
                IME_COMMIT_TEXT.to_string(),
            ))],
        );
        let committed = committed_result?;
        let final_text = format!("{INITIAL_TEXT}{IME_COMMIT_TEXT}");
        let star_range = required_range(&final_text, STAR_TEXT)?;
        let control_range = required_range(&final_text, CONTROL_STAR_TEXT)?;
        let zwj_range = required_range(&final_text, ZWJ_TEXT)?;
        let raster = &committed.evidence_text.raster;
        let texture_bounds = committed.evidence_text.record.texture_bounds;
        let offset_bounds = |range| {
            let mut bounds = crop_observation::bounds_for_range(raster, range)?;
            bounds.x = bounds.x.saturating_add(texture_bounds.x.max(0) as u32);
            bounds.y = bounds.y.saturating_add(texture_bounds.y.max(0) as u32);
            Ok(bounds)
        };
        let star_bounds = offset_bounds(star_range)?;
        let control_bounds = offset_bounds(control_range)?;
        let zwj_bounds = offset_bounds(zwj_range)?;
        let canvas_width = committed.evidence_composite.canvas.ui_rect().width;
        let star_crop_result = crop_observation::crop_for_composite(
            &committed.evidence_composite.rgba_pixels,
            canvas_width,
            star_bounds,
        );
        let star_crop = star_crop_result?;
        let control_crop_result = crop_observation::crop_for_composite(
            &committed.evidence_composite.rgba_pixels,
            canvas_width,
            control_bounds,
        );
        let control_crop = control_crop_result?;
        let hit_tests = vec![
            crop_observation::hit_test_observation("star", raster, star_bounds)?,
            crop_observation::hit_test_observation("control_star", raster, control_bounds)?,
            crop_observation::hit_test_observation("zwj", raster, zwj_bounds)?,
        ];
        let preedit_event_seen = preedit.evidence_text.events.iter().any(|event| {
            matches!(
                event,
                katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                    katana_ui_core::atom::TextAreaEvent::ImeComposition(value)
                ) if value.preedit == IME_PREEDIT_TEXT
            )
        });
        let commit_event_seen = committed.evidence_text.events.iter().any(|event| {
            matches!(
                event,
                katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                    katana_ui_core::atom::TextAreaEvent::ImeCommit(value)
                ) if value == IME_COMMIT_TEXT
            )
        });
        let final_frame = committed.frame();
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
                committed.evidence_text.record.frame.selection.caret,
            ),
            hit_tests,
            star_crop,
            control_crop,
            accesskit_text_snapshot_hash: final_frame.accessibility().snapshot_hash().to_string(),
            root_frame_hash: final_frame.record_hash().to_string(),
            root_record_hash: committed.evidence_text.artifact.frame_record_hash.clone(),
            root_rgba_hash: final_frame.rgba_hash().to_string(),
        };
        super::validation::KucUnicodeColorGlyphEvidenceBuilder::build(input)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core_text_raster::{
        PlatformEmojiFontCandidate, PlatformFontProfile, PlatformFontSha256,
    };

    #[test]
    fn required_range_returns_exact_grapheme_and_typed_missing_error() {
        assert_eq!(required_range("a⭐️b", "⭐️").ok(), Some((1, 7)));
        assert!(matches!(
            required_range("a⭐️b", "missing"),
            Err(KucUnicodeColorGlyphEvidenceError::RootTrace(error))
                if error == "missing range missing"
        ));
    }

    #[test]
    fn unresolved_catalog_is_rejected_before_root_trace_capture() {
        let policy = PlatformFontCatalogPolicy::new(
            PlatformFontProfile::Unsupported,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let catalog = PlatformFontCatalog::new(policy.clone());
        assert!(matches!(
            ensure_face_is_resolved_and_pinned(&catalog, &policy),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
        ));
    }

    #[test]
    fn errored_and_unpinned_catalogs_fail_closed() {
        let font_path = std::env::temp_dir().join(format!(
            "kuc-unicode-evidence-font-{}.ttf",
            std::process::id()
        ));
        std::fs::write(&font_path, b"font fixture").expect("write font fixture");
        let candidate = PlatformEmojiFontCandidate::new(font_path.clone(), "Noto Color Emoji")
            .with_expected_raw_file_sha256(PlatformFontSha256::digest(b"different"));
        let error_policy = PlatformFontCatalogPolicy::new(
            PlatformFontProfile::Linux,
            Vec::new(),
            Vec::new(),
            vec![candidate],
        );
        let error_catalog = PlatformFontCatalog::new(error_policy.clone());
        assert!(matches!(
            ensure_face_is_resolved_and_pinned(&error_catalog, &error_policy),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError { .. })
        ));
        std::fs::remove_file(&font_path).expect("remove font fixture");

        #[cfg(target_os = "macos")]
        let (profile, system_font, family) = (
            PlatformFontProfile::MacOs,
            std::path::PathBuf::from("/System/Library/Fonts/Apple Color Emoji.ttc"),
            "Apple Color Emoji",
        );
        #[cfg(target_os = "linux")]
        let (profile, system_font, family) = (
            PlatformFontProfile::Linux,
            std::path::PathBuf::from("/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf"),
            "Noto Color Emoji",
        );
        #[cfg(target_os = "windows")]
        let (profile, system_font, family) = (
            PlatformFontProfile::Windows,
            std::path::PathBuf::from("C:/Windows/Fonts/seguiemj.ttf"),
            "Segoe UI Emoji",
        );
        let system_font_bytes = std::fs::read(&system_font).expect("read system emoji font");
        let hash = PlatformFontSha256::digest(&system_font_bytes);
        let resolved_policy = PlatformFontCatalogPolicy::new(
            profile,
            Vec::new(),
            Vec::new(),
            vec![
                PlatformEmojiFontCandidate::new(system_font, family)
                    .with_expected_raw_file_sha256(hash),
            ],
        );
        let resolved_catalog = PlatformFontCatalog::new(resolved_policy.clone());
        assert!(ensure_face_is_resolved_and_pinned(&resolved_catalog, &resolved_policy).is_ok());
        let empty_policy =
            PlatformFontCatalogPolicy::new(profile, Vec::new(), Vec::new(), Vec::new());
        assert!(matches!(
            ensure_face_is_resolved_and_pinned(&resolved_catalog, &empty_policy),
            Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
        ));
    }
}
