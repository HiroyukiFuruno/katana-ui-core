use katana_ui_core::render_model::UiRect;
use katana_ui_core_egui_adapter::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core_egui_adapter::command_chrome::CommandChromePaintPlan;

pub(super) const RGBA_CHANNELS: usize = 4;
const ALPHA_CHANNEL: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CommandChromePlanPixels {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgba: Vec<u8>,
    pub(super) paint_plan_hash: String,
    pub(super) pixel_hash: String,
}

pub(super) fn render_command_chrome_plan(
    plan: &CommandChromePaintPlan,
    canvas: UiRect,
) -> Result<CommandChromePlanPixels, String> {
    let plans = [ArtifactPaintPlanRef::CommandChrome(plan)];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(canvas),
        plans: &plans,
    })
    .map_err(|error| error.to_string())?;
    Ok(CommandChromePlanPixels {
        width: canvas.width,
        height: canvas.height,
        rgba: frame.rgba_pixels,
        paint_plan_hash: frame.paint_plan_hash,
        pixel_hash: frame.pixel_hash,
    })
}

pub(super) fn paint_plan_has_star_variation_selector(plan: &CommandChromePaintPlan) -> bool {
    plan.operations.iter().any(|operation| {
        matches!(
            &operation.kind,
            katana_ui_core_egui_adapter::command_chrome::CommandChromePaintOperationKind::Texture { texture, .. }
                if texture.identity.contains("⭐️")
        )
    })
}

pub(super) fn paint_plan_has_colored_star_texture(plan: &CommandChromePaintPlan) -> bool {
    plan.operations.iter().any(|operation| {
        let katana_ui_core_egui_adapter::command_chrome::CommandChromePaintOperationKind::Texture {
            texture,
            ..
        } = &operation.kind
        else {
            return false;
        };
        texture.identity.contains("⭐️")
            && texture
                .rgba_pixels
                .chunks_exact(RGBA_CHANNELS)
                .any(|rgba| rgba[ALPHA_CHANNEL] > 0 && (rgba[0] != rgba[1] || rgba[1] != rgba[2]))
    })
}

pub(super) fn texture_identities(plan: &CommandChromePaintPlan) -> Vec<String> {
    plan.operations.iter().filter_map(|operation| match &operation.kind {
        katana_ui_core_egui_adapter::command_chrome::CommandChromePaintOperationKind::Texture { texture, .. } => Some(texture.identity.clone()),
        katana_ui_core_egui_adapter::command_chrome::CommandChromePaintOperationKind::Fill { .. }
        | katana_ui_core_egui_adapter::command_chrome::CommandChromePaintOperationKind::RoundedFill { .. } => None,
    }).collect()
}
