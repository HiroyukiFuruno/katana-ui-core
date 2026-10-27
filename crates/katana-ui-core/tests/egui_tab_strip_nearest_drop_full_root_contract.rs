#![cfg(feature = "egui")]

use std::sync::{Arc, Mutex};

use egui::epaint::Shape;
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, TabStripCorrelation, TabStripDropPolicy,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripProjection,
    TabStripProjectionLease, TabStripProposal, TabStripProposalOperation, TabStripProposalPort,
    TabStripProposalPortError, TabStripSurfaceCapabilities, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabPlacement, TabStripTabTarget, TabStripText,
    TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

const GHOST_RGBA: [u8; 4] = [62, 83, 101, 230];
const INDICATOR_RGBA: [u8; 4] = [121, 192, 255, 255];

#[derive(Clone, Default)]
struct RecordingPort(Arc<Mutex<Vec<TabStripProposalOperation>>>);

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        self.0.lock().unwrap().push(proposal.into_operation());
        Ok(())
    }
}

fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("nearest-drop").value("本文"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"nearest-drop-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("nearest-drop-text")),
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

fn projection() -> TabStripProjection {
    let tab = |id: &'static [u8], label: &str| {
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(id),
            TabStripText::new(label),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .draggable(true)
                .selectable(true),
        )
    };
    TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"nearest-correlation"),
    )
    .tab(tab(b"source", "Source tab with unequal width"))
    .tab(tab(b"middle", "B"))
    .tab(tab(b"right", "CCCCCCCC"))
    .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(true))
    .drop_policy(TabStripDropPolicy::NearestGhostCenter)
}

fn collapsed_only_projection() -> TabStripProjection {
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"collapsed-tab"),
        TabStripText::new("Collapsed tab"),
    )
    .capabilities(TabStripTabCapabilities::new().draggable(true));
    TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"collapsed-correlation"),
    )
    .group(
        TabStripGroupDescriptor::new(
            katana_ui_core::egui::text_command_surface::TabStripGroupTarget::from_opaque_bytes(
                b"collapsed-group",
            ),
            TabStripText::new("Collapsed group"),
        )
        .capabilities(
            TabStripGroupCapabilities::new()
                .collapsed(true)
                .collapsible(true),
        )
        .tab(tab),
    )
    .drop_policy(TabStripDropPolicy::NearestGhostCenter)
}

fn reordered_projection() -> TabStripProjection {
    let tab = |id: &'static [u8], label: &str| {
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(id),
            TabStripText::new(label),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .draggable(true)
                .selectable(true),
        )
    };
    TabStripProjection::new(
        2,
        TabStripCorrelation::from_opaque_bytes(b"nearest-correlation"),
    )
    .tab(tab(b"right", "CCCCCCCC"))
    .tab(tab(b"middle", "B"))
    .tab(tab(b"source", "Source tab with unequal width"))
    .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(true))
    .drop_policy(TabStripDropPolicy::NearestGhostCenter)
}

fn frame(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    context.enable_accesskit();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            root.show(ui).unwrap();
        },
    );
    output.textures_delta.clear();
    output
}

fn bounds(output: &egui::FullOutput, label: &str) -> egui::Rect {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.label() == Some(label))
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

fn has_fill(output: &egui::FullOutput, rgba: [u8; 4]) -> bool {
    let expected = egui::Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
    output
        .shapes
        .iter()
        .any(|clipped| matches!(&clipped.shape, Shape::Rect(rect) if rect.fill == expected))
}

fn filled_rect(output: &egui::FullOutput, rgba: [u8; 4]) -> egui::Rect {
    let expected = egui::Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
    output
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            Shape::Rect(rect) if rect.fill == expected => Some(rect.rect),
            _ => None,
        })
        .expect("painted rectangle")
}

enum DropLocation {
    Start,
    LeftOfBoundary,
    RightOfBoundary,
    Tie,
    End,
}

