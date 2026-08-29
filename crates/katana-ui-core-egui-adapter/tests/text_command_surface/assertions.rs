use katana_ui_core::render_model::UiRect;
use katana_ui_core_egui_adapter::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurfaceChild, EguiTextCommandSurfaceOutput,
};

use super::support::STAR_LABEL;

pub(crate) fn expected_artifact_order(
    output: &EguiTextCommandSurfaceOutput,
) -> Vec<EguiTextCommandSurfaceChild> {
    output.artifact_order().to_vec()
}

pub(crate) fn assert_artifact_output_contract(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected = expected_artifact_order(output);
    let plans = output.artifact_paint_plans()?;
    assert_eq!(output.artifact_order(), expected);
    assert_eq!(output.artifact_order().len(), plans.len());
    if output
        .artifact_order()
        .contains(&EguiTextCommandSurfaceChild::ContextMenu)
    {
        assert_eq!(
            output.artifact_order().last(),
            Some(&EguiTextCommandSurfaceChild::ContextMenu)
        );
    }

    for (position, (child, plan)) in output.artifact_order().iter().zip(plans.iter()).enumerate() {
        match (child, plan) {
            (EguiTextCommandSurfaceChild::Text, ArtifactPaintPlanRef::TextSurface(_)) => {}
            (EguiTextCommandSurfaceChild::TabStrip, ArtifactPaintPlanRef::TabStrip(_))
            | (EguiTextCommandSurfaceChild::TabStripOverlay, ArtifactPaintPlanRef::TabStrip(_)) => {
            }
            (
                EguiTextCommandSurfaceChild::SourceAddress,
                ArtifactPaintPlanRef::SourceAddress(_),
            ) => {}
            (EguiTextCommandSurfaceChild::Toolbar, ArtifactPaintPlanRef::CommandChrome(_)) => {}
            (EguiTextCommandSurfaceChild::Search, ArtifactPaintPlanRef::CommandChrome(_)) => {}
            (EguiTextCommandSurfaceChild::Floating, ArtifactPaintPlanRef::CommandChrome(_)) => {}
            (EguiTextCommandSurfaceChild::ContextMenu, ArtifactPaintPlanRef::ContextMenu(_)) => {}
            (EguiTextCommandSurfaceChild::StatusBar, ArtifactPaintPlanRef::StatusBar(_)) => {}
            (
                EguiTextCommandSurfaceChild::DiagnosticsList,
                ArtifactPaintPlanRef::DiagnosticsList(_),
            ) => {}
            (other_child, other_plan) => {
                panic!(
                    "artifact plan at position {position} is {other_plan:?} but child is {other_child:?}"
                );
            }
        }
    }
    if output
        .context_menu
        .as_ref()
        .is_some_and(|context_menu| context_menu.artifact.is_some())
    {
        assert!(matches!(
            plans.last(),
            Some(ArtifactPaintPlanRef::ContextMenu(_))
        ));
    }
    Ok(())
}

pub(crate) fn assert_inside(child: UiRect, root: UiRect) {
    assert!(
        child.x >= root.x
            && child.y >= root.y
            && child.x.saturating_add_unsigned(child.width)
                <= root.x.saturating_add_unsigned(root.width)
            && child.y.saturating_add_unsigned(child.height)
                <= root.y.saturating_add_unsigned(root.height)
    );
}

pub(crate) fn assert_floating_within_root(output: &EguiTextCommandSurfaceOutput, root: UiRect) {
    let floating = output
        .floating
        .as_ref()
        .expect("floating output")
        .record
        .as_ref()
        .expect("floating toolbar expected");
    assert_inside(floating.panel_bounds, root);
    assert_inside(floating.toolbar.bounds, root);
}

