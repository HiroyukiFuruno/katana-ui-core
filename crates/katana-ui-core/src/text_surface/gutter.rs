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

mod rows;
