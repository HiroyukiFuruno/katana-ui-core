use crate::render_model::UiInteractionState;

pub(super) const DEFAULT_RATIO_PERCENT: u8 = 50;
pub(super) const DEFAULT_MIN_PERCENT: u8 = 10;
pub(super) const DEFAULT_MAX_PERCENT: u8 = 90;
pub(super) const DEFAULT_HANDLE_WIDTH_PX: u8 = 6;
const WHOLE_RATIO: f32 = 1.0;
const PERCENT_SCALE: f32 = 100.0;

pub(super) fn interaction_with_ratio(percent: u8) -> UiInteractionState {
    UiInteractionState {
        value: percent.to_string(),
        ..UiInteractionState::default()
    }
}

pub(super) fn parse_ratio_percent(value: &str) -> Option<u8> {
    let ratio = value.parse::<f32>().ok()?;
    let percent = if ratio <= WHOLE_RATIO {
        ratio * PERCENT_SCALE
    } else {
        ratio
    };
    Some(percent.round().clamp(0.0, PERCENT_SCALE) as u8)
}
