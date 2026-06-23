use super::{
    ComponentAction, RowHeightProvider, StoryCatalog, StoryExample, UiAction, UiCallbackLog,
    UiStateId, VIRTUAL_FOCUSED_INDEX, VIRTUAL_ROW_HEIGHT, VIRTUAL_SCROLL_OFFSET,
    VIRTUAL_TOTAL_ROWS, VIRTUAL_VIEWPORT_HEIGHT, VirtualizationConfig, molecule,
};

pub(super) fn virtualization_story() -> StoryExample {
    let mut list = molecule::VirtualizedList::new("Virtualized list", virtualization_config());
    let target = list.state_id().clone();
    let scroll = list.apply_action(&UiAction::set_value(
        target.clone(),
        VIRTUAL_SCROLL_OFFSET.to_string(),
    ));
    let focus = list.apply_action(&UiAction::set_selected_index(target, VIRTUAL_FOCUSED_INDEX));
    let logs = vec![
        UiCallbackLog::new(
            UiStateId::new("state:VirtualizedList:storybook"),
            "virtualized_scroll",
            "offset=0",
            format!("events={:?}", scroll.callback_log),
        ),
        UiCallbackLog::new(
            UiStateId::new("state:VirtualizedList:storybook"),
            "virtualized_focus_keep",
            "focused=None",
            format!("events={:?}", focus.callback_log),
        ),
    ];
    StoryCatalog::interactive_story("virtualization", list, logs)
}

fn virtualization_config() -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count: VIRTUAL_TOTAL_ROWS,
        viewport_offset: 0,
        viewport_height: VIRTUAL_VIEWPORT_HEIGHT,
        overscan: 2,
        row_height_provider: RowHeightProvider::Fixed {
            height: VIRTUAL_ROW_HEIGHT,
        },
        keep_focused_in_window: true,
        focused_index: None,
    }
}
