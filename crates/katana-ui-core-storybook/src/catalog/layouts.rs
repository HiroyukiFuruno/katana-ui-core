use super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::{atom, layout, molecule};

const SPLIT_PANE_RESIZE_PERCENT: u8 = 64;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story("row", layout::Row::new().child(atom::Text::new("Row item"))),
        StoryCatalog::story(
            "column",
            layout::Column::new().child(atom::Text::new("Column item")),
        ),
        StoryCatalog::story(
            "stack",
            layout::Stack::new().child(atom::Text::new("Stack item")),
        ),
        StoryCatalog::story(
            "grid",
            layout::Grid::new()
                .child(atom::Text::new("Grid item"))
                .child(atom::Text::new("Grid item 2")),
        ),
        StoryCatalog::story(
            "scroll-area",
            layout::ScrollArea::new().child(atom::Text::new("Scroll item")),
        ),
        split_pane_story(),
        StoryCatalog::story(
            "align-center",
            layout::AlignCenter::new().child(atom::Text::new("Centered")),
        ),
        StoryCatalog::story(
            "theme-tokens",
            molecule::Card::new("Theme tokens")
                .child(atom::Badge::new("Light/Dark"))
                .child(atom::ColorSwatch::new("Accent")),
        ),
    ]
}

fn split_pane_story() -> StoryExample {
    let mut split = layout::SplitPane::new()
        .value("0.5")
        .child(atom::Text::new("Left"))
        .child(atom::Text::new("Right"));
    let target = split.state_id().clone();
    let result = split.apply_action(&UiAction::split_pane_resized(
        target,
        SPLIT_PANE_RESIZE_PERCENT,
    ));
    StoryCatalog::interactive_story("split-pane", split, result.callback_log)
}
