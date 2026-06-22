use super::screen_state_tabs_types::{TabsScreenGroup, TabsScreenState, TabsScreenTab};
use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabGroup, CloseableTabId, CloseableTabKeyboardInput, CloseableTabStrip,
    CloseableTabStripAction, CloseableTabStripEvent, CloseableTabTone,
};
#[cfg(test)]
use katana_ui_core::widget::molecules::{CloseableTabOverflowPlan, MeasuredCloseableTab};

const HEX_COLOR_RADIX: u32 = 16;
const FALLBACK_GROUP_COLOR: u32 = 0x4ec9b0;

impl TabsScreenState {
    pub(in crate::visual) fn apply_core_tab_action(
        &mut self,
        action: CloseableTabStripAction,
    ) -> Vec<CloseableTabStripEvent> {
        let mut strip = self.to_core_strip();
        let events = strip.apply_action(action);
        self.sync_from_core_strip(&strip);
        events
    }

    pub(in crate::visual) fn apply_core_tab_action_confirming_dirty(
        &mut self,
        action: CloseableTabStripAction,
    ) -> Vec<CloseableTabStripEvent> {
        let mut strip = self.to_core_strip();
        let mut events = strip.apply_action(action);
        for tab_id in close_request_tab_ids(&events) {
            events.extend(strip.apply_action(CloseableTabStripAction::ConfirmClose { tab_id }));
        }
        self.sync_from_core_strip(&strip);
        events
    }

    pub(in crate::visual) fn apply_core_tab_drag_end(
        &mut self,
        tab_id: CloseableTabId,
        committed: bool,
    ) -> Vec<CloseableTabStripEvent> {
        let mut strip = self.to_core_strip();
        let _ = strip.apply_action(CloseableTabStripAction::StartDrag {
            tab_id: tab_id.clone(),
        });
        let events = strip.apply_action(CloseableTabStripAction::EndDrag { committed });
        self.sync_from_core_strip(&strip);
        events
    }

    pub(in crate::visual) fn apply_core_tab_keyboard_input(
        &mut self,
        input: CloseableTabKeyboardInput,
    ) -> Vec<CloseableTabStripEvent> {
        let visible_tab_ids: Vec<CloseableTabId> = self
            .core_visual_tab_ids()
            .into_iter()
            .map(CloseableTabId::new)
            .collect();
        let mut strip = self.to_core_strip();
        let events = strip.apply_keyboard_input(input, &visible_tab_ids);
        self.sync_from_core_strip(&strip);
        events
    }

    fn to_core_strip(&self) -> CloseableTabStrip {
        let mut strip = CloseableTabStrip::new("Storybook tabs")
            .overflow_trigger_width(self.overflow_trigger_width)
            .collapsed_group_auto_expand_ms(self.collapsed_group_auto_expand_ms);
        for group in &self.groups {
            strip = strip.group(
                CloseableTabGroup::new(group.id.as_str(), group.title.as_str())
                    .color(format!("#{:06x}", group.color))
                    .collapsed(group.collapsed),
            );
        }
        for tab in &self.tabs {
            strip = strip.tab(core_tab(tab));
        }
        for tab in &self.recently_closed_tabs {
            strip = strip.recently_closed_tab(core_tab(tab));
        }
        if !self.active_tab_id.is_empty() {
            strip = strip.active_tab_id(self.active_tab_id.as_str());
        }
        strip
    }

    fn sync_from_core_strip(&mut self, strip: &CloseableTabStrip) {
        self.tabs = strip.options().tabs.iter().map(screen_tab).collect();
        self.groups = strip.options().groups.iter().map(screen_group).collect();
        self.active_tab_id = strip
            .state()
            .active_tab_id
            .as_ref()
            .map_or_else(String::new, |tab_id| tab_id.as_str().to_string());
        self.recently_closed_tabs = strip
            .state()
            .recently_closed_tabs
            .iter()
            .map(|closed| screen_tab(&closed.tab))
            .collect();
    }

