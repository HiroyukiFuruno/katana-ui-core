use super::{StorybookVisual, preview_detail};

const DARK_THEME: &str = "dark";
const DEFAULT_PRESET: usize = 0;

#[test]
fn storybook_clicked_counter_is_not_drawn_inside_component_body() {
    for page in [
        "text-input",
        "search-box",
        "select-box",
        "segmented-toggle",
        "color-swatch",
        "tooltip",
        "popover",
        "accordion",
        "split-pane",
        "modal",
        "modal-overlay",
        "color-picker-rgba",
        "code-diff",
        "badge",
        "key-cap",
        "card",
        "toggle",
        "checkbox",
        "radio",
    ] {
        let rendered = StorybookVisual.render_clicked_preset_with_scrollbar(
            DARK_THEME,
            page,
            DEFAULT_PRESET,
            0,
            true,
        );
        let body = preview_detail::component_action_hit_rect(page);
        let overlapping_labels: Vec<_> = rendered
            .text_runs()
            .iter()
            .filter(|run| run.text().starts_with("clicked "))
            .filter(|run| rects_intersect(body, run.rect()))
            .map(|run| run.text().to_string())
            .collect();

        assert!(
            overlapping_labels.is_empty(),
            "{page} must not draw Storybook-only clicked counters inside the component body: {overlapping_labels:?}"
        );
    }
}

fn rects_intersect(
    a: super::layout_metrics::LayoutRect,
    b: super::layout_metrics::LayoutRect,
) -> bool {
    a.x < b.right() && b.x < a.right() && a.y < b.bottom() && b.y < a.bottom()
}
