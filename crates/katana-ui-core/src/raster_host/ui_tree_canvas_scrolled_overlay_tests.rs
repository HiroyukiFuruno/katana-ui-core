use crate::raster_host::canvas::Canvas;
use crate::raster_host::{
    UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost, UiTreeTextRoleBaselineTypography,
};
use crate::render_model::{
    UiBorder, UiDimension, UiEdgeInsets, UiHostActionSpec, UiInteractionState, UiNode, UiNodeKind,
    UiPosition, UiScrollAreaProps, UiVariant, UiVisualRole,
};
use crate::test_assert::KucTestExpect;
use crate::theme::ThemeSnapshot;

const WIDTH: usize = 468;
const HEIGHT: usize = 588;
const FRAME_HEIGHT: u16 = 445;
const FRAME_TOP: u16 = 337;
const BUTTON_SIZE: u16 = 28;
const INSET: u16 = 8;
const ROW_GAP: u16 = 2;

#[test]
fn scrolled_overlay_hover_paints_in_the_visible_control() {
    for fractional in [false, true] {
        let typography = if fractional {
            UiTreeDocumentTypography::new()
                .with_body_baseline(UiTreeTextRoleBaselineTypography::new(14.0, 21.0, 16.0))
        } else {
            UiTreeDocumentTypography::new()
        };
        let host = UiTreeSurfaceHost::with_document_typography(ThemeSnapshot::dark(), typography);
        for (internal, external) in [(0, 300), (300, 0), (0, 580), (580, 0)] {
            let area = UiTreeRenderArea {
                x: 0,
                y: 0,
                width: WIDTH,
                height: HEIGHT,
                scroll_y: external as f32,
            };
            let normal = scene(false, internal);
            let hover = scene(true, internal);
            let mut before = Canvas::new(WIDTH, HEIGHT, 0);
            let mut after = Canvas::new(WIDTH, HEIGHT, 0);
            host.render(&mut before, &normal, area);
            host.render(&mut after, &hover, area);
            /* WHY: document 座標から投影する API には内部と外部を合算した scroll を渡す。 */
            let hit_area = UiTreeRenderArea {
                scroll_y: (internal + external) as f32,
                ..area
            };
            let nodes = host.viewport_node_hits(&normal, hit_area);
            assert!(
                nodes.iter().any(|hit| hit.node_id.as_str() == "control"),
                "control hit missing: baseline={fractional} internal={internal} external={external} nodes={nodes:?}"
            );
            let hit = nodes
                .iter()
                .find(|hit| hit.node_id.as_str() == "control")
                .kuc_expect("visible control node");
            let actions = host.host_action_hits(&normal, area);
            assert!(
                actions.iter().any(|hit| hit.action.action_id == "control"),
                "control action missing: baseline={fractional} internal={internal} external={external} actions={actions:?}"
            );
            let action = actions
                .iter()
                .find(|hit| hit.action.action_id == "control")
                .kuc_expect("visible control action");
            assert_eq!(hit.rect, action.rect);
            let changes = before
                .pixels()
                .iter()
                .zip(after.pixels())
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .inspect(|(index, _)| {
                    let x = index % WIDTH;
                    let y = index / WIDTH;
                    assert!(
                        x >= hit.rect.x
                            && x < hit.rect.x + hit.rect.width
                            && y >= hit.rect.y
                            && y < hit.rect.y + hit.rect.height,
                        "hover pixels must remain inside the viewport hit"
                    );
                })
                .count();
            assert!(
                changes > 0,
                "visible overlay must paint: fractional={fractional} internal={internal} external={external}"
            );
        }
    }
}

fn scene(hovered: bool, offset_y: u32) -> UiNode {
    let button = UiNode::new(UiNodeKind::Button, "")
        .variant(UiVariant::Icon)
        .width(UiDimension::px(BUTTON_SIZE))
        .height(UiDimension::px(BUTTON_SIZE))
        .stable_node_id("control")
        .host_action(UiHostActionSpec::command("control", "Control"))
        .interaction(UiInteractionState {
            hovered,
            ..UiInteractionState::default()
        });
    let common = button
        .props()
        .common
        .clone()
        .hover_border(UiBorder::solid(1, 0, "accent"));
    let overlay = UiNode::new(UiNodeKind::Column, "")
        .position(UiPosition::Absolute)
        .margin(UiEdgeInsets {
            bottom: UiDimension::px(INSET),
            right: UiDimension::px(INSET),
            ..UiEdgeInsets::default()
        })
        .child(UiNode::new(UiNodeKind::Row, "").child(button.common(common)))
        .child(
            UiNode::new(UiNodeKind::Row, "")
                .child(UiNode::new(UiNodeKind::Button, "").height(UiDimension::px(BUTTON_SIZE))),
        )
        .child(
            UiNode::new(UiNodeKind::Row, "")
                .child(UiNode::new(UiNodeKind::Button, "").height(UiDimension::px(BUTTON_SIZE))),
        );
    let common = overlay.props().common.clone().gap(UiDimension::px(ROW_GAP));
    let overlay = overlay.common(common);
    let frame = UiNode::new(UiNodeKind::Stack, "")
        .width(UiDimension::px(WIDTH as u16))
        .height(UiDimension::px(FRAME_HEIGHT))
        .visual_role(UiVisualRole::MediaFrame)
        .child(overlay);
    UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            offset_y,
            viewport_width: WIDTH as u32,
            viewport_height: HEIGHT as u32,
            content_width: WIDTH as u32,
            content_height: u32::from(FRAME_TOP + FRAME_HEIGHT),
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(UiNode::new(UiNodeKind::Spacer, "").height(UiDimension::px(FRAME_TOP)))
                .child(frame),
        )
}