    pub(in crate::visual) fn core_visual_tab_ids(&self) -> Vec<String> {
        self.to_core_strip()
            .visual_tabs()
            .iter()
            .map(|tab| tab.id.as_str().to_string())
            .collect()
    }

    #[cfg(test)]
    pub(in crate::visual) fn core_overflow_plan_for_test(
        &self,
        available_width: u16,
        measured_tabs: &[MeasuredCloseableTab],
    ) -> CloseableTabOverflowPlan {
        self.to_core_strip()
            .overflow_plan(available_width, measured_tabs)
    }
}

pub(in crate::visual) fn core_event_name(
    events: &[CloseableTabStripEvent],
    fallback: &'static str,
) -> &'static str {
    events.last().map_or(fallback, CloseableTabStripEvent::name)
}

fn close_request_tab_ids(events: &[CloseableTabStripEvent]) -> Vec<CloseableTabId> {
    events
        .iter()
        .filter_map(|event| match event {
            CloseableTabStripEvent::TabCloseRequested { tab_id } => Some(tab_id.clone()),
            _ => None,
        })
        .collect()
}

fn core_tab(tab: &TabsScreenTab) -> CloseableTab {
    let mut core = CloseableTab::new(tab.id.as_str(), tab.title.as_str())
        .pinned(tab.pinned)
        .dirty(tab.dirty)
        .closeable(tab.closeable)
        .groupable(tab.groupable)
        .tone(tab_tone(tab.tone));
    if tab.icon_visible {
        core = core.icon("svg");
    }
    if let Some(tooltip) = tab.tooltip.as_ref() {
        core = core.tooltip(tooltip.as_str());
    }
    if let Some(label) = tab.accessibility_label.as_ref() {
        core = core.accessibility_label(label.as_str());
    }
    if let Some(group_id) = tab.group_id.as_ref() {
        core = core.group_id(group_id.as_str());
    }
    core
}

fn screen_tab(tab: &CloseableTab) -> TabsScreenTab {
    TabsScreenTab {
        id: tab.id.as_str().to_string(),
        title: tab.title.clone(),
        icon_visible: tab.icon.is_some(),
        pinned: tab.pinned,
        dirty: tab.dirty,
        closeable: tab.closeable,
        groupable: tab.groupable,
        tone: tone_label(tab.tone),
        tooltip: tab.tooltip.clone(),
        group_id: tab
            .group_id
            .as_ref()
            .map(|group_id| group_id.as_str().to_string()),
        accessibility_label: tab.accessibility_label.clone(),
    }
}

fn screen_group(group: &CloseableTabGroup) -> TabsScreenGroup {
    TabsScreenGroup {
        id: group.id.as_str().to_string(),
        title: group.label.clone(),
        color: color_for_group(group.id.as_str(), group.color.as_str()),
        collapsed: group.collapsed,
    }
}

fn tab_tone(tone: &str) -> CloseableTabTone {
    match tone {
        "accent" => CloseableTabTone::Accent,
        "warning" => CloseableTabTone::Warning,
        "danger" => CloseableTabTone::Danger,
        "muted" => CloseableTabTone::Muted,
        _ => CloseableTabTone::Default,
    }
}

const fn tone_label(tone: CloseableTabTone) -> &'static str {
    match tone {
        CloseableTabTone::Accent => "accent",
        CloseableTabTone::Warning => "warning",
        CloseableTabTone::Danger => "danger",
        CloseableTabTone::Muted => "muted",
        CloseableTabTone::Default => "default",
    }
}

fn color_for_group(group_id: &str, color: &str) -> u32 {
    if let Some(hex) = color.strip_prefix('#')
        && let Ok(value) = u32::from_str_radix(hex, HEX_COLOR_RADIX)
    {
        return value;
    }
    if group_id == "docs" {
        return super::screen_state_tabs_types::DOCS_GROUP_COLOR;
    }
    FALLBACK_GROUP_COLOR
}
