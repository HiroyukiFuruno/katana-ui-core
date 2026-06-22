mod basic_story;
mod scroll_area_story;
mod split_pane_story;
use super::StoryExample;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        basic_story::row_story(),
        basic_story::column_story(),
        basic_story::stack_story(),
        basic_story::grid_story(),
        scroll_area_story::story(),
        split_pane_story::story(),
        basic_story::align_center_story(),
    ]
}
