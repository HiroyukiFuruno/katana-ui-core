use super::{UiBorder, UiCommonProps, UiCursor};

const CONTROL_HOVER_BORDER_WIDTH_PX: u16 = 1;
const CONTROL_HOVER_BORDER_RADIUS_PX: u16 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiInteractivePreset {
    cursor: UiCursor,
    hover_border: UiBorder,
}

impl UiInteractivePreset {
    #[must_use]
    pub fn control() -> Self {
        Self {
            cursor: UiCursor::Pointer,
            hover_border: UiBorder::solid(
                CONTROL_HOVER_BORDER_WIDTH_PX,
                CONTROL_HOVER_BORDER_RADIUS_PX,
                "control.hover.border",
            ),
        }
    }

    #[must_use]
    pub fn apply_to_common(&self, common: UiCommonProps) -> UiCommonProps {
        common
            .cursor(self.cursor)
            .hover_border(self.hover_border.clone())
    }

    #[must_use]
    pub fn apply_to_common_defaults(&self, common: UiCommonProps) -> UiCommonProps {
        let common = if common.cursor == UiCursor::Default {
            common.cursor(self.cursor)
        } else {
            common
        };
        if common.hover_border == UiBorder::default() {
            return common.hover_border(self.hover_border.clone());
        }
        common
    }
}
