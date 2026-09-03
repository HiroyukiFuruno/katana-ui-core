use super::Canvas;

const BACKGROUND: u32 = 0x000000;
const SELECTION: u32 = 0xff0000;

#[test]
fn canvas_exposes_copy_payload_for_selected_text_runs() {
    let mut canvas = Canvas::new(320, 200, BACKGROUND);
    canvas.record_text_run("Heading", 24, 32, 80, 20);
    canvas.record_text_run("Body text", 24, 64, 120, 20);

    assert_eq!(
        Some("Heading".to_string()),
        canvas.copy_text_in_selection(Some((24, 42)), Some((104, 42)))
    );
    assert_eq!(
        Some("Heading\nBody text".to_string()),
        canvas.copy_text_in_selection(Some((20, 28)), Some((180, 90)))
    );
}

#[test]
fn canvas_exposes_selection_highlight_for_selected_text_runs() {
    let mut canvas = Canvas::new(320, 200, BACKGROUND);
    canvas.record_text_run("Heading", 24, 32, 80, 20);

    assert!(canvas.draw_text_selection_highlight(Some((24, 42)), Some((104, 42)), SELECTION));
    assert_ne!(Some(BACKGROUND), pixel_at(&canvas, 24, 32));
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
