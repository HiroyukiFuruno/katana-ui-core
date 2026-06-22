use super::{CHECKBOX_PAGE, RADIO_PAGE, StorybookLiveInteractionAuditReport};

impl StorybookLiveInteractionAuditReport {
    #[must_use]
    pub fn summary(&self) -> String {
        let scenario_passed = |page: &str| {
            self.scenarios
                .iter()
                .any(|scenario| scenario.page == page && scenario.passed)
        };
        format!(
            "live_interactions={} passed={} checkbox_changed={} radio_changed={}",
            self.scenarios.len(),
            self.scenarios
                .iter()
                .filter(|scenario| scenario.passed)
                .count(),
            scenario_passed(CHECKBOX_PAGE),
            scenario_passed(RADIO_PAGE)
        )
    }
}
