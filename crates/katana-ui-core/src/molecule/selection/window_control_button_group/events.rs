use super::WindowControlKind;
use crate::window::{WindowCommand, WindowId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlButtonGroupEvent {
    ControlPressed { which: WindowControlKind },
    VisibilityChanged { visible: bool },
    FullscreenChanged { fullscreen: bool },
}

impl WindowControlButtonGroupEvent {
    #[must_use]
    pub fn window_command(self, window_id: WindowId) -> Option<WindowCommand> {
        match self {
            Self::ControlPressed { which } => Some(window_command_for(which, window_id)),
            Self::VisibilityChanged { .. } | Self::FullscreenChanged { .. } => None,
        }
    }
}

fn window_command_for(which: WindowControlKind, window_id: WindowId) -> WindowCommand {
    match which {
        WindowControlKind::Close => WindowCommand::Close { window_id },
        WindowControlKind::Minimize => WindowCommand::Minimize { window_id },
        WindowControlKind::Maximize => WindowCommand::Maximize { window_id },
        WindowControlKind::Restore => WindowCommand::Restore { window_id },
    }
}
