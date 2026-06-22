use super::visual_interaction_test_support::require_some;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_tabs, layout_metrics, storybook_ui_option_contract};
use katana_ui_core::widget::molecules::{CloseableTabStripAction, MeasuredCloseableTab};

const PAGE: &str = "tabs";
const CORE_PLAN_WIDTH: u16 = 170;
const DEFAULT_OVERFLOW_TRIGGER_WIDTH: u16 = 44;
const EXPANDED_OVERFLOW_TRIGGER_WIDTH: u16 = 72;
const DEFAULT_GROUP_AUTO_EXPAND_MS: u16 = 500;
const EXPANDED_GROUP_AUTO_EXPAND_MS: u16 = 1000;
const EARLY_GROUP_EXPAND_MS: u16 = 999;
const MEASURED_README_WIDTH: u16 = 80;
const MEASURED_EDITOR_WIDTH: u16 = 60;
const MEASURED_PREVIEW_WIDTH: u16 = 40;

#[test]
fn tabs_inspector_options_mutate_tab_model_state() -> Result<(), String> {
    for option in storybook_ui_option_contract::options_for_page(PAGE) {
        let mut state = tabs_state();
        let index = option_index(option.setting)?;
        let row = layout_metrics::inspector_setting_row_hit_rect(index);

        assert!(
            apply_click(&mut state, row.x + 1, row.y + 1),
            "tabs option `{}` was not clickable",
            option.setting
        );
        assert_tabs_option_state(option.setting, &state)?;
        assert_tabs_option_event(&state);
        assert_eq!(option.setting, state.screen_state.last_setting);
        assert_eq!(option.after, state.screen_state.last_setting_value);
        assert_eq!(
            expected_state_for_setting(option.setting)?,
            state.screen_state.state_label
        );
    }
    Ok(())
}

fn assert_tabs_option_event(state: &StorybookWindowState) {
    assert_ne!("none", state.screen_state.last_action);
    assert!(
        state.screen_state.last_event.starts_with("closeable_tab"),
        "tabs option must emit a closeable tab event, got `{}`",
        state.screen_state.last_event
    );
}

fn tabs_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn option_index(setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(PAGE)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing tabs option `{setting}`"))
}

fn assert_tabs_option_state(setting: &str, state: &StorybookWindowState) -> Result<(), String> {
    let active = active_tab(state)?;
    match setting {
        "tabs.add" => assert!(
            state
                .screen_state
                .tabs
                .tabs
                .iter()
                .any(|tab| tab.id == "notes.md")
        ),
        "tabs.close" => assert!(
            !state
                .screen_state
                .tabs
                .tabs
                .iter()
                .any(|tab| tab.id == "scratch.md")
        ),
        "tabs.pin" => assert!(active.pinned),
        "tabs.move" => assert!(
            tab_order(state).iter().position(|id| *id == "scratch.md")
                > tab_order(state).iter().position(|id| *id == "terminal")
        ),
        "tabs.group" => assert_eq!(Some("docs"), active.group_id.as_deref()),
        "tabs.overflow" => assert!(state.screen_state.tabs.overflow_open),
        "tabs.active_scroll" => assert_active_scroll_follow(state),
        "tabs.icon" => assert!(active.icon_visible),
        "tabs.dirty" => assert!(active.dirty),
        "tabs.closeable" => assert!(!active.closeable),
        "tabs.tone" => assert_eq!("warning", active.tone),
        "tabs.tooltip" => assert_eq!(Some("Open scratch buffer"), active.tooltip.as_deref()),
        "tabs.accessibility_label" => assert_eq!(
            Some("Scratch tab with unsaved draft"),
            active.accessibility_label.as_deref()
        ),
        "tabs.group_color" => assert_eq!(0x5aa65a, state.screen_state.tabs.groups[0].color),
        "tabs.group_collapsed" => assert!(state.screen_state.tabs.groups[0].collapsed),
        "tabs.overflow_width" => assert_core_overflow_width(state),
        "tabs.group_auto_expand" => assert_core_group_auto_expand_delay(state),
        _ => return Err(format!("unhandled tabs option `{setting}`")),
    }
    assert_default_tab_settings(setting, state);
    Ok(())
}

fn assert_active_scroll_follow(state: &StorybookWindowState) {
    assert_eq!("theme.rs", state.screen_state.tabs.active_tab_id);
    assert_eq!("select_tab_active_follow", state.screen_state.last_action);
    assert_eq!("closeable_tab_selected", state.screen_state.last_event);
    assert!(state.screen_state.tabs.scroll_x > 0);
}

