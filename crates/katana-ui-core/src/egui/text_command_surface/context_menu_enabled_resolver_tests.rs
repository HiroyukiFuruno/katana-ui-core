use super::KucContextMenuEnabledResolverError;
use crate::egui::text_command_surface::{
    EguiTextCommandSurfaceError, EguiTextCommandSurfaceRootError,
    EguiTextCommandSurfaceRootFactoryError,
};

#[test]
fn enabled_resolution_failure_keeps_its_type_across_error_boundaries() {
    let marker = KucContextMenuEnabledResolverError;
    let surface: EguiTextCommandSurfaceError = marker.into();
    assert!(surface.to_string().contains(&marker.to_string()));
    let root: EguiTextCommandSurfaceRootError = surface.into();
    assert!(matches!(
        root,
        EguiTextCommandSurfaceRootError::ContextMenuEnabledResolver(_)
    ));
    assert!(root.to_string().contains(&marker.to_string()));
    let facade: EguiTextCommandSurfaceRootFactoryError = root.into();
    assert!(matches!(
        facade,
        EguiTextCommandSurfaceRootFactoryError::ContextMenuEnabledResolver(_)
    ));
    assert_eq!(facade.to_string(), marker.to_string());
    let facade_from_surface: EguiTextCommandSurfaceRootFactoryError =
        EguiTextCommandSurfaceError::from(marker).into();
    assert!(matches!(
        facade_from_surface,
        EguiTextCommandSurfaceRootFactoryError::ContextMenuEnabledResolver(_)
    ));
    let root_from_marker: EguiTextCommandSurfaceRootError = marker.into();
    assert!(matches!(
        &root_from_marker,
        EguiTextCommandSurfaceRootError::ContextMenuEnabledResolver(_)
    ));
    assert!(root_from_marker.to_string().contains(&marker.to_string()));
}
