use super::WindowControlKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlButtonGroupAction {
    Press(WindowControlKind),
    SetHover(bool),
    SetFullscreen(bool),
}
