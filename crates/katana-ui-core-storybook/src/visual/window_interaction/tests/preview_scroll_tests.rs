use super::super::{StorybookWindowState, apply_click, click_content_y};
use crate::visual::panel_scroll_state::PanelScrollOffsets;
use crate::visual::panel_scroll_state::PanelScrollRegion;
use crate::visual::{layout_metrics, preview_detail};

#[test]
fn viewport_click_mapping_keeps_preview_actions_aligned_after_nested_scroll() {
    let mut state = StorybookWindowState {
        panel_scroll: PanelScrollOffsets {
            root_y: layout_metrics::SCROLL_STEP,
            preview_y: layout_metrics::SCROLL_STEP,
            ..PanelScrollOffsets::default()
        },
        scroll_y: layout_metrics::SCROLL_STEP,
        ..StorybookWindowState::default()
    };
    let target = preview_detail::button_action_hit_rect("button");
    let max_preview_y = crate::visual::panel_scroll_state::max_scroll_y_for(
        PanelScrollRegion::Preview,
        state.selected_page,
        state.tree_expansion,
    );
    let visible_y = target.y
        - state.panel_scroll.root_y
        - state
            .panel_scroll
            .offset_with_max(PanelScrollRegion::Preview, max_preview_y);
    let content_y = click_content_y(&state, target.x + 1, visible_y + 1);

    assert!(apply_click(&mut state, target.x + 1, content_y));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("button_press", state.screen_state.last_action);
    assert_eq!("button_clicked", state.screen_state.last_event);
}
