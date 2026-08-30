use super::root_frame_support::*;
use super::search_support::*;
use super::search_support_tail::*;
use super::support::*;
use super::*;

#[test]
fn physical_raw_input_routes_text_replace_and_navigation_operations() {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders(
            1,
            text_events.clone(),
            unit_events.clone(),
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let query = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️").center();
    let _ = run_search_root_frame(&context, &mut root, vec![pointer_button(query, true)]);
    let _ = run_search_root_frame(&context, &mut root, vec![pointer_button(query, false)]);
    let query_frame = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語 ⭐️👩‍💻".to_string(),
        ))],
    )
    .1;
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    query_frame
        .forward_events_once(&mut forwarder)
        .expect("query forwards");

    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let replacement =
        accesskit_bounds(&output, egui::accesskit::Role::TextInput, "置換 ⭐️").center();
    let _ = run_search_root_frame(&context, &mut root, vec![pointer_button(replacement, true)]);
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(replacement, false)],
    );
    let replacement_frame = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "置換後 ⭐️👩‍💻".to_string(),
        ))],
    )
    .1;
    replacement_frame
        .forward_events_once(&mut forwarder)
        .expect("replacement forwards");

    for (label, operation) in [
        ("置換 ⭐️", SanitizedSearchTextOperation::Replace),
        ("すべて置換 ⭐️", SanitizedSearchTextOperation::ReplaceAll),
    ] {
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, label);
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        frame
            .forward_events_once(&mut forwarder)
            .expect("replace operation forwards");
        assert!(text_events.borrow().iter().any(|(actual, value)| {
            *actual == operation && value == "置換後 ⭐️👩‍💻"
        }));
    }

    assert_eq!(
        text_events.borrow().as_slice(),
        &[
            (
                SanitizedSearchTextOperation::Query,
                "日本語 ⭐️👩‍💻".to_string()
            ),
            (
                SanitizedSearchTextOperation::Replacement,
                "置換後 ⭐️👩‍💻".to_string(),
            ),
            (
                SanitizedSearchTextOperation::Replace,
                "置換後 ⭐️👩‍💻".to_string(),
            ),
            (
                SanitizedSearchTextOperation::ReplaceAll,
                "置換後 ⭐️👩‍💻".to_string(),
            ),
        ]
    );

    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let mut next_root = factory
        .retain(input_with_recorders(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
        ))
        .expect("retain succeeds");
    let (output, _) = run_search_root_frame(&context, &mut next_root, Vec::new());
    let query = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️").center();
    let _ = run_search_root_frame(&context, &mut next_root, vec![pointer_button(query, true)]);
    let _ = run_search_root_frame(&context, &mut next_root, vec![pointer_button(query, false)]);
    let frame = run_search_root_frame(
        &context,
        &mut next_root,
        vec![key_press(egui::Key::ArrowDown)],
    )
    .1;
    frame
        .forward_events_once(&mut forwarder)
        .expect("next navigation forwards");
    assert_eq!(
        unit_events.borrow().as_slice(),
        &[SanitizedSearchUnitOperation::Next]
    );
    let frame = run_search_root_frame(
        &context,
        &mut next_root,
        vec![key_press(egui::Key::ArrowUp)],
    )
    .1;
    frame
        .forward_events_once(&mut forwarder)
        .expect("previous navigation forwards");
    assert_eq!(
        unit_events.borrow().as_slice(),
        &[
            SanitizedSearchUnitOperation::Next,
            SanitizedSearchUnitOperation::Previous,
        ]
    );
}
