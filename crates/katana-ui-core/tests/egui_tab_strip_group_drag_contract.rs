#![cfg(feature = "egui")]

use std::sync::{Arc, Mutex};

use egui::accesskit::Role;
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupPlacement,
    TabStripGroupTarget, TabStripProjection, TabStripProjectionLease, TabStripProposal,
    TabStripProposalOperation, TabStripProposalPort, TabStripProposalPortError, TabStripText,
    TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

#[derive(Clone)]
struct RecordingPort {
    operations: Arc<Mutex<Vec<TabStripProposalOperation>>>,
    group_order: Arc<Mutex<Vec<TabStripGroupTarget>>>,
}

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        let operation = proposal.into_operation();
        if let TabStripProposalOperation::ReorderGroup {
            source,
            destination,
        } = &operation
        {
            let mut order = self.group_order.lock().unwrap();
            if let Some(source_index) = order.iter().position(|target| target == source) {
                let source = order.remove(source_index);
                let mut destination_index = order
                    .iter()
                    .position(|target| match destination {
                        TabStripGroupPlacement::Before(placement_target)
                        | TabStripGroupPlacement::After(placement_target) => {
                            target == placement_target
                        }
                        TabStripGroupPlacement::EndOfStrip => false,
                    })
                    .expect("reorder destination must remain in host order");
                if matches!(destination, TabStripGroupPlacement::After(_)) {
                    destination_index += 1;
                }
                order.insert(destination_index, source);
            }
        }
        self.operations.lock().unwrap().push(operation);
        Ok(())
    }
}

fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("group-drag").value("host"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"group-drag-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("group-drag-text")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap()
}

fn projection(revision: u64, order: &[TabStripGroupTarget], draggable: bool) -> TabStripProjection {
    order.iter().fold(
        TabStripProjection::new(revision, TabStripCorrelation::from_opaque_bytes(b"drag")),
        |projection, target| {
            let label = if target == &TabStripGroupTarget::from_opaque_bytes(b"group-a") {
                "Group A"
            } else {
                "Group B"
            };
            projection.group(
                TabStripGroupDescriptor::new(target.clone(), TabStripText::new(label))
                    .capabilities(
                        TabStripGroupCapabilities::new()
                            .collapsible(true)
                            .draggable(draggable && label == "Group A"),
                    ),
            )
        },
    )
}

fn input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1280.0, 720.0),
        )),
        events,
        ..Default::default()
    }
}

fn frame(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    let mut full = context.run_ui(input(events), |ui| {
        root.show(ui).unwrap();
    });
    full.textures_delta.clear();
    full
}

fn bounds(full: &egui::FullOutput, label: &str) -> egui::Rect {
    full.platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == Role::Button && node.label() == Some(label))
                    .then(|| node.bounds())
                    .flatten()
                    .map(|rect| {
                        egui::Rect::from_min_max(
                            egui::pos2(rect.x0 as f32, rect.y0 as f32),
                            egui::pos2(rect.x1 as f32, rect.y1 as f32),
                        )
                    })
            })
        })
        .expect(label)
}

