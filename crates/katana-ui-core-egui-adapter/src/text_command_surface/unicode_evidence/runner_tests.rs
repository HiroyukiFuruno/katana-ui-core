use super::*;
use crate::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceError, EguiTextCommandSurfaceRoot,
    TextCommandSurfaceStyle,
};
use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeFamilyId, CommandChromeToolbar,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

const TEST_VIEWPORT_EXTENT: u32 = 16;

#[test]
fn pointer_button_populates_button_fields() {
    let event = pointer_button(egui::pos2(12.5, 34.0), true);
    assert!(matches!(
        event,
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            ..
        } if pos == egui::pos2(12.5, 34.0)
    ));
}

fn text_surface() -> TextSurface {
    TextSurface::new(TextSurfaceProps::new(
        TextArea::new("kuc-unicode-runner-test").value("capture"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, TEST_VIEWPORT_EXTENT, TEST_VIEWPORT_EXTENT),
    ))
}

fn basic_root() -> Result<EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceError> {
    EguiTextCommandSurfaceRoot::new(EguiTextCommandSurface::new(text_surface()))
}

#[test]
fn run_frame_returns_frame_for_a_non_failing_root() {
    let mut root = basic_root().expect("runner test root");
    let context = egui::Context::default();
    let style = TextCommandSurfaceStyle::standard().expect("runner style");
    let frame = run_frame(&context, &mut root, &style, Vec::new())
        .expect("runner should produce at least one frame");
    assert!(!frame.accesskit_update.nodes.is_empty());
}

#[test]
fn run_frame_rejects_duplicate_command_family_when_root_show_fails() {
    let family = CommandChromeFamilyId::new("kuc-unicode-duplicate");
    let surface = EguiTextCommandSurface::new(text_surface())
        .with_toolbar(
            CommandChromeToolbar::new()
                .command_family(family.clone())
                .action(CommandChromeAction::new("p", "P")),
        )
        .with_floating_toolbar(
            CommandChromeToolbar::new()
                .command_family(family)
                .action(CommandChromeAction::new("f", "F")),
            FloatingCommandToolbarVisibility::Visible,
        );
    let mut root = EguiTextCommandSurfaceRoot::new(surface).expect("duplicate family root");
    let context = egui::Context::default();
    let style = TextCommandSurfaceStyle::standard().expect("runner style");
    let message = match run_frame(&context, &mut root, &style, Vec::new()) {
        Ok(_) => panic!("duplicate family should reject"),
        Err(error) => error.to_string(),
    };
    assert!(message.contains("command family is mounted"));
}