pub(crate) fn overlay_outside_point(output: &EguiTextCommandSurfaceOutput) -> egui::Pos2 {
    let panel_bounds = output
        .floating
        .as_ref()
        .expect("floating output")
        .record
        .as_ref()
        .expect("floating toolbar expected")
        .panel_bounds;
    let dropdown_bounds = output
        .floating
        .as_ref()
        .expect("floating output")
        .record
        .as_ref()
        .and_then(|record| {
            record
                .toolbar
                .dropdown
                .as_ref()
                .map(|dropdown| dropdown.bounds)
        });
    let point = egui::pos2(
        output.root_bounds.x as f32 + output.root_bounds.width as f32 - 8.0,
        output.root_bounds.y as f32 + output.root_bounds.height as f32 - 8.0,
    );
    assert!(point.x >= panel_bounds.x as f32 && point.y >= panel_bounds.y as f32);
    assert!(
        point.x
            < output
                .root_bounds
                .x
                .saturating_add_unsigned(output.root_bounds.width) as f32
            && point.y
                < output
                    .root_bounds
                    .y
                    .saturating_add_unsigned(output.root_bounds.height) as f32
    );
    assert!(
        !contains_point(&panel_bounds, point),
        "outside point must be outside floating panel"
    );
    if let Some(dropdown_bounds) = dropdown_bounds {
        assert!(
            !contains_point(&dropdown_bounds, point),
            "outside point must be outside dropdown panel"
        );
    }
    point
}

fn contains_point(bounds: &UiRect, point: egui::Pos2) -> bool {
    point.x >= bounds.x as f32
        && point.x < bounds.x.saturating_add_unsigned(bounds.width) as f32
        && point.y >= bounds.y as f32
        && point.y < bounds.y.saturating_add_unsigned(bounds.height) as f32
}

pub(crate) fn floating_dropdown_trigger(
    output: &EguiTextCommandSurfaceOutput,
) -> Option<egui::Pos2> {
    output
        .floating
        .as_ref()?
        .record
        .as_ref()?
        .toolbar
        .actions
        .iter()
        .find_map(|action| {
            action
                .secondary_trigger_bounds
                .or(Some(action.bounds))
                .map(center)
        })
}

pub(crate) fn composite_hash(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<String, Box<dyn std::error::Error>> {
    let plans = output.artifact_paint_plans()?;
    Ok(ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(output.root_bounds),
        plans: &plans,
    })?
    .pixel_hash)
}

fn center(bounds: UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

pub(crate) fn assert_accesskit(
    full: &egui::FullOutput,
    root: UiRect,
    required_labels: &[&str],
    forbidden_labels: &[&str],
) {
    let update = full
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit update");
    let labels = accesskit_labels(&update.nodes);
    for expected in required_labels {
        let mut found = false;
        for (_, node) in &update.nodes {
            let Some(label) = node.label() else {
                continue;
            };
            if label.contains(expected) && node.bounds().is_some() {
                found = true;
                break;
            }
        }
        assert!(
            found,
            "missing AccessKit node label containing {expected} with bounds"
        );
    }
    for forbidden in forbidden_labels {
        assert!(
            labels.iter().all(|label| !label.contains(forbidden)),
            "unexpected AccessKit label containing {forbidden}"
        );
    }
    for (_, node) in &update.nodes {
        let bounds = node.bounds();
        if let Some(bounds) = bounds {
            assert!(
                bounds.x0 >= f64::from(root.x)
                    && bounds.y0 >= f64::from(root.y)
                    && bounds.x1 <= f64::from(root.x.saturating_add_unsigned(root.width))
                    && bounds.y1 <= f64::from(root.y.saturating_add_unsigned(root.height))
            );
        }
    }
    assert_labels_include_star(full);
}

fn accesskit_labels(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]) -> Vec<String> {
    nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(ToString::to_string))
        .collect()
}

fn assert_labels_include_star(full: &egui::FullOutput) {
    let update = full
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit update");
    assert!(
        update
            .nodes
            .iter()
            .any(|(_, node)| node.label().is_some_and(|label| label.contains(STAR_LABEL))),
        "expected an AccessKit label with star marker"
    );
}
