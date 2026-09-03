use super::*;
use crate::raster_host::ui_tree_canvas_hit_metrics::NODE_GAP;
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;
use crate::test_assert::KucTestExpect;
use katana_ui_core::render_model::UiTextProps;

#[test]
fn collects_accordion_header_action_rect_with_kuc_cursor() {
    let root = UiNode::from(Accordion::new("Show details").child(Text::new("body")));

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 3,
            y: 5,
            width: 220,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("ui.disclosure.toggle", hits[0].action.action_id);
    assert_eq!(UiCursor::Pointer, hits[0].cursor);
    assert_eq!(
        UiTreeHitRect {
            x: 3,
            y: 5,
            width: 220,
            height: 20,
        },
        hits[0].rect
    );
}

#[test]
fn open_accordion_collects_indented_and_html_aligned_child_hits() {
    for (role, expected_x) in [("", 19), ("html-accordion", 3)] {
        let root = UiNode::from(Accordion::new("Show details").open(true).child(
            UiNode::from(Text::new("body")).host_action(UiHostActionSpec::command("body", "Body")),
        ))
        .text(UiTextProps {
            role: role.to_string(),
            ..UiTextProps::default()
        });

        let hits = UiTreeHostActionHitCollector::collect(
            &root,
            UiTreeRenderArea {
                x: 3,
                y: 5,
                width: 220,
                height: 120,
                scroll_y: 0.0,
            },
        );

        assert!(
            hits.iter()
                .any(|hit| hit.action.action_id == "body" && hit.rect.x == expected_x),
            "{role}"
        );
    }
}

#[test]
fn unframed_image_hit_uses_natural_target_size_and_gap_advance() {
    let image: UiNode =
        ImageSurface::from_rgba("image", "hit-natural", 2, 2, [255, 0, 0, 255].repeat(4))
            .kuc_expect("valid image")
            .into();
    let image_common = image
        .props()
        .common
        .clone()
        .semantic_node_id("image-semantic");
    let image = image.common(image_common);
    let root = UiNode::new(UiNodeKind::Column, "").child(image).child(
        UiNode::from(Button::new("after"))
            .host_action(UiHostActionSpec::command("after", "After image")),
    );

    let area = UiTreeRenderArea {
        x: 7,
        y: 11,
        width: 80,
        height: 80,
        scroll_y: 0.0,
    };
    let hits = UiTreeHostActionHitCollector::collect(&root, area);
    let node_hits = UiTreeHostActionHitCollector::collect_node_hits_with_renderers(
        &root,
        area,
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "code"),
        UiTreeDocumentTypography::default(),
    );
    let image_hit = node_hits
        .iter()
        .find(|hit| {
            hit.semantic_node_id
                .as_ref()
                .is_some_and(|id| id.as_str() == "image-semantic")
        })
        .kuc_expect("image hit");
    let button_hit = hits
        .iter()
        .find(|hit| hit.action.action_id == "after")
        .kuc_expect("button hit");

    assert_eq!(7, image_hit.rect.x);
    assert_eq!(11, image_hit.rect.y);
    assert_eq!(2, image_hit.rect.width);
    assert_eq!(2, image_hit.rect.height);
    assert_eq!(11 + 2 + NODE_GAP, button_hit.rect.y);
}

#[test]
fn collects_checkbox_action_rect_with_kuc_cursor() {
    let root = UiNode::from(Checkbox::new("").value("[ ]"))
        .host_action(UiHostActionSpec::command("ui.task.toggle", "Toggle task").payload("list:0"));

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 9,
            y: 13,
            width: 160,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("ui.task.toggle", hits[0].action.action_id);
    assert_eq!(UiCursor::Pointer, hits[0].cursor);
    assert_eq!(
        UiTreeHitRect {
            x: 9,
            y: 13,
            width: 16,
            height: 20,
        },
        hits[0].rect
    );
}

#[test]
fn collects_task_row_action_rect_without_expanding_checkbox_hit() {
    let checkbox = UiNode::from(Checkbox::new("").value("[ ]"))
        .host_action(UiHostActionSpec::task_control("Toggle task", "list", 0));
    let row: UiNode = UiNode::from(
        Row::new()
            .value("[ ]")
            .child(checkbox)
            .child(Text::new("Pending task")),
    )
    .host_action(UiHostActionSpec::task_control("Toggle task", "list", 0));

    let hits = UiTreeHostActionHitCollector::collect(
        &row,
        UiTreeRenderArea {
            x: 9,
            y: 13,
            width: 160,
            height: 80,
            scroll_y: 0.0,
        },
    );
    let checkbox_hit = hits
        .iter()
        .find(|hit| hit.rect.width == 16)
        .kuc_expect("checkbox hit must stay on rendered checkbox");
    let row_hit = hits
        .iter()
        .find(|hit| hit.rect.width > 80)
        .kuc_expect("row body hit must be represented by row action");
    let row_action = row_hit
        .action
        .task_control_action_from_root(&row)
        .kuc_expect("row action must keep task marker contract");

    assert_eq!(checkbox_hit.action.action_id, row_hit.action.action_id);
    assert_eq!("[ ]", row_action.current_marker.marker());
    assert_eq!(0, row_action.row_index);
}

#[test]
fn collects_toggle_action_rect_from_rendered_switch_track() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let root = UiNode::from(Toggle::new("Dark").checked(true))
        .host_action(UiHostActionSpec::command("ui.toggle.dark", "Toggle dark"));
    let area = UiTreeRenderArea {
        x: 9,
        y: 13,
        width: 160,
        height: 80,
        scroll_y: 0.0,
    };
    let hits = UiTreeHostActionHitCollector::collect(&root, area);

    assert_eq!(1, hits.len());
    assert_eq!("ui.toggle.dark", hits[0].action.action_id);
    assert_eq!(UiCursor::Pointer, hits[0].cursor);
    assert_eq!(
        UiTreeHitRect {
            x: 9,
            y: 13,
            width: 48,
            height: 22,
        },
        hits[0].rect
    );

    let mut canvas = Canvas::new(120, 60, palette.background);
    UiTreeCanvasRenderer::new(theme).render(&mut canvas, &root, area);
    let track_center =
        (hits[0].rect.y + hits[0].rect.height / 2) * canvas.width() + hits[0].rect.x + 8;
    assert_eq!(palette.visual.accent, canvas.pixels()[track_center]);
}
