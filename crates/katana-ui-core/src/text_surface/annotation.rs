use crate::text_selection::UiTextSelectionRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceAnnotationStyle {
    Underline,
    Outline,
    Fill,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAnnotation {
    pub id: String,
    pub range: UiTextSelectionRange,
    pub visual_role: String,
    pub style: TextSurfaceAnnotationStyle,
    pub priority: i32,
    pub tooltip: String,
}

impl TextSurfaceAnnotation {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        range: UiTextSelectionRange,
        visual_role: impl Into<String>,
        style: TextSurfaceAnnotationStyle,
    ) -> Self {
        Self {
            id: id.into(),
            range,
            visual_role: visual_role.into(),
            style,
            priority: 0,
            tooltip: String::new(),
        }
    }

    #[must_use]
    pub const fn priority(mut self, value: i32) -> Self {
        self.priority = value;
        self
    }

    #[must_use]
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = value.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{TextSurfaceAnnotation, TextSurfaceAnnotationStyle};
    use crate::text_selection::UiTextSelectionRange;

    #[test]
    fn builders_preserve_priority_and_tooltip() {
        let annotation = TextSurfaceAnnotation::new(
            "diagnostic",
            UiTextSelectionRange::new(1, 2),
            "warning",
            TextSurfaceAnnotationStyle::Underline,
        )
        .priority(7)
        .tooltip("details");
        assert_eq!(annotation.priority, 7);
        assert_eq!(annotation.tooltip, "details");
    }
}
