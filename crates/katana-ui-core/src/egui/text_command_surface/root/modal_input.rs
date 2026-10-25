use super::super::types::EguiTextCommandSurfaceError;
use super::{EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootError};

pub(super) fn suspend(
    root: &mut EguiTextCommandSurfaceRoot,
) -> Result<(), EguiTextCommandSurfaceRootError> {
    root.surface.text.suspend_pointer_interaction();
    if let Some(tab_strip) = root.tab_strip.as_mut() {
        tab_strip
            .suspend_interaction()
            .map_err(EguiTextCommandSurfaceError::from)?;
    }
    Ok(())
}
