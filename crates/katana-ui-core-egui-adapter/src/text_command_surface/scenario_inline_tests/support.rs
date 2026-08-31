use super::*;
use crate::text_command_surface::EguiTextCommandSurfaceRootFactory;
use crate::text_command_surface::TabStripProposalOperation;
use crate::text_surface::{EguiTextSurfaceDrawLayer, TextSurfacePaintOperationKind};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdownCloseReason, CommandChromeToolbarEvent,
};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressEvent, SourceAddressPresentation, SourceAddressStrip,
};

fn render(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    stage: Option<&FullTextCommandSurfaceRawInputStage>,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut input = egui::RawInput::default();
    if let Some(stage) = stage {
        stage.apply_to(&mut input);
    }
    let mut output = None;
    let mut frame = context.run_ui(input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    frame.textures_delta.clear();
    output.expect("root frame").expect("root render")
}

fn render_input(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut output = None;
    let mut frame = context.run_ui(input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    frame.textures_delta.clear();
    output.expect("root frame").expect("root render")
}

fn render_public(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceHostRootFrame {
    let mut frame = None;
    let mut output = context.run_ui(input, |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            frame = Some(root.show(ui));
        });
    });
    output.textures_delta.clear();
    frame
        .expect("public root frame")
        .expect("public root render")
}

fn public_motion_input() -> egui::RawInput {
    let mut input = egui::RawInput::default();
    stage(Vec::new(), 1.0).apply_to(&mut input);
    input
}

fn retained(
    id: FullTextCommandSurfaceScenarioId,
) -> crate::text_command_surface::EguiTextCommandSurfaceHostRoot {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(id)
        .expect("scenario issues");
    EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("scenario lease"))
        .expect("scenario root retains")
}
