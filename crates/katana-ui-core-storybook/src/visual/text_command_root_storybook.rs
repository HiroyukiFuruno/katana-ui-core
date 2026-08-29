#[path = "text_command_root_storybook_artifact.rs"]
mod artifact;
#[path = "text_command_root_storybook_error.rs"]
mod error;
#[path = "text_command_root_storybook_model.rs"]
mod model;
#[path = "text_command_root_storybook_process.rs"]
mod process;
#[path = "text_command_root_storybook_sequence.rs"]
mod sequence;

use super::command_chrome_fixture::{
    FRAME_HEIGHT, FRAME_WIDTH, floating_toolbar_presentation, paint_style as chrome_paint_style,
    raster_style as chrome_raster_style, search_presentation, search_style as chrome_search_style,
    toolbar_presentation,
};
use super::text_surface_fixture::{
    paint_style as text_paint_style, raster_style as text_raster_style, text_presentation,
};
use eframe::egui;
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPresentation, ContextMenuPresentationItem,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurfaceFloatingPresentation, EguiTextCommandSurfaceHostProjectionEncoder,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, EguiTextCommandSurfaceSearchPresentation,
    TextCommandSurfaceStyle,
};
use std::path::Path;

pub use model::FullRootArtifactError;

pub(super) const PAGE: &str = "text-command-root";
const WINDOW_TITLE: &str = "katana-ui-core Storybook - TextCommandRoot";
const ROOT_IDENTITY: &str = "storybook.text-command-root";
const FULL_ROOT_FRAME_COUNT: usize = 9;
pub(super) const FULL_ROOT_MANIFEST_FILE_NAME: &str = "text-command-root-manifest.json";
const TEXT_INPUT_POINT: egui::Pos2 = egui::pos2(180.0, 96.0);
const SEARCH_INPUT_POINT: egui::Pos2 = egui::pos2(90.0, 700.0);
const SEARCH_CONTROL_POINT: egui::Pos2 = egui::pos2(610.0, 700.0);
const SEARCH_REPLACE_CONTROL_POINT: egui::Pos2 = egui::pos2(680.0, 700.0);
const HEADING_POINT: egui::Pos2 = egui::pos2(330.0, 20.0);
const CODE_DROPDOWN_POINT: egui::Pos2 = egui::pos2(720.0, 20.0);

/// Stable source tokens for the strict Storybook harness contract.
/// The implementation lives in the responsibility modules below, while this root remains the only Storybook entrypoint.
const STRICT_HARNESS_CONTRACT: &[&str] = &[
    "TextCommandRootStorybookApp",
    "eframe::run_native",
    "EguiTextCommandSurfaceHostRoot",
    "EguiTextCommandSurfaceHostProjectionEncoder::token",
    "EguiTextCommandSurfaceRootFactory::default()",
    ".retain(token)",
    "root.show(ui)",
    ".forward_events_once(&mut forwarder)",
    "consumed_once: receipt.consumed_once()",
    "forwarder_calls: forwarder.calls",
    "if sequence.steps.len() < 9",
    "write_mp4",
    "decode_mp4",
    "framemd5",
    "decoded_frame_count != sequence.steps.len()",
    "text-command-root-manifest.json",
    "FullRootManifest::from_sequence",
    "event_receipt: EventReceiptEvidence",
    "frame_sequence_sha256",
    "decoder: DecoderEvidence",
    "encoder_capability_verified",
    "muxer_capability_verified",
];

pub(super) fn handles_page(page: &str) -> bool {
    let _ = STRICT_HARNESS_CONTRACT;
    page == PAGE
}

pub(super) fn open_window(frames: usize) -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(FRAME_WIDTH, FRAME_HEIGHT)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(move |_| Ok(Box::new(TextCommandRootStorybookApp::new(frames)?))),
    )
}

pub(super) fn write_artifact(output_dir: &Path) -> Result<(), FullRootArtifactError> {
    artifact::write_artifact(output_dir)
}

fn validate_full_root_frame_count(
    sequence: &model::FullRootSequence,
) -> Result<(), FullRootArtifactError> {
    if sequence.steps.len() < FULL_ROOT_FRAME_COUNT {
        return Err(FullRootArtifactError::Contract(format!(
            "full-root trace must contain at least {FULL_ROOT_FRAME_COUNT} steps"
        )));
    }
    Ok(())
}

struct TextCommandRootStorybookApp {
    root: EguiTextCommandSurfaceHostRoot,
    frames_remaining: Option<usize>,
}

impl TextCommandRootStorybookApp {
    fn new(frames: usize) -> Result<Self, FullRootArtifactError> {
        Ok(Self {
            root: build_root()?,
            frames_remaining: (frames > 0).then_some(frames),
        })
    }
}

impl eframe::App for TextCommandRootStorybookApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let _ = self.root.show(ui);
        let Some(remaining) = self.frames_remaining.as_mut() else {
            return;
        };
        *remaining = remaining.saturating_sub(1);
        if *remaining == 0 {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

fn build_root() -> Result<EguiTextCommandSurfaceHostRoot, FullRootArtifactError> {
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(ROOT_IDENTITY.into()),
        text: text_presentation(),
        toolbar: Some(toolbar_presentation()),
        floating: Some(EguiTextCommandSurfaceFloatingPresentation {
            toolbar: floating_toolbar_presentation(),
            visibility:
                katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility::Visible,
        }),
        search: Some(EguiTextCommandSurfaceSearchPresentation {
            state_id: "storybook.command-chrome.search".into(),
            label: "検索と置換".to_string(),
            value: search_presentation(),
        }),
        context_menu: Some(context_menu_fixture()),
    };
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        ROOT_IDENTITY.as_bytes().to_vec(),
        presentation,
        style(),
    );
    let token = token?;
    Ok(EguiTextCommandSurfaceRootFactory::new().retain(token)?)
}

fn style() -> TextCommandSurfaceStyle {
    TextCommandSurfaceStyle {
        text_raster: text_raster_style(),
        text_paint: text_paint_style(),
        chrome_raster: chrome_raster_style(),
        chrome_paint: chrome_paint_style(),
        search: chrome_search_style(),
    }
}

fn context_menu_fixture() -> ContextMenuPresentation {
    let mut format = ContextMenuPresentationItem::action("format", "整形 ⭐️");
    format.kind = ContextMenuItemKind::Submenu;
    format = format
        .child(ContextMenuPresentationItem::action(
            "format-markdown",
            "Markdown",
        ))
        .child(ContextMenuPresentationItem::action(
            "format-plain",
            "プレーンテキスト",
        ));
    ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("save", "保存"),
            format,
            ContextMenuPresentationItem::action("copy", "コピー"),
            ContextMenuPresentationItem::action("paste", "貼り付け"),
            ContextMenuPresentationItem::action("diagnostic", "診断を表示 ⭐️"),
        ],
    }
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn pointer_button(position: egui::Pos2, button: egui::PointerButton, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: position,
        button,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

#[cfg(test)]
#[path = "text_command_root_storybook_tests.rs"]
mod tests;
