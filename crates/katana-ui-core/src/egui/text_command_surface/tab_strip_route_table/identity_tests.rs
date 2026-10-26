use super::super::{
    TabStripContextMenuPresentation, TabStripControlPresentation, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripProjection,
    TabStripSurfaceCapabilities, TabStripSwatchDescriptor, TabStripSwatchTarget,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use super::TabStripRouteTable;

#[test]
fn interaction_identity_distinguishes_every_tab_and_group_operation_variant() {
    let group_target = TabStripGroupTarget::from_opaque_bytes(b"group");
    let tab_target = TabStripTabTarget::from_opaque_bytes(b"tab");
    let tab_menu = TabStripContextMenuPresentation::new()
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Close"),
            TabStripText::new("Close"),
            TabStripMenuOperation::RequestClose,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Others"),
            TabStripText::new("Others"),
            TabStripMenuOperation::CloseOthers,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Left"),
            TabStripText::new("Left"),
            TabStripMenuOperation::CloseToLeft,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Right"),
            TabStripText::new("Right"),
            TabStripMenuOperation::CloseToRight,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Restore"),
            TabStripText::new("Restore"),
            TabStripMenuOperation::RestoreClosed,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Pin"),
            TabStripText::new("Pin"),
            TabStripMenuOperation::SetPinned(true),
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Create"),
            TabStripText::new("Create"),
            TabStripMenuOperation::CreateGroup,
        ))
        .entry(
            TabStripMenuEntry::submenu(TabStripText::new("Move"), TabStripText::new("Move")).child(
                TabStripMenuEntry::action(
                    TabStripText::new("Target"),
                    TabStripText::new("Target"),
                    TabStripMenuOperation::MoveToGroup(group_target.copy_for_route()),
                ),
            ),
        )
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Remove"),
            TabStripText::new("Remove"),
            TabStripMenuOperation::RemoveFromGroup,
        ));
    let tab = TabStripTabDescriptor::new(tab_target, TabStripText::new("tab"))
        .capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .closeable(true)
                .pinned(true)
                .groupable(true),
        )
        .trailing_control(TabStripControlPresentation::new(
            TabStripText::new("unpin"),
            TabStripText::new("unpin"),
        ))
        .context_menu(tab_menu);
    let group_popup = TabStripGroupPopupPresentation::new()
        .rename_placeholder(TabStripText::new("Rename"))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Ungroup"),
            TabStripText::new("Ungroup"),
            TabStripMenuOperation::Ungroup,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Close"),
            TabStripText::new("Close"),
            TabStripMenuOperation::CloseGroup,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Color"),
            TabStripText::new("Color"),
            TabStripMenuOperation::Recolor(TabStripSwatchTarget::from_opaque_bytes(b"swatch")),
        ));
    let group = TabStripGroupDescriptor::new(group_target, TabStripText::new("group"))
        .capabilities(
            TabStripGroupCapabilities::new()
                .collapsible(true)
                .renamable(true)
                .ungroupable(true)
                .closeable(true)
                .recolorable(true),
        )
        .popup(group_popup)
        .swatch(TabStripSwatchDescriptor::new(
            TabStripSwatchTarget::from_opaque_bytes(b"swatch"),
            crate::molecule::RgbaColor::new(1, 2, 3, 255),
        ));
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(tab)
        .group(group)
        .capabilities(TabStripSurfaceCapabilities::new().restore_available(true));
    let routes = TabStripRouteTable::from_projection(&projection);
    let paths = [
        "root-tab-0-label",
        "root-tab-0-trailing",
        "root-tab-0-menu-0",
        "root-tab-0-menu-1",
        "root-tab-0-menu-2",
        "root-tab-0-menu-3",
        "root-tab-0-menu-4",
        "root-tab-0-menu-5",
        "root-tab-0-menu-6",
        "root-tab-0-menu-7-0",
        "root-tab-0-menu-8",
        "root-group-0-header",
        "root-group-0-popup-rename",
        "root-group-0-popup-0",
        "root-group-0-popup-1",
        "root-group-0-popup-2",
        "root-group-0-popup-swatch-0",
    ];
    let parent = egui::Id::new("identity-parent");
    let ids: Vec<_> = paths
        .iter()
        .map(|path| {
            if *path != "root-group-0-popup-rename" {
                assert!(routes.proposal_for(path).is_some(), "route missing: {path}");
            }
            routes.interaction_id(parent, path)
        })
        .collect();
    for (index, id) in ids.iter().enumerate() {
        assert!(ids[index + 1..].iter().all(|other| other != id));
    }
}
