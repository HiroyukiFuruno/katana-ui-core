use super::constants::{TRACE_HEIGHT, TRACE_WIDTH};
use super::types::KucUnicodeColorGlyphEvidenceError;
use crate::egui::text_command_surface::{
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
    let mut result = Err(KucUnicodeColorGlyphEvidenceError::RootTrace(
        "root frame missing".into(),
    ));
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(TRACE_WIDTH, TRACE_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            result = match root.show(ui, style) {
                Ok(output) => Ok(output),
                Err(error) => Err(KucUnicodeColorGlyphEvidenceError::RootTrace(
                    error.to_string(),
                )),
            };
        },
    );
    let missing_accesskit_update = KucUnicodeColorGlyphEvidenceError::RootTrace(
        "retained root did not emit an AccessKit tree update".into(),
    );
    let accesskit_update = full_output.platform_output.accesskit_update.take();
    let accesskit_update = accesskit_update.ok_or(missing_accesskit_update)?;
    full_output.textures_delta.clear();
    let output = result?;
    Ok(CapturedFrame {
        output,
        accesskit_update,
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

#[cfg(test)]
#[path = "runner_tests.rs"]
mod tests;
