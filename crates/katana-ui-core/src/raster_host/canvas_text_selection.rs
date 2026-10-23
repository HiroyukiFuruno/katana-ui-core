use super::canvas_model::Canvas;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEXT_SELECTION_LINE: AtomicU64 = AtomicU64::new(0);

pub(in crate::raster_host) fn next_text_selection_line_id() -> u64 {
    NEXT_TEXT_SELECTION_LINE.fetch_add(1, Ordering::Relaxed)
}

impl Canvas {
    #[must_use]
    pub fn text_runs(&self) -> &[super::text_selection::SelectableTextRun] {
        &self.text_runs
    }

    #[must_use]
    pub fn copy_text_in_selection(
        &self,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
    ) -> Option<String> {
        let start = start?;
        let end = end?;
        let payload = super::text_selection::copy_payload_for_selection(
            &self.text_runs,
            super::text_selection::TextSelection::drag(start, end),
        );
        if payload.is_empty() {
            None
        } else {
            Some(payload)
        }
    }

    pub fn draw_text_selection_highlight(
        &mut self,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
        color: u32,
    ) -> bool {
        super::text_selection_overlay::draw_text_selection_highlight(self, start, end, color)
    }

    pub(crate) fn record_text_run(
        &mut self,
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        if text.is_empty() || width == 0 || height == 0 {
            return;
        }
        let run = match self.active_text_selection_line {
            Some(logical_line) => super::text_selection::SelectableTextRun::new_in_logical_line(
                text,
                x,
                y,
                width,
                height,
                Some(logical_line),
            ),
            None => super::text_selection::SelectableTextRun::new(text, x, y, width, height),
        };
        let run = self.with_active_text_selection_separator(run);
        self.text_runs.push(run);
    }

    #[cfg(test)]
    pub(crate) fn record_text_run_with_glyph_widths(
        &mut self,
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_widths: &[usize],
    ) {
        let mut glyph_x = 0usize;
        let glyph_positions = glyph_widths
            .iter()
            .map(|width| {
                let position = (glyph_x, (*width).max(1));
                glyph_x = glyph_x.saturating_add(position.1);
                position
            })
            .collect::<Vec<_>>();
        self.record_text_run_with_glyph_positions(text, x, y, width, height, &glyph_positions);
    }

    #[cfg(test)]
    pub(crate) fn record_text_run_with_glyph_positions(
        &mut self,
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_positions: &[(usize, usize)],
    ) {
        self.record_text_run_with_glyph_positions_with_bidi_direction(
            text,
            x,
            y,
            width,
            height,
            glyph_positions,
            None,
        );
    }

    pub(crate) fn record_text_run_with_glyph_positions_with_bidi_direction(
        &mut self,
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_positions: &[(usize, usize)],
        resolved_bidi_rtl: Option<bool>,
    ) {
        if text.is_empty() || width == 0 || height == 0 {
            return;
        }
        let run = match self.active_text_selection_line {
            Some(logical_line) => {
                super::text_selection::SelectableTextRun::with_glyph_positions_in_logical_line_with_bidi_direction(
                    text,
                    x,
                    y,
                    width,
                    height,
                    glyph_positions,
                    Some(logical_line),
                    resolved_bidi_rtl,
                )
            }
            None => super::text_selection::SelectableTextRun::with_glyph_positions_in_logical_line_with_bidi_direction(
                text,
                x,
                y,
                width,
                height,
                glyph_positions,
                None,
                resolved_bidi_rtl,
            ),
        };
        let run = self.with_active_text_selection_separator(run);
        self.text_runs.push(run);
    }

    pub(in crate::raster_host) fn with_active_text_selection_separator(
        &mut self,
        mut run: super::text_selection::SelectableTextRun,
    ) -> super::text_selection::SelectableTextRun {
        if self.active_text_selection_line.is_some() {
            run.visual_line = self.active_text_selection_visual_line;
            run.separator_before
                .clone_from(&self.active_text_selection_separator);
        }
        run
    }

    pub(crate) fn discard_text_runs_outside_current_clip_since(&mut self, start: usize) {
        let Some(clip) = self.clip else {
            return;
        };
        let start = start.min(self.text_runs.len());
        let pending = self.text_runs.split_off(start);
        let retained = pending
            .into_iter()
            .filter(|run| self.text_run_fits_clip(run, clip))
            .collect::<Vec<_>>();
        self.text_runs.extend(retained);
    }

    pub(in crate::raster_host) fn text_run_fits_clip(
        &self,
        run: &super::text_selection::SelectableTextRun,
        clip: super::canvas_clip::CanvasClip,
    ) -> bool {
        let rect = run.rect();
        let left = self.logical_to_physical_x(rect.x);
        let right = self.logical_to_physical_x(rect.right());
        let top = self.translate_physical_y(self.logical_to_physical_y(rect.y));
        let bottom = self.translate_physical_y(self.logical_to_physical_y(rect.bottom()));
        left >= clip.x && right <= clip.right() && top >= clip.y && bottom <= clip.bottom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_or_zero_sized_text_runs_are_ignored() {
        let mut canvas = Canvas::new(8, 8, 0);
        canvas.record_text_run("", 0, 0, 1, 1);
        canvas.record_text_run("text", 0, 0, 0, 1);
        canvas.record_text_run_with_glyph_widths("", 0, 0, 1, 1, &[]);
        canvas.record_text_run_with_glyph_widths("text", 0, 0, 1, 0, &[1]);
        canvas.record_text_run_in_logical_line("", 0, 0, 1, 1, Some(1));
        canvas.record_text_run_in_logical_line("text", 0, 0, 0, 1, Some(1));
        assert!(canvas.text_runs().is_empty());
    }

    #[test]
    fn logical_line_ids_do_not_change_canvas_or_text_run_equality() {
        let mut first = Canvas::new(8, 8, 0);
        first.with_text_selection_line(|canvas| {
            canvas.record_text_run("text", 0, 0, 4, 1);
        });
        let mut second = Canvas::new(8, 8, 0);
        second.with_text_selection_line(|canvas| {
            canvas.record_text_run("text", 0, 0, 4, 1);
        });

        assert_eq!(first.text_runs(), second.text_runs());
        assert_eq!(first, second);
    }

    #[test]
    fn explicit_separator_replaces_pending_automatic_separator_and_records_empty_line() {
        let mut canvas = Canvas::new(16, 8, 0);
        canvas.with_text_selection_line(|_canvas| {});

        canvas.with_text_selection_line_separator("\r", true, |canvas| {
            canvas.record_text_run_in_logical_line_with_separator("body", 0, 0, 4, 1, Some(7), "");
            canvas.record_empty_text_selection_line(0, 2, 1);
        });

        assert_eq!(2, canvas.text_runs().len());
        assert_eq!("", canvas.text_runs()[0].separator_before());
        assert_eq!("\r", canvas.text_runs()[1].separator_before());
    }
}
