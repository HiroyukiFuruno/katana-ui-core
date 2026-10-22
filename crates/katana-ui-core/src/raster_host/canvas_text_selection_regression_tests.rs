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
fn canvas_copy_separates_independent_siblings_sharing_a_y_coordinate() {
    let mut canvas = Canvas::new(320, 200, BACKGROUND);
    canvas.record_text_run("left sibling", 24, 32, 80, 20);
    canvas.record_text_run("right sibling", 120, 32, 88, 20);

    assert_eq!(
        Some("left sibling\nright sibling".to_string()),
        canvas.copy_text_in_selection(Some((20, 32)), Some((220, 52)))
    );
}

#[test]
fn canvas_copy_restores_explicit_separators_across_an_empty_logical_line() {
    let mut canvas = Canvas::new(320, 200, BACKGROUND);
    canvas.with_text_selection_line_separator("", false, |canvas| {
        canvas.record_text_run("alpha", 24, 32, 40, 20);
    });
    canvas.with_text_selection_line_separator("\r", true, |_canvas| {});
    canvas.with_text_selection_line_separator("\u{2029}", true, |canvas| {
        canvas.record_text_run("beta", 24, 96, 32, 20);
    });

    assert_eq!(
        Some("alpha\r\u{2029}beta".to_owned()),
        canvas.copy_text_in_selection(Some((20, 28)), Some((80, 120)))
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

#[test]
fn canvas_copy_keeps_separator_when_first_logical_fragment_is_unselected_or_clipped() {
    for clipped in [false, true] {
        let mut canvas = Canvas::new(320, 200, BACKGROUND);
        canvas.record_text_run("first", 0, 0, 40, 20);
        let start = canvas.text_runs().len();
        canvas.with_text_selection_line_separator("\r", true, |canvas| {
            canvas.record_text_run("אבג", 200, 40, 30, 20);
            canvas.record_text_run("code", 0, 40, 40, 20);
        });
        if clipped {
            canvas.with_clip(0, 0, 100, 100, &mut |canvas| {
                canvas.discard_text_runs_outside_current_clip_since(start);
            });
        }
        assert_eq!(
            Some("first\rcode".to_owned()),
            canvas.copy_text_in_selection(Some((0, 0)), Some((40, 45))),
        );
    }
}

#[test]
fn canvas_copy_preserves_rendered_empty_endpoint_paragraphs_through_viewport_and_blit() {
    use crate::atom::Text;
    use crate::raster_host::{CanvasBlitRequest, UiTreeCanvasRenderer, UiTreeRenderArea};
    use crate::render_model::UiTextSpan;
    use crate::theme::ThemeSnapshot;

    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 320,
        height: 200,
        scroll_y: 0.0,
    };
    for separator in ["\r", "\n", "\r\n", "\u{2028}", "\u{2029}"] {
        for text in [
            format!("alpha{separator}"),
            format!("{separator}alpha"),
            format!("{separator}alpha{separator}"),
            separator.to_owned(),
        ] {
            for spans in [false, true] {
                let mut root = Text::new(&text).into();
                if spans {
                    root = Text::new(&text)
                        .text_spans(vec![UiTextSpan::plain(&text)])
                        .into();
                }
                let mut source = Canvas::new(320, 200, BACKGROUND);
                renderer.render(&mut source, &root, area);
                let viewport = source.viewport_y(0, 200, BACKGROUND);
                let mut target = Canvas::new(320, 200, BACKGROUND);
                target.blit_canvas(
                    &viewport,
                    CanvasBlitRequest {
                        dest_x: 0,
                        dest_y: 0,
                        width: 320,
                        height: 200,
                        source_y: 0,
                    },
                );
                for canvas in [&source, &viewport, &target] {
                    assert_eq!(
                        Some(text.clone()),
                        canvas.copy_text_in_selection(Some((0, 0)), Some((319, 199))),
                        "spans={spans}, text={text:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn canvas_run_equality_includes_deterministic_separator_metadata() {
    let make = |separator| {
        let mut canvas = Canvas::new(80, 80, BACKGROUND);
        canvas.with_text_selection_line_separator(separator, true, |canvas| {
            canvas.record_text_run("b", 0, 20, 10, 20);
        });
        canvas
    };
    assert_eq!(make("\r"), make("\r"));
    assert_ne!(make("\r"), make("\n"));
    assert_ne!(make("\r").text_runs(), make("\n").text_runs());
}
