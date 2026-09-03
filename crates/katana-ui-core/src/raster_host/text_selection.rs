use super::layout_metrics::LayoutRect;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_selection::{
    UiTextGlyphBox, UiTextLineBox, UiTextSelectionModel, UiTextSelectionRange,
};
use unicode_segmentation::UnicodeSegmentation;

pub use super::text_selection_types::SelectableTextRun;
pub(in crate::raster_host) use super::text_selection_types::TextSelection;

impl SelectableTextRun {
    #[must_use]
    pub(super) fn new(text: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        let grapheme_count = text.graphemes(true).count().max(1);
        let glyph_width = (width.max(1) / grapheme_count).max(1) as u32;
        let model = UiTextSelectionModel::from_monospace_text(
            text,
            x as i32,
            y as i32,
            glyph_width,
            height.max(1) as u32,
        );
        Self {
            text: text.to_string(),
            rect: LayoutRect::new(x, y, width, height),
            model,
        }
    }

    #[must_use]
    pub(super) fn with_glyph_widths(
        text: &str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        glyph_widths: &[usize],
    ) -> Self {
        let mut glyphs = Vec::new();
        let mut byte_offset = 0usize;
        let mut glyph_x = x as i32;
        for (index, grapheme) in text.graphemes(true).enumerate() {
            let byte_start = byte_offset;
            byte_offset += grapheme.len();
            let glyph_width = glyph_widths.get(index).copied().unwrap_or(1).max(1);
            glyphs.push(
                UiTextGlyphBox::new(
                    index,
                    byte_start..byte_offset,
                    UiRect::new(glyph_x, y as i32, glyph_width as u32, height.max(1) as u32),
                    y as i32 + height.max(1) as i32,
                )
                .with_text(grapheme),
            );
            glyph_x += glyph_width as i32;
        }
        let model =
            UiTextSelectionModel::new(text, vec![UiTextLineBox::new(0..text.len(), glyphs)]);
        Self {
            text: text.to_string(),
            rect: LayoutRect::new(x, y, width, height),
            model,
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub(in crate::raster_host) fn rect(&self) -> LayoutRect {
        self.rect
    }

    #[must_use]
    pub fn x(&self) -> usize {
        self.rect.x
    }

    #[must_use]
    pub fn y(&self) -> usize {
        self.rect.y
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.rect.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.rect.height
    }

    #[must_use]
    pub fn right(&self) -> usize {
        self.rect.right()
    }

    #[must_use]
    pub fn bottom(&self) -> usize {
        self.rect.bottom()
    }

    #[must_use]
    pub(in crate::raster_host) fn model(&self) -> &UiTextSelectionModel {
        &self.model
    }
}

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
    runs.iter()
        .filter_map(|run| selected_text_for_run(run, selection))
        .collect::<Vec<_>>()
        .join("\n")
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
    let selection = run.model.drag_range(
        (start.0 as i32, start.1 as i32),
        (end.0 as i32, end.1 as i32),
    );
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
    fn selectable_run_width_and_reverse_drag_order_are_observable() {
        let run = SelectableTextRun::new("abc", 10, 20, 30, 12);
        assert_eq!(30, run.width());
        assert_eq!(
            ((10, 20), (40, 50)),
            ordered_selection_points(TextSelection::drag((40, 50), (10, 20)))
        );
    }
}
