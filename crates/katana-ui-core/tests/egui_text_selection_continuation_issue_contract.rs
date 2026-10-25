#![cfg(feature = "egui")]

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceFloatingPresentation, EguiTextCommandSurfaceHostProjectionEncoder,
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory,
    KucRootEventBatchContext, KucRootEventBatchForwarder, KucTextSelectionContinuationError,
    TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeToolbarPresentation,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use std::{cell::Cell, rc::Rc};

struct Forwarder(usize);
impl KucRootEventBatchForwarder for Forwarder {
    type Error = ();
    fn forward_root_event_batch(
        &mut self,
        _: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), ()> {
        self.0 += 1;
        Ok(())
    }
}

struct Fixture {
    root: EguiTextCommandSurfaceHostRoot,
    presentation: EguiTextCommandSurfacePresentation,
    selection: Rc<Cell<(usize, usize)>>,
    changes: Rc<Cell<usize>>,
    revision: u64,
}

impl Fixture {
    fn new(source: &str) -> Self {
        let surface = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("selection-fixture").value(source),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 1280, 720),
        ));
        let presentation = EguiTextCommandSurfacePresentation {
            text_state_id: None,
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: Some(EguiTextCommandSurfaceFloatingPresentation {
                toolbar: CommandChromeToolbarPresentation {
                    actions: vec![CommandChromeAction::new("format", "Format")],
                    groups: Vec::new(),
                    display_mode: CommandChromeDisplayMode::LabelOnly,
                    density: Default::default(),
                    overflow_strategy: Default::default(),
                },
                visibility: FloatingCommandToolbarVisibility::Visible,
            }),
            search: None,
            context_menu: None,
        };
        let selection = Rc::new(Cell::new((source.len(), source.len())));
        let changes = Rc::new(Cell::new(0));
        let root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(Self::lease(
                1,
                &presentation,
                Rc::clone(&selection),
                Rc::clone(&changes),
            ))
            .expect("root");
        Self {
            root,
            presentation,
            selection,
            changes,
            revision: 1,
        }
    }

    fn lease(
        revision: u64,
        presentation: &EguiTextCommandSurfacePresentation,
        selection: Rc<Cell<(usize, usize)>>,
        changes: Rc<Cell<usize>>,
    ) -> EguiTextCommandSurfaceHostProjectionLease {
        let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
            revision,
            b"continuation-root",
            presentation.clone(),
            TextCommandSurfaceStyle::standard().expect("style"),
        )
        .expect("token");
        EguiTextCommandSurfaceHostProjectionLease::new(
            token,
            move |context: KucRootEventBatchContext| {
                for event in context.text_events() {
                    match event {
                        TextSurfaceEvent::SelectionChanged {
                            selection_start,
                            selection_end,
                        } => selection.set((*selection_start, *selection_end)),
                        TextSurfaceEvent::TextArea(
                            katana_ui_core::atom::TextAreaEvent::Change(_),
                        ) => changes.set(changes.get() + 1),
                        _ => {}
                    }
                }
                Ok(None)
            },
        )
    }

    fn synchronize(&mut self) {
        self.revision += 1;
        let (start, end) = self.selection.get();
        self.presentation.text.selection_start = start;
        self.presentation.text.selection_end = end;
        self.root
            .synchronize_with_lease(Self::lease(
                self.revision,
                &self.presentation,
                Rc::clone(&self.selection),
                Rc::clone(&self.changes),
            ))
            .expect("synchronize");
    }

    fn frame(
        &mut self,
        context: &egui::Context,
        input: egui::RawInput,
    ) -> EguiTextCommandSurfaceHostRootFrame {
        let mut result = None;
        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 720.0),
                )),
                ..input
            },
            |ui| result = Some(self.root.show(ui).expect("show")),
        );
        output.textures_delta.clear();
        let result = result.expect("frame");
        let mut forwarder = Forwarder(0);
        result.forward_events_once(&mut forwarder).expect("forward");
        assert_eq!(forwarder.0, 1);
        assert!(result.forward_events_once(&mut forwarder).is_err());
        result
    }
}

#[test]
fn full_root_selection_continuation_handles_empty_short_and_trailing_lines_with_lease_echo() {
    for source in [
        "# Sample document\n\n日本語⭐️を編集してください。\n\n\u{0060}\u{0060}\u{0060}rust\nfn main() {}\n\u{0060}\u{0060}\u{0060}\n",
        "A\n\nB\n",
    ] {
        for synchronize in [false, true] {
            let context = egui::Context::default();
            let mut fixture = Fixture::new(source);
            let initial = fixture.frame(&context, Default::default());
            let mut continuation = initial
                .interaction_locator()
                .begin_text_selection()
                .expect("geometry");
            let mut completed = false;
            for step in 0..5 {
                if synchronize {
                    fixture.synchronize();
                }
                let mut input = egui::RawInput::default();
                continuation
                    .apply_to_raw_input_once(&mut input)
                    .expect("apply");
                assert!(matches!(
                    continuation.apply_to_raw_input_once(&mut input),
                    Err(KucTextSelectionContinuationError::AlreadyApplied)
                ));
                let frame = fixture.frame(&context, input);
                match continuation.advance(frame.interaction_locator()) {
                    Ok(Some(next)) => continuation = next,
                    Ok(None) => {
                        assert_eq!(step, 4);
                        completed = true;
                        break;
                    }
                    Err(error) => panic!("step {step}, sync={synchronize}: {error:?}"),
                }
            }
            assert!(completed);
            assert_ne!(fixture.selection.get().0, fixture.selection.get().1);
            assert_eq!(fixture.changes.get(), 0);
        }
    }
}

#[test]
fn full_root_selection_continuation_rejects_a_skipped_frame() {
    let context = egui::Context::default();
    let mut fixture = Fixture::new("ordinary text\n\nsecond line");
    let initial = fixture.frame(&context, Default::default());
    let mut continuation = initial
        .interaction_locator()
        .begin_text_selection()
        .expect("geometry");
    let mut input = egui::RawInput::default();
    continuation
        .apply_to_raw_input_once(&mut input)
        .expect("apply");
    fixture.frame(&context, input);
    let later = fixture.frame(&context, Default::default());
    assert!(matches!(
        continuation.advance(later.interaction_locator()),
        Err(KucTextSelectionContinuationError::FrameDiscontinuity)
    ));
}
