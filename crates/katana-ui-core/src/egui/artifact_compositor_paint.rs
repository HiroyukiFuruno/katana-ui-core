use super::artifact_compositor_blend::{self, RGBA_CHANNELS};
use super::artifact_compositor_geometry;
use super::artifact_compositor_hash;
use super::{
    ArtifactCompositeError, ArtifactCompositeFrame, ArtifactCompositeRequest, ArtifactPaintPlanRef,
};
use crate::egui::command_chrome::{CommandChromePaintOperationKind, CommandChromePaintPlan};
use crate::egui::context_menu::{ContextMenuPaintOperationKind, ContextMenuPaintPlan};
use crate::egui::diagnostics_list::{DiagnosticsListPaintOperationKind, DiagnosticsListPaintPlan};
use crate::egui::source_address_strip::{SourceAddressPaintOperationKind, SourceAddressPaintPlan};
use crate::egui::status_bar::{StatusBarPaintOperationKind, StatusBarPaintPlan};
use crate::egui::tab_strip_paint::{TabStripPaintOperationKind, TabStripPaintPlan};
use crate::egui::text_surface::{TextSurfacePaintOperationKind, TextSurfacePaintPlan};
use crate::render_model::UiRect;

#[path = "artifact_compositor_paint/texture.rs"]
mod texture;

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
            ArtifactPaintPlanRef::SourceAddress(plan) => {
                paint_source_address(&mut rgba_pixels, canvas, plan)?
            }
            ArtifactPaintPlanRef::StatusBar(plan) => {
                paint_status_bar(&mut rgba_pixels, canvas, plan)?
            }
            ArtifactPaintPlanRef::DiagnosticsList(plan) => {
                paint_diagnostics_list(&mut rgba_pixels, canvas, plan)?
            }
            ArtifactPaintPlanRef::TabStrip(plan) => {
                paint_tab_strip(&mut rgba_pixels, canvas, plan)?;
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

fn paint_tab_strip(
    pixels: &mut [u8],
    canvas: UiRect,
    plan: &TabStripPaintPlan,
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
            TabStripPaintOperationKind::Fill { bounds, color_rgba } => {
                artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
            }
            TabStripPaintOperationKind::Texture { bounds, texture } => {
                artifact_compositor_blend::texture(pixels, canvas, clip, *bounds, texture)?;
            }
        }
    }
    Ok(())
}

fn canvas_byte_length(canvas: UiRect) -> Result<usize, ArtifactCompositeError> {
    (canvas.width as usize)
        .checked_mul(canvas.height as usize)
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNELS))
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

fn paint_source_address(
    pixels: &mut [u8],
    canvas: UiRect,
    plan: &SourceAddressPaintPlan,
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
            SourceAddressPaintOperationKind::Input(input) => match input {
                TextSurfacePaintOperationKind::Fill { bounds, color_rgba } => {
                    artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
                }
                TextSurfacePaintOperationKind::Texture { bounds, texture } => {
                    artifact_compositor_blend::texture(pixels, canvas, clip, *bounds, texture)?;
                }
            },
            SourceAddressPaintOperationKind::Fill { bounds, color_rgba } => {
                artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
            }
            SourceAddressPaintOperationKind::Texture { bounds, texture } => {
                artifact_compositor_blend::texture(pixels, canvas, clip, *bounds, texture)?;
            }
        }
    }
    Ok(())
}

fn paint_status_bar(
    pixels: &mut [u8],
    canvas: UiRect,
    plan: &StatusBarPaintPlan,
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
            StatusBarPaintOperationKind::Fill { bounds, color_rgba } => {
                artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
            }
            StatusBarPaintOperationKind::Texture { bounds, texture } => {
                artifact_compositor_blend::texture(pixels, canvas, clip, *bounds, texture)?;
            }
        }
    }
    Ok(())
}

fn paint_diagnostics_list(
    pixels: &mut [u8],
    canvas: UiRect,
    plan: &DiagnosticsListPaintPlan,
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
            DiagnosticsListPaintOperationKind::Fill { bounds, color_rgba } => {
                artifact_compositor_blend::fill(pixels, canvas, clip, *bounds, *color_rgba)?;
            }
            DiagnosticsListPaintOperationKind::Texture { bounds, texture } => {
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
            CommandChromePaintOperationKind::RoundedFill {
                bounds,
                color_rgba,
                radius_px,
            } => {
                artifact_compositor_blend::rounded_fill(
                    pixels,
                    canvas,
                    clip,
                    *bounds,
                    *color_rgba,
                    *radius_px,
                )?;
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
