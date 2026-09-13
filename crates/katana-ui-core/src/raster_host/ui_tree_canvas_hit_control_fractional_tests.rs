use crate::raster_host::canvas::Canvas;
use crate::raster_host::{
    UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost, UiTreeTextRoleBaselineTypography,
};
use crate::test_assert::KucTestExpect;
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{
    UiBorder, UiDimension, UiHostActionSpec, UiInteractionState, UiNode, UiNodeId, UiNodeKind,
    UiVariant,
};
use katana_ui_core::theme::ThemeSnapshot;

const CONTROL_SIZE: u16 = 28;

#[test]
fn fractional_button_hits_contain_the_complete_hover_border_at_both_scales() {
    let host = UiTreeSurfaceHost::with_document_typography(
        ThemeSnapshot::dark(),
        UiTreeDocumentTypography::new()
            .with_heading_1_baseline(UiTreeTextRoleBaselineTypography::new(20.0, 31.5, 18.5)),
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 128,
        height: 100,
        scroll_y: 0.0,
    };
    for kind in [
        UiNodeKind::Button,
        UiNodeKind::TextButton,
        UiNodeKind::IconTextButton,
    ] {
        let normal = scene(kind, false);
        let hovered = scene(kind, true);
        let nodes = host.document_node_hits(&normal, area);
        let actions = host.document_host_action_hits(&normal, area);
        let rect = nodes
            .iter()
            .find(|hit| hit.node_id.as_str() == "control")
            .kuc_expect("control node hit")
            .rect;
        let action = actions
            .iter()
            .find(|hit| hit.action.action_id == "control")
            .kuc_expect("control action hit");
        assert_eq!(rect, action.rect);
        assert_eq!(31, rect.y);
        assert_eq!(29, rect.height);
        let following = nodes
            .iter()
            .find(|hit| hit.node_id.as_str() == "following")
            .kuc_expect("following node hit");
        assert_eq!(
            59, following.rect.y,
            "hit envelope must not change layout advance"
        );
        for scale in [1.0, 2.0] {
            let mut normal_canvas = Canvas::new_scaled(128, 100, scale, 0);
            let mut hovered_canvas = Canvas::new_scaled(128, 100, scale, 0);
            host.render(&mut normal_canvas, &normal, area);
            host.render(&mut hovered_canvas, &hovered, area);
            let changed = normal_canvas
                .pixels()
                .iter()
                .zip(hovered_canvas.pixels())
                .enumerate()
                .filter(|(_, (normal, hovered))| normal != hovered)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            assert!(!changed.is_empty(), "hover must paint a real border");
            for index in changed {
                let x = ((index % hovered_canvas.width()) as f32 / scale).floor() as usize;
                let y = ((index / hovered_canvas.width()) as f32 / scale).floor() as usize;
                assert!(
                    x >= rect.x
                        && x < rect.x + rect.width
                        && y >= rect.y
                        && y < rect.y + rect.height,
                    "hover paint must fit the hit envelope at every scale"
                );
            }
        }
    }
}

fn scene(kind: UiNodeKind, hovered: bool) -> UiNode {
    let heading: UiNode = Text::new("Heading").text_role("heading").into();
    let button = UiNode::new(kind, "control")
        .variant(UiVariant::Icon)
        .width(UiDimension::px(CONTROL_SIZE))
        .height(UiDimension::px(CONTROL_SIZE))
        .stable_node_id(UiNodeId::new("control"))
        .host_action(UiHostActionSpec::command("control", "control"))
        .interaction(UiInteractionState {
            hovered,
            ..UiInteractionState::default()
        });
    let common = button
        .props()
        .common
        .clone()
        .hover_border(UiBorder::solid(1, 0, "accent"));
    UiNode::new(UiNodeKind::Column, "")
        .child(heading)
        .child(button.common(common))
        .child(UiNode::from(Text::new("Following")).stable_node_id(UiNodeId::new("following")))
}
