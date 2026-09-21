use super::Canvas;
use super::canvas_clip::CanvasClip;
use super::ui_tree_canvas_types::PhysicalCanvasBlitRequest;

const BACKGROUND: u32 = 0x000000;
const FILL: u32 = 0xffffff;

#[test]
fn canvas_edge_contracts_cover_empty_clip_blit_selection_and_viewport() {
    assert!(CanvasClip::from_rect(4, 4, 0, 0, 4, 4).is_none());

    let source = Canvas::new(2, 2, FILL);
    let mut target = Canvas::new(2, 2, BACKGROUND);
    let request = PhysicalCanvasBlitRequest {
        dest_x: 0,
        dest_y: 3,
        dest_logical_x: 0,
        dest_logical_y: 3,
        width: 2,
        height: 1,
        source_y: 0,
        source_logical_y: 0.0,
    };
    assert!(target.copy_unclipped_canvas_row(&source, request, 0, 0));
    let zero_width = PhysicalCanvasBlitRequest {
        dest_y: 0,
        width: 0,
        ..request
    };
    assert!(target.copy_unclipped_canvas_row(&source, zero_width, 0, 0));

    assert_eq!(
        None,
        target.copy_text_in_selection(Some((0, 0)), Some((1, 1)))
    );
    target.record_text_run("", 0, 0, 1, 1);
    assert!(target.text_runs().is_empty());

    let empty = Canvas::new(0, 2, BACKGROUND);
    let viewport = empty.viewport_y(0, 1, FILL);
    assert_eq!(0, viewport.width());
    assert_eq!(1, viewport.logical_height());
}
