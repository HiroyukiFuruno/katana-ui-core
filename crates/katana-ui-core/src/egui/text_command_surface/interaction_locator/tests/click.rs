use super::super::types::OpaqueClickPhase;
use super::super::{
    KucInteractionActionClass, KucInteractionLocatorError, KucInteractionSelector,
    KucOpaqueClickContinuationError,
};
use super::common::{
    CLICK_EVENT_COUNT_THREE, CLICK_FRAME_PRESS, CLICK_FRAME_RELEASE, CLICK_FRAME_SOURCE,
    KUC_FRAME_STEP_ONE, click_geometry_locator, click_selector, locator, target,
};

#[test]
fn click_continuation_starts_with_a_real_pointer_aim_event() {
    use crate::atom::TextArea;
    use crate::egui::text_command_surface::{
        EguiTextCommandSurface, EguiTextCommandSurfaceRoot, TextCommandSurfaceStyle,
    };
    use crate::molecule::command_chrome::{CommandChromeAction, CommandChromeToolbar};
    use crate::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

    let text = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("click-aim").value("click aim"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 180),
    ));
    let surface = EguiTextCommandSurface::new(text)
        .with_toolbar(CommandChromeToolbar::new().action(CommandChromeAction::new("open", "Open")));
    let mut root = EguiTextCommandSurfaceRoot::with_identity("click-root", surface)
        .expect("click root retains");
    let context = egui::Context::default();
    let style = TextCommandSurfaceStyle::standard().expect("style");
    let mut output = None;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 180.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| output = Some(root.show(ui, &style)),
    );
    let output = output
        .expect("actual root frame ran")
        .expect("actual root frame renders");
    let mut continuation = output
        .interaction_locator()
        .begin_click(click_selector("open", KucInteractionActionClass::Toolbar))
        .expect("click continuation starts");
    let mut input = egui::RawInput::default();

    continuation
        .apply_to_raw_input_once(&mut input)
        .expect("aim event applies");

    assert!(matches!(
        input.events.as_slice(),
        [egui::Event::PointerMoved(_)]
    ));
}

#[test]
fn click_continuation_applies_once_and_advances_until_release() {
    let start = || {
        locator(
            "root",
            CLICK_FRAME_SOURCE,
            vec![target("open", KucInteractionActionClass::Toolbar, false)],
        )
        .begin_click(click_selector("open", KucInteractionActionClass::Toolbar))
        .expect("click continuation started")
    };

    {
        let mut input = egui::RawInput::default();
        let not_applied = start();
        assert!(matches!(
            not_applied.advance(&click_geometry_locator("root", CLICK_FRAME_PRESS)),
            Err(KucOpaqueClickContinuationError::NotApplied)
        ));
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert_eq!(
            continuation.apply_to_raw_input_once(&mut input),
            Err(KucOpaqueClickContinuationError::AlreadyApplied)
        );
    }

    {
        let mut input = egui::RawInput::default();
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert!(matches!(
            continuation.advance(&locator(
                "other",
                CLICK_FRAME_PRESS,
                vec![target("open", KucInteractionActionClass::Toolbar, false)],
            )),
            Err(KucOpaqueClickContinuationError::RootMismatch)
        ));
    }

    {
        let mut input = egui::RawInput::default();
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert!(matches!(
            continuation.advance(&locator(
                "root",
                CLICK_FRAME_RELEASE,
                vec![target("open", KucInteractionActionClass::Toolbar, false)],
            )),
            Err(KucOpaqueClickContinuationError::FrameDiscontinuity)
        ));
    }

    {
        let mut input = egui::RawInput::default();
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert!(matches!(
            continuation.advance(&locator(
                "root",
                CLICK_FRAME_PRESS,
                vec![target("open", KucInteractionActionClass::Toolbar, true)],
            )),
            Err(KucOpaqueClickContinuationError::Interaction(
                KucInteractionLocatorError::Disabled
            ))
        ));
    }

    let mut input = egui::RawInput::default();

    let mut next = start();
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));
    let mut next = next
        .advance(&locator(
            "root",
            CLICK_FRAME_PRESS,
            vec![target("open", KucInteractionActionClass::Toolbar, false)],
        ))
        .expect("press phase")
        .expect("click press");
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));
    assert_eq!(input.events.len(), 2);

    let mut final_continuation = next
        .advance(&locator(
            "root",
            CLICK_FRAME_RELEASE,
            vec![target("open", KucInteractionActionClass::Toolbar, false)],
        ))
        .expect("release phase")
        .expect("click release");
    assert_eq!(
        final_continuation.apply_to_raw_input_once(&mut input),
        Ok(())
    );
    assert_eq!(input.events.len(), CLICK_EVENT_COUNT_THREE);
    assert!(
        final_continuation
            .advance(&click_geometry_locator(
                "root",
                CLICK_FRAME_RELEASE + KUC_FRAME_STEP_ONE
            ))
            .is_ok()
    );
}