fn run_drag(
    location: DropLocation,
    escape: bool,
) -> (egui::FullOutput, Vec<TabStripProposalOperation>) {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let lease = TabStripProjectionLease::new(projection())
        .with_proposal_port(RecordingPort(Arc::clone(&recorded)));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let source = bounds(&initial, "Source tab with unequal width");
    let middle = bounds(&initial, "B");
    let right = bounds(&initial, "CCCCCCCC");
    let previous_boundary = (source.max.x + middle.min.x) / 2.0;
    let boundary = (middle.max.x + right.min.x) / 2.0;
    let pointer_x = match location {
        DropLocation::Start => source.min.x - 24.0,
        DropLocation::LeftOfBoundary => boundary - 4.0,
        DropLocation::RightOfBoundary => boundary + 4.0,
        DropLocation::Tie => (previous_boundary + boundary) / 2.0,
        DropLocation::End => right.max.x + 24.0,
    };
    let press = source.center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(press),
            egui::Event::PointerButton {
                pos: press,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let feedback = frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::pos2(pointer_x, press.y))],
    );
    assert!(
        has_fill(&feedback, GHOST_RGBA),
        "drag ghost must be painted"
    );
    assert!(
        has_fill(&feedback, INDICATOR_RGBA),
        "drop indicator must be painted"
    );
    if escape {
        frame(
            &context,
            &mut root,
            vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
    } else {
        frame(
            &context,
            &mut root,
            vec![egui::Event::PointerButton {
                pos: egui::pos2(pointer_x, press.y),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
    }
    (feedback, std::mem::take(&mut *recorded.lock().unwrap()))
}

#[test]
fn nearest_ghost_drop_uses_unequal_tab_geometry_and_opaque_boundaries() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let lease = TabStripProjectionLease::new(projection())
        .with_proposal_port(RecordingPort(Arc::clone(&recorded)));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let source = bounds(&initial, "Source tab with unequal width");
    let middle = bounds(&initial, "B");
    let right = bounds(&initial, "CCCCCCCC");
    assert_ne!(source.width(), middle.width());
    assert_ne!(middle.width(), right.width());
    let boundary = (middle.max.x + right.min.x) / 2.0;

    let source_point = source.center();
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
    let feedback = frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::pos2(
            boundary,
            source_point.y,
        ))],
    );
    assert!(has_fill(&feedback, GHOST_RGBA));
    assert!(has_fill(&feedback, INDICATOR_RGBA));
    let ghost = feedback
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            Shape::Rect(rect)
                if rect.fill == egui::Color32::from_rgba_unmultiplied(62, 83, 101, 230) =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .expect("ghost bounds");
    assert_eq!(ghost.width(), source.width());
    assert!((ghost.center().x - boundary).abs() < 0.1);
    assert!(ghost.height() > 0.0);
    let indicator = feedback
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            Shape::Rect(rect)
                if rect.fill == egui::Color32::from_rgba_unmultiplied(121, 192, 255, 255) =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .expect("drop indicator bounds");
    assert_eq!(indicator.width(), 2.0);
    assert!((indicator.center().x - boundary).abs() < 0.1);
    assert!(indicator.height() > 0.0);
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(boundary, source_point.y),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let operations = std::mem::take(&mut *recorded.lock().unwrap());
    assert!(operations.iter().any(
        |operation| matches!(operation, TabStripProposalOperation::FinishDrag {
        committed: true,
        destination: Some(TabStripTabPlacement::Before(target)),
    } if target == &TabStripTabTarget::from_opaque_bytes(b"right"))
    ));
}