#[test]
fn public_group_drag_forwards_one_typed_reorder_from_accesskit_bounds() {
    let source = TabStripGroupTarget::from_opaque_bytes(b"group-a");
    let destination = TabStripGroupTarget::from_opaque_bytes(b"group-b");
    let initial_projection =
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"drag"))
            .group(
                TabStripGroupDescriptor::new(source.clone(), TabStripText::new("Group A"))
                    .capabilities(
                        TabStripGroupCapabilities::new()
                            .collapsible(true)
                            .draggable(true),
                    ),
            )
            .group(
                TabStripGroupDescriptor::new(destination.clone(), TabStripText::new("Group B"))
                    .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
            );
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let group_order = Arc::new(Mutex::new(vec![source.clone(), destination.clone()]));
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_context| Ok(None))
        .with_tab_strip(
            TabStripProjectionLease::new(initial_projection).with_proposal_port(RecordingPort {
                operations: Arc::clone(&recorded),
                group_order: Arc::clone(&group_order),
            }),
        );
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease)
        .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let source_bounds = bounds(&initial, "Group A");
    let destination_bounds = bounds(&initial, "Group B");
    let source_point = source_bounds.center();
    let drop_point = egui::pos2(
        destination_bounds.max.x - 1.0,
        destination_bounds.center().y,
    );

    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source_point),
            egui::Event::PointerButton {
                pos: source_point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(drop_point)],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: drop_point,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    frame(&context, &mut root, Vec::new());

    let operations = recorded.lock().unwrap();
    let reorder = operations
        .iter()
        .filter_map(|operation| match operation {
            TabStripProposalOperation::ReorderGroup {
                source: actual_source,
                destination: TabStripGroupPlacement::After(actual_destination),
            } => Some((actual_source, actual_destination)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(reorder.len(), 1, "group drag must forward once");
    assert_eq!(*reorder[0].0, source);
    assert_eq!(*reorder[0].1, destination);
    assert_eq!(
        *group_order.lock().unwrap(),
        vec![destination.clone(), source.clone()]
    );

    let reordered = group_order.lock().unwrap().clone();
    root.synchronize_with_lease(
        EguiTextCommandSurfaceHostProjectionLease::new(token(2), |_context| Ok(None))
            .with_tab_strip(
                TabStripProjectionLease::new(projection(2, &reordered, true)).with_proposal_port(
                    RecordingPort {
                        operations: Arc::clone(&recorded),
                        group_order: Arc::clone(&group_order),
                    },
                ),
            ),
    )
    .unwrap();
    let reordered_frame = frame(&context, &mut root, Vec::new());
    assert!(bounds(&reordered_frame, "Group B").min.x < bounds(&reordered_frame, "Group A").min.x);
}

fn run_cancelled_group_drag(
    source_draggable: bool,
    escape: bool,
) -> (usize, bool, Vec<TabStripGroupTarget>) {
    let source = TabStripGroupTarget::from_opaque_bytes(b"group-a");
    let destination = TabStripGroupTarget::from_opaque_bytes(b"group-b");
    let order = Arc::new(Mutex::new(vec![source.clone(), destination.clone()]));
    let operations = Arc::new(Mutex::new(Vec::new()));
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_context| Ok(None))
        .with_tab_strip(
            TabStripProjectionLease::new(projection(
                1,
                &[source.clone(), destination.clone()],
                source_draggable,
            ))
            .with_proposal_port(RecordingPort {
                operations: Arc::clone(&operations),
                group_order: Arc::clone(&order),
            }),
        );
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease)
        .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let source_point = bounds(&initial, "Group A").center();
    let destination_point = bounds(&initial, "Group B").center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source_point),
            egui::Event::PointerButton {
                pos: source_point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    if escape {
        frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerMoved(destination_point),
                egui::Event::Key {
                    key: egui::Key::Escape,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
    } else {
        frame(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(source_point)],
        );
        frame(
            &context,
            &mut root,
            vec![egui::Event::PointerButton {
                pos: source_point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
    }
    frame(&context, &mut root, Vec::new());
    let operations = operations.lock().unwrap();
    let cancelled = matches!(
        operations.as_slice(),
        [TabStripProposalOperation::CancelDrag]
    );
    (operations.len(), cancelled, order.lock().unwrap().clone())
}

#[test]
fn disabled_self_drop_and_escape_keep_group_order_unchanged() {
    let (disabled_operation_count, disabled_cancelled, disabled_order) =
        run_cancelled_group_drag(false, false);
    assert_eq!(disabled_operation_count, 1);
    let _ = disabled_cancelled;
    assert_eq!(
        disabled_order,
        vec![
            TabStripGroupTarget::from_opaque_bytes(b"group-a"),
            TabStripGroupTarget::from_opaque_bytes(b"group-b"),
        ]
    );

    let (self_drop_operation_count, self_drop_cancelled, self_drop_order) =
        run_cancelled_group_drag(true, false);
    assert_eq!(self_drop_operation_count, 1);
    let _ = self_drop_cancelled;
    assert_eq!(self_drop_order, disabled_order);

    let (escape_operation_count, escape_cancelled, escape_order) =
        run_cancelled_group_drag(true, true);
    assert_eq!(escape_operation_count, 0);
    assert!(!escape_cancelled);
    assert_eq!(escape_order, disabled_order);
}
