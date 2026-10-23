use super::layout_metrics::LayoutRect;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_selection::UiTextSelectionRange;
use unicode_segmentation::UnicodeSegmentation;

pub use super::text_selection_types::SelectableTextRun;
pub(in crate::raster_host) use super::text_selection_types::TextSelection;

impl TextSelection {
    #[must_use]
    pub(in crate::raster_host) const fn drag(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub(in crate::raster_host) const fn start(self) -> (usize, usize) {
        self.start
    }

    pub(in crate::raster_host) const fn end(self) -> (usize, usize) {
        self.end
    }
}

#[must_use]
pub(in crate::raster_host) fn copy_payload_for_selection(
    runs: &[SelectableTextRun],
    selection: TextSelection,
) -> String {
    let mut payload = String::new();
    let mut previous_logical_line = None;
    let mut previous_visual_line = None;
    let mut has_selected_run = false;
    for run in runs {
        let Some(selected_text) = selected_text_for_run(run, selection) else {
            continue;
        };
        if has_selected_run {
            let separator = run.separator_before();
            if same_logical_text_line(previous_logical_line, run.logical_line()) {
                if run.visual_line.is_some()
                    && previous_visual_line != run.visual_line
                    && separator != "\n"
                {
                    payload.push_str(separator);
                }
            } else if separator.is_empty() {
                payload.push('\n');
            } else {
                payload.push_str(separator);
            }
        }
        payload.push_str(&selected_text);
        previous_logical_line = run.logical_line();
        previous_visual_line = run.visual_line;
        has_selected_run = true;
    }
    payload
}

fn same_logical_text_line(previous: Option<u64>, current: Option<u64>) -> bool {
    previous.is_some() && previous == current
}

#[must_use]
pub(in crate::raster_host) fn selected_text_run_rects(
    runs: &[SelectableTextRun],
    selection: TextSelection,
) -> Vec<LayoutRect> {
    runs.iter()
        .filter(|run| text_run_is_selected(run, selection))
        .flat_map(|run| selected_text_run_rects_for_run(run, selection))
        .collect()
}

fn text_run_is_selected(run: &SelectableTextRun, selection: TextSelection) -> bool {
    core_selection_for_run(run, selection).is_some()
}

fn selected_text_run_rects_for_run(
    run: &SelectableTextRun,
    selection: TextSelection,
) -> Vec<LayoutRect> {
    let Some(selection) = core_selection_for_run(run, selection) else {
        return Vec::new();
    };
    run.model()
        .highlight_rects(selection)
        .iter()
        .map(ui_rect_to_layout)
        .collect()
}

fn selected_text_for_run(run: &SelectableTextRun, selection: TextSelection) -> Option<String> {
    if run.text().is_empty() {
        let (anchor, focus) = ordered_selection_points(selection);
        let intersects = anchor != focus
            && focus.1 >= run.y()
            && anchor.1 <= run.bottom()
            && (anchor.1 < run.y() || anchor.0 <= run.x())
            && (focus.1 > run.bottom() || focus.0 >= run.x());
        return intersects.then(String::new);
    }
    if selected_text_run_rects_for_run(run, selection).is_empty() {
        return None;
    }
    let selected = run
        .model()
        .selected_text(core_selection_for_run(run, selection)?);
    (!selected.is_empty()).then_some(selected)
}

fn core_selection_for_run(
    run: &SelectableTextRun,
    selection: TextSelection,
) -> Option<UiTextSelectionRange> {
    let (anchor, focus) = ordered_selection_points(selection);
    if focus.1 < run.y() || anchor.1 > run.bottom() {
        return None;
    }
    let anchor_on_run = point_y_is_inside_run(anchor.1, run);
    let focus_on_run = point_y_is_inside_run(focus.1, run);
    let (start, end) = match (anchor_on_run, focus_on_run) {
        (true, true) => (anchor, focus),
        (true, false) => (anchor, (run.right(), anchor.1)),
        (false, true) => ((run.x(), focus.1), focus),
        (false, false) => ((run.x(), run.y()), (run.right(), run.y())),
    };
    let selection = if start.0 <= run.x() && end.0 >= run.right() {
        UiTextSelectionRange::new(0, run.text().graphemes(true).count())
    } else {
        run.model.drag_range(
            (start.0 as i32, start.1 as i32),
            (end.0 as i32, end.1 as i32),
        )
    };
    (!selection.is_collapsed()).then_some(selection)
}

fn ordered_selection_points(selection: TextSelection) -> ((usize, usize), (usize, usize)) {
    let start = selection.start();
    let end = selection.end();
    if start.1 < end.1 || (start.1 == end.1 && start.0 <= end.0) {
        (start, end)
    } else {
        (end, start)
    }
}

fn point_y_is_inside_run(y: usize, run: &SelectableTextRun) -> bool {
    y >= run.y() && y <= run.bottom()
}

fn ui_rect_to_layout(rect: &UiRect) -> LayoutRect {
    LayoutRect::new(
        rect.x.max(0) as usize,
        rect.y.max(0) as usize,
        rect.width as usize,
        rect.height as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        SelectableTextRun, TextSelection, copy_payload_for_selection, ordered_selection_points,
        selected_text_run_rects,
    };

    #[test]
    fn copy_payload_includes_all_text_runs_intersecting_the_selection_box() {
        let runs = [
            SelectableTextRun::new("outside", 0, 0, 40, 12),
            SelectableTextRun::new("Markdown Linter", 100, 100, 120, 20),
            SelectableTextRun::new("checked=true", 100, 126, 96, 20),
        ];

        assert_eq!(
            "Markdown Linter\nchecked=true",
            copy_payload_for_selection(&runs, TextSelection::drag((100, 100), (240, 150)))
        );
    }

    #[test]
    fn copy_payload_full_window_selection_includes_visible_shell_runs() {
        let runs = [
            SelectableTextRun::new("Files", 8, 12, 40, 14),
            SelectableTextRun::new("KDV settings", 8, 400, 90, 14),
            SelectableTextRun::new("KatanA Rendering", 460, 160, 220, 26),
            SelectableTextRun::new("command=none", 460, 860, 110, 14),
        ];

        assert_eq!(
            "Files\nKDV settings\nKatanA Rendering\ncommand=none",
            copy_payload_for_selection(&runs, TextSelection::drag((0, 0), (1000, 900)))
        );
    }

    #[test]
    fn selected_text_run_rects_clip_highlight_to_drag_bounds() {
        let runs = [
            SelectableTextRun::new("Heading", 100, 100, 120, 20),
            SelectableTextRun::new("Body", 100, 130, 120, 20),
        ];

        assert_eq!(
            vec![
                super::LayoutRect::new(117, 100, 17, 20),
                super::LayoutRect::new(134, 100, 17, 20),
            ],
            selected_text_run_rects(&runs, TextSelection::drag((112, 104), (142, 112)))
        );
    }

    #[test]
    fn copy_payload_clips_text_run_to_horizontal_drag_bounds() {
        let runs = [SelectableTextRun::new("Markdown Linter", 100, 100, 150, 20)];

        assert_eq!(
            "Markdown",
            copy_payload_for_selection(&runs, TextSelection::drag((100, 100), (180, 120)))
        );
    }

    #[test]
    fn multiline_selection_uses_text_flow_not_repeated_horizontal_slice() {
        let runs = [
            SelectableTextRun::with_glyph_widths("abc", 100, 100, 30, 20, &[10, 10, 10]),
            SelectableTextRun::with_glyph_widths("defgh", 100, 130, 50, 20, &[10, 10, 10, 10, 10]),
            SelectableTextRun::with_glyph_widths("ijk", 100, 160, 30, 20, &[10, 10, 10]),
        ];

        let selection = TextSelection::drag((105, 110), (125, 165));

        assert_eq!(
            "bc\ndefgh\nijk",
            copy_payload_for_selection(&runs, selection)
        );
        assert_eq!(
            vec![
                super::LayoutRect::new(110, 100, 10, 20),
                super::LayoutRect::new(120, 100, 10, 20),
                super::LayoutRect::new(100, 130, 10, 20),
                super::LayoutRect::new(110, 130, 10, 20),
                super::LayoutRect::new(120, 130, 10, 20),
                super::LayoutRect::new(130, 130, 10, 20),
                super::LayoutRect::new(140, 130, 10, 20),
                super::LayoutRect::new(100, 160, 10, 20),
                super::LayoutRect::new(110, 160, 10, 20),
                super::LayoutRect::new(120, 160, 10, 20),
            ],
            selected_text_run_rects(&runs, selection)
        );
    }

    #[test]
    fn copy_payload_restores_explicit_separators_before_the_next_logical_line() {
        let mut first = SelectableTextRun::new_in_logical_line("alpha", 0, 0, 40, 12, Some(1));
        let mut second = SelectableTextRun::new_in_logical_line("beta", 0, 24, 32, 12, Some(2));
        first.separator_before.clear();
        second.separator_before = "\r\u{2029}".to_owned();

        assert_eq!(
            "alpha\r\u{2029}beta",
            copy_payload_for_selection(&[first, second], TextSelection::drag((0, 0), (40, 36)))
        );
    }

    #[test]
    fn copy_payload_emits_wrapped_source_separator_once_per_visual_line() {
        let mut first = SelectableTextRun::new_in_logical_line("alpha", 0, 0, 40, 12, Some(1));
        first.visual_line = Some(10);
        let mut second = SelectableTextRun::new_in_logical_line_with_separator(
            "beta",
            0,
            24,
            32,
            12,
            Some(1),
            " ",
        );
        second.visual_line = Some(11);
        let mut third = SelectableTextRun::new_in_logical_line_with_separator(
            " gamma",
            32,
            24,
            40,
            12,
            Some(1),
            " ",
        );
        third.visual_line = Some(11);

        assert_eq!(
            "alpha beta gamma",
            copy_payload_for_selection(
                &[first, second, third],
                TextSelection::drag((0, 0), (80, 36)),
            )
        );
    }

    #[test]
    fn selectable_run_width_and_reverse_drag_order_are_observable() {
        let run = SelectableTextRun::new("abc", 10, 20, 30, 12);
        assert_eq!(30, run.width());
        assert_eq!(
            ((10, 20), (40, 50)),
            ordered_selection_points(TextSelection::drag((40, 50), (10, 20)))
        );
    }

    #[test]
    fn one_grapheme_rtl_run_uses_its_resolved_direction() {
        let run = SelectableTextRun::with_glyph_positions("\u{05d0}", 10, 0, 10, 10, &[(0, 10)]);

        assert_eq!(1, run.model().point_to_caret(11, 5));
        assert_eq!(0, run.model().point_to_caret(19, 5));
    }

    #[test]
    fn rtl_glyph_positions_keep_visual_selection_geometry_and_logical_copy_text() {
        let run = SelectableTextRun::with_glyph_positions(
            "אבג",
            100,
            100,
            30,
            20,
            &[(20, 10), (10, 10), (0, 10)],
        );

        assert_eq!(
            "אבג",
            copy_payload_for_selection(
                std::slice::from_ref(&run),
                TextSelection::drag((100, 110), (130, 110))
            )
        );
        assert_eq!(
            vec![super::LayoutRect::new(100, 100, 10, 20)],
            selected_text_run_rects(&[run], TextSelection::drag((100, 110), (110, 110)))
        );
    }
}