#[test]
fn nearest_ghost_left_right_and_tie_are_stable_and_cancel_is_typed() {
    let (start_feedback, start_operations) = run_drag(DropLocation::Start, false);
    assert!(has_fill(&start_feedback, GHOST_RGBA));
    assert!(start_operations.iter().any(|operation| matches!(
        operation,
        TabStripProposalOperation::FinishDrag {
            committed: true,
            destination: Some(TabStripTabPlacement::Before(target)),
        } if target == &TabStripTabTarget::from_opaque_bytes(b"source")
    )));

    let (left_feedback, left_operations) = run_drag(DropLocation::LeftOfBoundary, false);
    assert!(has_fill(&left_feedback, GHOST_RGBA));
    assert!(left_operations.iter().any(|operation| matches!(
        operation,
        TabStripProposalOperation::FinishDrag {
            committed: true,
            destination: Some(TabStripTabPlacement::Before(_))
        }
    )));

    let (tie_feedback, tie_operations) = run_drag(DropLocation::Tie, false);
    assert!(has_fill(&tie_feedback, GHOST_RGBA));
    assert!(tie_operations.iter().any(|operation| matches!(operation, TabStripProposalOperation::FinishDrag { committed: true, destination: Some(TabStripTabPlacement::Before(target)), } if target == &TabStripTabTarget::from_opaque_bytes(b"middle"))));

    let (right_feedback, right_operations) = run_drag(DropLocation::End, false);
    assert!(has_fill(&right_feedback, GHOST_RGBA));
    assert!(right_operations.iter().any(|operation| matches!(
        operation,
        TabStripProposalOperation::FinishDrag {
            committed: true,
            destination: Some(TabStripTabPlacement::EndOfStrip)
        }
    )));

    let (_, cancelled_operations) = run_drag(DropLocation::RightOfBoundary, true);
    assert!(
        cancelled_operations
            .iter()
            .any(|operation| matches!(operation, TabStripProposalOperation::CancelDrag))
    );
    assert!(!cancelled_operations.iter().any(|operation| matches!(
        operation,
        TabStripProposalOperation::FinishDrag {
            committed: true,
            ..
        }
    )));
}

#[test]
fn nearest_ghost_uses_noncentral_drag_anchor_for_ghost_and_indicator_centers() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let lease = TabStripProjectionLease::new(projection())
        .with_proposal_port(RecordingPort(Arc::clone(&recorded)));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let source = bounds(&initial, "Source tab with unequal width");
    let right = bounds(&initial, "CCCCCCCC");
    let boundary = (bounds(&initial, "B").max.x + right.min.x) / 2.0;
    let press = egui::pos2(source.min.x + 11.0, source.center().y);
    let center_offset = source.center().x - press.x;
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(press),
            egui::Event::PointerButton {
                pos: press,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let pointer = egui::pos2(boundary - center_offset, press.y);
    let feedback = frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(pointer)],
    );
    let ghost = filled_rect(&feedback, GHOST_RGBA);
    let indicator = filled_rect(&feedback, INDICATOR_RGBA);
    assert!((ghost.center().x - boundary).abs() < 0.1);
    assert!((indicator.center().x - boundary).abs() < 0.1);
    assert_eq!(ghost.width(), source.width());
    assert!(ghost.height() > 0.0);
    assert!(indicator.height() > 0.0);
}

#[test]
fn nearest_drop_after_lease_sync_moves_terminal_tab_to_strip_start() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let lease = TabStripProjectionLease::new(projection())
        .with_proposal_port(RecordingPort(Arc::clone(&recorded)));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(reordered_projection())
                .with_proposal_port(RecordingPort(Arc::clone(&recorded))),
        ),
    )
    .unwrap();
    let initial = frame(&context, &mut root, Vec::new());
    let first = bounds(&initial, "CCCCCCCC");
    let source = bounds(&initial, "Source tab with unequal width");
    let press = egui::pos2(source.min.x + 11.0, source.center().y);
    let center_offset = source.center().x - press.x;
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(press),
            egui::Event::PointerButton {
                pos: press,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let pointer = egui::pos2(first.min.x - center_offset, press.y);
    let feedback = frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(pointer)],
    );
    assert!((filled_rect(&feedback, GHOST_RGBA).center().x - first.min.x).abs() < 0.1);
    assert!((filled_rect(&feedback, INDICATOR_RGBA).center().x - first.min.x).abs() < 0.1);
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: pointer,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let operations = std::mem::take(&mut *recorded.lock().unwrap());
    assert!(operations.iter().any(|operation| matches!(
        operation,
        TabStripProposalOperation::FinishDrag {
            committed: true,
            destination: Some(TabStripTabPlacement::Before(target)),
        } if target == &TabStripTabTarget::from_opaque_bytes(b"right")
    )));
}

#[test]
fn nearest_drop_with_no_visible_tabs_is_fail_closed_for_empty_and_collapsed_headers() {
    for projection in [
        TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"empty-correlation"),
        )
        .drop_policy(TabStripDropPolicy::NearestGhostCenter),
        collapsed_only_projection(),
    ] {
        let recorded = Arc::new(Mutex::new(Vec::new()));
        let lease = TabStripProjectionLease::new(projection)
            .with_proposal_port(RecordingPort(Arc::clone(&recorded)));
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(
                katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                    token(1),
                    |_| Ok(None),
                )
                .with_tab_strip(lease),
            )
            .unwrap();
        let context = egui::Context::default();
        let output = frame(&context, &mut root, Vec::new());
        assert!(!has_fill(&output, GHOST_RGBA));
        assert!(!has_fill(&output, INDICATOR_RGBA));
        assert!(recorded.lock().unwrap().is_empty());
    }
}

