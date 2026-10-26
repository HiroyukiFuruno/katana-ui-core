use super::EguiTextCommandSurfaceOutput;
use super::types::{
    KucInteractionLocator, KucInteractionRequestError, KucOpaqueInteractionRequest,
    KucOpaqueSearchTraceContinuation, KucOpaqueTextSelectionContinuation,
    KucSearchTraceContinuationError, KucTextSelectionContinuationError, SearchTracePhase,
    TextSelectionGeometry, TextSelectionPhase,
};

const SELECTION_MIN_WIDTH_PX: i32 = 24;
const SELECTION_EDGE_INSET_PX: f32 = 8.0;
const SELECTION_START_FRACTION: f32 = 0.25;

impl KucOpaqueTextSelectionContinuation {
    /// Applies this KUC-issued pointer step exactly once to the next raw input frame.
    pub fn apply_to_raw_input_once(
        &mut self,
        input: &mut egui::RawInput,
    ) -> Result<(), KucTextSelectionContinuationError> {
        if self.applied {
            return Err(KucTextSelectionContinuationError::AlreadyApplied);
        }
        match self.phase {
            TextSelectionPhase::Aim => {
                input
                    .events
                    .push(egui::Event::PointerMoved(self.geometry.start));
            }
            TextSelectionPhase::Press => {
                input.events.push(pointer_button(self.geometry.start, true));
            }
            TextSelectionPhase::MoveToMidpoint => {
                input
                    .events
                    .push(egui::Event::PointerMoved(self.geometry.midpoint));
            }
            TextSelectionPhase::MoveToEnd => {
                input
                    .events
                    .push(egui::Event::PointerMoved(self.geometry.end));
            }
            TextSelectionPhase::Release => {
                input.events.push(pointer_button(self.geometry.end, false));
            }
        }
        self.applied = true;
        Ok(())
    }

    /// Consumes this step against the immediately following KUC root frame.
    pub fn advance(
        self,
        current: &KucInteractionLocator,
    ) -> Result<Option<Self>, KucTextSelectionContinuationError> {
        if !self.applied {
            return Err(KucTextSelectionContinuationError::NotApplied);
        }
        if current.root_identity != self.root_identity {
            return Err(KucTextSelectionContinuationError::RootMismatch);
        }
        if current.frame_serial != self.frame_serial.saturating_add(1) {
            return Err(KucTextSelectionContinuationError::FrameDiscontinuity);
        }
        let phase = match self.phase {
            TextSelectionPhase::Aim => TextSelectionPhase::Press,
            TextSelectionPhase::Press => TextSelectionPhase::MoveToMidpoint,
            TextSelectionPhase::MoveToMidpoint => TextSelectionPhase::MoveToEnd,
            TextSelectionPhase::MoveToEnd => TextSelectionPhase::Release,
            TextSelectionPhase::Release => {
                if self.requires_floating_output && !current.selection_established {
                    return Err(KucTextSelectionContinuationError::SelectionNotEstablished);
                }
                if self.requires_floating_output && !current.floating_visible {
                    return Err(KucTextSelectionContinuationError::FloatingNotVisible);
                }
                return Ok(None);
            }
        };
        Ok(Some(Self {
            root_identity: self.root_identity,
            frame_serial: current.frame_serial,
            geometry: self.geometry,
            phase,
            requires_floating_output: self.requires_floating_output,
            applied: false,
        }))
    }
}

impl KucOpaqueSearchTraceContinuation {
    /// Applies the next KUC-owned search trace step exactly once.
    pub fn apply_to_raw_input_once(
        &mut self,
        input: &mut egui::RawInput,
    ) -> Result<(), KucSearchTraceContinuationError> {
        if self.applied {
            return Err(KucSearchTraceContinuationError::AlreadyApplied);
        }
        match &mut self.phase {
            SearchTracePhase::Focus(request)
            | SearchTracePhase::Next(request)
            | SearchTracePhase::Previous(request)
            | SearchTracePhase::Close(request) => request
                .apply_to_raw_input_once(input)
                .map_err(KucSearchTraceContinuationError::Request)?,
            SearchTracePhase::Preedit => {
                input.events.push(egui::Event::Ime(egui::ImeEvent::Preedit {
                    text: String::from("かな"),
                    active_range_chars: None,
                }))
            }
            SearchTracePhase::Commit => {
                input
                    .events
                    .push(egui::Event::Ime(egui::ImeEvent::Commit(String::from(
                        "入力 ⭐️",
                    ))))
            }
            SearchTracePhase::VerifyClosed => {}
        }
        self.applied = true;
        Ok(())
    }

