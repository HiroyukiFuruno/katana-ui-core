use super::actions::CLOSEABLE_TAB_DRAG_TAG;
use super::bar::WorkspaceTabBar;
use super::identifiers::WorkspaceTabId;
use super::options::WorkspaceTab;
use crate::interaction::drag_and_drop::{
    DragData, DragMetadata, DragSource, DropEffect, DropIndicatorOrientation, DropTarget,
};
use crate::molecule::DragPreview;
use crate::render_model::UiNodeId;
use serde_json::json;

const DRAG_PREVIEW_OPACITY_PERCENT: u8 = 82;
const DIRTY_TAB_BADGE_COUNT: usize = 1;

impl WorkspaceTabBar {
    #[must_use]
    pub fn drag_source(&self, tab_id: &WorkspaceTabId) -> Option<DragSource> {
        let tab = self.tab_by_id(tab_id)?;
        let payload = DragData::new(CLOSEABLE_TAB_DRAG_TAG, json!({ "tab_id": tab.id.as_str() }))
            .metadata(drag_metadata(tab));
        Some(DragSource::new(tab_node_id(tab_id), payload).keyboard_draggable(true))
    }

    #[must_use]
    pub fn drop_target_for_tab(&self, tab_id: &WorkspaceTabId) -> Option<DropTarget> {
        self.tab_by_id(tab_id)?;
        let mut target = DropTarget::new(tab_node_id(tab_id))
            .accepted_tag(CLOSEABLE_TAB_DRAG_TAG)
            .effect(DropEffect::Move);
        target.indicator_orientation = DropIndicatorOrientation::Vertical;
        Some(target)
    }

    #[must_use]
    pub fn drag_preview_for_tab(&self, tab_id: &WorkspaceTabId) -> Option<DragPreview> {
        let tab = self.tab_by_id(tab_id)?;
        let mut preview =
            DragPreview::new(tab.title.clone()).opacity_percent(DRAG_PREVIEW_OPACITY_PERCENT);
        if let Some(icon) = tab.icon.as_ref() {
            preview = preview.icon(icon.svg_source.clone());
        }
        if tab.dirty {
            preview = preview.count_badge(DIRTY_TAB_BADGE_COUNT);
        }
        Some(preview)
    }

    fn tab_by_id(&self, tab_id: &WorkspaceTabId) -> Option<&WorkspaceTab> {
        self.options.tabs.iter().find(|tab| &tab.id == tab_id)
    }
}

fn drag_metadata(tab: &WorkspaceTab) -> DragMetadata {
    let mut metadata = DragMetadata::new()
        .label(tab.title.clone())
        .insert("tab_id", tab.id.as_str().to_string());
    if let Some(icon) = tab.icon.as_ref() {
        metadata = metadata.icon(icon.svg_source.clone());
    }
    if tab.dirty {
        metadata = metadata.insert("dirty", "true");
    }
    metadata
}

fn tab_node_id(tab_id: &WorkspaceTabId) -> UiNodeId {
    UiNodeId::new(format!("closeable-tab:{}", tab_id.as_str()))
}
