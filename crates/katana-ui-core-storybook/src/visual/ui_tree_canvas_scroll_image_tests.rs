use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea, UiTreeSurfaceHost};
use crate::test_assert::KucTestExpect;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::visual::ui_tree_canvas_scroll_image_support::{
    OFFSCREEN_LAYOUT_BUTTON_ID, hover_border_count, offscreen_incremental_layout_root, pixel_at,
    striped_image_rgba,
};
use katana_ui_core::atom::{Button, ImageSurface, Text};
use katana_ui_core::layout::{Row, Stack};
use katana_ui_core::render_model::{
    UiDimension, UiHostActionSpec, UiNode, UiNodeId, UiNodeKind, UiPosition, UiScrollAreaProps,
    UiTree, UiVisualRole,
};
use katana_ui_core::theme::ThemeSnapshot;

const IMAGE_TEST_VIEWPORT_WIDTH: usize = 16;
const IMAGE_TEST_VIEWPORT_HEIGHT: usize = 8;
const IMAGE_TEST_CONTENT_HEIGHT: u32 = 160;
const IMAGE_TEST_SCROLL_Y: u32 = 120;
const IMAGE_TEST_SAMPLE_X: usize = 4;
const IMAGE_TEST_SAMPLE_Y: usize = 4;
const IMAGE_TEST_LOWER_COLOR: u32 = 0x18c964;
const IMAGE_TEST_CONTROL_SIZE: u16 = 8;
const IMAGE_TEST_RGBA_BYTES: usize = 4;
const IMAGE_TEST_UPPER_ROW_LIMIT: u32 = 120;
const IMAGE_TEST_UPPER_RGBA: [u8; IMAGE_TEST_RGBA_BYTES] = [0x20, 0x24, 0x28, 0xff];
const IMAGE_TEST_LOWER_RGBA: [u8; IMAGE_TEST_RGBA_BYTES] = [0x18, 0xc9, 0x64, 0xff];
const DEEP_OVERLAY_VIEWPORT_WIDTH: usize = 180;
const DEEP_OVERLAY_VIEWPORT_HEIGHT: usize = 120;
const DEEP_OVERLAY_SCROLL_Y: u32 = 200;
const DEEP_OVERLAY_SPACER_HEIGHT: u16 = 260;
const DEEP_OVERLAY_STACK_HEIGHT: u16 = 64;
const DEEP_OVERLAY_CONTROL_SIZE: u16 = 28;
const DEEP_OVERLAY_MARGIN: u16 = 12;
const OFFSCREEN_SCROLL_Y: f32 = 40.0;
const DEEP_OVERLAY_BUTTON_ID: &str = "deep-copy";

#[test]
fn deep_partial_image_scroll_renders_visible_slice_from_viewport_sized_canvas() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        IMAGE_TEST_VIEWPORT_WIDTH,
        IMAGE_TEST_VIEWPORT_HEIGHT,
        palette.background,
    );
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: IMAGE_TEST_VIEWPORT_WIDTH as u32,
            viewport_height: IMAGE_TEST_VIEWPORT_HEIGHT as u32,
            offset_y: IMAGE_TEST_SCROLL_Y,
            content_height: IMAGE_TEST_CONTENT_HEIGHT,
            ..UiScrollAreaProps::default()
        })
        .child(deep_scroll_image_node());

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: IMAGE_TEST_VIEWPORT_WIDTH,
            height: IMAGE_TEST_VIEWPORT_HEIGHT,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        Some(IMAGE_TEST_LOWER_COLOR),
        pixel_at(&canvas, IMAGE_TEST_SAMPLE_X, IMAGE_TEST_SAMPLE_Y),
        "deep partial image scroll must render the lower visible slice, not the image top"
    );
}

#[test]
fn partial_media_frame_stack_renders_visible_image_slice() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        IMAGE_TEST_VIEWPORT_WIDTH,
        IMAGE_TEST_VIEWPORT_HEIGHT,
        palette.background,
    );
    let root = media_frame_stack_with_overlay_control();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: IMAGE_TEST_VIEWPORT_WIDTH,
            height: IMAGE_TEST_VIEWPORT_HEIGHT,
            scroll_y: IMAGE_TEST_SCROLL_Y as f32,
        },
    );

    assert_eq!(
        Some(IMAGE_TEST_LOWER_COLOR),
        pixel_at(&canvas, IMAGE_TEST_SAMPLE_X, IMAGE_TEST_SAMPLE_Y),
        "partial MediaFrame Stack must render the visible image slice, not the image top"
    );
}

#[test]
fn deep_scroll_absolute_overlay_hover_border_matches_host_action_hit_rect() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let root = deep_scroll_absolute_overlay_root();
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: DEEP_OVERLAY_VIEWPORT_WIDTH,
        height: DEEP_OVERLAY_VIEWPORT_HEIGHT,
        scroll_y: DEEP_OVERLAY_SCROLL_Y as f32,
    };
    let hits = UiTreeSurfaceHost::new(theme.clone()).viewport_host_action_hits(&root, area);
    let hit = hits
        .iter()
        .find(|hit| hit.action.target.as_str() == DEEP_OVERLAY_BUTTON_ID)
        .kuc_expect("deep overlay host action hit must be present");
    let hovered =
        UiTree::new(root).with_hovered_node_id(Some(&UiNodeId::new(DEEP_OVERLAY_BUTTON_ID)));
    let mut canvas = Canvas::new(
        DEEP_OVERLAY_VIEWPORT_WIDTH,
        DEEP_OVERLAY_VIEWPORT_HEIGHT,
        palette.background,
    );

    UiTreeCanvasRenderer::new(theme).render(&mut canvas, hovered.root(), area);

    assert_eq!(
        Some(palette.visual.hover_border),
        pixel_at(&canvas, hit.rect.x, hit.rect.y),
        "deep overlay hover border must be painted at the same y as the KUC host action hit rect"
    );
}