#[test]
fn click_continuation_reports_missing_action_if_target_disappears() {
    let source = locator(
        "root",
        KUC_FRAME_STEP_ONE,
        vec![target("open", KucInteractionActionClass::Toolbar, false)],
    );
    let mut continuation = source
        .begin_click(KucInteractionSelector::new(
            "open",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("click continuation started");
    let mut input = egui::RawInput::default();

    assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
    let next_frame = locator("root", KUC_FRAME_STEP_ONE + 1, Vec::new());
    assert!(matches!(
        continuation.advance(&next_frame),
        Err(KucOpaqueClickContinuationError::Interaction(
            KucInteractionLocatorError::Missing
        ))
    ));
}

#[test]
fn begin_click_fails_closed_when_the_requested_target_is_missing() {
    let source = locator("root", KUC_FRAME_STEP_ONE, Vec::new());

    assert!(matches!(
        source.begin_click(click_selector(
            "missing",
            KucInteractionActionClass::Toolbar,
        )),
        Err(KucInteractionLocatorError::Missing)
    ));
}

#[test]
fn context_click_uses_secondary_pointer_for_press_and_release() {
    let source = locator(
        "root",
        KUC_FRAME_STEP_ONE,
        vec![target(
            "context",
            KucInteractionActionClass::TextSurfaceContextTarget,
            false,
        )],
    );
    let selector = click_selector(
        "context",
        KucInteractionActionClass::TextSurfaceContextTarget,
    );
    let mut aim = source.begin_click(selector).expect("context click starts");
    let mut raw = egui::RawInput::default();
    aim.apply_to_raw_input_once(&mut raw).expect("aim applies");

    let pressed_locator = locator(
        "root",
        KUC_FRAME_STEP_ONE + 1,
        vec![target(
            "context",
            KucInteractionActionClass::TextSurfaceContextTarget,
            false,
        )],
    );
    let mut press = aim
        .advance(&pressed_locator)
        .expect("press advances")
        .expect("press step");
    let mut press_raw = egui::RawInput::default();
    press
        .apply_to_raw_input_once(&mut press_raw)
        .expect("press applies");
    assert!(matches!(
        press_raw.events.as_slice(),
        [egui::Event::PointerButton {
            button: egui::PointerButton::Secondary,
            pressed: true,
            ..
        }]
    ));

    let released_locator = locator(
        "root",
        KUC_FRAME_STEP_ONE + 2,
        vec![target(
            "context",
            KucInteractionActionClass::TextSurfaceContextTarget,
            false,
        )],
    );
    let mut release = press
        .advance(&released_locator)
        .expect("release advances")
        .expect("release step");
    let mut release_raw = egui::RawInput::default();
    release
        .apply_to_raw_input_once(&mut release_raw)
        .expect("release applies");
    assert!(matches!(
        release_raw.events.as_slice(),
        [egui::Event::PointerButton {
            button: egui::PointerButton::Secondary,
            pressed: false,
            ..
        }]
    ));
}

#[test]
fn click_continuation_rejects_hidden_duplicate_and_overlapping_targets() {
    fn applied_start() -> super::super::KucOpaqueClickContinuation {
        let source = locator(
            "root",
            KUC_FRAME_STEP_ONE,
            vec![target("open", KucInteractionActionClass::Toolbar, false)],
        );
        let mut continuation = source
            .begin_click(click_selector("open", KucInteractionActionClass::Toolbar))
            .expect("click starts");
        continuation
            .apply_to_raw_input_once(&mut egui::RawInput::default())
            .expect("aim applies");
        continuation
    }

    let mut hidden = locator(
        "root",
        KUC_FRAME_STEP_ONE + 1,
        vec![target("open", KucInteractionActionClass::Toolbar, false)],
    );
    hidden
        .hidden
        .insert(("open".to_owned(), KucInteractionActionClass::Toolbar));
    assert!(matches!(
        applied_start().advance(&hidden),
        Err(KucOpaqueClickContinuationError::Interaction(
            KucInteractionLocatorError::Hidden
        ))
    ));

    let duplicate = locator(
        "root",
        KUC_FRAME_STEP_ONE + 1,
        vec![
            target("open", KucInteractionActionClass::Toolbar, false),
            target("open", KucInteractionActionClass::Toolbar, false),
        ],
    );
    assert!(matches!(
        applied_start().advance(&duplicate),
        Err(KucOpaqueClickContinuationError::Interaction(
            KucInteractionLocatorError::Ambiguous
        ))
    ));

    let mut overlapping = locator(
        "root",
        KUC_FRAME_STEP_ONE + 1,
        vec![target("open", KucInteractionActionClass::Toolbar, false)],
    );
    overlapping
        .ambiguous_bounds
        .push(overlapping.targets[0].evidence.bounds);
    assert!(matches!(
        applied_start().advance(&overlapping),
        Err(KucOpaqueClickContinuationError::Interaction(
            KucInteractionLocatorError::Ambiguous
        ))
    ));
}

#[test]
fn click_event_aim_phase_returns_pointer_move_for_real_target_coordinates() {
    let locator = locator(
        "root",
        KUC_FRAME_STEP_ONE,
        vec![target("open", KucInteractionActionClass::Toolbar, false)],
    );
    let event = locator
        .click_event(
            &click_selector("open", KucInteractionActionClass::Toolbar),
            OpaqueClickPhase::Aim,
        )
        .expect("aim phase should produce pointer-move event");

    assert!(matches!(event, egui::Event::PointerMoved { .. }));
}
