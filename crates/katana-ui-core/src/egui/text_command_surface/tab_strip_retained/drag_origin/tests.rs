use super::super::super::tab_strip_proposal_port::{
    TabStripProposal, TabStripProposalOperation, TabStripProposalPort, TabStripProposalPortError,
    TabStripProposalPortHandle,
};
use super::super::{
    TabStripDragState, TabStripRetainedError, TabStripRetainedState, TabStripRouteTable,
};
use crate::egui::text_command_surface::{
    TabStripCorrelation, TabStripProjection, TabStripProjectionLease, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

struct RecordingPort {
    operations: Rc<RefCell<Vec<RecordedOperation>>>,
    reject_cancel: Rc<Cell<bool>>,
}

struct RecordedOperation {
    kind: &'static str,
    correlation: TabStripCorrelation,
    target: Option<TabStripTabTarget>,
}

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        let correlation = proposal.correlation();
        match proposal.into_operation() {
            TabStripProposalOperation::CancelDrag if self.reject_cancel.get() => {
                Err(TabStripProposalPortError::Rejected)
            }
            TabStripProposalOperation::CancelDrag => {
                self.operations.borrow_mut().push(RecordedOperation {
                    kind: "cancel",
                    correlation,
                    target: None,
                });
                Ok(())
            }
            TabStripProposalOperation::SelectTab(target) => {
                self.operations.borrow_mut().push(RecordedOperation {
                    kind: "select",
                    correlation,
                    target: Some(target),
                });
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn state_with_port(port: Option<TabStripProposalPortHandle>) -> TabStripRetainedState {
    let target = TabStripTabTarget::from_opaque_bytes(b"origin-tab");
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"origin-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(target.clone(), TabStripText::new("origin")).capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .draggable(true),
        ),
    );
    let config = crate::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(crate::text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    let mut state = TabStripRetainedState::from_lease(
        TabStripProjectionLease::new(projection),
        catalog,
        config,
    )
    .expect("origin fixture state");
    state.port = port;
    state.drag = Some(TabStripDragState {
        source: target.clone(),
        origin: None,
        label: TabStripText::new("origin"),
        pointer: egui::Pos2::ZERO,
    });
    state
}

fn port(
    operations: Rc<RefCell<Vec<RecordedOperation>>>,
    reject_cancel: Rc<Cell<bool>>,
) -> TabStripProposalPortHandle {
    TabStripProposalPortHandle::new(RecordingPort {
        operations,
        reject_cancel,
    })
}

#[test]
fn bind_keeps_terminal_origin_separate_from_current_port() {
    let old_operations = Rc::new(RefCell::new(Vec::<RecordedOperation>::new()));
    let new_operations = Rc::new(RefCell::new(Vec::<RecordedOperation>::new()));
    let mut state = state_with_port(Some(port(
        Rc::clone(&old_operations),
        Rc::new(Cell::new(false)),
    )));
    let target = state.drag.as_ref().expect("drag").source.clone();
    state.bind_drag_origin();
    state.port = Some(port(Rc::clone(&new_operations), Rc::new(Cell::new(false))));
    state.projection = TabStripProjection::new(
        2,
        TabStripCorrelation::from_opaque_bytes(b"new-correlation"),
    )
    .tab(TabStripTabDescriptor::new(
        target.clone(),
        TabStripText::new("new"),
    ));
    state.routes = TabStripRouteTable::from_projection(&state.projection);
    state.bind_drag_origin();
    let current_correlation = state.routes.correlation_for_proposal();
    let expected_current_correlation = current_correlation.clone();
    state
        .forward_proposal(
            current_correlation,
            TabStripProposalOperation::SelectTab(target.clone()),
        )
        .expect("current operation");
    let mut drag = state.drag.take().expect("drag");
    state
        .forward_drag_terminal(&mut drag, TabStripProposalOperation::CancelDrag)
        .expect("terminal operation");
    let new_operations = new_operations.borrow();
    let new_operation = &new_operations[0];
    assert_eq!(new_operation.kind, "select");
    assert_eq!(new_operation.correlation, expected_current_correlation);
    assert_eq!(new_operation.target.as_ref(), Some(&target));
    let old_operations = old_operations.borrow();
    let old_operation = &old_operations[0];
    assert_eq!(old_operation.kind, "cancel");
    assert_eq!(
        old_operation.correlation,
        TabStripCorrelation::from_opaque_bytes(b"origin-correlation"),
    );
}

#[test]
fn bound_without_origin_port_does_not_fallback_to_current_port() {
    let new_operations = Rc::new(RefCell::new(Vec::<RecordedOperation>::new()));
    let mut state = state_with_port(None);
    state.bind_drag_origin();
    state.port = Some(port(Rc::clone(&new_operations), Rc::new(Cell::new(false))));
    let mut drag = state.drag.take().expect("drag");
    assert!(matches!(
        state.forward_drag_terminal(&mut drag, TabStripProposalOperation::CancelDrag),
        Err(TabStripRetainedError::MissingPort)
    ));
    state.drag = Some(drag);
    assert!(new_operations.borrow().is_empty());
}

#[test]
fn rejected_origin_cancel_preserves_drag_until_retry_succeeds() {
    let old_operations = Rc::new(RefCell::new(Vec::<RecordedOperation>::new()));
    let reject_cancel = Rc::new(Cell::new(true));
    let new_operations = Rc::new(RefCell::new(Vec::<RecordedOperation>::new()));
    let mut state = state_with_port(Some(port(
        Rc::clone(&old_operations),
        Rc::clone(&reject_cancel),
    )));
    state.bind_drag_origin();
    state.port = Some(port(Rc::clone(&new_operations), Rc::new(Cell::new(false))));
    assert!(matches!(
        state.suspend_interaction(),
        Err(TabStripRetainedError::Port(
            TabStripProposalPortError::Rejected
        ))
    ));
    assert!(state.drag.is_some());
    assert!(
        state
            .drag
            .as_ref()
            .is_some_and(|drag| drag.origin.is_some())
    );
    assert!(new_operations.borrow().is_empty());
    reject_cancel.set(false);
    state.suspend_interaction().expect("cancel retry");
    assert!(state.drag.is_none());
    state
        .suspend_interaction()
        .expect("idempotent cancellation");
    assert!(new_operations.borrow().is_empty());
    let old_operations = old_operations.borrow();
    assert_eq!(old_operations.len(), 1);
    let old_operation = &old_operations[0];
    assert_eq!(old_operation.kind, "cancel");
    assert_eq!(
        old_operation.correlation,
        TabStripCorrelation::from_opaque_bytes(b"origin-correlation"),
    );
}
