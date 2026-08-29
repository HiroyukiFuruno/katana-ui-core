use katana_ui_core::atom::{Icon, Input, TextArea};
use katana_ui_core::layout::Column;
use katana_ui_core::render_model::{
    UiIconProps, UiNode, UiNodeKind, UiSlotPlacement, UiSlotSpec, UiSvgIconPixelPlan,
    UiSvgIconRenderPlan, UiSvgIconViewBox, UiSvgPaintPolicy, UiTextEntryProps, UiTree,
    UiTreeSemantics,
};

const CALLER_FOLDER_SVG: &str = "<svg data-caller-icon=\"folder\"/>";
const CALLER_SEARCH_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"><circle cx=\"11\" cy=\"11\" r=\"8\"/><line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\"/></svg>";
const CALLER_CLEAR_SVG: &str = "<svg data-caller-icon=\"clear\"/>";
const CALLER_NOTE_SVG: &str = "<svg data-caller-icon=\"note\"/>";
const CALLER_FORMAT_SVG: &str = "<svg data-caller-icon=\"format\"/>";

#[test]
fn svg_icon_render_plan_collects_external_svg_sources_and_callbacks() {
    let tree = UiTree::new(
        Column::new()
            .child(Icon::new("Folder").svg_source(CALLER_FOLDER_SVG))
            .child(
                Input::new("Search")
                    .leading_svg_icon_slot("Search icon", CALLER_SEARCH_SVG)
                    .trailing_svg_icon_button("Clear search", CALLER_CLEAR_SVG, "search.clear"),
            )
            .child(
                TextArea::new("Notes")
                    .leading_svg_icon_slot("Note icon", CALLER_NOTE_SVG)
                    .trailing_svg_icon_button("Format notes", CALLER_FORMAT_SVG, "notes.format"),
            ),
    );

    let plans = UiSvgIconRenderPlan::collect_from_tree(&tree);

    assert_eq!(5, plans.len());
    assert_eq!(
        Some(CALLER_FOLDER_SVG),
        find_plan(&plans, UiNodeKind::Icon, "", "").map(|plan| plan.svg_source.as_str())
    );
    assert_eq!(
        Some((UiSlotPlacement::Leading, CALLER_SEARCH_SVG)),
        find_plan(&plans, UiNodeKind::Input, "Search icon", "").and_then(|plan| plan
            .placement
            .map(|placement| (placement, plan.svg_source.as_str())))
    );
    assert_eq!(
        Some((UiSlotPlacement::Trailing, CALLER_CLEAR_SVG)),
        find_plan(&plans, UiNodeKind::Input, "Clear search", "search.clear").and_then(|plan| plan
            .placement
            .map(|placement| (placement, plan.svg_source.as_str())))
    );
    assert_eq!(
        Some(CALLER_NOTE_SVG),
        find_plan(&plans, UiNodeKind::TextArea, "Note icon", "")
            .map(|plan| plan.svg_source.as_str())
    );
    assert_eq!(
        Some(CALLER_FORMAT_SVG),
        find_plan(&plans, UiNodeKind::TextArea, "Format notes", "notes.format")
            .map(|plan| plan.svg_source.as_str())
    );
}

#[test]
fn svg_icon_render_plan_preserves_external_svg_metadata_for_adapters() -> Result<(), String> {
    let tree = UiTree::new(
        Input::new("Search").leading_icon_slot(
            "Search icon",
            UiIconProps::new(CALLER_SEARCH_SVG)
                .view_box("0 0 24 24")
                .path_summary("search circle and handle")
                .paint_policy(UiSvgPaintPolicy::StrokeOnly)
                .role("search")
                .color_token("input.foreground")
                .theme_token("input.icon"),
        ),
    );

    let plans = UiSvgIconRenderPlan::collect_from_tree(&tree);
    let plan = find_plan(&plans, UiNodeKind::Input, "Search icon", "")
        .ok_or_else(|| "leading svg icon plan should exist".to_string())?;

    assert_eq!(CALLER_SEARCH_SVG, plan.svg_source);
    assert_eq!("0 0 24 24", plan.view_box);
    assert_eq!("search circle and handle", plan.path_summary);
    assert_eq!(UiSvgPaintPolicy::StrokeOnly, plan.paint_policy);
    assert_eq!("search", plan.role);
    assert_eq!("input.foreground", plan.color_token);
    assert_eq!("input.icon", plan.theme_token);
    Ok(())
}

