use super::canvas_model::Canvas;

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
        self.text_runs
            .push(super::text_selection::SelectableTextRun::new(
                text, x, y, width, height,
            ));
    }

    pub(crate) fn record_text_run_with_glyph_widths(
        &mut self,
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_widths: &[usize],
    ) {
        if text.is_empty() || width == 0 || height == 0 {
            return;
        }
        self.text_runs
            .push(super::text_selection::SelectableTextRun::with_glyph_widths(
                text,
                x,
                y,
                width,
                height,
                glyph_widths,
            ));
    }
}
