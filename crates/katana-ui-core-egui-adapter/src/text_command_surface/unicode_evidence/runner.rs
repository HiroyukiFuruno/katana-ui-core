use super::constants::{TRACE_HEIGHT, TRACE_WIDTH};
use super::types::KucUnicodeColorGlyphEvidenceError;
use crate::text_command_surface::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootOutput, TextCommandSurfaceStyle,
};

pub(super) struct CapturedFrame {
    pub output: EguiTextCommandSurfaceRootOutput,
    pub accesskit_update: egui::accesskit::TreeUpdate,
}

pub(super) fn run_frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    style: &TextCommandSurfaceStyle,
    events: Vec<egui::Event>,
) -> Result<CapturedFrame, KucUnicodeColorGlyphEvidenceError> {
    context.enable_accesskit();
    let mut result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(TRACE_WIDTH, TRACE_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui, style)),
    );
    let accesskit_update = full_output.platform_output.accesskit_update.take();
    full_output.textures_delta.clear();
    let output = result
        .ok_or_else(|| KucUnicodeColorGlyphEvidenceError::RootTrace("root frame missing".into()))?
        .map_err(|error| KucUnicodeColorGlyphEvidenceError::RootTrace(error.to_string()))?;
    Ok(CapturedFrame {
        output,
        accesskit_update: accesskit_update.ok_or_else(|| {
            KucUnicodeColorGlyphEvidenceError::RootTrace(
                "retained root did not emit an AccessKit tree update".into(),
            )
        })?,
    })
}

pub(super) fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}
