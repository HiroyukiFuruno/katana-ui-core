use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiNode, UiStateId};
use katana_ui_core::{atom, layout};

const FLEX_GAP_PX: f32 = 8.0;
const GRID_GAP_PX: f32 = 12.0;

pub(super) fn row_story() -> StoryExample {
    let row = layout::Row::new()
        .axis(layout::LayoutAxis::Horizontal)
        .gap(layout::Length::px(FLEX_GAP_PX))
        .align(layout::Alignment::Start)
        .overflow(layout::OverflowBehavior::Fit)
        .value("alignment=start")
        .child(atom::Text::new("Row item"))
        .child(atom::Text::new("Row item 2"))
        .child(atom::Text::new(
            "settings: axis=Horizontal gap=8 overflow=fit",
        ))
        .child(atom::Text::new("state: interaction.value=alignment=start"))
        .child(atom::Text::new("event: layout_changed"))
        .child(atom::Text::new("action: row_align"));
    let target = UiNode::from(row.clone()).props().state_id.clone();
    let logs = vec![layout_action_log(
        &target,
        "row_align",
        "event=layout_ready alignment=start",
        "event=layout_changed alignment=center",
    )];

    StoryCatalog::interactive_story("row", row, logs)
}

pub(super) fn column_story() -> StoryExample {
    let column = layout::Column::new()
        .axis(layout::LayoutAxis::Vertical)
        .gap(layout::Length::px(FLEX_GAP_PX))
        .align(layout::Alignment::Start)
        .overflow(layout::OverflowBehavior::Fit)
        .value("alignment=start")
        .child(atom::Text::new("Column item"))
        .child(atom::Text::new("Column item 2"))
        .child(atom::Text::new(
            "settings: axis=Vertical gap=8 overflow=fit",
        ))
        .child(atom::Text::new("state: interaction.value=alignment=start"))
        .child(atom::Text::new("event: layout_changed"))
        .child(atom::Text::new("action: column_align"));
    let target = UiNode::from(column.clone()).props().state_id.clone();
    let logs = vec![layout_action_log(
        &target,
        "column_align",
        "event=layout_ready alignment=start",
        "event=layout_changed alignment=center",
    )];

    StoryCatalog::interactive_story("column", column, logs)
}

pub(super) fn stack_story() -> StoryExample {
    let stack = layout::Stack::new()
        .align(layout::Alignment::Start)
        .value("z_order=0 selected=0")
        .child(atom::Text::new("Stack item"))
        .child(atom::Text::new("Stack item 2"))
        .child(atom::Text::new("settings: axis=Overlay gap=0 overflow=fit"))
        .child(atom::Text::new(
            "state: interaction.value=z_order=0 selected=0",
        ))
        .child(atom::Text::new("event: z_order_changed"))
        .child(atom::Text::new("action: stack_reorder"));
    let target = UiNode::from(stack.clone()).props().state_id.clone();
    let logs = vec![layout_action_log(
        &target,
        "stack_reorder",
        "event=order_ready z_order=0 selected=0",
        "event=z_order_changed z_order=2 selected=0",
    )];

    StoryCatalog::interactive_story("stack", stack, logs)
}

pub(super) fn grid_story() -> StoryExample {
    let grid = layout::Grid::new()
        .axis(layout::LayoutAxis::Both)
        .gap(layout::Length::px(GRID_GAP_PX))
        .align(layout::Alignment::Start)
        .overflow(layout::OverflowBehavior::Fit)
        .value("selected=0")
        .child(atom::Text::new("Grid item"))
        .child(atom::Text::new("Grid item 2"))
        .child(atom::Text::new("Grid item 3"))
        .child(atom::Text::new("settings: axis=Both gap=12 overflow=fit"))
        .child(atom::Text::new("state: interaction.value=selected=0"))
        .child(atom::Text::new("event: grid_cell_selected"))
        .child(atom::Text::new("action: grid_select"));
    let target = UiNode::from(grid.clone()).props().state_id.clone();
    let logs = vec![layout_action_log(
        &target,
        "grid_select",
        "event=grid_cell_ready selected=0",
        "event=grid_cell_selected selected=1",
    )];

    StoryCatalog::interactive_story("grid", grid, logs)
}

pub(super) fn align_center_story() -> StoryExample {
    let align = layout::AlignCenter::new()
        .value("alignment=center")
        .child(atom::Text::new("Centered"))
        .child(atom::Text::new("event: align_center_applied"));
    let target = UiNode::from(align.clone()).props().state_id.clone();
    let logs = vec![layout_action_log(
        &target,
        "align_center_apply",
        "alignment=start",
        "event=align_center_applied alignment=center",
    )];

    StoryCatalog::interactive_story("align-center", align, logs)
}

fn layout_action_log(
    target: &UiStateId,
    action: &'static str,
    before: &'static str,
    after: &'static str,
) -> UiCallbackLog {
    UiCallbackLog::new(target.clone(), action, before, after)
}
