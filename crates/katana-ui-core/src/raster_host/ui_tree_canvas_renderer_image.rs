use crate::raster_host::UiTreeCanvasRenderer;
use crate::raster_host::canvas::Canvas;
use crate::raster_host::ui_tree_canvas_hit_metrics::{NODE_GAP, dimension_px};
use crate::raster_host::ui_tree_canvas_image_blit::transformed_image_blit_request;
use crate::raster_host::ui_tree_canvas_image_cache::try_blit_cached_image;
use crate::raster_host::ui_tree_canvas_image_metrics::{
    image_target_size, logical_image_extent, logical_image_height_exact, logical_image_width_exact,
};
use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::raster_host::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiNode, UiVisualRole};

const EXPORT_MEDIA_FRAME_TOP_MARGIN: usize = 18;

impl UiTreeCanvasRenderer {
    pub(super) fn draw_image(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let image = &node.props().image_surface;
        let width = logical_image_width_exact(image);
        let height = logical_image_height_exact(image);
        let max_width = area.width.saturating_sub(x.saturating_sub(area.x)).max(1);
        let requested_height = dimension_px(&node.props().common.height);
        let media_frame = matches!(
            node.props().visual_role,
            UiVisualRole::MediaFrame | UiVisualRole::ExportMediaFrame
        );
        let (target_width, target_height) =
            image_target_size(width, height, max_width, requested_height);
        let intrinsic_width = logical_image_extent(image.width, image.content_scale) as f32;
        let reference_capture_raster_extent = media_frame
            && canvas.uses_reference_capture_image_surface_extents()
            && intrinsic_width <= max_width as f32;
        let fixed_display_extent = !reference_capture_raster_extent;
        let draw_width = raster_scaled_extent(
            target_width,
            canvas.scale_factor(),
            canvas.raster_scale_factor(),
            fixed_display_extent,
        );
        let draw_height = raster_scaled_extent(
            target_height,
            canvas.scale_factor(),
            canvas.raster_scale_factor(),
            fixed_display_extent,
        );
        let base_x = if media_frame {
            x.saturating_add(max_width.saturating_sub(draw_width) / 2)
        } else {
            x
        };
        let base_y = if node.props().visual_role == UiVisualRole::ExportMediaFrame {
            (*y).saturating_add(EXPORT_MEDIA_FRAME_TOP_MARGIN)
        } else if media_frame && requested_height > draw_height {
            (*y).saturating_add(requested_height.saturating_sub(draw_height) / 2)
        } else {
            *y
        };
        let scrolled_base_y = (base_y as i64).saturating_sub(area.scroll_y.round().max(0.0) as i64);
        let container_height = image_container_height(requested_height, draw_height, media_frame);
        if media_frame {
            canvas.fill_rect(x, *y, max_width, container_height, palette.background);
        }
        if let Some(request) = transformed_image_blit_request(
            image,
            base_x,
            scrolled_base_y,
            draw_width,
            draw_height,
            x,
            *y,
            max_width,
            container_height,
        ) {
            canvas.with_clip(x, *y, max_width, container_height, &mut |canvas| {
                if !try_blit_cached_image(canvas, image, request, draw_width, draw_height) {
                    canvas.blit_rgba(request);
                }
            });
        }
        if node.props().common.selectable && !image.selection_text.is_empty() {
            canvas.record_text_run(
                &image.selection_text,
                base_x,
                scrolled_text_y(scrolled_base_y),
                draw_width,
                draw_height,
            );
        }
        let advance = if requested_height > 0 {
            requested_height
        } else {
            target_height.saturating_add(NODE_GAP)
        };
        *y = y.saturating_add(advance);
    }
}

fn image_container_height(requested_height: usize, draw_height: usize, media_frame: bool) -> usize {
    if requested_height == 0 {
        return draw_height;
    }
    if media_frame {
        requested_height.max(draw_height)
    } else {
        requested_height
    }
}

fn scrolled_text_y(scrolled_base_y: i64) -> usize {
    if scrolled_base_y.is_negative() {
        0
    } else {
        scrolled_base_y as usize
    }
}

fn raster_scaled_extent(
    value: usize,
    layout_scale: f32,
    raster_scale: f32,
    fixed_layout_height: bool,
) -> usize {
    if fixed_layout_height {
        return value;
    }
    let layout_scale = normalized_scale_factor(layout_scale);
    let raster_scale = normalized_scale_factor(raster_scale);
    if raster_scale <= layout_scale {
        return value;
    }
    ((value as f64 * f64::from(raster_scale) / f64::from(layout_scale)).round() as usize).max(1)
}

fn normalized_scale_factor(scale_factor: f32) -> f32 {
    if scale_factor.is_finite() && scale_factor >= 1.0 {
        scale_factor
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::test_assert::KucTestExpect;
    use katana_ui_core::render_model::{
        UiDimension, UiImageSurfaceProps, UiImageSurfaceTransform, UiNodeKind,
    };
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn image_geometry_helpers_cover_negative_scroll_and_scale_boundaries() {
        assert_eq!(0, scrolled_text_y(-1));
        assert_eq!(8, raster_scaled_extent(8, 2.0, 1.0, false));
        assert_eq!(1.0, normalized_scale_factor(f32::NAN));
    }

    #[test]
    fn image_renderer_preserves_media_frame_selection_and_export_offsets() {
        let image = UiImageSurfaceProps::new("image", 8, 4, vec![0xaa; 8 * 4 * 4])
            .kuc_expect("test image must be valid")
            .content_scale(200)
            .display_size(4, 2)
            .selection_text("image text")
            .transform(UiImageSurfaceTransform::new(125, -1, 1));
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::new(theme);
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 24,
            height: 24,
            scroll_y: 1.0,
        };

        let standard = UiNode::new(UiNodeKind::ImageSurface, "")
            .image_surface(image.clone())
            .selectable(true);
        let mut standard_canvas = Canvas::new(24, 24, palette.background);
        let mut standard_y = 0;
        renderer.draw_image(
            &mut standard_canvas,
            &standard,
            0,
            &mut standard_y,
            area,
            palette,
        );
        assert!(standard_y > 0);
        assert!(
            standard_canvas
                .text_runs()
                .iter()
                .any(|run| run.text() == "image text")
        );

        let media = UiNode::new(UiNodeKind::ImageSurface, "")
            .image_surface(image.clone())
            .visual_role(UiVisualRole::MediaFrame)
            .height(UiDimension::Px(12));
        let mut media_canvas =
            Canvas::new_scaled_with_raster_scale(24, 24, 1.0, 2.0, palette.background)
                .with_reference_capture_image_surface_extents();
        let mut media_y = 0;
        renderer.draw_image(&mut media_canvas, &media, 0, &mut media_y, area, palette);
        assert_eq!(12, media_y);

        let export = UiNode::new(UiNodeKind::ImageSurface, "")
            .image_surface(image)
            .visual_role(UiVisualRole::ExportMediaFrame)
            .height(UiDimension::Px(12));
        let mut export_canvas = Canvas::new(24, 32, palette.background);
        let mut export_y = 0;
        renderer.draw_image(&mut export_canvas, &export, 0, &mut export_y, area, palette);
        assert_eq!(12, export_y);

        assert_eq!(5, image_container_height(0, 5, false));
        assert_eq!(7, image_container_height(7, 5, true));
        assert_eq!(7, image_container_height(7, 5, false));
        assert_eq!(4, scrolled_text_y(4));
        assert_eq!(6, raster_scaled_extent(3, 1.0, 2.0, false));
        assert_eq!(2.0, normalized_scale_factor(2.0));
    }
}