#[test]
fn active_drag_cancels_once_on_empty_or_collapsed_reprojection() {
    for collapsed in [false, true] {
        let old = Arc::new(Mutex::new(Vec::new()));
        let new = Arc::new(Mutex::new(Vec::new()));
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(
                katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_| Ok(None))
                    .with_tab_strip(TabStripProjectionLease::new(projection()).with_proposal_port(RecordingPort(Arc::clone(&old)))),
            ).unwrap();
        let context = egui::Context::default();
        let initial = frame(&context, &mut root, Vec::new());
        let source = bounds(&initial, "Source tab with unequal width");
        let press = source.center();
        frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerMoved(press),
                egui::Event::PointerButton {
                    pos: press,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let dragged = frame(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(press + egui::vec2(30.0, 0.0))],
        );
        assert!(has_fill(&dragged, GHOST_RGBA));
        let next = if collapsed {
            collapsed_only_projection()
        } else {
            TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"empty-next"))
                .drop_policy(TabStripDropPolicy::NearestGhostCenter)
        };
        root.synchronize_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(2), |_| Ok(None))
                .with_tab_strip(TabStripProjectionLease::new(next).with_proposal_port(RecordingPort(Arc::clone(&new)))),
        ).unwrap();
        let after = frame(&context, &mut root, Vec::new());
        assert!(!has_fill(&after, GHOST_RGBA));
        assert!(!has_fill(&after, INDICATOR_RGBA));
        frame(
            &context,
            &mut root,
            vec![egui::Event::PointerButton {
                pos: press + egui::vec2(30.0, 0.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        assert_eq!(
            old.lock()
                .unwrap()
                .iter()
                .filter(|op| matches!(op, TabStripProposalOperation::CancelDrag))
                .count(),
            1
        );
        assert!(new.lock().unwrap().is_empty());
    }
}

fn overflow_projection(revision: u64) -> TabStripProjection {
    let mut projection = TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"overflow-correlation"),
    )
    .drop_policy(TabStripDropPolicy::NearestGhostCenter)
    .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(true));
    for i in 0..8 {
        projection = projection.tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(format!("overflow-{i}").into_bytes()),
                TabStripText::new(format!("Overflow tab number {i}")),
            )
            .capabilities(
                TabStripTabCapabilities::new()
                    .selectable(true)
                    .draggable(true),
            ),
        );
    }
    projection
}

fn overflow_frame(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    context.enable_accesskit();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(260.0, 300.0),
            )),
            events,
            time: Some(1.0 + context.cumulative_frame_nr() as f64),
            ..Default::default()
        },
        |ui| {
            root.show(ui).unwrap();
        },
    );
    output.textures_delta.clear();
    output
}

#[test]
fn nearest_overflow_uses_visible_edge_instead_of_logical_end() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_| Ok(None))
            .with_tab_strip(TabStripProjectionLease::new(overflow_projection(1)).with_proposal_port(RecordingPort(Arc::clone(&recorded))))
    ).unwrap();
    let context = egui::Context::default();
    let initial = overflow_frame(&context, &mut root, Vec::new());
    let source = bounds(&initial, "Overflow tab number 0");
    let candidate = bounds(&initial, "Overflow tab number 1");
    assert!(candidate.min.x < 260.0 && candidate.max.x > 0.0);
    assert!(
        bounds(&initial, "Overflow tab number 2").min.x >= 260.0,
        "fixture requires an offscreen suffix"
    );
    let start = source.center();
    overflow_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(start),
            egui::Event::PointerButton {
                pos: start,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let end = egui::pos2(900.0, start.y);
    let output = overflow_frame(&context, &mut root, vec![egui::Event::PointerMoved(end)]);
    let indicator = filled_rect(&output, INDICATOR_RGBA);
    assert!(
        (indicator.center().x - candidate.max.x.min(260.0)).abs() < 0.1,
        "indicator must use the last visible edge"
    );
    overflow_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: end,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let operations = recorded.lock().unwrap();
    assert!(operations.iter().any(|operation| matches!(operation, TabStripProposalOperation::FinishDrag { committed: true, destination: Some(TabStripTabPlacement::After(target)) } if target == &TabStripTabTarget::from_opaque_bytes(b"overflow-1"))));
}

