use super::artifact_compositor_blend::{self, RGBA_CHANNELS, TextureRef};
use super::artifact_compositor_geometry;
use super::artifact_compositor_hash;
use super::{
    ArtifactCompositeError, ArtifactCompositeFrame, ArtifactCompositeRequest, ArtifactPaintPlanRef,
};
use crate::command_chrome::{
    CommandChromePaintOperationKind, CommandChromePaintPlan, CommandChromePaintTexture,
};
use crate::context_menu::{
    ContextMenuPaintOperationKind, ContextMenuPaintPlan, ContextMenuPaintTexture,
};
use crate::text_surface::{
    TextSurfacePaintOperationKind, TextSurfacePaintPlan, TextSurfacePaintTexture,
};
use katana_ui_core::render_model::UiRect;

pub(super) fn compose(
    request: ArtifactCompositeRequest<'_>,
) -> Result<ArtifactCompositeFrame, ArtifactCompositeError> {
    let canvas = request.canvas.ui_rect();
    artifact_compositor_geometry::validate_canvas(canvas)?;
    let mut rgba_pixels = vec![0; canvas_byte_length(canvas)?];
    for plan in request.plans {
        match plan {
            ArtifactPaintPlanRef::TextSurface(plan) => {
                paint_text_surface(&mut rgba_pixels, canvas, plan)?
            }
            ArtifactPaintPlanRef::CommandChrome(plan) => {
                paint_command_chrome(&mut rgba_pixels, canvas, plan)?;
            }
            ArtifactPaintPlanRef::ContextMenu(plan) => {
                paint_context_menu(&mut rgba_pixels, canvas, plan)?;
            }
        }
    }
    let non_transparent_pixel_count = rgba_pixels
        .as_chunks::<RGBA_CHANNELS>()
        .0
        .iter()
        .filter(|pixel| pixel[RGBA_CHANNELS - 1] != 0)
        .count();
    Ok(ArtifactCompositeFrame {
        canvas: request.canvas,
        pixel_hash: artifact_compositor_hash::hash_bytes(&rgba_pixels),
        paint_plan_hash: artifact_compositor_hash::paint_plan_hash(request.plans)?,
        rgba_pixels,
        non_transparent_pixel_count,
    })
}

fn canvas_byte_length(canvas: UiRect) -> Result<usize, ArtifactCompositeError> {
    u64::from(canvas.width)
        .checked_mul(u64::from(canvas.height))
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNELS as u64))
        .and_then(|bytes| usize::try_from(bytes).ok())
        .ok_or(ArtifactCompositeError::Overflow {
            context: "sizing canvas RGBA bytes",
        })
}

fn paint_text_surface(
    pixels: &mut [u8],
    canvas: UiRect,
    plan: &TextSurfacePaintPlan,
) -> Result<(), ArtifactCompositeError> {
    for operation in &plan.operations {
        let Some(clip) = artifact_compositor_geometry::clip_rect(
            canvas,
            plan.surface_bounds,
            operation.clip_bounds,
        )?
        else {
            continue;
        };
        match &operation.kind {
            TextSurfacePaintOperationKind::Fill { bounds, color_rgba } => {
                artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
            }
            TextSurfacePaintOperationKind::Texture { bounds, texture } => {
                artifact_compositor_blend::texture(pixels, canvas, clip, *bounds, texture)?;
            }
        }
    }
    Ok(())
}

fn paint_command_chrome(
    pixels: &mut [u8],
    canvas: UiRect,
    plan: &CommandChromePaintPlan,
) -> Result<(), ArtifactCompositeError> {
    for operation in &plan.operations {
        let Some(clip) = artifact_compositor_geometry::clip_rect(
            canvas,
            plan.surface_bounds,
            operation.clip_bounds,
        )?
        else {
            continue;
        };
        match &operation.kind {
            CommandChromePaintOperationKind::Fill { bounds, color_rgba } => {
                artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
            }
            CommandChromePaintOperationKind::Texture { bounds, texture } => {
                artifact_compositor_blend::texture(pixels, canvas, clip, *bounds, texture)?;
            }
        }
    }
    Ok(())
}

fn paint_context_menu(
    pixels: &mut [u8],
    canvas: UiRect,
    plan: &ContextMenuPaintPlan,
) -> Result<(), ArtifactCompositeError> {
    for operation in &plan.operations {
        let Some(clip) = artifact_compositor_geometry::clip_rect(
            canvas,
            plan.surface_bounds,
            operation.clip_bounds,
        )?
        else {
            continue;
        };
        match &operation.kind {
            ContextMenuPaintOperationKind::Fill { bounds, color_rgba } => {
                artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
            }
            ContextMenuPaintOperationKind::Texture { bounds, texture } => {
                artifact_compositor_blend::texture(pixels, canvas, clip, *bounds, texture)?;
            }
        }
    }
    Ok(())
}

