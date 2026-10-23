use super::canvas_model::Canvas;
use super::canvas_text_selection::next_text_selection_line_id;
use super::text_selection::SelectableTextRun;

pub(in crate::raster_host) fn resolve_text_selection_line_id(
    logical_line: &mut Option<u64>,
    separator_is_explicit: bool,
) -> u64 {
    if separator_is_explicit {
        *logical_line = Some(next_text_selection_line_id());
    }
    *logical_line.get_or_insert_with(next_text_selection_line_id)
}

impl Canvas {
    #[cfg(test)]
    pub(in crate::raster_host) fn with_text_selection_line<T>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.with_text_selection_line_separator("\n", false, operation)
    }

    #[cfg(test)]
    pub(in crate::raster_host) fn with_text_selection_line_separator<T>(
        &mut self,
        separator_before: &str,
        separator_is_explicit: bool,
        operation: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.with_text_selection_line_id(
            next_text_selection_line_id(),
            separator_before,
            separator_is_explicit,
            operation,
        )
    }

    pub(in crate::raster_host) fn with_text_selection_line_id<T>(
        &mut self,
        logical_line: u64,
        separator_before: &str,
        separator_is_explicit: bool,
        operation: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let previous =
            self.begin_text_selection_line(logical_line, separator_before, separator_is_explicit);
        let result = operation(self);
        self.finish_text_selection_line(previous);
        result
    }

    fn begin_text_selection_line(
        &mut self,
        logical_line: u64,
        separator_before: &str,
        separator_is_explicit: bool,
    ) -> (Option<u64>, Option<u64>, String, bool, usize) {
        let previous = self.active_text_selection_line;
        let previous_visual_line = self.active_text_selection_visual_line;
        let previous_separator = std::mem::take(&mut self.active_text_selection_separator);
        let previous_is_auto = self.active_text_selection_separator_is_auto;
        self.active_text_selection_line = Some(logical_line);
        self.active_text_selection_visual_line = Some(next_text_selection_line_id());
        if separator_is_explicit && self.pending_text_selection_separator_is_auto {
            self.pending_text_selection_separator.clear();
        }
        self.active_text_selection_separator =
            std::mem::take(&mut self.pending_text_selection_separator);
        self.active_text_selection_separator
            .push_str(separator_before);
        self.active_text_selection_separator_is_auto = !separator_is_explicit;
        (
            previous,
            previous_visual_line,
            previous_separator,
            previous_is_auto,
            self.text_runs.len(),
        )
    }

    fn finish_text_selection_line(
        &mut self,
        previous: (Option<u64>, Option<u64>, String, bool, usize),
    ) {
        let (previous_line, previous_visual_line, previous_separator, previous_is_auto, run_start) =
            previous;
        self.clear_consumed_text_selection_separator(run_start);
        self.pending_text_selection_separator =
            std::mem::take(&mut self.active_text_selection_separator);
        self.pending_text_selection_separator_is_auto =
            self.active_text_selection_separator_is_auto;
        self.active_text_selection_line = previous_line;
        self.active_text_selection_visual_line = previous_visual_line;
        self.active_text_selection_separator = previous_separator;
        self.active_text_selection_separator_is_auto = previous_is_auto;
    }

    fn clear_consumed_text_selection_separator(&mut self, run_start: usize) {
        if self.text_runs.len() <= run_start {
            return;
        }
        self.active_text_selection_separator.clear();
        self.active_text_selection_separator_is_auto = false;
    }

    #[cfg(test)]
    pub(in crate::raster_host) fn record_text_run_in_logical_line(
        &mut self,
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        logical_line: Option<u64>,
    ) {
        self.record_text_run_in_logical_line_with_separator(
            text,
            x,
            y,
            width,
            height,
            logical_line,
            "",
        );
    }

    pub(in crate::raster_host) fn record_text_run_in_logical_line_with_separator(
        &mut self,
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        logical_line: Option<u64>,
        separator_before: &str,
    ) {
        if text.is_empty() || width == 0 || height == 0 {
            return;
        }
        self.text_runs
            .push(SelectableTextRun::new_in_logical_line_with_separator(
                text,
                x,
                y,
                width,
                height,
                logical_line,
                separator_before,
            ));
    }

    pub(in crate::raster_host) fn record_empty_text_selection_line(
        &mut self,
        x: usize,
        y: usize,
        height: usize,
    ) {
        let run = SelectableTextRun::new_in_logical_line(
            "",
            x,
            y,
            1,
            height.max(1),
            self.active_text_selection_line,
        );
        let run = self.with_active_text_selection_separator(run);
        if self
            .clip
            .is_none_or(|clip| self.text_run_fits_clip(&run, clip))
        {
            self.text_runs.push(run);
        }
    }
}
