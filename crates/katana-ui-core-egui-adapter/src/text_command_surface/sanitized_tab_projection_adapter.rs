use super::{SanitizedTabGroup, SanitizedTabProjection};
use crate::closeable_tab_strip_adapter::{CloseableTabStripAdapter, CloseableTabStripClosedFrame};
use katana_ui_core::molecule::structured::{
    CloseableTab, CloseableTabClosePresentation, CloseableTabGroup, CloseableTabStrip,
};
#[cfg(test)]
use katana_ui_core::molecule::structured::CloseableTabStripEvent;

#[path = "sanitized_tab_projection_adapter/route.rs"]
mod route;
pub(crate) use route::SanitizedTabProjectionClosedEvent;
use route::SanitizedTabProjectionRouteTable;

pub(crate) struct SanitizedTabProjectionAdapter {
    strip: CloseableTabStrip,
    routes: SanitizedTabProjectionRouteTable,
}

pub(crate) struct SanitizedTabProjectionFrame {
    closed_frame: CloseableTabStripClosedFrame,
    widget_rect: egui::Rect,
    tab_rects: Vec<egui::Rect>,
    group_rects: Vec<egui::Rect>,
    closed_events: Vec<SanitizedTabProjectionClosedEvent>,
    #[cfg(test)]
    structural_tab_rects: Vec<(String, egui::Rect)>,
    #[cfg(test)]
    structural_close_rects: Vec<(String, egui::Rect)>,
    #[cfg(test)]
    raw_events: Vec<CloseableTabStripEvent>,
}

pub(crate) struct SanitizedTabProjectionBoundaryFacts<'a> {
    pub(crate) closed_frame: &'a CloseableTabStripClosedFrame,
    pub(crate) widget_rect: egui::Rect,
    #[cfg(test)]
    pub(crate) tab_rects: &'a [(String, egui::Rect)],
    #[cfg(test)]
    pub(crate) close_rects: &'a [(String, egui::Rect)],
    pub(crate) closed_event_count: usize,
    #[cfg(test)]
    pub(crate) events: &'a [CloseableTabStripEvent],
}

impl SanitizedTabProjectionAdapter {
    pub(crate) fn from_projection(projection: Option<&SanitizedTabProjection>) -> Self {
        let (strip, routes) = projection.map_or_else(empty_projection_state, projection_to_state);
        Self { strip, routes }
    }

    pub(crate) fn replace_projection(&mut self, projection: Option<&SanitizedTabProjection>) {
        let (strip, routes) = projection.map_or_else(empty_projection_state, projection_to_state);
        self.strip = strip;
        self.routes = routes;
    }

    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<
        SanitizedTabProjectionFrame,
        crate::closeable_tab_strip_adapter::CloseableTabStripAdapterError,
    > {
        let rendered = CloseableTabStripAdapter.show(ui, &mut self.strip)?;
        let closed_events = rendered
            .events()
            .iter()
            .filter_map(|event| self.routes.route_event(event))
            .collect();
        Ok(SanitizedTabProjectionFrame {
            closed_frame: rendered.closed_frame().clone(),
            widget_rect: rendered.widget_rect(),
            tab_rects: rendered.tab_rects().iter().map(|(_, rect)| *rect).collect(),
            group_rects: rendered
                .group_rects()
                .iter()
                .map(|(_, rect)| *rect)
                .collect(),
            closed_events,
            #[cfg(test)]
            structural_tab_rects: rendered.tab_rects().to_vec(),
            #[cfg(test)]
            structural_close_rects: rendered.close_rects().to_vec(),
            #[cfg(test)]
            raw_events: rendered.events().to_vec(),
        })
    }

    #[cfg(test)]
    pub(crate) fn active_tab_id(&self) -> Option<&str> {
        self.strip
            .state()
            .active_tab_id
            .as_ref()
            .map(|id| id.as_str())
    }
}

impl SanitizedTabProjectionFrame {
    pub(crate) fn into_closed_events(self) -> Vec<SanitizedTabProjectionClosedEvent> {
        self.closed_events
    }

