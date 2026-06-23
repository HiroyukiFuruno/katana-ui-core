use crate::visual::{Canvas, UiTreeHostActionHit, UiTreeRenderArea, UiTreeStorybookHost};
use katana_document_viewer::PreviewOutput;
use katana_ui_core::render_model::{UiCursor, UiNodeId};

#[path = "document_viewer/adapter.rs"]
mod adapter;
#[path = "document_viewer/adapter_layout.rs"]
mod adapter_layout;
#[path = "document_viewer/adapter_slideshow.rs"]
mod adapter_slideshow;
#[path = "document_viewer/adapter_types.rs"]
mod adapter_types;
#[path = "document_viewer/asset_index.rs"]
mod asset_index;
#[path = "document_viewer/config.rs"]
mod config;
#[path = "document_viewer/diagram_control_resolver.rs"]
mod diagram_control_resolver;
#[path = "document_viewer/error.rs"]
mod error;
#[path = "document_viewer/html_details.rs"]
mod html_details;
#[path = "document_viewer/media_control_icons.rs"]
pub(crate) mod media_control_icons;
#[path = "document_viewer/node_factory.rs"]
mod node_factory;
#[path = "document_viewer/node_labels.rs"]
mod node_labels;

pub use adapter_types::{KucViewerAdapter, KucViewerPlan};
pub use config::KucViewerConfig;
pub use diagram_control_resolver::KucDiagramControlResolver;
pub use error::KucViewerError;
#[cfg(test)]
pub(super) use node_factory::KucNodeFactory;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DocumentViewerStorybookHost {
    adapter: KucViewerAdapter,
}

impl DocumentViewerStorybookHost {
    pub fn project(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
    ) -> Result<KucViewerPlan, KucViewerError> {
        Ok(self.adapter.render(output, config))
    }

    pub fn render(
        &self,
        canvas: &mut Canvas,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
    ) -> Result<KucViewerPlan, KucViewerError> {
        let plan = self.project(output, config)?;
        Self::ui_tree_host(config).render(canvas, plan.paint_request.tree().root(), area);
        Ok(plan)
    }

    pub fn host_action_hits(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
    ) -> Result<Vec<UiTreeHostActionHit>, KucViewerError> {
        let plan = self.project(output, config)?;
        Ok(Self::ui_tree_host(config).host_action_hits(plan.paint_request.tree().root(), area))
    }

    pub fn host_action_hit_at(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Result<Option<UiTreeHostActionHit>, KucViewerError> {
        let hits = self.host_action_hits(output, config, area)?;
        Ok(UiTreeStorybookHost::filter_host_action_hits_at(&hits, x, y)
            .into_iter()
            .next())
    }

    pub fn cursor_at(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Result<UiCursor, KucViewerError> {
        let hits = self.host_action_hits(output, config, area)?;
        Ok(UiTreeStorybookHost::cursor_for_host_action_hits_at(
            &hits, x, y,
        ))
    }

    pub fn hovered_action_node_id_at(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Result<Option<UiNodeId>, KucViewerError> {
        let hits = self.host_action_hits(output, config, area)?;
        Ok(UiTreeStorybookHost::hovered_action_node_id_for_host_action_hits_at(&hits, x, y))
    }

    fn ui_tree_host(config: &KucViewerConfig) -> UiTreeStorybookHost {
        UiTreeStorybookHost::new(config.theme.clone())
    }
}

#[cfg(test)]
#[path = "document_viewer/config_tests.rs"]
mod config_tests;
#[cfg(test)]
#[path = "document_viewer_node_contract_tests.rs"]
mod node_contract_tests;
#[cfg(test)]
#[path = "document_viewer/node_labels_tests.rs"]
mod node_labels_tests;
#[cfg(test)]
#[path = "document_viewer_tests.rs"]
mod tests;
