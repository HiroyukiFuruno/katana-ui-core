use crate::visual::text_command_surface_integration_tests::facts;
use katana_ui_core::render_model::UiRect;
use katana_ui_core_egui_adapter::artifact_compositor::ArtifactPaintPlanRef;
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurfaceChild, EguiTextCommandSurfaceOutput,
};

pub(crate) struct Assertions;

impl Assertions {
    pub(crate) fn assert_inside(name: &str, child: UiRect, root: UiRect) {
        assert!(
            child.x >= root.x
                && child.y >= root.y
                && child.x.saturating_add_unsigned(child.width)
                    <= root.x.saturating_add_unsigned(root.width)
                && child.y.saturating_add_unsigned(child.height)
                    <= root.y.saturating_add_unsigned(root.height),
            "{name} must be inside root"
        );
    }

    pub(crate) fn assert_artifact_output_contract(
        output: &EguiTextCommandSurfaceOutput,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expected = facts::FrameFacts::expected_artifact_order(output);
        let plans = output.artifact_paint_plans()?;
        assert_eq!(output.artifact_order(), expected);
        assert_eq!(output.artifact_order().len(), plans.len());
        for (position, (child, plan)) in
            output.artifact_order().iter().zip(plans.iter()).enumerate()
        {
            let matches_child = matches!(
                (child, plan),
                (
                    EguiTextCommandSurfaceChild::Text,
                    ArtifactPaintPlanRef::TextSurface(_)
                ) | (
                    EguiTextCommandSurfaceChild::Toolbar
                        | EguiTextCommandSurfaceChild::Search
                        | EguiTextCommandSurfaceChild::Floating,
                    ArtifactPaintPlanRef::CommandChrome(_)
                ) | (
                    EguiTextCommandSurfaceChild::ContextMenu,
                    ArtifactPaintPlanRef::ContextMenu(_)
                )
            );
            assert!(
                matches_child,
                "artifact plan at position {position} does not match child {child:?}"
            );
        }
        Ok(())
    }

    pub(crate) fn assert_accesskit(
        full: &egui::FullOutput,
        root: UiRect,
        required_labels: &[&str],
        forbidden_labels: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let update = full
            .platform_output
            .accesskit_update
            .as_ref()
            .ok_or(std::io::Error::other("accesskit update must be present"))?;
        let labels = Self::collect_accesskit_labels(update);
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
                "missing AccessKit label containing {expected} with bounds"
            );
        }
        for forbidden in forbidden_labels {
            assert!(
                labels.iter().all(|label| !label.contains(forbidden)),
                "unexpected AccessKit label containing {forbidden}"
            );
        }
        for (_, node) in &update.nodes {
            if let Some(bounds) = node.bounds() {
                assert!(
                    bounds.x0 >= f64::from(root.x)
                        && bounds.y0 >= f64::from(root.y)
                        && bounds.x1 <= f64::from(root.x.saturating_add_unsigned(root.width))
                        && bounds.y1 <= f64::from(root.y.saturating_add_unsigned(root.height)),
                    "accesskit node bounds out of root"
                );
            }
        }
        assert!(
            labels.iter().any(|label| label.contains('⭐')),
            "expected an AccessKit label with star marker"
        );
        Ok(())
    }

    fn collect_accesskit_labels(update: &egui::accesskit::TreeUpdate) -> Vec<String> {
        update
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label().map(ToString::to_string))
            .collect()
    }
}
