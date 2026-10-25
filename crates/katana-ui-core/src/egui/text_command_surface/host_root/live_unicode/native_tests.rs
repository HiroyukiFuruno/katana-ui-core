use super::{checked_u32, valid_hash};

use crate::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory,
    KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
};
use crate::egui::text_command_surface::{
    EguiTextCommandSurfacePresentation, TextCommandSurfaceStyle,
};
use crate::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use crate::molecule::selection::ContextMenuEvent;
use crate::render_model::{UiStateId, UiTextSpan};
use crate::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

const OBSERVATION_VIEWPORT_WIDTH: u32 = 1280;
const OBSERVATION_VIEWPORT_HEIGHT: u32 = 720;

#[derive(Default)]
struct Forwarder;

impl KucRootEventBatchDispatcher for Forwarder {
    type Error = String;

    fn dispatch_text_events(&mut self, _: Vec<TextSurfaceEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_toolbar_events(&mut self, _: Vec<CommandChromeToolbarEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_floating_events(
        &mut self,
        _: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_search_events(&mut self, _: Vec<CommandChromeSearchEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_context_menu_events(&mut self, _: Vec<ContextMenuEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_status_bar_events(
        &mut self,
        _: Vec<crate::molecule::StatusBarEvent>,
    ) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_diagnostics_list_events(
        &mut self,
        _: Vec<crate::molecule::DiagnosticsListEvent>,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl KucRootEventBatchForwarder for Forwarder {
    type Error = String;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), String> {
        transport
            .dispatch_once(self)
            .map(|_| ())
            .map_err(|error| format!("{error:?}"))
    }
}

fn frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
) -> crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame {
    let mut output = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(
                    OBSERVATION_VIEWPORT_WIDTH as f32,
                    OBSERVATION_VIEWPORT_HEIGHT as f32,
                ),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                output = Some(root.show(ui).expect("host root frame"));
            });
        },
    );
    full_output.textures_delta.clear();
    output.expect("host root frame")
}

fn root() -> EguiTextCommandSurfaceHostRoot {
    let text = TextSurface::new(TextSurfaceProps::new(
        crate::atom::TextArea::new("native-unit-text").value("日本語 ⭐️ / ☆"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(
            0,
            0,
            OBSERVATION_VIEWPORT_WIDTH,
            OBSERVATION_VIEWPORT_HEIGHT,
        ),
    ));
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("native-unit-text")),
        text: TextSurfacePresentation::from_props(text.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    };
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"native-unit-target".to_vec(),
        presentation,
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("root")
}

#[test]
fn native_hash_and_range_helpers_enforce_the_wire_boundaries() {
    assert!(valid_hash(&"a".repeat(64)));
    assert!(!valid_hash("not-a-sha256"));
    assert_eq!(checked_u32(36).expect("small range"), 36);
    assert_eq!(checked_u32(usize::MAX).is_err(), usize::BITS > 32);
}

#[test]
fn native_observations_reject_corrupted_private_metadata() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = root();
    let observed = frame(&context, &mut root);
    let text = "日本語 ⭐️ / ☆";
    let primary_start = text.find("⭐️").expect("primary");
    let control_start = text.find('☆').expect("control");
    let primary = observed
        .observe_unicode((primary_start, primary_start + "⭐️".len()))
        .expect("primary observation");
    let control = observed
        .observe_unicode((control_start, control_start + "☆".len()))
        .expect("control observation");
    let receipt = observed
        .forward_events_once(&mut Forwarder)
        .expect("forward receipt");
    primary
        .native_observations(&control, &receipt, (0, "日本語".len()))
        .expect("valid native observations");

    let mut missing_font = primary.clone();
    missing_font.font_identity.0 = None;
    assert!(
        missing_font
            .native_observations(&control, &receipt, (0, "日本語".len()))
            .is_err()
    );

    let mut invalid_font = primary.clone();
    invalid_font.font_identity.0 = Some("invalid".into());
    assert!(
        invalid_font
            .native_observations(&control, &receipt, (0, "日本語".len()))
            .is_err()
    );

    let mut missing_accesskit = primary.clone();
    missing_accesskit.accesskit = None;
    assert!(
        missing_accesskit
            .native_observations(&control, &receipt, (0, "日本語".len()))
            .is_err()
    );

    let mut missing_node = primary.clone();
    missing_node.accesskit_node_id = None;
    assert!(
        missing_node
            .native_observations(&control, &receipt, (0, "日本語".len()))
            .is_err()
    );
}
