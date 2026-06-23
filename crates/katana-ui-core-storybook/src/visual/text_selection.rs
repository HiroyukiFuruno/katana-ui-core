use super::layout_metrics::LayoutRect;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_selection::{
    UiTextGlyphBox, UiTextLineBox, UiTextSelectionModel, UiTextSelectionRange,
};
use unicode_segmentation::UnicodeSegmentation;

pub use super::text_selection_types::SelectableTextRun;
pub(in crate::visual) use super::text_selection_types::TextSelection;

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
    pub(in crate::visual) fn rect(&self) -> LayoutRect {
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
    pub(in crate::visual) fn model(&self) -> &UiTextSelectionModel {
        &self.model
    }
}

impl TextSelection {
    #[must_use]
    pub(in crate::visual) const fn drag(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub(in crate::visual) const fn start(self) -> (usize, usize) {
        self.start
    }

    pub(in crate::visual) const fn end(self) -> (usize, usize) {
        self.end
    }
}

#[must_use]
pub(in crate::visual) fn copy_payload_for_selection(
    runs: &[SelectableTextRun],
    selection: TextSelection,
) -> String {
    runs.iter()
        .filter_map(|run| selected_text_for_run(run, selection))
        .collect::<Vec<_>>()
        .join("\n")
}

#[must_use]
pub(in crate::visual) fn selected_text_run_rects(
    runs: &[SelectableTextRun],
    selection: TextSelection,
) -> Vec<LayoutRect> {
    runs.iter()
        .filter(|run| text_run_is_selected(run, selection))
        .flat_map(|run| selected_text_run_rects_for_run(run, selection))
        .collect()
}

fn text_run_is_selected(run: &SelectableTextRun, selection: TextSelection) -> bool {
    selection_intersects_run(run, selection)
        && !core_selection_for_run(run, selection).is_collapsed()
}

fn selected_text_run_rects_for_run(
    run: &SelectableTextRun,
    selection: TextSelection,
) -> Vec<LayoutRect> {
    if !selection_intersects_run(run, selection) {
        return Vec::new();
    }
    run.model()
        .highlight_rects(core_selection_for_run(run, selection))
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
        .selected_text(core_selection_for_run(run, selection));
    (!selected.is_empty()).then_some(selected)
}

fn selection_intersects_run(run: &SelectableTextRun, selection: TextSelection) -> bool {
    let (start_x, start_y) = selection.start();
    let (end_x, end_y) = selection.end();
    let min_x = start_x.min(end_x);
    let max_x = start_x.max(end_x);
    let min_y = start_y.min(end_y);
    let max_y = start_y.max(end_y);
    max_x >= run.x() && min_x <= run.right() && max_y >= run.y() && min_y <= run.bottom()
}

fn core_selection_for_run(
    run: &SelectableTextRun,
    selection: TextSelection,
) -> UiTextSelectionRange {
    run.model.drag_range(
        (selection.start().0 as i32, selection.start().1 as i32),
        (selection.end().0 as i32, selection.end().1 as i32),
    )
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
        SelectableTextRun, TextSelection, copy_payload_for_selection, selected_text_run_rects,
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
}
