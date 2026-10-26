use super::{TextSurfaceAutomaticGutterPresentation, TextSurfaceGutter, TextSurfaceGutterRow};
use crate::text_surface::layout_model::TextSurfaceLayout;
use std::collections::BTreeSet;

impl TextSurfaceGutter {
    #[must_use]
    pub const fn new(width: u32) -> Self {
        Self {
            width,
            rows: Vec::new(),
            automatic_numbered: false,
            controlled_automatic: None,
        }
    }

    #[must_use]
    pub fn row(mut self, value: TextSurfaceGutterRow) -> Self {
        self.rows.push(value);
        self
    }

    /// Derives numbered rows and their geometry from the KUC text layout.
    ///
    /// `rows` remains a sparse override collection in this mode; callers must not enumerate
    /// every line or calculate a row position themselves.
    #[must_use]
    pub const fn automatic_numbered(mut self) -> Self {
        self.automatic_numbered = true;
        self
    }

    #[must_use]
    pub(crate) fn from_controlled_automatic(value: TextSurfaceAutomaticGutterPresentation) -> Self {
        Self {
            width: 0,
            rows: Vec::new(),
            automatic_numbered: true,
            controlled_automatic: Some(value),
        }
    }

    pub(crate) fn is_controlled_automatic(&self) -> bool {
        self.controlled_automatic.is_some()
    }

    pub(crate) fn layout_derived_width(&self, layout: &TextSurfaceLayout) -> u32 {
        if !self.is_controlled_automatic() {
            return self.width;
        }
        let label_digits = layout
            .lines
            .iter()
            .map(|line| line.logical_row.saturating_add(1).to_string().len() as u32)
            .max()
            .unwrap_or(1);
        let line_height = layout
            .lines
            .iter()
            .map(|line| line.bounds.height)
            .max()
            .unwrap_or(1);
        let digit_extent = (line_height / 2).max(1);
        digit_extent.saturating_mul(label_digits)
    }

    #[must_use]
    pub(crate) fn resolved_rows(&self, layout: &TextSurfaceLayout) -> Vec<TextSurfaceGutterRow> {
        if !self.automatic_numbered {
            return self.rows.clone();
        }
        let mut seen_logical_rows = BTreeSet::new();
        layout
            .lines
            .iter()
            .filter_map(|line| {
                if !seen_logical_rows.insert(line.logical_row) {
                    return None;
                }
                let mut row = self
                    .controlled_automatic
                    .as_ref()
                    .and_then(|value| value.range_override_for(layout, line.logical_row))
                    .map(|override_value| TextSurfaceGutterRow {
                        logical_row: line.logical_row,
                        display_label: String::new(),
                        marker_id: Some(override_value.marker_id.clone()),
                        accessibility_label: override_value.accessibility_label.clone(),
                        accessibility_description: override_value.accessibility_description.clone(),
                        visual_role: override_value.visual_role.clone(),
                        icon: override_value.icon.clone(),
                    })
                    .or_else(|| {
                        self.controlled_automatic
                            .as_ref()
                            .and_then(|value| value.override_for(line.logical_row))
                            .map(|override_value| TextSurfaceGutterRow {
                                logical_row: line.logical_row,
                                display_label: String::new(),
                                marker_id: override_value.marker_id.clone(),
                                accessibility_label: override_value.accessibility_label.clone(),
                                accessibility_description: override_value
                                    .accessibility_description
                                    .clone(),
                                visual_role: override_value.visual_role.clone(),
                                icon: None,
                            })
                    })
                    .or_else(|| {
                        self.rows
                            .iter()
                            .find(|row| row.logical_row == line.logical_row)
                            .cloned()
                    })
                    .unwrap_or_else(|| TextSurfaceGutterRow::new(line.logical_row, String::new()));
                row.display_label = line.logical_row.saturating_add(1).to_string();
                Some(row)
            })
            .collect()
    }
}
