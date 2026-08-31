use crate::text_command_surface::{EguiTextCommandSurfaceError, TextCommandSurfaceStyle};

pub(super) fn default_style() -> Result<TextCommandSurfaceStyle, EguiTextCommandSurfaceError> {
    TextCommandSurfaceStyle::standard()
}
