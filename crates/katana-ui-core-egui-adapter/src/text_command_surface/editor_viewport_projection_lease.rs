use katana_ui_core::render_model::UiImageSurfaceProps;

const DEFAULT_SPLIT_RATIO_PERCENT: u8 = 50;
const MIN_SPLIT_RATIO_PERCENT: u8 = 10;
const MAX_SPLIT_RATIO_PERCENT: u8 = 90;

/// Consuming, non-wire projection for the generic editor preview viewport.
pub struct EditorViewportProjectionLease {
    pub(super) preview: UiImageSurfaceProps,
    pub(super) split_ratio_percent: u8,
}

impl EditorViewportProjectionLease {
    #[must_use]
    pub fn new(preview: UiImageSurfaceProps) -> Self {
        Self {
            preview,
            split_ratio_percent: DEFAULT_SPLIT_RATIO_PERCENT,
        }
    }

    pub fn with_split_ratio_percent(
        mut self,
        split_ratio_percent: u8,
    ) -> Result<Self, EditorViewportProjectionError> {
        if !(MIN_SPLIT_RATIO_PERCENT..=MAX_SPLIT_RATIO_PERCENT).contains(&split_ratio_percent) {
            return Err(EditorViewportProjectionError::InvalidSplitRatio);
        }
        self.split_ratio_percent = split_ratio_percent;
        Ok(self)
    }
}

impl std::fmt::Debug for EditorViewportProjectionLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EditorViewportProjectionLease(..)")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorViewportProjectionError {
    InvalidSplitRatio,
}

impl std::fmt::Display for EditorViewportProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("editor viewport split ratio must be between 10 and 90 percent")
    }
}

impl std::error::Error for EditorViewportProjectionError {}

pub(super) fn clamp_split_ratio_percent(value: i32) -> u8 {
    value.clamp(
        MIN_SPLIT_RATIO_PERCENT as i32,
        MAX_SPLIT_RATIO_PERCENT as i32,
    ) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preview() -> UiImageSurfaceProps {
        UiImageSurfaceProps::new("preview", 1, 1, vec![1, 2, 3, 255]).expect("preview")
    }

    #[test]
    fn lease_is_opaque_and_ratio_is_fail_closed() {
        let lease = EditorViewportProjectionLease::new(preview());
        assert_eq!(format!("{lease:?}"), "EditorViewportProjectionLease(..)");
        let error = EditorViewportProjectionLease::new(preview())
            .with_split_ratio_percent(9)
            .expect_err("ratio below ten percent must fail closed");
        assert_eq!(error, EditorViewportProjectionError::InvalidSplitRatio);
        assert_eq!(
            error.to_string(),
            "editor viewport split ratio must be between 10 and 90 percent"
        );
        assert!(
            EditorViewportProjectionLease::new(preview())
                .with_split_ratio_percent(90)
                .is_ok()
        );
    }

    #[test]
    fn retained_ratio_clamps_pointer_and_keyboard_updates() {
        assert_eq!(clamp_split_ratio_percent(-1), 10);
        assert_eq!(clamp_split_ratio_percent(44), 44);
        assert_eq!(clamp_split_ratio_percent(101), 90);
    }
}
