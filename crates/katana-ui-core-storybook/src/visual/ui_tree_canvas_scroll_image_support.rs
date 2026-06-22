use super::canvas::Canvas;
use katana_ui_core::atom::{Button, Text};
use katana_ui_core::layout::{Row, Stack};
use katana_ui_core::render_model::{
    UiDimension, UiHostActionSpec, UiNode, UiNodeKind, UiPosition, UiScrollAreaProps,
};

const RGBA_CHANNELS: usize = 4;
type RgbaPixel = [u8; RGBA_CHANNELS];
const DEEP_OVERLAY_CONTROL_SIZE: u16 = 28;
const DEEP_OVERLAY_MARGIN: u16 = 12;
const OFFSCREEN_CLIPPED_HEIGHT: u16 = 12;
const OFFSCREEN_BODY_HEIGHT: u16 = 64;
const OFFSCREEN_CONTENT_HEIGHT: u32 = 136;
const OFFSCREEN_SPACER_HEIGHT: u16 = 40;

pub(super) const OFFSCREEN_LAYOUT_BUTTON_ID: &str = "offscreen-copy";

pub(super) fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

pub(super) fn hover_border_count(
    canvas: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) -> usize {
    let right = x.saturating_add(width).min(canvas.width());
    let bottom = y.saturating_add(height).min(canvas.height());
    (y..bottom)
        .flat_map(|row| (x..right).map(move |column| row * canvas.width() + column))
        .filter(|index| canvas.pixels()[*index] == color)
        .count()
}

pub(super) fn striped_image_rgba(
    width: usize,
    height: u32,
    bytes_per_pixel: usize,
    upper_row_limit: u32,
    upper_rgba: RgbaPixel,
    lower_rgba: RgbaPixel,
) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(width * height as usize * bytes_per_pixel);
    for y in 0..height {
        let color = if y < upper_row_limit {
            upper_rgba
        } else {
            lower_rgba
        };
        for _ in 0..width {
            rgba.extend_from_slice(&color);
        }
    }
    rgba
}

pub(super) fn offscreen_incremental_layout_root(
    viewport_width: u32,
    viewport_height: u32,
) -> UiNode {
    let clipped_layout: UiNode = UiNode::new(UiNodeKind::Column, "")
        .height(UiDimension::Px(OFFSCREEN_CLIPPED_HEIGHT))
        .child(UiNode::from(Text::new(
            "this child is taller than its clipped parent",
        )));
    let button: UiNode = UiNode::from(
        Button::new("C").host_action(UiHostActionSpec::command("copy-code", "Copy code")),
    )
    .stable_node_id(OFFSCREEN_LAYOUT_BUTTON_ID)
    .width(UiDimension::Px(DEEP_OVERLAY_CONTROL_SIZE))
    .height(UiDimension::Px(DEEP_OVERLAY_CONTROL_SIZE));
    let stack: UiNode = UiNode::from(
        Stack::new()
            .child(UiNode::from(Text::new("body")).height(UiDimension::Px(OFFSCREEN_BODY_HEIGHT)))
            .child(
                UiNode::from(Row::new().child(button))
                    .position(UiPosition::Absolute)
                    .margin(katana_ui_core::render_model::UiEdgeInsets {
                        top: UiDimension::Px(DEEP_OVERLAY_MARGIN),
                        right: UiDimension::Px(DEEP_OVERLAY_MARGIN),
                        ..Default::default()
                    }),
            ),
    )
    .height(UiDimension::Px(OFFSCREEN_BODY_HEIGHT));
    UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width,
            viewport_height,
            content_height: OFFSCREEN_CONTENT_HEIGHT,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(clipped_layout)
                .child(
                    UiNode::new(UiNodeKind::Stack, "")
                        .height(UiDimension::Px(OFFSCREEN_SPACER_HEIGHT)),
                )
                .child(stack),
        )
}
