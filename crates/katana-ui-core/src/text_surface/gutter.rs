pub(super) use super::gutter_types::{
    TextSurfaceAutomaticGutterOverride, TextSurfaceAutomaticGutterPresentation,
    TextSurfaceAutomaticGutterRangeOverride, TextSurfaceGutter, TextSurfaceGutterRangeStartAnchor,
    TextSurfaceGutterRow, TextSurfaceGutterRowId,
};

impl TextSurfaceAutomaticGutterOverride {
    #[must_use]
    pub fn new() -> Self {
        Self {
            marker_id: None,
            accessibility_label: String::new(),
            accessibility_description: None,
            visual_role: String::new(),
        }
    }

    #[must_use]
    pub fn marker_id(mut self, value: impl Into<String>) -> Self {
        self.marker_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn accessibility_description(mut self, value: impl Into<String>) -> Self {
        self.accessibility_description = Some(value.into());
        self
    }

    #[must_use]
    pub fn visual_role(mut self, value: impl Into<String>) -> Self {
        self.visual_role = value.into();
        self
    }
}

impl Default for TextSurfaceAutomaticGutterOverride {
    fn default() -> Self {
        Self::new()
    }
}

impl TextSurfaceAutomaticGutterPresentation {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            overrides: Vec::new(),
            range_overrides: Vec::new(),
            hovered_rows: Vec::new(),
        }
    }

    #[must_use]
    pub fn override_range(mut self, value: TextSurfaceAutomaticGutterRangeOverride) -> Self {
        self.range_overrides.push(value);
        self
    }

    #[must_use]
    pub fn override_row(
        mut self,
        row_id: TextSurfaceGutterRowId,
        value: TextSurfaceAutomaticGutterOverride,
    ) -> Self {
        self.overrides.retain(|(current, _)| current != &row_id);
        self.overrides.push((row_id, value));
        self
    }

    pub(crate) fn override_for(
        &self,
        logical_row: usize,
    ) -> Option<&TextSurfaceAutomaticGutterOverride> {
        let row_id = TextSurfaceGutterRowId::for_logical_row(logical_row);
        self.overrides
            .iter()
            .find(|(current, _)| current == &row_id)
            .map(|(_, value)| value)
    }

    fn range_override_for(
        &self,
        layout: &super::layout_model::TextSurfaceLayout,
        logical_row: usize,
    ) -> Option<&TextSurfaceAutomaticGutterRangeOverride> {
        self.range_overrides
            .iter()
            .enumerate()
            .filter(|(_, value)| range_start_row(layout, value) == Some(logical_row))
            .max_by(|(left_index, left), (right_index, right)| {
                left.priority
                    .cmp(&right.priority)
                    .then_with(|| right_index.cmp(left_index))
            })
            .map(|(_, value)| value)
    }
}

fn range_start_row(
    layout: &super::layout_model::TextSurfaceLayout,
    value: &TextSurfaceAutomaticGutterRangeOverride,
) -> Option<usize> {
    if value.byte_start > value.byte_end
        || value.byte_end > layout.text().len()
        || !layout.text().is_char_boundary(value.byte_start)
        || !layout.text().is_char_boundary(value.byte_end)
    {
        return None;
    }
    let start = layout.text()[..value.byte_start]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count();
    let row = if matches!(
        value.start_anchor,
        TextSurfaceGutterRangeStartAnchor::FollowingLine
    ) && layout.text().as_bytes().get(value.byte_start) == Some(&b'\n')
    {
        start.saturating_add(1)
    } else {
        start
    };
    layout
        .lines
        .iter()
        .any(|line| line.logical_row == row)
        .then_some(row)
}

impl Default for TextSurfaceAutomaticGutterPresentation {
    fn default() -> Self {
        Self::new()
    }
}

impl TextSurfaceGutterRow {
    #[must_use]
    pub fn new(logical_row: usize, display_label: impl Into<String>) -> Self {
        Self {
            logical_row,
            display_label: display_label.into(),
            marker_id: None,
            accessibility_label: String::new(),
            accessibility_description: None,
            visual_role: String::new(),
            icon: None,
        }
    }

    #[must_use]
    pub fn marker_id(mut self, value: impl Into<String>) -> Self {
        self.marker_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn accessibility_description(mut self, value: impl Into<String>) -> Self {
        self.accessibility_description = Some(value.into());
        self
    }

    #[must_use]
    pub fn visual_role(mut self, value: impl Into<String>) -> Self {
        self.visual_role = value.into();
        self
    }
}

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

    pub(crate) fn layout_derived_width(
        &self,
        layout: &super::layout_model::TextSurfaceLayout,
    ) -> u32 {
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
    pub(crate) fn resolved_rows(
        &self,
        layout: &super::layout_model::TextSurfaceLayout,
    ) -> Vec<TextSurfaceGutterRow> {
        if !self.automatic_numbered {
            return self.rows.clone();
        }
        layout
            .lines
            .iter()
            .map(|line| {
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
                row
            })
            .collect()
    }
}
