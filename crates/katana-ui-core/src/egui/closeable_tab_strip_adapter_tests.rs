use super::CloseableTabStripAdapter;
use crate::molecule::structured::{
    CloseableTab, CloseableTabClosePresentation, CloseableTabGroup, CloseableTabStrip,
    CloseableTabStripEvent,
};

const SCREEN_SIZE: egui::Vec2 = egui::vec2(600.0, 100.0);

#[test]
fn pointer_click_selects_a_generic_tab_through_egui_response() {
    let context = egui::Context::default();
    let mut strip = CloseableTabStrip::new("tabs")
        .tab(crate::molecule::structured::CloseableTab::new(
            "first", "one",
        ))
        .tab(crate::molecule::structured::CloseableTab::new(
            "target",
            "selected-target-with-known-width",
        ))
        .active_tab_id("first");

    let adapter = CloseableTabStripAdapter;
    let first_frame = run_frame(&context, &adapter, &mut strip, Vec::new());
    let target = first_frame
        .tab_rects()
        .iter()
        .find(|(tab_id, _)| tab_id == "target")
        .expect("target tab response exists")
        .1
        .center();
    let _ = run_frame(
        &context,
        &adapter,
        &mut strip,
        vec![pointer_button(target, true)],
    );
    let release_frame = run_frame(
        &context,
        &adapter,
        &mut strip,
        vec![pointer_button(target, false)],
    );

    assert_eq!(
        strip.state().active_tab_id.as_ref().map(|id| id.as_str()),
        Some("target")
    );
    assert!(release_frame.events().iter().any(|event| {
        matches!(event, CloseableTabStripEvent::TabSelected { tab_id } if tab_id.as_str() == "target")
    }));
}

#[test]
fn real_egui_render_exposes_group_and_tab_widget_responses() {
    let context = egui::Context::default();
    let adapter = CloseableTabStripAdapter;
    let mut strip = CloseableTabStrip::new("tabs")
        .group(CloseableTabGroup::new("documents", "Documents"))
        .tab(CloseableTab::new("readme", "README").group_id("documents"))
        .tab(CloseableTab::new("notes", "Notes"));

    let rendered = run_frame(&context, &adapter, &mut strip, Vec::new());

    assert!(rendered.widget_rect().height() > 0.0);
    assert_eq!(rendered.group_rects().len(), 1);
    assert_eq!(rendered.group_rects()[0].0, "documents");
    assert_eq!(rendered.tab_rects().len(), 2);
    assert!(
        rendered
            .group_rects()
            .iter()
            .all(|(_, rect)| rect.height() > 0.0)
    );
    assert!(
        rendered
            .tab_rects()
            .iter()
            .all(|(_, rect)| rect.width() > 0.0)
    );
}

#[test]
fn real_egui_render_covers_each_tab_bucket_and_nested_group() {
    let context = egui::Context::default();
    let adapter = CloseableTabStripAdapter;
    let mut strip = CloseableTabStrip::new("tabs")
        .group(CloseableTabGroup::new("root", "Root"))
        .group(CloseableTabGroup::new("child", "Child").parent_group("root"))
        .tab(
            CloseableTab::new("pinned", "Pinned")
                .pinned(true)
                .dirty(true),
        )
        .tab(CloseableTab::new("root-tab", "Root tab").group_id("root"))
        .tab(CloseableTab::new("child-tab", "Child tab").group_id("child"))
        .tab(CloseableTab::new("unknown", "Unknown").group_id("missing"))
        .tab(CloseableTab::new("ungrouped", "Ungrouped"));

    let rendered = run_frame(&context, &adapter, &mut strip, Vec::new());
    let tab_ids = rendered
        .tab_rects()
        .iter()
        .map(|(tab_id, _)| tab_id.as_str())
        .collect::<Vec<_>>();
    let group_ids = rendered
        .group_rects()
        .iter()
        .map(|(group_id, _)| group_id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        tab_ids,
        ["pinned", "root-tab", "child-tab", "unknown", "ungrouped"]
    );
    assert_eq!(group_ids, ["child", "root"]);
}

#[test]
fn collapsed_group_omits_its_tab_rows() {
    let context = egui::Context::default();
    let adapter = CloseableTabStripAdapter;
    let mut strip = CloseableTabStrip::new("tabs")
        .group(CloseableTabGroup::new("collapsed", "Collapsed").collapsed(true))
        .tab(CloseableTab::new("hidden", "Hidden").group_id("collapsed"));
    let rendered = run_frame(&context, &adapter, &mut strip, Vec::new());
    assert!(rendered.tab_rects().is_empty());
    assert_eq!(rendered.group_rects().len(), 1);
}

#[test]
fn pointer_click_toggles_a_group_through_the_actual_header_response() {
    let context = egui::Context::default();
    let adapter = CloseableTabStripAdapter;
    let mut strip = CloseableTabStrip::new("tabs")
        .group(CloseableTabGroup::new("documents", "Documents"))
        .tab(CloseableTab::new("readme", "README").group_id("documents"));
    let first = run_frame(&context, &adapter, &mut strip, Vec::new());
    let point = first.group_rects()[0].1.center();
    let _ = run_frame(
        &context,
        &adapter,
        &mut strip,
        vec![pointer_button(point, true)],
    );
    let released = run_frame(
        &context,
        &adapter,
        &mut strip,
        vec![pointer_button(point, false)],
    );
    assert!(released.events().iter().any(|event| matches!(
        event,
        CloseableTabStripEvent::GroupCollapseChanged { group_id, .. }
            if group_id.as_str() == "documents"
    )));
}

#[test]
fn pointer_click_requests_close_through_the_actual_close_response() {
    let context = egui::Context::default();
    let adapter = CloseableTabStripAdapter;
    let mut strip =
        CloseableTabStrip::new("tabs").tab(CloseableTab::new("notes", "Notes").close_presentation(
            CloseableTabClosePresentation::new("×", "Close Notes", "Close Notes"),
        ));
    let first = run_frame(&context, &adapter, &mut strip, Vec::new());
    let point = first.close_rects()[0].1.center();
    let _ = run_frame(
        &context,
        &adapter,
        &mut strip,
        vec![pointer_button(point, true)],
    );
    let released = run_frame(
        &context,
        &adapter,
        &mut strip,
        vec![pointer_button(point, false)],
    );
    assert!(released.events().iter().any(|event| matches!(
        event,
        CloseableTabStripEvent::TabCloseRequested { tab_id }
            | CloseableTabStripEvent::TabClosed { tab_id }
            if tab_id.as_str() == "notes"
    )));
}

fn run_frame(
    context: &egui::Context,
    adapter: &CloseableTabStripAdapter,
    strip: &mut CloseableTabStrip,
    events: Vec<egui::Event>,
) -> super::CloseableTabStripRender {
    let mut output = None;
    crate::egui::run_ui_discard(
        context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
            events,
            ..egui::RawInput::default()
        },
        |ui| output = Some(adapter.show(ui, strip)),
    );
    output.expect("tab strip frame is produced")
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}
