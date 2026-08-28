use katana_ui_core::interaction::placement::{Placement, PlacementResult, Point, Rect, Size};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeToolbar,
    CommandChromeToolbarAction, FloatingCommandToolbar, FloatingCommandToolbarAction,
    FloatingCommandToolbarCloseReason, FloatingCommandToolbarEvent, FloatingCommandToolbarLayout,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::render_model::{UiIconProps, UiNodeId};

#[test]
fn open_uses_shared_engine_to_flip_the_toolbar_bounds() {
    let mut toolbar = floating_toolbar(FloatingCommandToolbarLayout::new(
        Rect::new(20, 86, 8, 8),
        Size::new(40, 24),
        Rect::new(0, 0, 100, 100),
    ));

    let events = toolbar.apply_action(FloatingCommandToolbarAction::Open);
    let placement = PlacementResult {
        placement_used: Placement::TopStart,
        position: Point::new(20, 62),
        arrow_offset: None,
        clamped: false,
    };

    assert_eq!(
        vec![FloatingCommandToolbarEvent::Opened { placement }],
        events
    );
    assert!(toolbar.is_open());
    assert_eq!(Some(placement), toolbar.placement_model());
    assert_eq!(Some(Rect::new(20, 62, 40, 24)), toolbar.bounds_model());
}

#[test]
fn default_initial_visibility_is_closed_without_placement_or_bounds() {
    let toolbar = floating_toolbar(default_layout());

    assert_eq!(
        FloatingCommandToolbarVisibility::Closed,
        toolbar.visibility_model()
    );
    assert!(!toolbar.is_open());
    assert_eq!(None, toolbar.placement_model());
    assert_eq!(None, toolbar.bounds_model());
}

#[test]
fn visible_initial_state_resolves_placement_and_bounds_without_open_event() {
    let toolbar = floating_toolbar(FloatingCommandToolbarLayout::new(
        Rect::new(20, 86, 8, 8),
        Size::new(40, 24),
        Rect::new(0, 0, 100, 100),
    ))
    .initial_visibility(FloatingCommandToolbarVisibility::Visible);
    let placement = PlacementResult {
        placement_used: Placement::TopStart,
        position: Point::new(20, 62),
        arrow_offset: None,
        clamped: false,
    };

    assert_eq!(
        FloatingCommandToolbarVisibility::Visible,
        toolbar.visibility_model()
    );
    assert!(toolbar.is_open());
    assert_eq!(Some(placement), toolbar.placement_model());
    assert_eq!(Some(Rect::new(20, 62, 40, 24)), toolbar.bounds_model());
}

#[test]
fn visible_initial_state_is_ready_for_adapter_query_without_an_open_action() {
    let toolbar = floating_toolbar(default_layout())
        .initial_visibility(FloatingCommandToolbarVisibility::Visible);

    assert!(toolbar.is_open());
    assert!(toolbar.placement_model().is_some());
    assert!(toolbar.bounds_model().is_some());
}

#[test]
fn visible_initial_state_does_not_synthesize_an_opened_event() {
    let mut toolbar = floating_toolbar(default_layout())
        .initial_visibility(FloatingCommandToolbarVisibility::Visible);

    let events = toolbar.apply_action(FloatingCommandToolbarAction::Open);

    assert!(matches!(
        events.as_slice(),
        [FloatingCommandToolbarEvent::Repositioned { .. }]
    ));
}

#[test]
fn consumer_surface_click_closes_once_and_requests_focus_return_once() {
    let mut toolbar =
        floating_toolbar(default_layout()).focus_return_target(UiNodeId::new("surface"));
    let _ = toolbar.apply_action(FloatingCommandToolbarAction::Open);

    assert_eq!(
        vec![
            FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::ConsumerSurfaceClick,
            },
            FloatingCommandToolbarEvent::FocusReturnRequested {
                target: UiNodeId::new("surface"),
            },
        ],
        toolbar.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::ConsumerSurfaceClick,
        })
    );
    assert!(!toolbar.is_open());
    assert!(
        toolbar
            .apply_action(FloatingCommandToolbarAction::Dismiss {
                reason: FloatingCommandToolbarCloseReason::ConsumerSurfaceClick,
            })
            .is_empty()
    );
}

#[test]
fn outside_click_and_escape_use_explicit_close_reasons() {
    let mut toolbar = floating_toolbar(default_layout());
    let _ = toolbar.apply_action(FloatingCommandToolbarAction::Open);

    assert_eq!(
        vec![FloatingCommandToolbarEvent::Closed {
            reason: FloatingCommandToolbarCloseReason::OutsideClick,
        }],
        toolbar.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::OutsideClick,
        })
    );

    let _ = toolbar.apply_action(FloatingCommandToolbarAction::Open);
    assert_eq!(
        vec![FloatingCommandToolbarEvent::Closed {
            reason: FloatingCommandToolbarCloseReason::Escape,
        }],
        toolbar.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::Escape,
        })
    );
}

#[test]
fn toolbar_interaction_retains_focus_and_emits_typed_inner_event_without_host_work() {
    let mut toolbar = floating_toolbar(default_layout());
    let _ = toolbar.apply_action(FloatingCommandToolbarAction::Open);

    assert_eq!(
        vec![
            FloatingCommandToolbarEvent::FocusRetained,
            FloatingCommandToolbarEvent::Toolbar {
                event: katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent::CommandActivated {
                    action_id: "bold".into(),
                },
            },
        ],
        toolbar.apply_action(FloatingCommandToolbarAction::Toolbar {
            action: CommandChromeToolbarAction::activate("bold"),
        })
    );
    assert!(toolbar.is_open());
}

#[test]
fn layout_update_reuses_the_same_engine_record_while_open() {
    let mut toolbar = floating_toolbar(default_layout());
    let _ = toolbar.apply_action(FloatingCommandToolbarAction::Open);
    let layout = FloatingCommandToolbarLayout::new(
        Rect::new(6, 6, 8, 8),
        Size::new(40, 24),
        Rect::new(0, 0, 100, 100),
    );

    let events = toolbar.apply_action(FloatingCommandToolbarAction::UpdateLayout { layout });

    assert_eq!(1, events.len());
    assert!(matches!(
        events[0],
        FloatingCommandToolbarEvent::Repositioned { placement } if placement.clamped
    ));
    assert_eq!(layout, toolbar.layout_model());
    assert_eq!(Some(Rect::new(8, 14, 40, 24)), toolbar.bounds_model());
}

fn floating_toolbar(layout: FloatingCommandToolbarLayout) -> FloatingCommandToolbar {
    let command_toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::IconOnly)
        .action(
            CommandChromeAction::new("bold", "Bold")
                .icon(icon())
                .tooltip("Bold"),
        );
    FloatingCommandToolbar::new(command_toolbar, layout)
}

fn default_layout() -> FloatingCommandToolbarLayout {
    FloatingCommandToolbarLayout::new(
        Rect::new(40, 40, 8, 8),
        Size::new(40, 24),
        Rect::new(0, 0, 100, 100),
    )
}

fn icon() -> UiIconProps {
    UiIconProps::new("<svg viewBox=\"0 0 16 16\"><path d=\"M1 1h14v14H1z\"/></svg>")
}