#[test]
fn nearest_scrolled_viewport_uses_visible_boundaries_and_start_port_once() {
    for (move_right, cancel) in [(false, false), (true, false), (true, true)] {
        let started = Arc::new(Mutex::new(Vec::new()));
        let replacement = Arc::new(Mutex::new(Vec::new()));
        let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_| Ok(None))
                .with_tab_strip(TabStripProjectionLease::new(overflow_projection(1)).with_proposal_port(RecordingPort(Arc::clone(&started))))
        ).unwrap();
        let context = egui::Context::default();
        let initial = overflow_frame(&context, &mut root, Vec::new());
        let mut tabs: Vec<_> = (0..8)
            .map(|i| (i, bounds(&initial, &format!("Overflow tab number {i}"))))
            .collect();
        overflow_frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerMoved(egui::pos2(100.0, 15.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(-240.0, 0.0),
                    phase: egui::TouchPhase::Move,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let mut settled = false;
        for _ in 0..80 {
            let previous = tabs[0].1;
            let scrolled = overflow_frame(&context, &mut root, Vec::new());
            if let Some(update) = scrolled.platform_output.accesskit_update.as_ref() {
                for (index, rect) in &mut tabs {
                    let label = format!("Overflow tab number {index}");
                    if let Some(value) = update.nodes.iter().find_map(|(_, node)| {
                        (node.label() == Some(label.as_str()))
                            .then(|| node.bounds())
                            .flatten()
                    }) {
                        *rect = egui::Rect::from_min_max(
                            egui::pos2(value.x0 as f32, value.y0 as f32),
                            egui::pos2(value.x1 as f32, value.y1 as f32),
                        );
                    }
                }
            }
            if tabs[0].1.min.x < 0.0 && (tabs[0].1.min.x - previous.min.x).abs() < 0.01 {
                settled = true;
                break;
            }
        }
        assert!(settled, "scroll geometry should settle before the drag");
        let visible: Vec<_> = tabs
            .iter()
            .filter(|(_, rect)| rect.max.x > 0.0 && rect.min.x < 260.0)
            .collect();
        let (first_index, first_rect) = **visible.first().expect("visible first");
        let (last_index, last_rect) = **visible.last().expect("visible last");
        assert!(
            first_index > 0,
            "wheel must scroll the first tab completely out"
        );
        assert!(last_index < 7, "fixture requires a nonvisible suffix");
        let (_, source_rect) = **visible
            .iter()
            .max_by(|a, b| {
                a.1.intersect(egui::Rect::from_min_max(
                    egui::pos2(0.0, 0.0),
                    egui::pos2(260.0, 300.0),
                ))
                .width()
                .total_cmp(
                    &b.1.intersect(egui::Rect::from_min_max(
                        egui::pos2(0.0, 0.0),
                        egui::pos2(260.0, 300.0),
                    ))
                    .width(),
                )
            })
            .unwrap();
        let visible_source = source_rect.intersect(egui::Rect::from_min_max(
            egui::pos2(0.0, 0.0),
            egui::pos2(260.0, 300.0),
        ));
        let press = visible_source.center();
        let desired_x = if move_right {
            last_rect.max.x.min(260.0)
        } else {
            first_rect.min.x.max(0.0)
        };
        let pointer = egui::pos2(desired_x - (source_rect.center().x - press.x), press.y);
        overflow_frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerMoved(press),
                egui::Event::PointerButton {
                    pos: press,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let feedback = overflow_frame(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(pointer)],
        );
        assert!(
            (filled_rect(&feedback, GHOST_RGBA).center().x - desired_x).abs() < 0.1,
            "ghost={:?}, expected={desired_x}, source={source_rect:?}, press={press:?}, pointer={pointer:?}",
            filled_rect(&feedback, GHOST_RGBA)
        );
        assert!((filled_rect(&feedback, INDICATOR_RGBA).center().x - desired_x).abs() < 0.1);
        root.synchronize_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(2), |_| Ok(None))
                .with_tab_strip(TabStripProjectionLease::new(overflow_projection(2)).with_proposal_port(RecordingPort(Arc::clone(&replacement))))
        ).unwrap();
        overflow_frame(&context, &mut root, Vec::new());
        let event = if cancel {
            egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }
        } else {
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }
        };
        overflow_frame(&context, &mut root, vec![event]);
        let operations = started.lock().unwrap();
        assert_eq!(
            operations
                .iter()
                .filter(|op| matches!(
                    op,
                    TabStripProposalOperation::FinishDrag { .. }
                        | TabStripProposalOperation::CancelDrag
                ))
                .count(),
            1
        );
        if cancel {
            assert!(
                operations
                    .iter()
                    .any(|op| matches!(op, TabStripProposalOperation::CancelDrag))
            );
        } else if move_right {
            let target =
                TabStripTabTarget::from_opaque_bytes(format!("overflow-{last_index}").into_bytes());
            assert!(operations.iter().any(|op| matches!(op, TabStripProposalOperation::FinishDrag { committed: true, destination: Some(TabStripTabPlacement::After(value)) } if value == &target)));
        } else {
            let target = TabStripTabTarget::from_opaque_bytes(
                format!("overflow-{first_index}").into_bytes(),
            );
            assert!(operations.iter().any(|op| matches!(op, TabStripProposalOperation::FinishDrag { committed: true, destination: Some(TabStripTabPlacement::Before(value)) } if value == &target)));
        }
        assert!(
            replacement.lock().unwrap().is_empty(),
            "terminal stays on the start port"
        );
    }
}

