use katana_ui_core::event::{DragEvent, UiEvent};
use katana_ui_core::interaction::drag_and_drop::{DragData, DropEffect};
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