    pub(crate) fn boundary_facts(&self) -> SanitizedTabProjectionBoundaryFacts<'_> {
        SanitizedTabProjectionBoundaryFacts {
            closed_frame: &self.closed_frame,
            widget_rect: self.widget_rect,
            #[cfg(test)]
            tab_rects: &self.structural_tab_rects,
            #[cfg(test)]
            close_rects: &self.structural_close_rects,
            closed_event_count: self.closed_events.len(),
            #[cfg(test)]
            events: &self.raw_events,
        }
    }

    pub(crate) fn has_render_facts(&self) -> bool {
        let facts = self.boundary_facts();
        facts.closed_frame.has_closed_fact()
            && facts.widget_rect.width() > 0.0
            && facts.widget_rect.height() > 0.0
            && self.tab_rects.iter().all(|rect| rect.width() > 0.0)
            && self.group_rects.iter().all(|rect| rect.height() > 0.0)
            && facts.closed_event_count <= self.tab_rects.len() + self.group_rects.len()
    }
}

fn empty_projection_state() -> (CloseableTabStrip, SanitizedTabProjectionRouteTable) {
    (
        CloseableTabStrip::new("sanitized-tab-strip").stable_state_id("sanitized-tab-strip"),
        SanitizedTabProjectionRouteTable::default(),
    )
}

fn projection_to_state(
    projection: &SanitizedTabProjection,
) -> (CloseableTabStrip, SanitizedTabProjectionRouteTable) {
    let mut strip = CloseableTabStrip::new("sanitized-tab-strip");
    let mut active_id = None;
    let mut path = Vec::new();
    let mut routes = SanitizedTabProjectionRouteTable::default();
    let mut groups = projection.groups.iter().enumerate().collect::<Vec<_>>();
    groups.sort_by_key(|(index, group)| (group.order, *index));
    for (index, group) in groups {
        path.push(index);
        append_group(group, &path, None, &mut strip, &mut active_id, &mut routes);
        path.pop();
    }
    if let Some(active_id) = active_id {
        strip = strip.active_tab_id(active_id);
    }
    (strip.stable_state_id("sanitized-tab-strip"), routes)
}

#[cfg(test)]
fn projection_to_strip(projection: &SanitizedTabProjection) -> CloseableTabStrip {
    projection_to_state(projection).0
}

fn append_group(
    group: &SanitizedTabGroup,
    path: &[usize],
    parent_group_id: Option<&str>,
    strip: &mut CloseableTabStrip,
    active_id: &mut Option<String>,
    routes: &mut SanitizedTabProjectionRouteTable,
) {
    let group_id = structural_id("group", path);
    routes.insert_group(group_id.clone(), &group.target);
    let mut value = CloseableTabGroup::new(group_id.as_str(), &group.label);
    if let Some(parent_group_id) = parent_group_id {
        value = value.parent_group(parent_group_id);
    }
    *strip = strip.clone().group(value);

    let mut tabs = group.tabs.iter().enumerate().collect::<Vec<_>>();
    tabs.sort_by_key(|(index, tab)| (tab.order, *index));
    for (index, tab) in tabs {
        let tab_id = structural_id("tab", &[path, &[index]].concat());
        routes.insert_tab(tab_id.clone(), &tab.target);
        let mut value = CloseableTab::new(tab_id.as_str(), &tab.label)
            .dirty(tab.capabilities.dirty)
            .pinned(tab.capabilities.pinned)
            .closeable(tab.capabilities.close)
            .group_id(group_id.as_str());
        if let Some(icon) = &tab.icon {
            value = value.svg_icon(icon.clone());
        }
        if let Some(presentation) = &tab.close_presentation {
            value = value.close_presentation(CloseableTabClosePresentation::new(
                presentation.visible_label.clone(),
                presentation.tooltip.clone(),
                presentation.accessibility_label.clone(),
            ));
        }
        if tab.capabilities.active && active_id.is_none() {
            *active_id = Some(tab_id.clone());
        }
        *strip = strip.clone().tab(value);
    }

    let mut nested = group.groups.iter().enumerate().collect::<Vec<_>>();
    nested.sort_by_key(|(index, child)| (child.order, *index));
    for (index, child) in nested {
        let mut child_path = path.to_vec();
        child_path.push(index);
        append_group(
            child,
            &child_path,
            Some(group_id.as_str()),
            strip,
            active_id,
            routes,
        );
    }
}

fn structural_id(kind: &str, path: &[usize]) -> String {
    let suffix = path
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("-");
    format!("sanitized-{kind}-{suffix}")
}

#[cfg(test)]
#[path = "sanitized_tab_projection_adapter_tests.rs"]
mod tests;