    /// Advances this trace from the immediately following KUC root frame.
    pub fn advance(
        self,
        current: &KucInteractionLocator,
    ) -> Result<Option<Self>, KucSearchTraceContinuationError> {
        if !self.applied {
            return Err(KucSearchTraceContinuationError::NotApplied);
        }
        if current.root_identity != self.root_identity {
            return Err(KucSearchTraceContinuationError::RootMismatch);
        }
        if current.frame_serial != self.frame_serial.saturating_add(1) {
            return Err(KucSearchTraceContinuationError::FrameDiscontinuity);
        }
        let phase = match self.phase {
            SearchTracePhase::Focus(_) => {
                if !current.search_query_focused {
                    return Err(KucSearchTraceContinuationError::FocusNotEstablished);
                }
                SearchTracePhase::Preedit
            }
            SearchTracePhase::Preedit => SearchTracePhase::Commit,
            SearchTracePhase::Commit => {
                SearchTracePhase::Next(current.search_control_request("next")?)
            }
            SearchTracePhase::Next(_) => {
                SearchTracePhase::Previous(current.search_control_request("previous")?)
            }
            SearchTracePhase::Previous(_) => {
                SearchTracePhase::Close(current.search_control_request("close")?)
            }
            SearchTracePhase::Close(_) => SearchTracePhase::VerifyClosed,
            SearchTracePhase::VerifyClosed => {
                if current.search_visible {
                    return Err(KucSearchTraceContinuationError::CloseNotApplied);
                }
                return Ok(None);
            }
        };
        Ok(Some(Self {
            root_identity: self.root_identity,
            frame_serial: current.frame_serial,
            phase,
            applied: false,
        }))
    }
}

impl TextSelectionGeometry {
    pub(super) fn from_output(output: &EguiTextCommandSurfaceOutput) -> Option<Self> {
        let content = output.text.record.frame.content_bounds;
        let viewport = output.text.record.frame.viewport_bounds;
        let left = content.x.max(viewport.x);
        let top = content.y.max(viewport.y);
        let right = content
            .x
            .saturating_add_unsigned(content.width)
            .min(viewport.x.saturating_add_unsigned(viewport.width));
        let bottom = content
            .y
            .saturating_add_unsigned(content.height)
            .min(viewport.y.saturating_add_unsigned(viewport.height));
        let width = right.saturating_sub(left);
        let height = bottom.saturating_sub(top);
        if width <= 0 || height <= 0 || viewport.width <= SELECTION_MIN_WIDTH_PX as u32 {
            return None;
        }
        let row = output.text.raster.grapheme_bounds.iter().find(|glyph| {
            let y = content.y as f32 + glyph.y;
            y >= top as f32
                && y + glyph.height <= bottom as f32
                && output
                    .text
                    .raster
                    .text
                    .get(glyph.byte_start..glyph.byte_end)
                    .is_some_and(|text| {
                        !text
                            .chars()
                            .all(|character| matches!(character, '\n' | '\r'))
                    })
        })?;
        let y = content.y as f32 + row.y + row.height / 2.0;
        let start_x =
            (content.x as f32 + row.x + row.width * SELECTION_START_FRACTION).max(left as f32);
        let end_x = (right as f32 - SELECTION_EDGE_INSET_PX)
            .max(start_x + SELECTION_MIN_WIDTH_PX as f32)
            .min(viewport.x.saturating_add_unsigned(viewport.width) as f32 - 1.0);
        let start = egui::pos2(start_x, y);
        let end = egui::pos2(end_x, y);
        Some(Self {
            start,
            midpoint: egui::pos2((start.x + end.x) / 2.0, y),
            end,
        })
    }
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

impl KucOpaqueInteractionRequest {
    pub fn apply_to_raw_input_once(
        &mut self,
        input: &mut egui::RawInput,
    ) -> Result<(), KucInteractionRequestError> {
        if self.queued {
            return Err(KucInteractionRequestError::AlreadyQueued);
        }
        input.events.append(&mut self.events);
        self.queued = true;
        Ok(())
    }
}
