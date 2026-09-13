use crate::atom::Text;
use crate::raster_host::{
    Canvas, UiTreeDocumentTypography, UiTreeRenderArea, UiTreeSurfaceHost,
    UiTreeTextRoleBaselineTypography,
};
use crate::render_model::{
    UiCommonProps, UiDimension, UiEdgeInsets, UiHostActionSpec, UiNode, UiNodeId, UiNodeKind,
    UiScrollAreaProps, UiTree, UiVisualRole,
};
use crate::theme::ThemeSnapshot;

const AREA_WIDTH: usize = 160;
const AREA_HEIGHT: usize = 80;
const BODY_FONT_SIZE: f32 = 14.0;
const BODY_BASELINE: f32 = 16.0;
const FRACTIONAL_LINE_BOX: f32 = 31.5;
const VIEWPORT_HEIGHT: u32 = 64;
const PARTIAL_SOURCE_Y: u32 = 14;
const PARTIAL_VIEWPORT_HEIGHT: usize = 40;
const PARTIAL_CONTENT_HEIGHT: u32 = 68;
const SECOND_CHILD_LEFT: u16 = 100;

#[test]
fn fractional_scroll_visibility_matches_for_normal_and_auto_hover_text() {
    let host = fractional_host();

    for (scroll_y, target_visible) in [(31.25, true), (31.5, false), (31.75, false)] {
        let normal = fractional_scroll_tree(false);
        let hovered = fractional_scroll_tree(true);
        let area = render_area(scroll_y);
        let normal_actions = host.host_action_hits(normal.root(), area);
        let hovered_actions = host.host_action_hits(hovered.root(), area);

        assert_eq!(
            target_visible,
            has_action(&normal_actions, "target"),
            "normal target action visibility at scroll_y={scroll_y}"
        );
        assert_eq!(
            target_visible,
            has_action(&hovered_actions, "target"),
            "hover target action visibility at scroll_y={scroll_y}"
        );
        assert_eq!(
            action_rect(&normal_actions, "following"),
            action_rect(&hovered_actions, "following"),
            "hover wrapping must keep the following hit at scroll_y={scroll_y}"
        );
        if !target_visible {
            let mut normal_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);
            let mut hovered_canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);

            host.render(&mut normal_canvas, normal.root(), area);
            host.render(&mut hovered_canvas, hovered.root(), area);

            assert_eq!(
                normal_canvas.pixels(),
                hovered_canvas.pixels(),
                "fully scrolled target must not leave hover pixels at scroll_y={scroll_y}"
            );
        }
    }
}

#[test]
fn partial_hover_surface_with_multiple_children_keeps_the_visible_second_child() {
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            offset_y: PARTIAL_SOURCE_Y,
            viewport_width: AREA_WIDTH as u32,
            viewport_height: PARTIAL_VIEWPORT_HEIGHT as u32,
            content_width: AREA_WIDTH as u32,
            content_height: PARTIAL_CONTENT_HEIGHT,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Stack, "")
                .visual_role(UiVisualRole::HoverSurface)
                .child(Text::new("First").text_role("body"))
                .child(
                    UiNode::from(Text::new("Second").text_role("body"))
                        .common(UiCommonProps::default().margin(UiEdgeInsets {
                            left: UiDimension::px(SECOND_CHILD_LEFT),
                            ..UiEdgeInsets::default()
                        }))
                        .stable_node_id(UiNodeId::new("second"))
                        .host_action(UiHostActionSpec::command("second", "Second")),
                ),
        );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: AREA_WIDTH,
        height: PARTIAL_VIEWPORT_HEIGHT,
        scroll_y: 0.0,
    };
    let second = action_rect(&host.host_action_hits(&root, area), "second");
    let mut canvas = Canvas::new(AREA_WIDTH, PARTIAL_VIEWPORT_HEIGHT, 0);

    host.render(&mut canvas, &root, area);

    let background = canvas.pixels()[second.y * AREA_WIDTH + AREA_WIDTH - 1];
    let visible_second_glyph = (second.y..second.y.saturating_add(second.height)).any(|y| {
        (second.x..second.x.saturating_add(second.width))
            .any(|x| canvas.pixels()[y * AREA_WIDTH + x] != background)
    });
    assert!(
        visible_second_glyph,
        "the visible second child must draw inside its node/action hit: second={second:?}, background={background:#x}"
    );
}

fn fractional_host() -> UiTreeSurfaceHost {
    UiTreeSurfaceHost::with_document_typography(
        ThemeSnapshot::dark(),
        UiTreeDocumentTypography::new().with_body_baseline(UiTreeTextRoleBaselineTypography::new(
            BODY_FONT_SIZE,
            FRACTIONAL_LINE_BOX,
            BODY_BASELINE,
        )),
    )
}

fn fractional_scroll_tree(hovered: bool) -> UiTree {
    let target = UiNode::from(Text::new("Target").text_role("body"))
        .stable_node_id(UiNodeId::new("target"))
        .host_action(UiHostActionSpec::command("target", "Target"));
    let tree = UiTree::new(
        UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                viewport_width: AREA_WIDTH as u32,
                viewport_height: VIEWPORT_HEIGHT,
                content_width: AREA_WIDTH as u32,
                content_height: VIEWPORT_HEIGHT * 2,
                ..UiScrollAreaProps::default()
            })
            .child(
                UiNode::new(UiNodeKind::Column, "").child(target).child(
                    UiNode::from(Text::new("Following").text_role("body"))
                        .stable_node_id(UiNodeId::new("following"))
                        .host_action(UiHostActionSpec::command("following", "Following")),
                ),
            ),
    );
    if hovered {
        tree.with_hover_surface_for_node_id(Some(&UiNodeId::new("target")))
    } else {
        tree
    }
}

fn render_area(scroll_y: f32) -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: AREA_WIDTH,
        height: AREA_HEIGHT,
        scroll_y,
    }
}

fn has_action(hits: &[crate::raster_host::UiTreeHostActionHit], action_id: &str) -> bool {
    hits.iter().any(|hit| hit.action.action_id == action_id)
}

fn action_rect(
    hits: &[crate::raster_host::UiTreeHostActionHit],
    action_id: &str,
) -> crate::raster_host::UiTreeHitRect {
    hits.iter()
        .find(|hit| hit.action.action_id == action_id)
        .expect("visible host action hit")
        .rect
}