#[test]
fn nearest_visible_internal_end_does_not_require_global_end_capability() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let projection = overflow_projection(1)
        .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(false));
    let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_| Ok(None))
            .with_tab_strip(TabStripProjectionLease::new(projection).with_proposal_port(RecordingPort(Arc::clone(&recorded))))
    ).unwrap();
    let context = egui::Context::default();
    let initial = overflow_frame(&context, &mut root, Vec::new());
    let start = bounds(&initial, "Overflow tab number 0").center();
    let edge = bounds(&initial, "Overflow tab number 1").max.x.min(260.0);
    assert!(bounds(&initial, "Overflow tab number 2").min.x >= 260.0);
    overflow_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(start),
            egui::Event::PointerButton {
                pos: start,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let pointer = egui::pos2(900.0, start.y);
    let feedback = overflow_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(pointer)],
    );
    assert!((filled_rect(&feedback, INDICATOR_RGBA).center().x - edge).abs() < 0.1);
    overflow_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: pointer,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert!(recorded.lock().unwrap().iter().any(|op| matches!(op, TabStripProposalOperation::FinishDrag { committed: true, destination: Some(TabStripTabPlacement::After(target)) } if target == &TabStripTabTarget::from_opaque_bytes(b"overflow-1"))));
}

#[test]
fn nearest_true_logical_end_still_requires_end_capability() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let no_end = projection()
        .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(false));
    let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_| Ok(None))
            .with_tab_strip(TabStripProjectionLease::new(no_end).with_proposal_port(RecordingPort(Arc::clone(&recorded))))
    ).unwrap();
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let source = bounds(&initial, "Source tab with unequal width").center();
    let last = bounds(&initial, "CCCCCCCC");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source),
            egui::Event::PointerButton {
                pos: source,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let pointer = egui::pos2(last.max.x + 100.0, source.y);
    let feedback = frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(pointer)],
    );
    assert!(has_fill(&feedback, GHOST_RGBA));
    assert!(
        !has_fill(&feedback, INDICATOR_RGBA),
        "unavailable logical end must not advertise a drop"
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: pointer,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let operations = recorded.lock().unwrap();
    assert_eq!(
        operations
            .iter()
            .filter(|op| matches!(op, TabStripProposalOperation::CancelDrag))
            .count(),
        1
    );
    assert!(!operations.iter().any(|op| matches!(
        op,
        TabStripProposalOperation::FinishDrag {
            committed: true,
            ..
        }
    )));
}
