use super::*;
use crate::egui::text_command_surface::tab_strip_proposal_port::{
    TabStripProposal, TabStripProposalOperation, TabStripProposalPort, TabStripProposalPortError,
};
use crate::egui::text_command_surface::{
    EguiTextCommandSurfaceRootEventTransport, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder, TabStripProjectionLease, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget,
};
use crate::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use crate::molecule::selection::ContextMenuEvent;
use crate::text_surface::TextSurfaceEvent;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

struct NoopDispatcher;

impl KucRootEventBatchDispatcher for NoopDispatcher {
    type Error = ();

    fn dispatch_text_events(&mut self, _: Vec<TextSurfaceEvent>) -> Result<(), ()> {
        Ok(())
    }
    fn dispatch_toolbar_events(&mut self, _: Vec<CommandChromeToolbarEvent>) -> Result<(), ()> {
        Ok(())
    }
    fn dispatch_floating_events(&mut self, _: Vec<FloatingCommandToolbarEvent>) -> Result<(), ()> {
        Ok(())
    }
    fn dispatch_search_events(&mut self, _: Vec<CommandChromeSearchEvent>) -> Result<(), ()> {
        Ok(())
    }
    fn dispatch_context_menu_events(&mut self, _: Vec<ContextMenuEvent>) -> Result<(), ()> {
        Ok(())
    }
}

struct NoopForwarder;

impl KucRootEventBatchForwarder for NoopForwarder {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), ()> {
        transport
            .dispatch_once(&mut NoopDispatcher)
            .map(|_| ())
            .map_err(|_| ())
    }
}

struct RecordingPort {
    operations: Rc<RefCell<Vec<&'static str>>>,
    reject_cancel: Rc<Cell<bool>>,
}

const TEST_SCREEN_WIDTH: f32 = 800.0;
const TEST_SCREEN_HEIGHT: f32 = 600.0;

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        match proposal.into_operation() {
            TabStripProposalOperation::CancelDrag if self.reject_cancel.get() => {
                Err(TabStripProposalPortError::Rejected)
            }
            TabStripProposalOperation::CancelDrag => {
                self.operations.borrow_mut().push("cancel");
                Ok(())
            }
            TabStripProposalOperation::StartDrag(_) => {
                self.operations.borrow_mut().push("start");
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn tab_projection(
    revision: u64,
    target: &TabStripTabTarget,
    draggable: bool,
) -> TabStripProjection {
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(format!("drag-correlation-{revision}")),
    )
    .tab(
        TabStripTabDescriptor::new(target.clone(), TabStripText::new("drag source")).capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .draggable(draggable),
        ),
    )
}

fn render_root(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> EguiTextCommandSurfaceHostRootFrame {
    let mut frame = None;
    let mut output = context.run_ui(input, |ui| {
        frame = Some(root.show(ui));
    });
    output.textures_delta.clear();
    frame.expect("root frame").expect("root render")
}

fn source_center(context: &egui::Context, root: &mut EguiTextCommandSurfaceHostRoot) -> egui::Pos2 {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            frame = Some(root.show(ui));
        },
    );
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(_, node)| (node.label() == Some("drag source")).then(|| node.bounds()))
        })
        .flatten()
        .expect("drag source accessibility bounds");
    output.textures_delta.clear();
    frame.expect("root frame").expect("root render");
    egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    )
}

