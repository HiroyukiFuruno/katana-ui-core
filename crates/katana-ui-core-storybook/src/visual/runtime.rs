use katana_ui_core::window::ModalWindowPlacementError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorybookRuntimeReport {
    pub state_reflected: bool,
    pub overlay_rendered: bool,
    pub modal_plan_same_display: bool,
    pub modal_plan_frontmost: bool,
}

impl StorybookRuntimeReport {
    #[must_use]
    pub fn summary(self) -> String {
        format!(
            "state_reflected={} overlay_rendered={} modal_plan_same_display={} modal_plan_frontmost={}",
            self.state_reflected,
            self.overlay_rendered,
            self.modal_plan_same_display,
            self.modal_plan_frontmost
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorybookWindowRun {
    pub frames: usize,
    pub modal_window_opened: bool,
    pub same_display: bool,
    pub frontmost: bool,
    pub state_reflected: bool,
    pub overlay_rendered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorybookKeyboardRuntimeReport {
    pub checkbox_focused: bool,
    pub checkbox_toggled: bool,
    pub modal_closed: bool,
    pub unavailable_clipboard_ignored: bool,
}

impl StorybookKeyboardRuntimeReport {
    #[must_use]
    pub const fn passed(self) -> bool {
        self.checkbox_focused
            && self.checkbox_toggled
            && self.modal_closed
            && self.unavailable_clipboard_ignored
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorybookMouseTraceRuntimeReport {
    pub pointer_values_formatted: bool,
    pub optional_index_formatted: bool,
    pub progress_segment_formatted: bool,
}

impl StorybookMouseTraceRuntimeReport {
    #[must_use]
    pub const fn passed(self) -> bool {
        self.pointer_values_formatted
            && self.optional_index_formatted
            && self.progress_segment_formatted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorybookDependencyRuntimeReport {
    pub missing_tab_group_close_ignored: bool,
    pub same_tab_group_move_ignored: bool,
    pub tab_group_removal_emitted: bool,
}

impl StorybookDependencyRuntimeReport {
    #[must_use]
    pub const fn passed(self) -> bool {
        self.missing_tab_group_close_ignored
            && self.same_tab_group_move_ignored
            && self.tab_group_removal_emitted
    }
}

impl StorybookWindowRun {
    #[must_use]
    pub fn summary(self) -> String {
        format!(
            "frames={} modal_window_opened={} same_display={} frontmost={} state_reflected={} overlay_rendered={}",
            self.frames,
            self.modal_window_opened,
            self.same_display,
            self.frontmost,
            self.state_reflected,
            self.overlay_rendered
        )
    }
}

#[derive(Debug)]
pub enum StorybookVisualError {
    Window(minifb::Error),
    Eframe(eframe::Error),
    Placement(ModalWindowPlacementError),
}

impl fmt::Display for StorybookVisualError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Window(error) => write!(formatter, "{error}"),
            Self::Eframe(error) => write!(formatter, "{error}"),
            Self::Placement(error) => write!(formatter, "{error:?}"),
        }
    }
}

impl std::error::Error for StorybookVisualError {}

impl From<minifb::Error> for StorybookVisualError {
    fn from(error: minifb::Error) -> Self {
        Self::Window(error)
    }
}

impl From<ModalWindowPlacementError> for StorybookVisualError {
    fn from(error: ModalWindowPlacementError) -> Self {
        Self::Placement(error)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ModalWindowPlacementError, StorybookDependencyRuntimeReport,
        StorybookKeyboardRuntimeReport, StorybookMouseTraceRuntimeReport, StorybookRuntimeReport,
        StorybookVisualError, StorybookWindowRun,
    };

    #[test]
    fn runtime_and_window_reports_include_every_field() {
        let runtime = StorybookRuntimeReport {
            state_reflected: true,
            overlay_rendered: false,
            modal_plan_same_display: true,
            modal_plan_frontmost: false,
        };
        assert!(runtime.summary().contains("state_reflected=true"));

        let window = StorybookWindowRun {
            frames: 3,
            modal_window_opened: true,
            same_display: true,
            frontmost: false,
            state_reflected: true,
            overlay_rendered: false,
        };
        assert!(window.summary().contains("frames=3"));

        assert!(
            StorybookKeyboardRuntimeReport {
                checkbox_focused: true,
                checkbox_toggled: true,
                modal_closed: true,
                unavailable_clipboard_ignored: true,
            }
            .passed()
        );
        assert!(
            StorybookMouseTraceRuntimeReport {
                pointer_values_formatted: true,
                optional_index_formatted: true,
                progress_segment_formatted: true,
            }
            .passed()
        );
        assert!(
            StorybookDependencyRuntimeReport {
                missing_tab_group_close_ignored: true,
                same_tab_group_move_ignored: true,
                tab_group_removal_emitted: true,
            }
            .passed()
        );
    }

    #[test]
    fn visual_error_formats_window_and_placement_sources() {
        let window = StorybookVisualError::from(minifb::Error::WindowCreate("test".to_string()));
        assert_eq!("Failed to create window", window.to_string());

        let placement = StorybookVisualError::from(ModalWindowPlacementError::ParentOutsideDisplay);
        assert_eq!("ParentOutsideDisplay", placement.to_string());

        let eframe = StorybookVisualError::Eframe(eframe::Error::AppCreation(Box::new(
            std::io::Error::other("eframe failed"),
        )));
        assert!(eframe.to_string().contains("eframe failed"));
    }
}
