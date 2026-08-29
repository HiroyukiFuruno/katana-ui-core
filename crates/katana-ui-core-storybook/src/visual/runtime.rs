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

impl From<eframe::Error> for StorybookVisualError {
    fn from(error: eframe::Error) -> Self {
        Self::Eframe(error)
    }
}

impl From<ModalWindowPlacementError> for StorybookVisualError {
    fn from(error: ModalWindowPlacementError) -> Self {
        Self::Placement(error)
    }
}
