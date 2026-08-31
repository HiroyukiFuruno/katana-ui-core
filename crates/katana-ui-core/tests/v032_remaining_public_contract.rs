use katana_ui_core::interaction::{
    UiGestureSurface, UiSurfaceGestureCapabilities, UiSurfaceGestureCommand,
    UiSurfaceGestureController, UiSurfaceGestureInput, UiSurfaceGestureOverride, UiSurfacePoint,
};
use katana_ui_core::molecule::structured::{
    DiagnosticAction, DiagnosticId, DiagnosticItem, DiagnosticKeyboardInput, DiagnosticLocation,
    DiagnosticSeverity, DiagnosticsGroupBy, DiagnosticsList, DiagnosticsListAction,
    DiagnosticsListEvent, DiagnosticsSortBy, SourceAddressAction, SourceAddressEntry,
    SourceAddressEvent, SourceAddressPresentation, SourceAddressStrip,
};
use katana_ui_core::molecule::toolbar::{
    ToolbarKeyboardInput, ToolbarKeyboardNavigator, ToolbarKeyboardResult,
};
use katana_ui_core::render_model::{UiRect, UiStateId};
use std::cell::Cell;

#[test]
fn public_gesture_controller_exercises_every_input_and_override_route() {
    let pan_target = UiStateId::new("pan");
    let rich_target = UiStateId::new("rich");
    let pan = UiGestureSurface::new(pan_target.clone(), UiRect::new(0, 0, 20, 20))
        .capabilities(UiSurfaceGestureCapabilities::default().pointer_pan(true));
    let rich = UiGestureSurface::new(rich_target.clone(), UiRect::new(30, 0, 20, 20)).capabilities(
        UiSurfaceGestureCapabilities::default()
            .smooth_scroll_pan(true)
            .zoom(true)
            .fullscreen(true),
    );
    let passive = UiGestureSurface::new("passive", UiRect::new(60, 0, 20, 20));
    let mut controller = UiSurfaceGestureController::new(vec![pan, rich, passive]);

    let callback_called = Cell::new(false);
    let miss = controller.apply_with_override(
        UiSurfaceGestureInput::PointerDown {
            pointer_id: 1,
            position: UiSurfacePoint::new(100, 100),
        },
        |_| {
            callback_called.set(true);
            UiSurfaceGestureOverride::Ignore
        },
    );
    assert!(!miss.captured);
    assert!(!callback_called.get());

    let passive_down = controller.apply(UiSurfaceGestureInput::PointerDown {
        pointer_id: 2,
        position: UiSurfacePoint::new(65, 5),
    });
    assert_eq!(passive_down.target, Some(UiStateId::new("passive")));
    assert!(!passive_down.captured);

    assert!(
        controller
            .apply(UiSurfaceGestureInput::PointerDown {
                pointer_id: 3,
                position: UiSurfacePoint::new(5, 5),
            })
            .captured
    );
    assert!(
        !controller
            .apply(UiSurfaceGestureInput::PointerMove {
                pointer_id: 4,
                position: UiSurfacePoint::new(6, 6),
            })
            .captured
    );
    let moved = controller.apply_with_override(
        UiSurfaceGestureInput::PointerMove {
            pointer_id: 3,
            position: UiSurfacePoint::new(8, 9),
        },
        |_| UiSurfaceGestureOverride::UseDefault,
    );
    assert_eq!(
        moved.command,
        Some(UiSurfaceGestureCommand::PanBy {
            delta_x: 3.0,
            delta_y: 4.0,
        })
    );
    assert!(
        controller
            .apply(UiSurfaceGestureInput::PointerUp {
                pointer_id: 3,
                position: UiSurfacePoint::new(8, 9),
            })
            .captured
    );
    assert!(
        !controller
            .apply(UiSurfaceGestureInput::PointerUp {
                pointer_id: 3,
                position: UiSurfacePoint::new(8, 9),
            })
            .captured
    );

    let scrolled = controller.apply(UiSurfaceGestureInput::SmoothScroll {
        position: UiSurfacePoint::new(35, 5),
        delta_x: 2.0,
        delta_y: -3.0,
    });
    assert_eq!(
        scrolled.command,
        Some(UiSurfaceGestureCommand::PanBy {
            delta_x: 2.0,
            delta_y: -3.0,
        })
    );
    assert!(
        !controller
            .apply(UiSurfaceGestureInput::SmoothScroll {
                position: UiSurfacePoint::new(65, 5),
                delta_x: 1.0,
                delta_y: 1.0,
            })
            .captured
    );

    let zoomed = controller.apply_with_override(
        UiSurfaceGestureInput::Zoom {
            multiplier: 1.5,
            center: UiSurfacePoint::new(35, 5),
        },
        |_| {
            UiSurfaceGestureOverride::Command(UiSurfaceGestureCommand::ZoomBy {
                multiplier: 2.0,
                center: UiSurfacePoint::new(35, 5),
            })
        },
    );
    assert_eq!(
        zoomed.command,
        Some(UiSurfaceGestureCommand::ZoomBy {
            multiplier: 2.0,
            center: UiSurfacePoint::new(35, 5),
        })
    );
    let ignored = controller.apply_with_override(
        UiSurfaceGestureInput::Zoom {
            multiplier: 1.25,
            center: UiSurfacePoint::new(35, 5),
        },
        |_| UiSurfaceGestureOverride::Ignore,
    );
    assert!(!ignored.captured);
    assert!(ignored.command.is_none());
    for multiplier in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(
            !controller
                .apply(UiSurfaceGestureInput::Zoom {
                    multiplier,
                    center: UiSurfacePoint::new(35, 5),
                })
                .captured
        );
    }

    assert!(controller.set_fullscreen(&rich_target, true).is_some());
    assert_eq!(controller.set_fullscreen(&pan_target, true), None);
    assert_eq!(
        controller.set_fullscreen(&UiStateId::new("missing"), true),
        None
    );
}

