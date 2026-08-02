use katana_ui_core::event::{DragEvent, DragEventRouteNode, DragEventRouting, UiEvent};
use katana_ui_core::interaction::drag_and_drop::{
    DndPoint, DndRect, DragData, DropAcceptance, DropEffect, DropIndicator, DropIndicatorKind,
};
use katana_ui_core::render_model::UiNodeId;

#[test]
fn drag_events_serialize_in_deterministic_order() -> Result<(), serde_json::Error> {
    let source = UiNodeId::new("tab-a");
    let target = UiNodeId::new("tab-b");
    let data = DragData::new("katana-ui-core/tab-id", serde_json::json!("tab-a"));
    let events = vec![
        UiEvent::Drag(DragEvent::DragStart {
            source: source.clone(),
            data: data.clone(),
        }),
        UiEvent::Drag(DragEvent::DragEnter {
            target: target.clone(),
            data: data.clone(),
        }),
        UiEvent::Drag(DragEvent::Drop {
            target,
            data,
            effect: DropEffect::Move,
        }),
        UiEvent::Drag(DragEvent::DragEnd {
            source,
            committed: true,
        }),
    ];

    let encoded = serde_json::to_string(&events)?;
    let start = encoded.find("DragStart").unwrap_or(usize::MAX);
    let enter = encoded.find("DragEnter").unwrap_or(usize::MAX);
    let drop = encoded.find("Drop").unwrap_or(usize::MAX);
    let end = encoded.find("DragEnd").unwrap_or(usize::MAX);

    assert!(start < enter);
    assert!(enter < drop);
    assert!(drop < end);
    Ok(())
}

#[test]
fn drag_event_routes_skip_disabled_ancestors_and_keep_phase_order() {
    let target = UiNodeId::new("target");
    let parent = DragEventRouteNode::enabled(UiNodeId::new("parent"));
    let disabled = DragEventRouteNode::disabled(UiNodeId::new("disabled"));
    let root = DragEventRouteNode::new(UiNodeId::new("root"), false);

    assert_eq!(
        vec![target, UiNodeId::new("parent"), UiNodeId::new("root"),],
        DragEventRouting::bubble_route(
            UiNodeId::new("target"),
            vec![parent.clone(), disabled.clone(), root.clone()],
        )
    );
    assert_eq!(
        vec![UiNodeId::new("parent"), UiNodeId::new("root")],
        DragEventRouting::capture_route(vec![parent, disabled, root]),
    );

    let data = DragData::new("text/plain", serde_json::json!("payload"));
    let variants = [
        DragEvent::DragMove {
            source: UiNodeId::new("source"),
            position: DndPoint::new(10.0, 20.0),
        },
        DragEvent::DragLeave {
            target: UiNodeId::new("target"),
        },
        DragEvent::DragOver {
            target: UiNodeId::new("target"),
            position: DndPoint::new(10.0, 20.0),
            acceptance: DropAcceptance::Accept {
                effect: DropEffect::Copy,
                indicator: DropIndicator::new(
                    DropIndicatorKind::Inside,
                    DndRect::new(0.0, 0.0, 20.0, 20.0),
                ),
            },
        },
        DragEvent::DragCancel {
            source: UiNodeId::new("source"),
            reason: "cancelled".to_string(),
        },
    ];
    assert_eq!(4, variants.len());
    assert_eq!(serde_json::json!("payload"), data.payload);
}
