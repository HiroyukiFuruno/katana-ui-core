use super::{
    SanitizedTabGroup, SanitizedTabGroupTarget, SanitizedTabProjection, SanitizedTabTarget,
};
use crate::egui::closeable_tab_strip_adapter::{CloseableTabStripAdapter, CloseableTabStripClosedFrame};
use crate::molecule::structured::{
    CloseableTab, CloseableTabClosePresentation, CloseableTabGroup, CloseableTabStrip,
    CloseableTabStripEvent,
};
use std::collections::BTreeMap;

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
        crate::egui::closeable_tab_strip_adapter::CloseableTabStripAdapterError,
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

#[derive(Default)]
struct SanitizedTabProjectionRouteTable {
    routes: BTreeMap<String, SanitizedTabProjectionRoute>,
}

enum SanitizedTabProjectionRoute {
    Tab(SanitizedRoutedTabTarget),
    Group(SanitizedRoutedGroupTarget),
}

pub(crate) struct SanitizedRoutedTabTarget {
    opaque: Box<[u8]>,
}

pub(crate) struct SanitizedRoutedGroupTarget {
    opaque: Box<[u8]>,
}

impl SanitizedRoutedTabTarget {
    fn from_target(target: &SanitizedTabTarget) -> Self {
        Self {
            opaque: target.opaque.to_vec().into_boxed_slice(),
        }
    }

    fn independent_copy(&self) -> Self {
        Self {
            opaque: self.opaque.to_vec().into_boxed_slice(),
        }
    }
}

impl SanitizedRoutedGroupTarget {
    fn from_target(target: &SanitizedTabGroupTarget) -> Self {
        Self {
            opaque: target.opaque.to_vec().into_boxed_slice(),
        }
    }

    fn independent_copy(&self) -> Self {
        Self {
            opaque: self.opaque.to_vec().into_boxed_slice(),
        }
    }
}

impl std::fmt::Debug for SanitizedRoutedTabTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedRoutedTabTarget(..)")
    }
}

impl std::fmt::Debug for SanitizedRoutedGroupTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedRoutedGroupTarget(..)")
    }
}

pub(crate) enum SanitizedTabProjectionClosedEvent {
    TabActivated(SanitizedRoutedTabTarget),
    TabCloseRequested(SanitizedRoutedTabTarget),
    GroupCollapseChanged {
        target: SanitizedRoutedGroupTarget,
        collapsed: bool,
    },
}

impl SanitizedTabProjectionClosedEvent {
    pub(crate) fn read_for_transport(&self) {
        match self {
            Self::TabActivated(target) => {
                let _ = target.opaque.len();
            }
            Self::TabCloseRequested(target) => {
                let _ = target.opaque.len();
            }
            Self::GroupCollapseChanged { target, collapsed } => {
                let _ = target.opaque.len();
                let _ = collapsed;
            }
        }
    }
}

impl std::fmt::Debug for SanitizedTabProjectionClosedEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self;
        formatter.write_str("SanitizedTabProjectionClosedEvent(..)")
    }
}

impl SanitizedTabProjectionRouteTable {
    fn insert_tab(&mut self, structural_id: String, target: &SanitizedTabTarget) {
        self.routes.insert(
            structural_id,
            SanitizedTabProjectionRoute::Tab(SanitizedRoutedTabTarget::from_target(target)),
        );
    }

    fn insert_group(&mut self, structural_id: String, target: &SanitizedTabGroupTarget) {
        self.routes.insert(
            structural_id,
            SanitizedTabProjectionRoute::Group(SanitizedRoutedGroupTarget::from_target(target)),
        );
    }

    fn route_event(
        &self,
        event: &CloseableTabStripEvent,
    ) -> Option<SanitizedTabProjectionClosedEvent> {
        match event {
            CloseableTabStripEvent::TabSelected { tab_id } => {
                let SanitizedTabProjectionRoute::Tab(target) = self.routes.get(tab_id.as_str())?
                else {
                    return None;
                };
                Some(SanitizedTabProjectionClosedEvent::TabActivated(
                    target.independent_copy(),
                ))
            }
            CloseableTabStripEvent::TabCloseRequested { tab_id } => {
                let SanitizedTabProjectionRoute::Tab(target) = self.routes.get(tab_id.as_str())?
                else {
                    return None;
                };
                Some(SanitizedTabProjectionClosedEvent::TabCloseRequested(
                    target.independent_copy(),
                ))
            }
            CloseableTabStripEvent::GroupCollapseChanged {
                group_id,
                collapsed,
            } => {
                let SanitizedTabProjectionRoute::Group(target) =
                    self.routes.get(group_id.as_str())?
                else {
                    return None;
                };
                Some(SanitizedTabProjectionClosedEvent::GroupCollapseChanged {
                    target: target.independent_copy(),
                    collapsed: *collapsed,
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SanitizedTabGroup, SanitizedTabProjectionAdapter, SanitizedTabProjectionClosedEvent,
        projection_to_state, projection_to_strip, structural_id,
    };
    use crate::egui::text_command_surface::sanitized_document_root::sanitized_tab_projection::{
        SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation,
        SanitizedTabGroupTarget, SanitizedTabProjection, SanitizedTabTarget,
    };
    use crate::molecule::structured::{
        CloseableTabGroupId, CloseableTabId, CloseableTabStripEvent,
    };
    use crate::render_model::UiIconProps;

    const SCREEN_SIZE: egui::Vec2 = egui::vec2(600.0, 160.0);

    #[test]
    fn nested_groups_map_to_generic_groups_with_parent_relationship_and_order() {
        let child_a = SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([3]),
            30,
            "Child A",
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([3]),
            2,
            "child-tab-a",
        ));
        let child_b = SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([4]),
            10,
            "Child B",
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([4]),
            1,
            "child-tab-b",
        ));
        let parent =
            SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([2]), 1, "Parent")
                .tab(SanitizedTab::new(
                    SanitizedTabTarget::from_opaque_bytes([2]),
                    1,
                    "parent-tab",
                ))
                .group(child_a)
                .group(child_b);