fn assert_core_overflow_width(state: &StorybookWindowState) {
    let measured_tabs = [
        MeasuredCloseableTab::new("readme.md", MEASURED_README_WIDTH),
        MeasuredCloseableTab::new("editor.rs", MEASURED_EDITOR_WIDTH),
        MeasuredCloseableTab::new("preview.rs", MEASURED_PREVIEW_WIDTH),
    ];
    let plan = state
        .screen_state
        .tabs
        .core_overflow_plan_for_test(CORE_PLAN_WIDTH, &measured_tabs);

    assert_eq!(
        EXPANDED_OVERFLOW_TRIGGER_WIDTH,
        state.screen_state.tabs.overflow_trigger_width
    );
    assert_eq!(vec!["readme.md"], tab_ids(&plan.visible_tab_ids));
    assert!(plan.overflow_visible);
}

fn tab_ids(ids: &[katana_ui_core::widget::molecules::CloseableTabId]) -> Vec<&str> {
    ids.iter().map(|id| id.as_str()).collect()
}

fn assert_core_group_auto_expand_delay(state: &StorybookWindowState) {
    let mut tabs = state.screen_state.tabs.clone();
    tabs.groups[0].collapsed = true;

    assert_eq!(
        EXPANDED_GROUP_AUTO_EXPAND_MS,
        state.screen_state.tabs.collapsed_group_auto_expand_ms
    );

    let early = tabs.apply_core_tab_action(CloseableTabStripAction::HoverCollapsedGroupForDrop {
        group_id: "docs".into(),
        elapsed_ms: EARLY_GROUP_EXPAND_MS,
    });
    let expanded =
        tabs.apply_core_tab_action(CloseableTabStripAction::HoverCollapsedGroupForDrop {
            group_id: "docs".into(),
            elapsed_ms: EXPANDED_GROUP_AUTO_EXPAND_MS,
        });

    assert!(early.is_empty());
    assert_eq!("closeable_tab_group_collapse_changed", expanded[0].name());
}

fn expected_state_for_setting(setting: &str) -> Result<&'static str, String> {
    let state = match setting {
        "tabs.add" => "tabs.count=6 active=notes.md",
        "tabs.close" => "tabs.count=5 active=scratch.md",
        "tabs.pin" => "tabs.pinned=true left-fixed",
        "tabs.move" => "tabs.order=changed",
        "tabs.group" => "tabs.group=Docs",
        "tabs.overflow" => "tabs.overflow=menu",
        "tabs.active_scroll" => "tabs.active_scroll=follow",
        "tabs.icon" => "tabs.icon=svg",
        "tabs.dirty" => "tabs.dirty=true",
        "tabs.closeable" => "tabs.closeable=false",
        "tabs.tone" => "tabs.tone=warning",
        "tabs.tooltip" => "tabs.tooltip=visible",
        "tabs.accessibility_label" => "tabs.a11y=custom",
        "tabs.group_color" => "tabs.group_color=accent",
        "tabs.group_collapsed" => "tabs.group=collapsed",
        "tabs.overflow_width" => "tabs.overflow_width=72",
        "tabs.group_auto_expand" => "tabs.group_auto_expand=1000",
        _ => {
            return Err(format!(
                "missing state expectation for tabs option `{setting}`"
            ));
        }
    };
    Ok(state)
}

fn assert_default_tab_settings(setting: &str, state: &StorybookWindowState) {
    if setting != "tabs.overflow_width" {
        assert_eq!(
            DEFAULT_OVERFLOW_TRIGGER_WIDTH,
            state.screen_state.tabs.overflow_trigger_width
        );
    }
    if setting != "tabs.group_auto_expand" {
        assert_eq!(
            DEFAULT_GROUP_AUTO_EXPAND_MS,
            state.screen_state.tabs.collapsed_group_auto_expand_ms
        );
    }
}

fn active_tab(
    state: &StorybookWindowState,
) -> Result<&super::screen_state_tabs::TabsScreenTab, String> {
    require_some(state.screen_state.tabs.active_tab(), "active tab exists")
}

fn tab_order(state: &StorybookWindowState) -> Vec<&str> {
    dedicated_tabs::tab_ids_for_test(&state.screen_state.tabs)
}
