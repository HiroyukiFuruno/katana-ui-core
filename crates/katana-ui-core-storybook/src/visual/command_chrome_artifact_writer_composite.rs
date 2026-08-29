use crate::visual::command_chrome_artifact::{CommandChromePlanPixels, RGBA_CHANNELS};
use crate::visual::command_chrome_script_types::CommandChromeArtifactError;
use katana_ui_core::render_model::UiRect;
use katana_ui_core_egui_adapter::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core_egui_adapter::command_chrome::CommandChromePaintPlan;

const ALPHA_CHANNEL: usize = 3;

pub(super) fn has_non_zero_pixel(rgba: &[u8]) -> bool {
    rgba.chunks_exact(RGBA_CHANNELS)
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
