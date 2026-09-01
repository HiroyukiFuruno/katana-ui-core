use super::*;

#[test]
fn artifact_order_places_children_in_composite_layer_order() {
    let order = artifact_order_for_root(RootArtifactChildren {
        tab_strip: true,
        tab_strip_overlay: true,
        source_address: true,
        toolbar: true,
        toolbar_dropdown_open: true,
        search: true,
        floating_open: true,
        context_menu_open: true,
        status_bar: true,
        diagnostics_list: true,
        preview: true,
    });
    assert_eq!(
        order,
        vec![
            EguiTextCommandSurfaceChild::TabStrip,
            EguiTextCommandSurfaceChild::SourceAddress,
            EguiTextCommandSurfaceChild::Search,
            EguiTextCommandSurfaceChild::Text,
            EguiTextCommandSurfaceChild::Preview,
            EguiTextCommandSurfaceChild::DiagnosticsList,
            EguiTextCommandSurfaceChild::StatusBar,
            EguiTextCommandSurfaceChild::Toolbar,
            EguiTextCommandSurfaceChild::Floating,
            EguiTextCommandSurfaceChild::ContextMenu,
            EguiTextCommandSurfaceChild::TabStripOverlay,
        ]
    );
}

#[test]
fn closed_children_and_dropdown_state_change_order_without_fallbacks() {
    let order = artifact_order_for_root(RootArtifactChildren {
        tab_strip: false,
        tab_strip_overlay: false,
        source_address: false,
        toolbar: true,
        toolbar_dropdown_open: false,
        search: false,
        floating_open: false,
        context_menu_open: false,
        status_bar: false,
        diagnostics_list: false,
        preview: false,
    });
    assert_eq!(
        order,
        vec![
            EguiTextCommandSurfaceChild::Toolbar,
            EguiTextCommandSurfaceChild::Text
        ]
    );
}
