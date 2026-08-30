use super::adapter::EguiSourceAddressStripAdapter;
use super::interaction::Interaction;
use super::raster::Raster;
use super::types::{
    EguiSourceAddressStripError, EguiSourceAddressStripOutput, SourceAddressFrameEventClass,
    SourceAddressPaintPlan, SourceAddressRenderStyle,
};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressEvent, SourceAddressStrip,
};

impl EguiSourceAddressStripOutput {
    pub(super) fn record(&mut self, event: SourceAddressEvent) {
        self.event_classes.push(match event {
            SourceAddressEvent::DraftChanged => SourceAddressFrameEventClass::DraftChanged,
            SourceAddressEvent::EnabledChanged => SourceAddressFrameEventClass::EnabledChanged,
            SourceAddressEvent::Focused => SourceAddressFrameEventClass::Focused,
            SourceAddressEvent::Blurred => SourceAddressFrameEventClass::Blurred,
            SourceAddressEvent::HistoryOpened => SourceAddressFrameEventClass::HistoryOpened,
            SourceAddressEvent::HistoryClosed => SourceAddressFrameEventClass::HistoryClosed,
            SourceAddressEvent::CandidatesOpened => SourceAddressFrameEventClass::CandidatesOpened,
            SourceAddressEvent::CandidatesClosed => SourceAddressFrameEventClass::CandidatesClosed,
            SourceAddressEvent::HistorySelected => SourceAddressFrameEventClass::HistorySelected,
            SourceAddressEvent::CandidateSelected => {
                SourceAddressFrameEventClass::CandidateSelected
            }
            SourceAddressEvent::Submitted(submission) => {
                self.submissions.push(submission);
                SourceAddressFrameEventClass::Submitted
            }
        });
    }

    pub(super) fn record_all(&mut self, events: Vec<SourceAddressEvent>) {
        for event in events {
            self.record(event);
        }
    }
}

pub(super) fn show_entries(
    adapter: &mut EguiSourceAddressStripAdapter,
    paint_plan: &mut SourceAddressPaintPlan,
    ui: &mut egui::Ui,
    strip: &mut SourceAddressStrip,
    history: bool,
    style: &SourceAddressRenderStyle,
) -> Result<Vec<SourceAddressEvent>, EguiSourceAddressStripError> {
    let entries: Vec<_> = if history {
        strip
            .history()
            .iter()
            .map(|entry| {
                (
                    entry.presentation().visible().to_owned(),
                    entry.presentation().tooltip().to_owned(),
                    entry.presentation().accessibility().to_owned(),
                )
            })
            .collect()
    } else {
        strip
            .candidates()
            .iter()
            .map(|entry| {
                (
                    entry.presentation().visible().to_owned(),
                    entry.presentation().tooltip().to_owned(),
                    entry.presentation().accessibility().to_owned(),
                )
            })
            .collect()
    };
    let mut events = Vec::new();
    for (index, (visible, tooltip, accessibility)) in entries.iter().enumerate() {
        let button = Raster::raster_button(
            adapter,
            paint_plan,
            ui,
            visible,
            tooltip,
            strip.enabled(),
            style,
        )?;
        Interaction::publish_button_accessibility(
            ui,
            button.id,
            button.rect,
            accessibility,
            strip.enabled(),
        );
        if Interaction::activated_by_pointer_or_accesskit(ui, &button) {
            let action = if history {
                SourceAddressAction::SelectHistory(index)
            } else {
                SourceAddressAction::SelectCandidate(index)
            };
            if let Some(event) = strip.apply_action(action) {
                events.push(event);
            }
        }
    }
    Ok(events)
}