impl TextureRef for TextSurfacePaintTexture {
    fn identity(&self) -> &str {
        &self.identity
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn rgba_pixels(&self) -> &[u8] {
        &self.rgba_pixels
    }
}

impl TextureRef for CommandChromePaintTexture {
    fn identity(&self) -> &str {
        &self.identity
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn rgba_pixels(&self) -> &[u8] {
        &self.rgba_pixels
    }
}

impl TextureRef for ContextMenuPaintTexture {
    fn identity(&self) -> &str {
        &self.identity
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn rgba_pixels(&self) -> &[u8] {
        &self.rgba_pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact_compositor::{ArtifactCanvasBounds, ArtifactCompositeRequest};
    use crate::command_chrome::CommandChromePaintOperation;
    use crate::context_menu::{
        ContextMenuPaintOperation, ContextMenuPaintOperationKind, ContextMenuPaintPlan,
    };
    use crate::text_surface::{
        EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
        TextSurfacePaintPlan, TextSurfacePaintTexture,
    };
    use katana_ui_core::render_model::UiRect;

    #[test]
    fn compose_counts_pixels_and_applies_command_and_context_menu_layers() {
        let texture = TextSurfacePaintTexture {
            identity: "texture-1".to_owned(),
            width: 1,
            height: 1,
            rgba_pixels: vec![0, 255, 0, 255],
        };
        let text_plan = TextSurfacePaintPlan {
            surface_bounds: UiRect::new(0, 0, 2, 1),
            viewport_bounds: UiRect::new(0, 0, 2, 1),
            operations: vec![TextSurfacePaintOperation {
                layer: EguiTextSurfaceDrawLayer::Background,
                clip_bounds: UiRect::new(0, 0, 2, 1),
                kind: TextSurfacePaintOperationKind::Texture {
                    bounds: UiRect::new(0, 0, 2, 1),
                    texture,
                },
            }],
        };
        let chrome_plan = crate::command_chrome::CommandChromePaintPlan {
            surface_bounds: UiRect::new(1, 0, 2, 1),
            operations: vec![CommandChromePaintOperation {
                layer: crate::command_chrome::EguiCommandChromeDrawLayer::PanelFill,
                clip_bounds: UiRect::new(1, 0, 1, 1),
                kind: crate::command_chrome::CommandChromePaintOperationKind::Fill {
                    bounds: UiRect::new(1, 0, 1, 1),
                    color_rgba: [255, 0, 0, 128],
                },
            }],
        };
        let context_plan = ContextMenuPaintPlan {
            surface_bounds: UiRect::new(0, 0, 2, 1),
            operations: vec![ContextMenuPaintOperation {
                clip_bounds: UiRect::new(0, 0, 2, 1),
                kind: ContextMenuPaintOperationKind::Fill {
                    bounds: UiRect::new(0, 0, 1, 1),
                    color_rgba: [0, 0, 255, 255],
                },
            }],
        };
        let plans = [
            ArtifactPaintPlanRef::TextSurface(&text_plan),
            ArtifactPaintPlanRef::CommandChrome(&chrome_plan),
            ArtifactPaintPlanRef::ContextMenu(&context_plan),
        ];
        let request = ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 2, 1)),
            plans: &plans,
        };
        let frame = super::super::ArtifactCompositor::compose(request)
            .expect("valid paint input should compose");
        assert_eq!(frame.canvas.ui_rect(), UiRect::new(0, 0, 2, 1));
        assert_eq!(frame.non_transparent_pixel_count, 2);
    }

