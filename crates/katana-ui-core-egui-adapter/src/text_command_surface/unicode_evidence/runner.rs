use super::constants::{TRACE_HEIGHT, TRACE_WIDTH};
use super::types::KucUnicodeColorGlyphEvidenceError;
use crate::text_command_surface::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootOutput, TextCommandSurfaceStyle,
};

pub(super) fn run_frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    style: &TextCommandSurfaceStyle,
    events: Vec<egui::Event>,
) -> Result<EguiTextCommandSurfaceRootOutput, KucUnicodeColorGlyphEvidenceError> {
    let mut result = None;
    let mut output = context.run_ui(
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
    output.textures_delta.clear();
    let frame = result.ok_or(KucUnicodeColorGlyphEvidenceError::RootTrace(
        "root frame missing".into(),
    ))?;
    frame.map_err(root_trace_error)
}

fn root_trace_error(
    error: crate::text_command_surface::EguiTextCommandSurfaceRootError,
) -> KucUnicodeColorGlyphEvidenceError {
    KucUnicodeColorGlyphEvidenceError::RootTrace(error.to_string())
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
mod tests {
    use super::*;

    #[test]
    fn root_trace_errors_remain_closed() {
        let error = root_trace_error(
            crate::text_command_surface::EguiTextCommandSurfaceRootError::Serialization(
                "opaque".into(),
            ),
        );
        assert!(matches!(
            error,
            KucUnicodeColorGlyphEvidenceError::RootTrace(_)
        ));
    }
}