#[test]
fn offscreen_incremental_layout_uses_measured_height_for_overlay_hit_alignment() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let root = offscreen_incremental_layout_root(
        DEEP_OVERLAY_VIEWPORT_WIDTH as u32,
        DEEP_OVERLAY_VIEWPORT_HEIGHT as u32,
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: DEEP_OVERLAY_VIEWPORT_WIDTH,
        height: DEEP_OVERLAY_VIEWPORT_HEIGHT,
        scroll_y: OFFSCREEN_SCROLL_Y,
    };
    let hits = UiTreeSurfaceHost::new(theme.clone()).viewport_host_action_hits(&root, area);
    let hit = hits
        .iter()
        .find(|hit| hit.action.target.as_str() == OFFSCREEN_LAYOUT_BUTTON_ID)
        .kuc_expect("offscreen layout host action hit must be present");
    let hovered =
        UiTree::new(root).with_hovered_node_id(Some(&UiNodeId::new(OFFSCREEN_LAYOUT_BUTTON_ID)));
    let mut canvas = Canvas::new(
        DEEP_OVERLAY_VIEWPORT_WIDTH,
        DEEP_OVERLAY_VIEWPORT_HEIGHT,
        palette.background,
    );

    UiTreeCanvasRenderer::new(theme).render(&mut canvas, hovered.root(), area);

    assert!(
        hover_border_count(
            &canvas,
            hit.rect.x,
            hit.rect.y,
            hit.rect.width,
            hit.rect.height,
            palette.visual.hover_border
        ) > 0,
        "offscreen incremental layout must skip by measured height before painting overlay controls"
    );
}

fn deep_scroll_image_node() -> UiNode {
    let image: UiNode = ImageSurface::from_rgba(
        "image",
        "image",
        IMAGE_TEST_VIEWPORT_WIDTH as u32,
        IMAGE_TEST_CONTENT_HEIGHT,
        test_striped_image_rgba(),
    )
    .kuc_expect("test image must be valid")
    .into();
    image.height(UiDimension::Px(IMAGE_TEST_CONTENT_HEIGHT as u16))
}

fn media_frame_stack_with_overlay_control() -> UiNode {
    let image: UiNode = ImageSurface::from_rgba(
        "image",
        "image",
        IMAGE_TEST_VIEWPORT_WIDTH as u32,
        IMAGE_TEST_CONTENT_HEIGHT,
        test_striped_image_rgba(),
    )
    .kuc_expect("test image must be valid")
    .into();
    let image = image.height(UiDimension::Px(IMAGE_TEST_CONTENT_HEIGHT as u16));
    let control: UiNode = UiNode::from(Button::new("Z"))
        .position(UiPosition::Absolute)
        .width(UiDimension::Px(IMAGE_TEST_CONTROL_SIZE))
        .height(UiDimension::Px(IMAGE_TEST_CONTROL_SIZE));
    UiNode::from(Stack::new().child(image).child(control))
        .height(UiDimension::Px(IMAGE_TEST_CONTENT_HEIGHT as u16))
        .visual_role(UiVisualRole::MediaFrame)
}

fn deep_scroll_absolute_overlay_root() -> UiNode {
    let spacer =
        UiNode::new(UiNodeKind::Stack, "").height(UiDimension::Px(DEEP_OVERLAY_SPACER_HEIGHT));
    UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: DEEP_OVERLAY_VIEWPORT_WIDTH as u32,
            viewport_height: DEEP_OVERLAY_VIEWPORT_HEIGHT as u32,
            offset_y: 0,
            content_height: u32::from(DEEP_OVERLAY_SPACER_HEIGHT + DEEP_OVERLAY_STACK_HEIGHT),
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(spacer)
                .child(deep_scroll_absolute_overlay_stack()),
        )
}

fn deep_scroll_absolute_overlay_stack() -> UiNode {
    let body: UiNode =
        UiNode::from(Text::new("body")).height(UiDimension::Px(DEEP_OVERLAY_STACK_HEIGHT));
    let button: UiNode = UiNode::from(
        Button::new("C").host_action(UiHostActionSpec::command("copy-code", "Copy code")),
    )
    .stable_node_id(DEEP_OVERLAY_BUTTON_ID)
    .position(UiPosition::Absolute)
    .width(UiDimension::Px(DEEP_OVERLAY_CONTROL_SIZE))
    .height(UiDimension::Px(DEEP_OVERLAY_CONTROL_SIZE));
    let controls: UiNode = UiNode::from(Row::new().child(button))
        .stable_node_id(DEEP_OVERLAY_BUTTON_ID)
        .host_action(UiHostActionSpec::command("copy-code", "Copy code"))
        .position(UiPosition::Absolute)
        .margin(katana_ui_core::render_model::UiEdgeInsets {
            top: UiDimension::Px(DEEP_OVERLAY_MARGIN),
            right: UiDimension::Px(DEEP_OVERLAY_MARGIN),
            ..Default::default()
        });
    UiNode::from(Stack::new().child(body).child(controls))
        .height(UiDimension::Px(DEEP_OVERLAY_STACK_HEIGHT))
}

fn test_striped_image_rgba() -> Vec<u8> {
    striped_image_rgba(
        IMAGE_TEST_VIEWPORT_WIDTH,
        IMAGE_TEST_CONTENT_HEIGHT,
        IMAGE_TEST_RGBA_BYTES,
        IMAGE_TEST_UPPER_ROW_LIMIT,
        IMAGE_TEST_UPPER_RGBA,
        IMAGE_TEST_LOWER_RGBA,
    )
}