    #[test]
    fn compose_rejects_invalid_texture_size() {
        let invalid_texture = TextSurfacePaintTexture {
            identity: "invalid".to_owned(),
            width: 2,
            height: 1,
            rgba_pixels: vec![255],
        };
        let text_plan = TextSurfacePaintPlan {
            surface_bounds: UiRect::new(0, 0, 2, 1),
            viewport_bounds: UiRect::new(0, 0, 2, 1),
            operations: vec![TextSurfacePaintOperation {
                layer: EguiTextSurfaceDrawLayer::Background,
                clip_bounds: UiRect::new(0, 0, 2, 1),
                kind: TextSurfacePaintOperationKind::Texture {
                    bounds: UiRect::new(0, 0, 2, 1),
                    texture: invalid_texture,
                },
            }],
        };
        let plans = [ArtifactPaintPlanRef::TextSurface(&text_plan)];
        let request = ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 2, 1)),
            plans: &plans,
        };
        assert!(matches!(
            super::super::ArtifactCompositor::compose(request),
            Err(ArtifactCompositeError::TextureByteLength { .. })
        ));
    }

    #[test]
    fn compose_covers_clipped_layers_and_each_texture_representation() {
        let outside = UiRect::new(10, 10, 1, 1);
        let text_plan = TextSurfacePaintPlan {
            surface_bounds: UiRect::new(0, 0, 2, 1),
            viewport_bounds: UiRect::new(0, 0, 2, 1),
            operations: vec![TextSurfacePaintOperation {
                layer: EguiTextSurfaceDrawLayer::Background,
                clip_bounds: outside,
                kind: TextSurfacePaintOperationKind::Fill {
                    bounds: outside,
                    color_rgba: [1; 4],
                },
            }],
        };
        let chrome_texture = CommandChromePaintTexture {
            identity: "chrome-texture".into(),
            width: 1,
            height: 1,
            rgba_pixels: vec![1, 2, 3, 255],
        };
        assert_eq!(TextureRef::identity(&chrome_texture), "chrome-texture");
        let chrome_plan = CommandChromePaintPlan {
            surface_bounds: UiRect::new(0, 0, 2, 1),
            operations: vec![
                CommandChromePaintOperation {
                    layer: crate::command_chrome::EguiCommandChromeDrawLayer::TextTexture,
                    clip_bounds: UiRect::new(0, 0, 1, 1),
                    kind: CommandChromePaintOperationKind::Texture {
                        bounds: UiRect::new(0, 0, 1, 1),
                        texture: chrome_texture,
                    },
                },
                CommandChromePaintOperation {
                    layer: crate::command_chrome::EguiCommandChromeDrawLayer::PanelFill,
                    clip_bounds: outside,
                    kind: CommandChromePaintOperationKind::Fill {
                        bounds: outside,
                        color_rgba: [2; 4],
                    },
                },
            ],
        };
        let context_texture = ContextMenuPaintTexture {
            identity: "context-texture".into(),
            width: 1,
            height: 1,
            rgba_pixels: vec![4, 5, 6, 255],
        };
        assert_eq!(TextureRef::identity(&context_texture), "context-texture");
        let context_plan = ContextMenuPaintPlan {
            surface_bounds: UiRect::new(0, 0, 2, 1),
            operations: vec![
                ContextMenuPaintOperation {
                    clip_bounds: UiRect::new(1, 0, 1, 1),
                    kind: ContextMenuPaintOperationKind::Texture {
                        bounds: UiRect::new(1, 0, 1, 1),
                        texture: context_texture,
                    },
                },
                ContextMenuPaintOperation {
                    clip_bounds: outside,
                    kind: ContextMenuPaintOperationKind::Fill {
                        bounds: outside,
                        color_rgba: [3; 4],
                    },
                },
            ],
        };
        let plans = [
            ArtifactPaintPlanRef::TextSurface(&text_plan),
            ArtifactPaintPlanRef::CommandChrome(&chrome_plan),
            ArtifactPaintPlanRef::ContextMenu(&context_plan),
        ];
        let frame = super::super::ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 2, 1)),
            plans: &plans,
        })
        .expect("valid texture representations must compose");
        assert_eq!(frame.non_transparent_pixel_count, 2);
    }

    #[test]
    fn canvas_byte_length_rejects_platform_size_overflow() {
        assert!(matches!(
            canvas_byte_length(UiRect::new(0, 0, u32::MAX, u32::MAX)),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
    }

    #[test]
    fn each_paint_plan_propagates_invalid_surface_geometry() {
        let canvas = UiRect::new(0, 0, 1, 1);
        let overflow = UiRect::new(i32::MAX, 0, 1, 1);
        let text = TextSurfacePaintPlan {
            surface_bounds: overflow,
            viewport_bounds: canvas,
            operations: vec![TextSurfacePaintOperation {
                layer: EguiTextSurfaceDrawLayer::Background,
                clip_bounds: canvas,
                kind: TextSurfacePaintOperationKind::Fill {
                    bounds: canvas,
                    color_rgba: [0; 4],
                },
            }],
        };
        let chrome = CommandChromePaintPlan {
            surface_bounds: overflow,
            operations: vec![CommandChromePaintOperation {
                layer: crate::command_chrome::EguiCommandChromeDrawLayer::PanelFill,
                clip_bounds: canvas,
                kind: CommandChromePaintOperationKind::Fill {
                    bounds: canvas,
                    color_rgba: [0; 4],
                },
            }],
        };
        let context = ContextMenuPaintPlan {
            surface_bounds: overflow,
            operations: vec![ContextMenuPaintOperation {
                clip_bounds: canvas,
                kind: ContextMenuPaintOperationKind::Fill {
                    bounds: canvas,
                    color_rgba: [0; 4],
                },
            }],
        };
        let mut pixels = vec![0; 4];
        assert!(paint_text_surface(&mut pixels, canvas, &text).is_err());
        assert!(paint_command_chrome(&mut pixels, canvas, &chrome).is_err());
        assert!(paint_context_menu(&mut pixels, canvas, &context).is_err());
    }
}
