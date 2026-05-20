#[path = "startup_state_panel_story.rs"]
mod startup_state_panel_story;
#[path = "window_control_story.rs"]
mod window_control_story;

use super::super::StoryExample;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        window_control_story::example(),
        startup_state_panel_story::example(),
    ]
}
