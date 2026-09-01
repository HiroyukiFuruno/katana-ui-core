use super::*;
use crate::egui::text_command_surface::{
    TabStripContextMenuPresentation, TabStripCorrelation, TabStripGroupDescriptor,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripProjection,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};

fn projection_for_path_tests() -> TabStripProjection {
    TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"correlation"))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"root-tab-without-menu"),
                TabStripText::new("Root Tab"),
            )
            .tooltip(TabStripText::new("no path test")),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"root-group"),
                TabStripText::new("Root group"),
            )
            .group(
                TabStripGroupDescriptor::new(
                    TabStripGroupTarget::from_opaque_bytes(b"child-group"),
                    TabStripText::new("Child group"),
                )
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"leaf-tab"),
                        TabStripText::new("Leaf tab"),
                    )
                    .context_menu(
                        TabStripContextMenuPresentation::new()
                            .entry(TabStripMenuEntry::action(
                                TabStripText::new("Close"),
                                TabStripText::new("Close"),
                                TabStripMenuOperation::RequestClose,
                            ))
                            .entry(TabStripMenuEntry::action(
                                TabStripText::new("Rename"),
                                TabStripText::new("Rename"),
                                TabStripMenuOperation::SetPinned(true),
                            )),
                    ),
                )
                .group(TabStripGroupDescriptor::new(
                    TabStripGroupTarget::from_opaque_bytes(b"grand-child-group"),
                    TabStripText::new("Grand-child group"),
                )),
            )
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"grouped-tab"),
                    TabStripText::new("Grouped tab"),
                )
                .context_menu(TabStripContextMenuPresentation::new()),
            ),
        )
}

#[test]
fn tab_menu_for_path_is_fail_closed_for_root_tab_and_deep_paths() {
    let projection = projection_for_path_tests();

    assert!(tab_menu_for_path(&projection, "root-tab-0").is_none());
    assert!(tab_menu_for_path(&projection, "root-group-0-tab-0").is_some());
    assert!(tab_menu_for_path(&projection, "root-group-0-group-0-tab-0",).is_some());
    assert!(tab_menu_for_path(&projection, "root-group-0-group-0").is_none());
    assert!(tab_menu_for_path(&projection, "root-group-0-group-1").is_none());
}

#[test]
fn group_for_path_returns_nested_group_or_fail_closed_for_unknown() {
    let projection = projection_for_path_tests();

    assert!(group_for_path(&projection, "root-group-0").is_some());
    assert!(group_for_path(&projection, "root-group-0-group-0").is_some());
    assert!(group_for_path(&projection, "root-group-0-group-0-group-0").is_some());
    assert!(group_for_path(&projection, "root-group-0-group-1").is_none());
}

#[test]
fn union_bounds_is_fail_closed_for_empty_input_and_rounds_single_rect() {
    assert!(union_bounds(&[] as &[egui::Rect]).is_none());
    let first = egui::Rect::from_min_size(egui::pos2(1.0, 1.0), egui::vec2(4.0, 5.0));
    let second = egui::Rect::from_min_size(egui::pos2(-2.0, 0.0), egui::vec2(1.0, 1.0));

    let combined = union_bounds(&[first, second]).expect("both bounds should combine");
    assert!(combined.min.x <= first.min.x && combined.min.x <= second.min.x);
    assert!(combined.max.y >= first.max.y.max(second.max.y));
}

#[test]
fn ui_rect_clamps_negative_dimensions_to_zero() {
    let negative = egui::Rect::from_min_max(egui::pos2(10.0, 10.0), egui::pos2(8.0, 6.0));
    let compact = ui_rect(negative);
    assert_eq!(compact.width, 0);
    assert_eq!(compact.height, 0);
}