#[test]
fn preserving_state_preflights_tab_cancel_before_replacing_router_or_body() {
    let target = TabStripTabTarget::from_opaque_bytes(b"preserved-drag-source");
    let old_operations = Rc::new(RefCell::new(Vec::new()));
    let reject_cancel = Rc::new(Cell::new(true));
    let active_router = Rc::new(RefCell::new(Vec::<&'static str>::new()));
    let style = TextCommandSurfaceStyle::standard().expect("style");
    let initial_token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"preserving-drag-root",
        presentation(),
        style.clone(),
    )
    .expect("initial token");
    let initial_lease = EguiTextCommandSurfaceHostProjectionLease::new(initial_token, {
        let active_router = Rc::clone(&active_router);
        move |_context| {
            active_router.borrow_mut().push("old");
            Ok(None)
        }
    })
    .with_tab_strip(
        TabStripProjectionLease::new(tab_projection(1, &target, true)).with_proposal_port(
            RecordingPort {
                operations: Rc::clone(&old_operations),
                reject_cancel: Rc::clone(&reject_cancel),
            },
        ),
    );
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(initial_lease)
        .expect("initial root");
    let context = egui::Context::default();
    context.enable_accesskit();
    let source = source_center(&context, &mut root);
    let mut press = egui::RawInput::default();
    press.events.push(egui::Event::PointerMoved(source));
    press.events.push(egui::Event::PointerButton {
        pos: source,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    });
    render_root(&context, &mut root, press);
    let mut move_input = egui::RawInput::default();
    move_input
        .events
        .push(egui::Event::PointerMoved(source + egui::vec2(48.0, 0.0)));
    render_root(&context, &mut root, move_input);

    let body_before = presentation().text.value;
    let new_operations = Rc::new(RefCell::new(Vec::new()));
    let stale_presentation = {
        let mut presentation = presentation();
        presentation.text.value = "stale new body".to_owned();
        presentation
    };
    let invalid = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            b"preserving-drag-root",
            stale_presentation.clone(),
            style.clone(),
        )
        .expect("invalid-source token"),
        {
            let active_router = Rc::clone(&active_router);
            move |_context| {
                active_router.borrow_mut().push("new");
                Ok(None)
            }
        },
    )
    .with_tab_strip(
        TabStripProjectionLease::new(tab_projection(2, &target, false)).with_proposal_port(
            RecordingPort {
                operations: Rc::clone(&new_operations),
                reject_cancel: Rc::new(Cell::new(false)),
            },
        ),
    );
    assert!(
        root.synchronize_with_lease_preserving_state(invalid)
            .is_err()
    );
    assert_eq!(root.process.presentation_revision(), 1);
    assert_eq!(
        root.process
            .root
            .selection_surface()
            .state()
            .text_area
            .value,
        body_before
    );
    assert_eq!(old_operations.borrow().as_slice(), ["start"]);
    assert!(new_operations.borrow().is_empty());
    let old_frame = render_root(&context, &mut root, egui::RawInput::default());
    old_frame
        .forward_events_once(&mut NoopForwarder)
        .expect("old router frame");
    assert_eq!(active_router.borrow().last().copied(), Some("old"));

    reject_cancel.set(false);
    let retry = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            b"preserving-drag-root",
            stale_presentation,
            style,
        )
        .expect("retry token"),
        {
            let active_router = Rc::clone(&active_router);
            move |_context| {
                active_router.borrow_mut().push("retry");
                Ok(None)
            }
        },
    )
    .with_tab_strip(
        TabStripProjectionLease::new(tab_projection(2, &target, false)).with_proposal_port(
            RecordingPort {
                operations: Rc::new(RefCell::new(Vec::new())),
                reject_cancel: Rc::new(Cell::new(false)),
            },
        ),
    );
    root.synchronize_with_lease_preserving_state(retry)
        .expect("retry should replace the lease after cancel succeeds");
    assert_eq!(root.process.presentation_revision(), 2);
    assert_eq!(
        root.process
            .root
            .selection_surface()
            .state()
            .text_area
            .value,
        body_before
    );
    assert_eq!(old_operations.borrow().as_slice(), ["start", "cancel"]);
    let new_frame = render_root(&context, &mut root, egui::RawInput::default());
    new_frame
        .forward_events_once(&mut NoopForwarder)
        .expect("new router frame");
    assert_eq!(active_router.borrow().last().copied(), Some("retry"));
}
