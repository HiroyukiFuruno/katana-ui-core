const RGBA_CHANNELS: usize = 4;

use super::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactory,
};
use crate::atom::TextArea;
use crate::egui::text_command_surface::{
    EguiTextCommandSurfacePresentation, KucGlobalAction, KucGlobalActionTarget,
    KucGlobalActionsProjection, KucGlobalShortcut, TextCommandSurfaceStyle,
};
use crate::render_model::{UiRect, UiStateId};
use crate::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;

struct Pixels {
    rgba: Vec<u8>,
    canvas: UiRect,
    trigger: egui::Rect,
    action: Option<egui::Rect>,
}

fn lease(revision: u64) -> EguiTextCommandSurfaceHostProjectionLease {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("global-pixel").value("本文⭐️"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, WIDTH, HEIGHT),
    ));
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"global-pixel-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("global-pixel")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    let action = KucGlobalAction::new(
        KucGlobalActionTarget::from_opaque_bytes(b"global-pixel-target"),
        "Pixel action",
        KucGlobalShortcut::new(
            egui::Key::F,
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        ),
    );
    EguiTextCommandSurfaceHostProjectionLease::new(token, |_| Ok(None))
        .with_global_actions(KucGlobalActionsProjection::new("Global actions").action(action))
}

fn frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
    scale: f32,
) -> Pixels {
    context.enable_accesskit();
    context.set_pixels_per_point(scale);
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(WIDTH as f32, HEIGHT as f32),
            )),
            events,
            ..Default::default()
        },
        |ui| frame = Some(root.show(ui).expect("frame")),
    );
    output.textures_delta.clear();
    let nodes = &output
        .platform_output
        .accesskit_update
        .expect("AccessKit output")
        .nodes;
    let bounds = |label: &str| {
        nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .and_then(|(_, node)| {
                node.bounds().map(|rect| {
                    egui::Rect::from_min_max(
                        egui::pos2(rect.x0 as f32, rect.y0 as f32),
                        egui::pos2(rect.x1 as f32, rect.y1 as f32),
                    )
                })
            })
    };
    let frame = frame.expect("host frame");
    Pixels {
        rgba: frame.output.rgba_pixels().to_vec(),
        canvas: frame.output.evidence_composite.canvas.ui_rect(),
        trigger: bounds("Global actions").expect("trigger bounds"),
        action: bounds("Pixel action"),
    }
}

fn pixel(frame: &Pixels, x: i32, y: i32) -> [u8; RGBA_CHANNELS] {
    let offset = ((y - frame.canvas.y) as usize * frame.canvas.width as usize
        + (x - frame.canvas.x) as usize)
        * RGBA_CHANNELS;
    frame.rgba[offset..offset + RGBA_CHANNELS]
        .try_into()
        .expect("pixel")
}

fn crop(frame: &Pixels, rect: egui::Rect) -> Vec<[u8; RGBA_CHANNELS]> {
    (rect.min.y as i32..rect.max.y as i32)
        .flat_map(|y| (rect.min.x as i32..rect.max.x as i32).map(move |x| pixel(frame, x, y)))
        .collect()
}

#[path = "global_action_pixel_tests/hidpi.rs"]
mod hidpi;

#[test]
fn global_menu_composite_has_measured_rows_and_removes_action_after_escape() {
    let menu_background = TextCommandSurfaceStyle::standard()
        .expect("style")
        .chrome_paint
        .action_rgba;
    let context = egui::Context::default();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1))
        .expect("root");
    let closed = frame(&context, &mut root, Vec::new(), 1.0);
    assert_eq!(
        pixel(
            &closed,
            closed.trigger.min.x as i32 + 1,
            closed.trigger.min.y as i32 + 1
        ),
        menu_background
    );
    assert!(closed.action.is_none());

    let center = closed.trigger.center();
    let opened = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(center),
            egui::Event::PointerButton {
                pos: center,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: center,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        1.0,
    );
    let action = opened.action.expect("open action bounds");
    assert_eq!(
        pixel(&opened, action.min.x as i32 + 1, action.min.y as i32 + 1),
        menu_background
    );
    assert!(
        crop(&opened, action)
            .iter()
            .any(|value| *value != menu_background)
    );

    let hidden = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        1.0,
    );
    assert!(hidden.action.is_none());
}

#[test]
fn empty_global_menu_label_returns_error_without_host_effect() {
    use std::cell::Cell;
    use std::rc::Rc;
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("global-pixel").value("body"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, WIDTH, HEIGHT),
    ));
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"global-pixel-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("global-pixel")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    let routed = Rc::new(Cell::new(0));
    let observed = Rc::clone(&routed);
    let action = KucGlobalAction::new(
        KucGlobalActionTarget::from_opaque_bytes(b"target"),
        "Action",
        KucGlobalShortcut::new(
            egui::Key::F,
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        ),
    );
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(token, move |_| {
        observed.set(observed.get() + 1);
        Ok(None)
    })
    .with_global_actions(KucGlobalActionsProjection::new("").action(action));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease)
        .expect("root");
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            }],
            ..Default::default()
        },
        |ui| {
            result = Some(root.show(ui));
        },
    );
    output.textures_delta.clear();
    assert!(
        matches!(result, Some(Err(super::EguiTextCommandSurfaceRootFactoryError::Root(message))) if message.contains("platform text raster request must not be empty"))
    );
    assert_eq!(routed.get(), 0);
    assert!(root.process.root.global_renderer.is_some());
}
