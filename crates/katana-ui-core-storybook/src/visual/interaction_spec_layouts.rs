use super::{StorybookInteractionSpec, spec};

pub(super) fn for_page(page: &str) -> Option<StorybookInteractionSpec> {
    match page {
        "panel" => Some(spec(
            "panel_scroll_preview",
            "panel_scroll_changed",
            "panel.scrollbar_visibility",
            "hidden",
            "panel_scroll=advanced",
        )),
        "row" => Some(spec(
            "row_align",
            "layout_changed",
            "layout.align",
            "center",
            "align=center",
        )),
        "column" => Some(spec(
            "column_align",
            "layout_changed",
            "layout.align",
            "center",
            "align=center",
        )),
        "stack" => Some(spec(
            "stack_reorder",
            "z_order_changed",
            "interaction.selected_index",
            "1",
            "z=1",
        )),
        "grid" => Some(spec(
            "grid_select",
            "grid_changed",
            "interaction.selected_index",
            "4",
            "cell=4",
        )),
        "scroll-area" => Some(spec(
            "scroll_to",
            "scroll_changed",
            "interaction.value",
            "80",
            "scroll=80",
        )),
        "split-pane" => Some(spec(
            "split_drag",
            "split_ratio_changed",
            "interaction.value",
            "0.64",
            "ratio=0.64",
        )),
        "align-center" => Some(spec(
            "align_measure",
            "alignment_changed",
            "layout.align",
            "center",
            "centered=true",
        )),
        _ => None,
    }
}
