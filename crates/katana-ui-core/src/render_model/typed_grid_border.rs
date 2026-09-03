use serde::{Deserialize, Serialize};

const THICK_STROKE_WIDTH_PX: usize = 3;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiGridBorderLineStyle {
    #[default]
    None,
    Hair,
    Thin,
    Medium,
    Thick,
    Double,
    Dotted,
    Dashed,
    DashDot,
    DashDotDot,
    MediumDashed,
    MediumDashDot,
    MediumDashDotDot,
    SlantDashDot,
    Solid,
}

impl UiGridBorderLineStyle {
    #[must_use]
    pub const fn is_visible(self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    pub const fn stroke_width_px(self) -> usize {
        match self {
            Self::None => 0,
            Self::Hair
            | Self::Thin
            | Self::Dotted
            | Self::Dashed
            | Self::DashDot
            | Self::DashDotDot
            | Self::Solid => 1,
            Self::Medium
            | Self::MediumDashed
            | Self::MediumDashDot
            | Self::MediumDashDotDot
            | Self::SlantDashDot => 2,
            Self::Thick | Self::Double => THICK_STROKE_WIDTH_PX,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridBorderSide {
    pub line_style: UiGridBorderLineStyle,
    pub color: Option<String>,
}

impl UiGridBorderSide {
    #[must_use]
    pub fn solid(color: impl Into<String>) -> Self {
        Self {
            line_style: UiGridBorderLineStyle::Solid,
            color: Some(color.into()),
        }
    }

    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.line_style.is_visible()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridCellBorders {
    #[serde(default)]
    pub left: UiGridBorderSide,
    #[serde(default)]
    pub right: UiGridBorderSide,
    #[serde(default)]
    pub top: UiGridBorderSide,
    #[serde(default)]
    pub bottom: UiGridBorderSide,
}

#[cfg(test)]
mod tests {
    use super::{
        THICK_STROKE_WIDTH_PX, UiGridBorderLineStyle, UiGridBorderSide, UiGridCellBorders,
    };

    #[test]
    fn border_sides_keep_their_visibility_and_stroke_width_contract() {
        assert!(!UiGridBorderSide::default().is_visible());
        assert_eq!(0, UiGridBorderLineStyle::None.stroke_width_px());
        assert!(UiGridBorderSide::solid("#B7C4CE").is_visible());
        assert_eq!(
            THICK_STROKE_WIDTH_PX,
            UiGridBorderLineStyle::Double.stroke_width_px()
        );
        assert_eq!(UiGridCellBorders::default(), UiGridCellBorders::default());
    }
}
