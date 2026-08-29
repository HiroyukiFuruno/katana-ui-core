use crate::visual::command_chrome_artifact::{CommandChromePlanPixels, RGBA_CHANNELS};
use crate::visual::command_chrome_script_types::CommandChromeArtifactError;
use katana_ui_core::render_model::UiRect;
use katana_ui_core_egui_adapter::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core_egui_adapter::command_chrome::CommandChromePaintPlan;

const ALPHA_CHANNEL: usize = 3;

pub(super) fn has_non_zero_pixel(rgba: &[u8]) -> bool {
    rgba.as_chunks::<RGBA_CHANNELS>()
        .0
        .iter()
        .any(|pixel| pixel[ALPHA_CHANNEL] != 0)
}

pub(super) fn render_composite_pixels(
    canvas: UiRect,
    toolbar: &CommandChromePaintPlan,
    floating: Option<&CommandChromePaintPlan>,
    search: &CommandChromePaintPlan,
) -> Result<CommandChromePlanPixels, CommandChromeArtifactError> {
    let mut plans = vec![ArtifactPaintPlanRef::CommandChrome(toolbar)];
    if let Some(floating) = floating {
        plans.push(ArtifactPaintPlanRef::CommandChrome(floating));
    }
    plans.push(ArtifactPaintPlanRef::CommandChrome(search));
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(canvas),
        plans: &plans,
    })
    .map_err(|error| CommandChromeArtifactError::Contract(error.to_string()))?;
    Ok(CommandChromePlanPixels {
        width: canvas.width,
        height: canvas.height,
        rgba: frame.rgba_pixels,
        paint_plan_hash: frame.paint_plan_hash,
        pixel_hash: frame.pixel_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::{RGBA_CHANNELS, has_non_zero_pixel, render_composite_pixels};
    use katana_ui_core::render_model::UiRect;
    use katana_ui_core_egui_adapter::command_chrome::CommandChromePaintOperationKind::Fill;
    use katana_ui_core_egui_adapter::command_chrome::{
        CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
        CommandChromePaintTexture, EguiCommandChromeDrawLayer,
    };

    const CANVAS: UiRect = UiRect::new(0, 0, 4, 4);
    const SEARCH: UiRect = UiRect::new(0, 0, 4, 4);

    #[test]
    fn render_composite_pixels_with_floating_collects_multiple_layers()
    -> Result<(), crate::visual::command_chrome_script_types::CommandChromeArtifactError> {
        let toolbar_plan = CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![fill_operation([10, 0, 0, 255])],
        };
        let floating_plan = CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![fill_operation([0, 20, 0, 255])],
        };
        let search_plan = CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![fill_operation([0, 0, 0, 0])],
        };

        let pixels =
            render_composite_pixels(CANVAS, &toolbar_plan, Some(&floating_plan), &search_plan)?;

        assert_eq!(CANVAS.width, pixels.width);
        assert_eq!(CANVAS.height, pixels.height);
        assert!(!pixels.rgba.is_empty());
        assert!(!pixels.paint_plan_hash.is_empty());
        assert!(!pixels.pixel_hash.is_empty());
        assert_eq!(
            pixels.rgba.len(),
            (CANVAS.width as usize) * (CANVAS.height as usize) * RGBA_CHANNELS
        );
        assert_eq!(&pixels.rgba[0..RGBA_CHANNELS], &[0, 20, 0, 255]);
        Ok(())
    }

    #[test]
    fn render_composite_pixels_without_floating_skips_branch()
    -> Result<(), crate::visual::command_chrome_script_types::CommandChromeArtifactError> {
        let toolbar_plan = CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![fill_operation([0, 0, 30, 255])],
        };
        let search_plan = CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![fill_operation([0, 0, 0, 255])],
        };

        let pixels = render_composite_pixels(CANVAS, &toolbar_plan, None, &search_plan)?;

        assert_eq!(&pixels.rgba[0..RGBA_CHANNELS], &[0, 0, 0, 255]);
        Ok(())
    }

    #[test]
    fn render_composite_pixels_rejects_invalid_textures() {
        let invalid_texture_plan = CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![texture_operation(
                "invalid",
                CommandChromePaintTexture {
                    identity: "invalid".into(),
                    width: 2,
                    height: 2,
                    rgba_pixels: vec![0; 3],
                },
            )],
        };
        let search_plan = transparent_fill_plan();
        let result = render_composite_pixels(CANVAS, &invalid_texture_plan, None, &search_plan);
        assert!(result.is_err());
    }

    #[test]
    fn has_non_zero_pixel_ignores_rgb_only() {
        assert!(has_non_zero_pixel(&[1, 2, 3, 4, 5, 6, 7, 0]));
        assert!(!has_non_zero_pixel(&[1, 2, 3, 0, 5, 6, 7, 0]));
    }

    fn fill_operation(color: [u8; RGBA_CHANNELS]) -> CommandChromePaintOperation {
        CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::PanelFill,
            clip_bounds: CANVAS,
            kind: Fill {
                bounds: CANVAS,
                color_rgba: color,
            },
        }
    }

    fn transparent_fill_plan() -> CommandChromePaintPlan {
        CommandChromePaintPlan {
            surface_bounds: SEARCH,
            operations: vec![fill_operation([0, 0, 0, 0])],
        }
    }

    fn texture_operation(
        _identity: &str,
        texture: CommandChromePaintTexture,
    ) -> CommandChromePaintOperation {
        CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::IconTexture,
            clip_bounds: CANVAS,
            kind: CommandChromePaintOperationKind::Texture {
                bounds: UiRect::new(0, 0, 1, 1),
                texture,
            },
        }
    }
}