        let root = SanitizedTabProjection::new(vec![
            SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([1]), 2, "Root").tab(
                SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 1, "root-tab"),
            ),
            parent,
        ]);

        let strip = projection_to_strip(&root);
        let groups = &strip.options().groups;

        let parent_id = structural_id("group", &[1]);
        let child_high_order_id = structural_id("group", &[1, 0]);
        let child_low_order_id = structural_id("group", &[1, 1]);
        let root_id = structural_id("group", &[0]);

        assert_eq!(groups.len(), 4);
        assert_eq!(
            groups
                .iter()
                .map(|group| group.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                parent_id.as_str(),
                child_low_order_id.as_str(),
                child_high_order_id.as_str(),
                root_id.as_str(),
            ]
        );

        let parent_group = groups
            .iter()
            .find(|group| group.id.as_str() == parent_id)
            .expect("parent group exists");
        assert_eq!(parent_group.parent_group_id, None);

        let child_low_order_group = groups
            .iter()
            .find(|group| group.id.as_str() == child_low_order_id)
            .expect("low-order child group exists");
        assert_eq!(
            child_low_order_group
                .parent_group_id
                .as_ref()
                .map(|value| value.as_str()),
            Some(parent_id.as_str())
        );

        let child_high_order_group = groups
            .iter()
            .find(|group| group.id.as_str() == child_high_order_id)
            .expect("high-order child group exists");
        assert_eq!(
            child_high_order_group
                .parent_group_id
                .as_ref()
                .map(|value| value.as_str()),
            Some(parent_id.as_str())
        );

        let root_group = groups.iter().find(|group| group.id.as_str() == root_id);
        let root_group = root_group.expect("root group exists");
        assert_eq!(root_group.parent_group_id, None);
    }

    #[test]
    fn raw_input_pointer_selection_routes_one_opaque_tab_activation() {
        let projection = selectable_projection();
        let context = egui::Context::default();
        let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));

        let first = run_frame(&context, &mut adapter, Vec::new());
        let target = first
            .boundary_facts()
            .tab_rects
            .iter()
            .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
            .map(|(_, rect)| rect.center())
            .expect("second tab response exists");

        let _ = run_frame(&context, &mut adapter, vec![pointer_button(target, true)]);
        let released = run_frame(&context, &mut adapter, vec![pointer_button(target, false)]);
        let closed_events = released.into_closed_events();

        assert_eq!(closed_events.len(), 1);
        assert!(matches!(
            closed_events.as_slice(),
            [SanitizedTabProjectionClosedEvent::TabActivated(_)]
        ));
        assert_eq!(adapter.active_tab_id(), Some("sanitized-tab-0-1"));
    }

    #[test]
    fn unknown_structural_widget_ids_fail_closed_for_tab_and_group_events() {
        let projection = selectable_projection();
        let (_, routes) = projection_to_state(&projection);

        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabSelected {
                    tab_id: CloseableTabId::new("stale-tab-widget"),
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabSelected {
                    tab_id: CloseableTabId::new("sanitized-group-0"),
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabCloseRequested {
                    tab_id: CloseableTabId::new("sanitized-group-0"),
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
                    group_id: CloseableTabGroupId::new("sanitized-tab-0-0"),
                    collapsed: true,
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::OverflowOpened {
                    hidden_tab_ids: Vec::new(),
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
                    group_id: CloseableTabGroupId::new("stale-group-widget"),
                    collapsed: true,
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabCloseRequested {
                    tab_id: CloseableTabId::new("stale-tab-widget"),
                })
                .is_none()
        );
    }

    #[test]
    fn closed_route_events_and_public_declarations_hide_targets_and_widget_ids() {
        let projection = selectable_projection();
        let (_, routes) = projection_to_state(&projection);
        let event = routes
            .route_event(&CloseableTabStripEvent::TabSelected {
                tab_id: CloseableTabId::new("sanitized-tab-0-1"),
            })
            .expect("recognized internal route produces a closed event");

        let debug = format!("{event:?}");
        assert_eq!(debug, "SanitizedTabProjectionClosedEvent(..)");
        assert!(!debug.contains("de"));
        assert!(!debug.contains("sanitized-tab"));

        let source = include_str!("sanitized_tab_projection_adapter.rs_body.inc");
        let production_source = source.split("#[cfg(test)]").next().unwrap_or(source);
        for forbidden_public_declaration in [
            "pub enum SanitizedTabProjectionClosedEvent",
            "pub struct SanitizedRoutedTabTarget",
            "pub struct SanitizedRoutedGroupTarget",
        ] {
            assert!(
                !production_source.contains(forbidden_public_declaration),
                "closed routing leaked a public declaration: {forbidden_public_declaration}"
            );
        }
        assert!(!production_source.contains("CloseTab"));
        assert!(!production_source.contains("TabClosed"));
    }

    #[test]
    fn recognized_routes_cover_all_closed_event_variants_and_opaque_debug() {
        let projection = closeable_projection();
        let (_, routes) = projection_to_state(&projection);
        let events = [
            CloseableTabStripEvent::TabSelected {
                tab_id: CloseableTabId::new("sanitized-tab-0-0"),
            },
            CloseableTabStripEvent::TabCloseRequested {
                tab_id: CloseableTabId::new("sanitized-tab-0-1"),
            },
            CloseableTabStripEvent::GroupCollapseChanged {
                group_id: CloseableTabGroupId::new("sanitized-group-0"),
                collapsed: true,
            },
        ];
        let routed = events
            .iter()
            .map(|event| routes.route_event(event).expect("known route"))
            .collect::<Vec<_>>();
        for event in &routed {
            event.read_for_transport();
        }
        let fingerprint = super::super::super::sanitized_document_root_transport::SanitizedEventFingerprints::tab_event_fingerprint(&routed);
        assert!(!fingerprint.is_empty());
        assert!(matches!(
            &routed[0],
            SanitizedTabProjectionClosedEvent::TabActivated(target)
                if format!("{target:?}") == "SanitizedRoutedTabTarget(..)"
        ));
        assert!(matches!(
            &routed[2],
            SanitizedTabProjectionClosedEvent::GroupCollapseChanged { target, collapsed: true }
                if format!("{target:?}") == "SanitizedRoutedGroupTarget(..)"
        ));
    }

    #[test]
    fn physical_close_emits_opaque_intent_without_selection_or_mutation() {
        let projection = closeable_projection();
        let context = egui::Context::default();
        let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));
        let first = run_frame(&context, &mut adapter, Vec::new());
        let close_target = first
            .boundary_facts()
            .close_rects
            .iter()
            .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
            .map(|(_, rect)| rect.center())
            .expect("close response exists");

        let _ = run_frame(
            &context,
            &mut adapter,
            vec![pointer_button(close_target, true)],
        );
        let released = run_frame(
            &context,
            &mut adapter,
            vec![pointer_button(close_target, false)],
        );
        assert!(
            released
                .boundary_facts()
                .events
                .iter()
                .all(|event| { matches!(event, CloseableTabStripEvent::TabCloseRequested { .. }) })
        );
        assert!(matches!(
            released.into_closed_events().as_slice(),
            [SanitizedTabProjectionClosedEvent::TabCloseRequested(_)]
        ));
        assert_eq!(adapter.active_tab_id(), Some("sanitized-tab-0-0"));
        assert_eq!(adapter.strip.options().tabs.len(), 2);
    }

    #[test]
    fn close_capability_without_presentation_has_no_close_affordance() {
        let projection = selectable_projection();
        let context = egui::Context::default();
        let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));
        let frame = run_frame(&context, &mut adapter, Vec::new());
        assert!(frame.boundary_facts().close_rects.is_empty());
    }

    fn closeable_projection() -> SanitizedTabProjection {
        SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0xaa]),
            0,
            "Documents",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([0x01]), 0, "First")
                .with_icon(UiIconProps::new("<svg/>"))
                .with_capabilities(SanitizedTabCapabilities::new().active_state(true)),
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([0x02]), 1, "Second")
                .with_capabilities(SanitizedTabCapabilities::new().close_state(true))
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "×",
                    "Close tab",
                    "Close second tab",
                )),
        )])
    }

    fn selectable_projection() -> SanitizedTabProjection {
        SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0xaa, 0xbb]),
            0,
            "Documents",
        )
        .tab(
            SanitizedTab::new(
                SanitizedTabTarget::from_opaque_bytes([0x01, 0x02]),
                0,
                "First",
            )
            .with_capabilities(SanitizedTabCapabilities::new().active_state(true)),
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]),
            1,
            "Second",
        ))])
    }

    fn run_frame(
        context: &egui::Context,
        adapter: &mut SanitizedTabProjectionAdapter,
        events: Vec<egui::Event>,
    ) -> super::SanitizedTabProjectionFrame {
        let mut output = None;
        crate::egui::run_ui_discard(
            context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
                events,
                ..egui::RawInput::default()
            },
            |ui| output = Some(adapter.show(ui).expect("sanitized tab render succeeds")),
        );
        output.expect("sanitized tab frame is produced")
    }

    fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }
}
