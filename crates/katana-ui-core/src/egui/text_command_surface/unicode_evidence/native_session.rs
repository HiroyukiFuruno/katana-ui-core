mod observations;

use super::{KucUnicodeColorGlyphEvidenceError, KucUnicodeColorGlyphEvidenceOptions, surface};
use crate::egui::text_command_surface::{EguiTextCommandSurface, EguiTextCommandSurfaceRoot};
use crate::native_ime_evidence::{
    NativeFrameObservations, NativeObservationHashes, RgbaCropObservation,
};
use crate::text_raster::{PlatformFontSha256, PlatformTextRasterResources};
use crate::text_surface::{
    TextSurfaceEvent, TextSurfaceFocusRequest, TextSurfaceFocusRequestToken,
    TextSurfacePresentation,
};

/// Retained KUC root used by the OS event-loop producer.
///
/// This adapter accepts egui input for rendering and contract tests. It does not
/// attest native origin. Only a trusted, owned OS event loop can establish that
/// provenance; its transcript is verified separately from these observations.
pub struct KucNativeUnicodeEvidenceSession {
    context: egui::Context,
    root: EguiTextCommandSurfaceRoot,
    frame_serial: u64,
    preedit: Option<String>,
}

/// One root frame and, on an accepted IME commit, its numerical observations.
pub struct KucNativeUnicodeEvidenceFrame {
    pub platform_output: egui::PlatformOutput,
    pub rgba_pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub preedit: Option<String>,
    pub observations: Option<(
        NativeFrameObservations,
        RgbaCropObservation,
        NativeObservationHashes,
    )>,
}

impl KucNativeUnicodeEvidenceSession {
    pub fn new(
        mut options: KucUnicodeColorGlyphEvidenceOptions,
    ) -> Result<Self, KucUnicodeColorGlyphEvidenceError> {
        let (path, hash) = options
            .config
            .emoji_candidates
            .iter()
            .find_map(|path| {
                std::fs::read(path)
                    .ok()
                    .map(|bytes| (path.clone(), PlatformFontSha256::digest(&bytes)))
            })
            .ok_or_else(|| {
                KucUnicodeColorGlyphEvidenceError::RootTrace(
                    "native producer requires a readable emoji font".into(),
                )
            })?;
        options.config.emoji_candidates = vec![path];
        options.config.emoji_candidate_sha256 = vec![hash];
        let mut evidence_surface = surface::evidence_surface();
        let mut presentation = TextSurfacePresentation::from_props(evidence_surface.props());
        presentation.focus_request = Some(TextSurfaceFocusRequest::new(
            TextSurfaceFocusRequestToken::new("kuc-native-ime-evidence-initial-focus"),
            true,
        ));
        evidence_surface.synchronize_presentation(presentation);
        let root = EguiTextCommandSurfaceRoot::with_text_raster_resources(
            options.root_identity,
            EguiTextCommandSurface::new(evidence_surface),
            PlatformTextRasterResources::new(options.config),
        );
        let face = root.evidence_catalog().emoji_face();
        if !matches!(
            face.availability,
            crate::text_raster::PlatformColorEmojiAvailability::Resolved
        ) {
            return Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable {
                face: Box::new(face.clone()),
            });
        }
        let context = egui::Context::default();
        context.enable_accesskit();
        Ok(Self {
            context,
            root,
            frame_serial: 0,
            preedit: None,
        })
    }

    #[must_use]
    pub fn context(&self) -> egui::Context {
        self.context.clone()
    }

    pub fn run_frame(
        &mut self,
        input: egui::RawInput,
    ) -> Result<KucNativeUnicodeEvidenceFrame, KucUnicodeColorGlyphEvidenceError> {
        self.frame_serial = self.frame_serial.checked_add(1).ok_or_else(|| {
            KucUnicodeColorGlyphEvidenceError::RootTrace("native frame serial overflow".into())
        })?;
        let mut rendered = Err(root_trace("native root frame missing"));
        let style = surface::trace_style();
        let mut full_output = self.context.run_ui(input, |ui| {
            rendered = self.root.show(ui, &style).map_err(root_trace);
        });
        /* WHY: 表示は root compositor の RGBA を使うため、egui の texture delta はエラー経路でも明示的に消費する。 */
        full_output.textures_delta.clear();
        let output = rendered?;
        let mut committed = None;
        for event in &output.evidence_text.events {
            match event {
                TextSurfaceEvent::TextArea(crate::atom::TextAreaEvent::ImeComposition(value))
                    if !value.preedit.is_empty() =>
                {
                    self.preedit = Some(value.preedit.clone())
                }
                TextSurfaceEvent::TextArea(crate::atom::TextAreaEvent::ImeCommit(value))
                    if !value.is_empty() =>
                {
                    committed = Some(value.as_str())
                }
                _ => {}
            }
        }
        let measured = if let Some(commit) = committed {
            if self.preedit.is_none() {
                return Err(KucUnicodeColorGlyphEvidenceError::RootTrace(
                    "native commit has no accepted preedit frame".into(),
                ));
            }
            let update = full_output
                .platform_output
                .accesskit_update
                .as_ref()
                .ok_or(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode)?;
            Some(observations::extract(
                &self.root,
                &output,
                update,
                commit,
                self.frame_serial,
            )?)
        } else {
            None
        };
        let dimensions = output.frame().dimensions();
        Ok(KucNativeUnicodeEvidenceFrame {
            platform_output: full_output.platform_output,
            rgba_pixels: output.rgba_pixels().to_vec(),
            width: dimensions.width(),
            height: dimensions.height(),
            preedit: self.preedit.clone(),
            observations: measured,
        })
    }
}

fn root_trace(error: impl std::fmt::Display) -> KucUnicodeColorGlyphEvidenceError {
    KucUnicodeColorGlyphEvidenceError::RootTrace(error.to_string())
}

#[cfg(test)]
mod tests;
