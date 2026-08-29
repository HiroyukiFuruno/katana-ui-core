use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::{TabsScreenState, TabsScreenUpdate, tabs_update};
use katana_ui_core::widget::molecules::{CloseableTabId, CloseableTabStripAction};

const EXPANDED_OVERFLOW_TRIGGER_WIDTH: u16 = 72;
const EXPANDED_GROUP_AUTO_EXPAND_MS: u16 = 1000;
const GROUP_COLOR_PRESET_COLOR: u32 = 0x5aa65a;

impl TabsScreenState {
    pub(in crate::visual) fn apply_contract_option(
        &mut self,
        setting: &'static str,
    ) -> Option<TabsScreenUpdate> {
        match setting {
            "tabs.icon" => self.active_tab_mut()?.icon_visible = true,
            "tabs.dirty" => self.active_tab_mut()?.dirty = true,
            "tabs.closeable" => self.active_tab_mut()?.closeable = false,
            "tabs.tone" => self.active_tab_mut()?.tone = "warning",
            "tabs.tooltip" => {
                self.active_tab_mut()?.tooltip = Some("Open scratch buffer".to_string());
            }
            "tabs.accessibility_label" => {
                self.active_tab_mut()?.accessibility_label =
                    Some("Scratch tab with unsaved draft".to_string());
            }
            "tabs.group_color" => self.groups.first_mut()?.color = GROUP_COLOR_PRESET_COLOR,
            "tabs.group_collapsed" => self.groups.first_mut()?.collapsed = true,
            "tabs.active_scroll" => return Some(self.apply_active_scroll_follow()),
            "tabs.overflow_width" => self.overflow_trigger_width = EXPANDED_OVERFLOW_TRIGGER_WIDTH,
            "tabs.group_auto_expand" => {
                self.collapsed_group_auto_expand_ms = EXPANDED_GROUP_AUTO_EXPAND_MS;
            }
            _ => return None,
        }
        Some(tabs_update(
            "tab_option_changed",
            "closeable_tab_option_changed",
            setting,
            value_for_setting(setting),
            state_for_setting(setting),
        ))
    }

    fn apply_active_scroll_follow(&mut self) -> TabsScreenUpdate {
        self.add_many_for_overflow();
        let events = self.apply_core_tab_action(CloseableTabStripAction::SelectTab {
            tab_id: CloseableTabId::new("theme.rs"),
        });
        self.scroll_x = super::dedicated_tabs_scroll::scroll_x(self);
        tabs_update(
            "select_tab_active_follow",
            core_event_name(&events, "closeable_tab_select_missing"),
            "tabs.active_scroll",
            "follow",
            "tabs.active_scroll=follow",
        )
    }

    fn active_tab_mut(&mut self) -> Option<&mut super::screen_state_tabs_types::TabsScreenTab> {
        self.tabs
            .iter_mut()
            .find(|tab| tab.id == self.active_tab_id)
    }
}

fn value_for_setting(setting: &str) -> &'static str {
    match setting {
        "tabs.icon" => "svg",
        "tabs.dirty" => "true",
        "tabs.closeable" => "false",
        "tabs.tone" => "warning",
        "tabs.tooltip" => "visible",
        "tabs.accessibility_label" => "custom",
        "tabs.group_color" => "accent",
        "tabs.group_collapsed" => "true",
        "tabs.active_scroll" => "follow",
        "tabs.overflow_width" => "72",
        "tabs.group_auto_expand" => "1000",
        _ => "changed",
    }
}

fn state_for_setting(setting: &str) -> &'static str {
    match setting {
        "tabs.icon" => "tabs.icon=svg",
        "tabs.dirty" => "tabs.dirty=true",
        "tabs.closeable" => "tabs.closeable=false",
        "tabs.tone" => "tabs.tone=warning",
        "tabs.tooltip" => "tabs.tooltip=visible",
        "tabs.accessibility_label" => "tabs.a11y=custom",
        "tabs.group_color" => "tabs.group_color=accent",
        "tabs.group_collapsed" => "tabs.group=collapsed",
        "tabs.active_scroll" => "tabs.active_scroll=follow",
        "tabs.overflow_width" => "tabs.overflow_width=72",
        "tabs.group_auto_expand" => "tabs.group_auto_expand=1000",
        _ => "changed",
    }
}

#[cfg(test)]
mod tests {
    use super::{state_for_setting, value_for_setting};

    #[test]
    fn unknown_tab_setting_uses_changed_fallbacks() {
        assert_eq!("changed", value_for_setting("unknown"));
        assert_eq!("changed", state_for_setting("unknown"));
    }
}