#[test]
fn svg_icon_pixel_plan_preserves_viewbox_scale_and_paint_contract() -> Result<(), String> {
    let tree = UiTree::new(
        Input::new("Search").leading_icon_slot(
            "Search icon",
            UiIconProps::new(CALLER_SEARCH_SVG)
                .view_box("0 0 24 24")
                .path_summary("search circle and handle")
                .paint_policy(UiSvgPaintPolicy::StrokeOnly)
                .role("search")
                .color_token("input.foreground")
                .theme_token("input.icon"),
        ),
    );

    let plans = UiSvgIconPixelPlan::collect_from_tree(&tree);
    let plan = plans
        .first()
        .ok_or_else(|| "svg icon pixel plan should exist".to_string())?;

    assert!(plan.pixel_ready);
    assert_eq!(16, plan.viewport.width);
    assert_eq!(16, plan.viewport.height);
    assert_eq!(Some(UiSvgIconViewBox::new(0, 0, 24, 24)), plan.view_box);
    assert_eq!(666, plan.scale_x_milli);
    assert_eq!(666, plan.scale_y_milli);
    assert_eq!(UiSvgPaintPolicy::StrokeOnly, plan.paint_policy);
    assert_eq!("input.foreground", plan.color_token);
    assert_eq!("input.icon", plan.theme_token);
    Ok(())
}

#[test]
fn svg_plan_ignores_missing_sources_and_handles_invalid_viewbox() {
    let empty_tree = UiTree::new(
        Column::new()
            .child(Icon::new("No source"))
            .child(Input::new("Text prefix").leading_slot("Prefix"))
            .child(Icon::new("Blank source").svg_source("   ")),
    );
    assert!(UiSvgIconRenderPlan::collect_from_tree(&empty_tree).is_empty());

    let invalid_tree = UiTree::new(
        Icon::new("Invalid viewbox")
            .svg_source(CALLER_FOLDER_SVG)
            .icon_view_box("0 0 0 16"),
    );
    let plans = UiSvgIconPixelPlan::collect_from_tree(&invalid_tree);
    assert_eq!(1, plans.len());
    assert!(!plans[0].pixel_ready);
    assert_eq!(0, plans[0].scale_x_milli);
    assert_eq!(0, plans[0].scale_y_milli);
    assert_eq!(None, plans[0].view_box);

    for invalid_view_box in [
        "",
        "x 0 24 24",
        "0 y 24 24",
        "0 0 width 24",
        "0 0 24 height",
        "0 0 24 24 extra",
        "0 0 24 0",
    ] {
        let tree = UiTree::new(
            Icon::new("Invalid")
                .svg_source(CALLER_FOLDER_SVG)
                .icon_view_box(invalid_view_box),
        );
        assert_eq!(
            None,
            UiSvgIconPixelPlan::collect_from_tree(&tree)[0].view_box
        );
    }

    let trailing_icon =
        UiNode::new(UiNodeKind::Input, "Trailing icon").text_entry(UiTextEntryProps {
            trailing_slot: Some(UiSlotSpec::icon(
                UiSlotPlacement::Trailing,
                "Trailing",
                UiIconProps::new(CALLER_CLEAR_SVG),
            )),
            ..UiTextEntryProps::default()
        });
    let trailing_plans = UiSvgIconRenderPlan::collect_from_tree(&UiTree::new(trailing_icon));
    assert_eq!(1, trailing_plans.len());
    assert_eq!("Trailing", trailing_plans[0].slot_label);
}

#[test]
fn semantic_fingerprint_changes_when_text_entry_svg_or_callback_changes() {
    let base = UiTree::new(
        Input::new("Search")
            .leading_svg_icon_slot("Search icon", CALLER_SEARCH_SVG)
            .trailing_svg_icon_button("Clear search", CALLER_CLEAR_SVG, "search.clear"),
    );
    let changed_svg = UiTree::new(
        Input::new("Search")
            .leading_svg_icon_slot("Search icon", CALLER_NOTE_SVG)
            .trailing_svg_icon_button("Clear search", CALLER_CLEAR_SVG, "search.clear"),
    );
    let changed_callback = UiTree::new(
        Input::new("Search")
            .leading_svg_icon_slot("Search icon", CALLER_SEARCH_SVG)
            .trailing_svg_icon_button("Clear search", CALLER_CLEAR_SVG, "search.reset"),
    );

    assert_ne!(
        UiTreeSemantics::fingerprint(&base),
        UiTreeSemantics::fingerprint(&changed_svg)
    );
    assert_ne!(
        UiTreeSemantics::fingerprint(&base),
        UiTreeSemantics::fingerprint(&changed_callback)
    );
}

fn find_plan<'a>(
    plans: &'a [UiSvgIconRenderPlan],
    node_kind: UiNodeKind,
    slot_label: &str,
    callback: &str,
) -> Option<&'a UiSvgIconRenderPlan> {
    plans.iter().find(|plan| {
        plan.node_kind == node_kind && plan.slot_label == slot_label && plan.callback == callback
    })
}
