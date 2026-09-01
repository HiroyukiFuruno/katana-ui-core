use katana_ui_core::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor,
};
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceChild, EguiTextCommandSurfaceOutput,
};
use katana_ui_core::render_model::UiRect;

const ARTIFACT_PLAN_COUNT_VISIBLE_TEXT: usize = 3;
const ARTIFACT_PLAN_COUNT_VISIBLE_TEXT_AND_FLOATING: usize = 4;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FrameFacts {
    pub(crate) root: UiRect,
    pub(crate) text: UiRect,
    pub(crate) toolbar: UiRect,
    pub(crate) search: UiRect,
    pub(crate) floating: Option<UiRect>,
    pub(crate) floating_panel: Option<UiRect>,
    pub(crate) artifact_plan_count: usize,
    pub(crate) artifact_order: Vec<EguiTextCommandSurfaceChild>,
    pub(crate) composite_hash: String,
    pub(crate) labels: Vec<String>,
}

impl FrameFacts {
    pub(crate) fn collect(
        full: &egui::FullOutput,
        output: &EguiTextCommandSurfaceOutput,
    ) -> Result<FrameFacts, Box<dyn std::error::Error>> {
        let update = full
            .platform_output
            .accesskit_update
            .as_ref()
            .ok_or(std::io::Error::other("accesskit update was absent"))?;
        let labels = update
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label().map(str::to_string))
            .collect();
        let plans = output.artifact_paint_plans()?;
        let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(output.root_bounds),
            plans: &plans,
        })?;

        Ok(FrameFacts {
            root: output.root_bounds,
            text: output.text.record.frame.content_bounds,
            toolbar: output
                .toolbar
                .as_ref()
                .ok_or(std::io::Error::other("toolbar output was absent"))?
                .record
                .bounds,
            search: output
                .search
                .as_ref()
                .ok_or(std::io::Error::other("search output was absent"))?
                .record
                .bounds,
            floating: output
                .floating
                .as_ref()
                .and_then(|value| value.record.as_ref())
                .map(|record| record.toolbar.bounds),
            floating_panel: output
                .floating
                .as_ref()
                .and_then(|value| value.record.as_ref())
                .map(|record| record.panel_bounds),
            artifact_plan_count: plans.len(),
            artifact_order: output.artifact_order().to_vec(),
            composite_hash: composite.pixel_hash,
            labels,
        })
    }

    pub(crate) fn composite_hash(
        output: &EguiTextCommandSurfaceOutput,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let plans = output.artifact_paint_plans()?;
        Ok(ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(output.root_bounds),
            plans: &plans,
        })?
        .pixel_hash)
    }

    pub(crate) fn expected_artifact_order(
        output: &EguiTextCommandSurfaceOutput,
    ) -> Vec<EguiTextCommandSurfaceChild> {
        output.artifact_order().to_vec()
    }

    pub(crate) fn expected_plan_count(visible_floating: bool) -> usize {
        if visible_floating {
            ARTIFACT_PLAN_COUNT_VISIBLE_TEXT_AND_FLOATING
        } else {
            ARTIFACT_PLAN_COUNT_VISIBLE_TEXT
        }
    }
}
