use super::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactory,
};
use crate::atom::TextArea;
use crate::egui::text_command_surface::{
    EguiTextCommandSurfacePresentation, TabStripCorrelation, TabStripGroupCapabilities,
    TabStripGroupColorPresentation, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripProjection, TabStripProjectionLease, TabStripText, TextCommandSurfaceStyle,
};
use crate::molecule::RgbaColor;
use crate::render_model::{UiRect, UiStateId};
use crate::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;
const RGBA_CHANNELS: usize = 4;
const MARKER: RgbaColor = RgbaColor::new(233, 44, 55, 255);
const OTHER_BACKGROUND: RgbaColor = RgbaColor::new(24, 88, 144, 255);

struct Pixels {
    rgba: Vec<u8>,
    canvas: UiRect,
    first: egui::Rect,
    second: egui::Rect,
}

fn lease(revision: u64, first: RgbaColor) -> EguiTextCommandSurfaceHostProjectionLease {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("color-pixel-body").value("本文⭐️"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, WIDTH, HEIGHT),
    ));
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"color-pixel-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("color-pixel-body")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap();
    let group = |id: &'static [u8], label: &str, color| {
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(id),
            TabStripText::new(label),
        )
        .capabilities(TabStripGroupCapabilities::new().collapsible(true))
        .color_presentation(TabStripGroupColorPresentation::new(
            MARKER, color, color, color,
        ))
    };
    EguiTextCommandSurfaceHostProjectionLease::new(token, |_| Ok(None)).with_tab_strip(
        TabStripProjectionLease::new(
            TabStripProjection::new(revision, TabStripCorrelation::from_opaque_bytes(b"colors"))
                .group(group(b"first", "色A", first))
                .group(group(b"second", "色B", OTHER_BACKGROUND)),
        ),
    )
}

fn frame(context: &egui::Context, root: &mut EguiTextCommandSurfaceHostRoot) -> Pixels {
    context.enable_accesskit();
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(WIDTH as f32, HEIGHT as f32),
            )),
            ..Default::default()
        },
        |ui| frame = Some(root.show(ui).unwrap()),
    );
    output.textures_delta.clear();
    let nodes = &output
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes;
    let bounds = |label: &str| {
        let (_, node) = nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .unwrap();
        let rect = node.bounds().unwrap();
        egui::Rect::from_min_max(
            egui::pos2(rect.x0 as f32, rect.y0 as f32),
            egui::pos2(rect.x1 as f32, rect.y1 as f32),
        )
    };
    let frame = frame.unwrap();
    Pixels {
        rgba: frame.output.rgba_pixels().to_vec(),
        canvas: frame.output.evidence_composite.canvas.ui_rect(),
        first: bounds("色A"),
        second: bounds("色B"),
    }
}

fn pixel(frame: &Pixels, x: i32, y: i32) -> [u8; RGBA_CHANNELS] {
    let column = usize::try_from(x - frame.canvas.x).unwrap();
    let row = usize::try_from(y - frame.canvas.y).unwrap();
    let offset = (row * usize::try_from(frame.canvas.width).unwrap() + column) * RGBA_CHANNELS;
    frame.rgba[offset..offset + RGBA_CHANNELS]
        .try_into()
        .unwrap()
}

fn crop(frame: &Pixels, rect: egui::Rect) -> Vec<[u8; RGBA_CHANNELS]> {
    let mut pixels = Vec::new();
    for y in rect.min.y.ceil() as i32..rect.max.y.floor() as i32 {
        for x in rect.min.x.ceil() as i32..rect.max.x.floor() as i32 {
            pixels.push(pixel(frame, x, y));
        }
    }
    pixels
}

#[test]
fn public_root_color_changes_only_target_pixels_and_composites_alpha() {
    let context = egui::Context::default();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, RgbaColor::new(180, 20, 60, 255)))
        .unwrap();
    let before = frame(&context, &mut root);
    let x = before.first.min.x as i32 + 5;
    let y = before.first.min.y as i32 + 3;
    assert_eq!(pixel(&before, x, y), [180, 20, 60, 255]);
    root.synchronize_with_lease(lease(2, RgbaColor::new(12, 180, 96, 255)))
        .unwrap();
    let after = frame(&context, &mut root);
    assert_eq!(pixel(&after, x, y), [12, 180, 96, 255]);
    assert_eq!(before.first, after.first);
    assert_eq!(before.second, after.second);
    assert_eq!(crop(&before, before.second), crop(&after, after.second));
    let body = egui::Rect::from_min_max(
        egui::pos2(0.0, before.first.max.y + 20.0),
        egui::pos2(600.0, 280.0),
    );
    assert_eq!(crop(&before, body), crop(&after, body));

    root.synchronize_with_lease(lease(3, RgbaColor::new(12, 180, 96, 0)))
        .unwrap();
    let clear = frame(&context, &mut root);
    let base = pixel(&clear, x, y);
    root.synchronize_with_lease(lease(4, RgbaColor::new(12, 180, 96, 128)))
        .unwrap();
    let alpha = frame(&context, &mut root);
    let blended = pixel(&alpha, x, y);
    for channel in 0..3 {
        let expected =
            (u32::from([12_u8, 180, 96][channel]) * 128 + u32::from(base[channel]) * 127) / 255;
        assert!(i32::from(blended[channel]).abs_diff(expected as i32) <= 1);
    }
    assert_eq!(blended[3], 255);
}
