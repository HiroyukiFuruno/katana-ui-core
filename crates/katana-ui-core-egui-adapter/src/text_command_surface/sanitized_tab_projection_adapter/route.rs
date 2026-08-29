use super::super::{SanitizedTabGroupTarget, SanitizedTabTarget};
use katana_ui_core::molecule::structured::CloseableTabStripEvent;
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct SanitizedTabProjectionRouteTable {
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
            Self::TabActivated(target) | Self::TabCloseRequested(target) => {
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
    pub(super) fn insert_tab(&mut self, structural_id: String, target: &SanitizedTabTarget) {
        self.routes.insert(
            structural_id,
            SanitizedTabProjectionRoute::Tab(SanitizedRoutedTabTarget::from_target(target)),
        );
    }

    pub(super) fn insert_group(&mut self, structural_id: String, target: &SanitizedTabGroupTarget) {
        self.routes.insert(
            structural_id,
            SanitizedTabProjectionRoute::Group(SanitizedRoutedGroupTarget::from_target(target)),
        );
    }

    pub(super) fn route_event(
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