#[test]
fn public_toolbar_keyboard_navigation_covers_all_keys_and_boundaries() {
    let cases = [
        (
            ToolbarKeyboardInput::ArrowLeft,
            ToolbarKeyboardResult::new(Some(0), None),
        ),
        (
            ToolbarKeyboardInput::ArrowUp,
            ToolbarKeyboardResult::new(Some(0), None),
        ),
        (
            ToolbarKeyboardInput::ArrowRight,
            ToolbarKeyboardResult::new(Some(2), None),
        ),
        (
            ToolbarKeyboardInput::ArrowDown,
            ToolbarKeyboardResult::new(Some(2), None),
        ),
        (
            ToolbarKeyboardInput::Home,
            ToolbarKeyboardResult::new(Some(0), None),
        ),
        (
            ToolbarKeyboardInput::End,
            ToolbarKeyboardResult::new(Some(2), None),
        ),
        (
            ToolbarKeyboardInput::Enter,
            ToolbarKeyboardResult::new(Some(1), Some(1)),
        ),
        (
            ToolbarKeyboardInput::Space,
            ToolbarKeyboardResult::new(Some(1), Some(1)),
        ),
        (
            ToolbarKeyboardInput::Escape,
            ToolbarKeyboardResult::new(Some(1), None),
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(ToolbarKeyboardNavigator::apply(Some(1), 3, input), expected);
    }
    assert_eq!(
        ToolbarKeyboardNavigator::apply(Some(99), 3, ToolbarKeyboardInput::End),
        ToolbarKeyboardResult::new(Some(2), None)
    );
    assert_eq!(
        ToolbarKeyboardNavigator::apply(None, 0, ToolbarKeyboardInput::Enter),
        ToolbarKeyboardResult::new(None, None)
    );
}

#[test]
fn public_source_address_contract_covers_open_close_reenable_and_opaque_debug() {
    let presentation = SourceAddressPresentation::new(
        String::from("visible"),
        String::from("tooltip"),
        String::from("accessible"),
    );
    assert_eq!(presentation.visible(), "visible");
    assert_eq!(presentation.tooltip(), "tooltip");
    assert_eq!(presentation.accessibility(), "accessible");

    let history = SourceAddressEntry::new(
        SourceAddressPresentation::new("history", "history tip", "history access"),
        b"secret-history".to_vec(),
    );
    assert_eq!(history.presentation().visible(), "history");
    assert_eq!(format!("{history:?}"), "SourceAddressEntry(..)");
    let candidate = SourceAddressEntry::new(
        SourceAddressPresentation::new("candidate", "candidate tip", "candidate access"),
        b"secret-candidate".to_vec(),
    );

    let mut strip = SourceAddressStrip::new(presentation);
    strip.set_history(vec![history]);
    strip.set_candidates(vec![candidate]);
    assert_eq!(strip.history().len(), 1);
    assert_eq!(strip.candidates().len(), 1);
    assert!(strip.presentation().visible().contains("visible"));

    assert!(matches!(
        strip.apply_action(SourceAddressAction::SetDraft("draft".into())),
        Some(SourceAddressEvent::DraftChanged)
    ));
    assert!(
        strip
            .apply_action(SourceAddressAction::SetDraft("draft".into()))
            .is_none()
    );
    assert!(matches!(
        strip.apply_action(SourceAddressAction::OpenHistory),
        Some(SourceAddressEvent::HistoryOpened)
    ));
    assert!(
        strip
            .apply_action(SourceAddressAction::OpenHistory)
            .is_none()
    );
    assert!(matches!(
        strip.apply_action(SourceAddressAction::CloseHistory),
        Some(SourceAddressEvent::HistoryClosed)
    ));
    assert!(
        strip
            .apply_action(SourceAddressAction::CloseHistory)
            .is_none()
    );
    assert!(
        strip
            .apply_action(SourceAddressAction::SelectHistory(9))
            .is_none()
    );
    assert!(matches!(
        strip.apply_action(SourceAddressAction::SelectHistory(0)),
        Some(SourceAddressEvent::HistorySelected)
    ));
    assert_eq!(strip.selected_history(), Some(0));

    assert!(matches!(
        strip.apply_action(SourceAddressAction::OpenCandidates),
        Some(SourceAddressEvent::CandidatesOpened)
    ));
    assert!(matches!(
        strip.apply_action(SourceAddressAction::CloseCandidates),
        Some(SourceAddressEvent::CandidatesClosed)
    ));
    assert!(
        strip
            .apply_action(SourceAddressAction::SelectCandidate(9))
            .is_none()
    );
    assert!(matches!(
        strip.apply_action(SourceAddressAction::SelectCandidate(0)),
        Some(SourceAddressEvent::CandidateSelected)
    ));
    assert_eq!(strip.selected_candidate(), Some(0));

    assert!(matches!(
        strip.apply_action(SourceAddressAction::SetFocused(true)),
        Some(SourceAddressEvent::Focused)
    ));
    assert!(matches!(
        strip.apply_action(SourceAddressAction::SetFocused(false)),
        Some(SourceAddressEvent::Blurred)
    ));
    assert!(matches!(
        strip.apply_action(SourceAddressAction::SetEnabled(false)),
        Some(SourceAddressEvent::EnabledChanged)
    ));
    assert!(
        strip
            .apply_action(SourceAddressAction::SetFocused(true))
            .is_none()
    );
    assert!(matches!(
        strip.apply_action(SourceAddressAction::SetEnabled(true)),
        Some(SourceAddressEvent::EnabledChanged)
    ));
    assert!(strip.enabled());

    let submitted_draft = match strip.apply_action(SourceAddressAction::Submit) {
        Some(SourceAddressEvent::Submitted(submission)) => Some(submission.into_draft()),
        _ => None,
    };
    assert_eq!(submitted_draft.as_deref(), Some("candidate"));
    let debug = format!("{strip:?}");
    assert!(!debug.contains("secret-history"));
    assert!(!debug.contains("secret-candidate"));
}

#[test]
fn public_diagnostics_contract_covers_empty_scope_and_keyboard_boundaries() {
    let fixed = DiagnosticItem::new(
        "error",
        DiagnosticSeverity::Error,
        "error",
        DiagnosticLocation::new("main.rs", 1, 1),
    )
    .quickfix(DiagnosticAction::new("fix", "Fix"))
    .scope("one");
    let warning = DiagnosticItem::new(
        "warning",
        DiagnosticSeverity::Warning,
        "warning",
        DiagnosticLocation::new("main.rs", 2, 1),
    )
    .scope("two");
    let mut list = DiagnosticsList::new("Diagnostics")
        .scope("one", "One", "Scope one")
        .scope("two", "Two", "Scope two")
        .item(fixed)
        .item(warning);
    let scopes = list.render_snapshot().scopes;
    let first_scope = scopes[0].key.clone();
    let second_scope = scopes[1].key.clone();

    let mut boundary_list = DiagnosticsList::new("Boundary diagnostics")
        .scope("one", "One", "Scope one")
        .scope("two", "Two", "Scope two");
    let missing_scope = DiagnosticsList::new("Other diagnostics")
        .scope("missing", "Missing", "Missing scope")
        .render_snapshot()
        .scopes[0]
        .key
        .clone();
    assert!(
        boundary_list
            .apply_action(DiagnosticsListAction::SelectScope(missing_scope))
            .is_empty()
    );
    assert!(matches!(
        boundary_list
            .apply_action(DiagnosticsListAction::Keyboard(
                DiagnosticKeyboardInput::ScopePrevious,
            ))
            .as_slice(),
        [DiagnosticsListEvent::ScopeSelected { scope_key }] if scope_key.as_str() == "two"
    ));

    for action in [
        DiagnosticsListAction::SetGroupBy(DiagnosticsGroupBy::Severity),
        DiagnosticsListAction::SetSortBy(DiagnosticsSortBy::Severity),
        DiagnosticsListAction::SetSeverityFilter(
            [DiagnosticSeverity::Error, DiagnosticSeverity::Warning]
                .into_iter()
                .collect(),
        ),
    ] {
        assert_eq!(
            list.apply_action(action),
            vec![DiagnosticsListEvent::FilterChanged]
        );
    }

    assert!(matches!(
        list.apply_action(DiagnosticsListAction::SelectScope(second_scope.clone()))
            .as_slice(),
        [DiagnosticsListEvent::ScopeSelected { .. }]
    ));
    assert!(
        list.apply_action(DiagnosticsListAction::SelectScope(second_scope))
            .is_empty()
    );

    assert!(matches!(
        list.apply_action(DiagnosticsListAction::Select(DiagnosticId::new("error")))
            .as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { .. }]
    ));
    assert!(matches!(
        list.apply_action(DiagnosticsListAction::ToggleFixPreview(DiagnosticId::new(
            "error"
        )))
        .as_slice(),
        [DiagnosticsListEvent::DiagnosticFixPreviewToggled { expanded: true, .. }]
    ));
    for input in [
        DiagnosticKeyboardInput::ArrowLeft,
        DiagnosticKeyboardInput::Enter,
        DiagnosticKeyboardInput::Space,
    ] {
        assert!(
            list.apply_action(DiagnosticsListAction::Keyboard(input))
                .is_empty(),
            "hidden retained selection must not activate from {input:?}"
        );
    }

    assert!(matches!(
        list.apply_action(DiagnosticsListAction::SelectScope(first_scope))
            .as_slice(),
        [DiagnosticsListEvent::ScopeSelected { .. }]
    ));
    assert!(matches!(
        list.apply_action(DiagnosticsListAction::Keyboard(
            DiagnosticKeyboardInput::ArrowLeft
        ))
        .as_slice(),
        [DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            expanded: false,
            ..
        }]
    ));
    assert!(matches!(
        list.apply_action(DiagnosticsListAction::Keyboard(
            DiagnosticKeyboardInput::Enter
        ))
        .as_slice(),
        [DiagnosticsListEvent::NavigateRequested { .. }]
    ));
    assert!(matches!(
        list.apply_action(DiagnosticsListAction::Keyboard(
            DiagnosticKeyboardInput::Space
        ))
        .as_slice(),
        [DiagnosticsListEvent::DiagnosticFixApplied { .. }]
    ));

    for input in [
        DiagnosticKeyboardInput::ArrowUp,
        DiagnosticKeyboardInput::ArrowDown,
        DiagnosticKeyboardInput::ArrowRight,
        DiagnosticKeyboardInput::F8,
        DiagnosticKeyboardInput::ShiftF8,
        DiagnosticKeyboardInput::ScopeNext,
        DiagnosticKeyboardInput::ScopePrevious,
    ] {
        let events = list.apply_action(DiagnosticsListAction::Keyboard(input));
        assert!(events.len() <= 1);
    }

    assert!(
        list.apply_action(DiagnosticsListAction::ApplyFix(DiagnosticId::new(
            "missing"
        )))
        .is_empty()
    );
    assert_eq!(
        list.apply_action(DiagnosticsListAction::OpenBulkPreview),
        vec![DiagnosticsListEvent::BulkFixPreviewOpened]
    );
    assert!(matches!(
        list.apply_action(DiagnosticsListAction::ConfirmBulkApply)
            .as_slice(),
        [DiagnosticsListEvent::BulkFixApplied { .. }]
    ));

    list.set_scopes(Vec::new());
    assert!(list.render_snapshot().state.selected_scope_key.is_none());
    for input in [
        DiagnosticKeyboardInput::ArrowUp,
        DiagnosticKeyboardInput::ArrowDown,
        DiagnosticKeyboardInput::ArrowLeft,
        DiagnosticKeyboardInput::ArrowRight,
        DiagnosticKeyboardInput::Enter,
        DiagnosticKeyboardInput::Space,
        DiagnosticKeyboardInput::F8,
        DiagnosticKeyboardInput::ShiftF8,
        DiagnosticKeyboardInput::ScopeNext,
        DiagnosticKeyboardInput::ScopePrevious,
    ] {
        let _events =
            DiagnosticsList::new("Empty").apply_action(DiagnosticsListAction::Keyboard(input));
    }
}
