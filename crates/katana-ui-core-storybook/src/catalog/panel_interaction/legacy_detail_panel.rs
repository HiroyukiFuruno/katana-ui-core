use crate::catalog::StoryExample;

pub(super) fn state_line(example: &StoryExample, marker: &str) -> String {
    let props = example.tree.root().props();
    format!(
        "{marker} state: id={} root_y={} viewport={} content={} children={}",
        props.state_id.as_str(),
        props.panel.scroll_y,
        props.panel.viewport_height,
        props.panel.content_height,
        example.tree.root().children().len()
    )
}

pub(super) fn event_line(marker: &str) -> String {
    format!("{marker} event: wheel_y wheel_x scrollbar_drag visibility_toggle")
}

pub(super) fn action_line(marker: &str) -> String {
    format!("{marker} action: scroll_preview_y scroll_preview_x toggle_scrollbar")
}

pub(super) fn quality_line(marker: &str) -> String {
    format!("{marker} quality: nested_state_identity axis_isolation scrollbar_toggle clip")
}
