use super::super::screen_state::StorybookScreenState;
use super::super::storybook_ui_option_contract::StorybookUiOptionContract;
use katana_ui_core::atom::ProgressBar;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::UiNode;

const PROGRESS_DEFAULT_PERCENT: u8 = 65;
const PROGRESS_EMPTY_PERCENT: u8 = 0;
const PROGRESS_STEP_PERCENT: u8 = 17;
const PROGRESS_TICK_TARGET_PERCENT: u8 = 82;
const PROGRESS_MAX_PERCENT: u8 = 99;
const PROGRESS_FULL_PERCENT: u8 = 100;
const PROGRESS_TICK_MS: u16 = 250;

impl StorybookScreenState {
    pub(in crate::visual) fn register_progress_bar_change(&mut self) {
        self.action_count += 1;
        self.apply_core_progress_percent(next_progress_percent(self.progress_percent));
        self.last_action = "progress_change";
        self.last_event = "progress_changed";
        self.state_label = progress_state_label(self.progress_percent);
    }

    pub(in crate::visual) fn register_progress_bar_timed_tick(&mut self, elapsed_ms: u16) {
        self.progress_elapsed_ms = self.progress_elapsed_ms.saturating_add(elapsed_ms);
        let ticks = usize::from(self.progress_elapsed_ms / PROGRESS_TICK_MS);
        if ticks == 0 {
            return;
        }
        self.progress_elapsed_ms %= PROGRESS_TICK_MS;
        self.action_count += ticks;
        for _ in 0..ticks {
            self.apply_core_progress_percent(next_progress_percent(self.progress_percent));
        }
        self.last_action = "progress_tick";
        self.last_event = "progress_changed";
        self.state_label = progress_state_label(self.progress_percent);
    }

    pub(in crate::visual) const fn has_progress_state(&self) -> bool {
        self.progress_changed
    }

    pub(in crate::visual) const fn progress_percent(&self) -> u8 {
        self.progress_percent
    }

    pub(in crate::visual) fn register_progress_bar_contract_setting(
        &mut self,
        option: StorybookUiOptionContract,
    ) -> bool {
        if option.setting != "progress.percent" {
            return false;
        }
        let Some(percent) = parse_progress_percent(option.after) else {
            return false;
        };
        self.settings_revision += 1;
        self.progress_changed = true;
        self.progress_percent = percent;
        self.last_action = "settings_progress_option";
        self.last_event = "atom_settings_changed";
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.state_label = progress_bar_percent_state_label(percent);
        true
    }

    fn apply_core_progress_percent(&mut self, percent: u8) {
        let mut progress =
            ProgressBar::new("Progress").progress(self.progress_changed, self.progress_percent);
        let target = progress.state_id().clone();
        let result = progress.apply_action(&UiAction::progress_changed(target, true, percent));
        assert!(
            result.handled,
            "the Storybook progress action must target its own state"
        );
        let node: UiNode = progress.into();
        self.progress_changed = node.props().determinate;
        self.progress_percent = node.props().progress_percent;
    }
}

fn next_progress_percent(current: u8) -> u8 {
    if current >= PROGRESS_MAX_PERCENT {
        return PROGRESS_EMPTY_PERCENT;
    }
    current
        .saturating_add(PROGRESS_STEP_PERCENT)
        .min(PROGRESS_MAX_PERCENT)
}

const fn progress_state_label(percent: u8) -> &'static str {
    match percent {
        PROGRESS_EMPTY_PERCENT => "percent=0",
        PROGRESS_DEFAULT_PERCENT => "percent=65",
        PROGRESS_TICK_TARGET_PERCENT => "percent=82",
        PROGRESS_MAX_PERCENT => "percent=99",
        _ => "percent=changed",
    }
}

fn parse_progress_percent(value: &str) -> Option<u8> {
    value
        .parse::<u8>()
        .ok()
        .map(|it| it.min(PROGRESS_FULL_PERCENT))
}

const fn progress_bar_percent_state_label(percent: u8) -> &'static str {
    match percent {
        PROGRESS_EMPTY_PERCENT => "progress_bar.percent=0",
        PROGRESS_DEFAULT_PERCENT => "progress_bar.percent=65",
        PROGRESS_TICK_TARGET_PERCENT => "progress_bar.percent=82",
        PROGRESS_MAX_PERCENT => "progress_bar.percent=99",
        PROGRESS_FULL_PERCENT => "progress_bar.percent=100",
        _ => "progress_bar.percent=changed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_boundaries_cover_invalid_settings_wrap_and_all_labels() {
        let mut state = StorybookScreenState::default();
        assert!(
            !state.register_progress_bar_contract_setting(StorybookUiOptionContract {
                setting: "progress.percent",
                before: "65",
                after: "invalid",
            })
        );
        assert_eq!(
            PROGRESS_EMPTY_PERCENT,
            next_progress_percent(PROGRESS_MAX_PERCENT)
        );
        assert_eq!("percent=65", progress_state_label(PROGRESS_DEFAULT_PERCENT));
        assert_eq!("percent=changed", progress_state_label(1));
        assert_eq!(Some(PROGRESS_FULL_PERCENT), parse_progress_percent("255"));
        assert_eq!(None, parse_progress_percent("invalid"));

        for (percent, expected) in [
            (PROGRESS_EMPTY_PERCENT, "progress_bar.percent=0"),
            (PROGRESS_DEFAULT_PERCENT, "progress_bar.percent=65"),
            (PROGRESS_TICK_TARGET_PERCENT, "progress_bar.percent=82"),
            (PROGRESS_MAX_PERCENT, "progress_bar.percent=99"),
            (PROGRESS_FULL_PERCENT, "progress_bar.percent=100"),
            (1, "progress_bar.percent=changed"),
        ] {
            assert_eq!(expected, progress_bar_percent_state_label(percent));
        }
    }
}
