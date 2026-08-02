use super::StorybookWindowState;
use crate::visual::preview_detail;

const PAGE: &str = "settings-list";
const QUERY_PRESET_INDEX: usize = 4;
const RESET_PRESET_INDEX: usize = 5;
const DEFAULT_COLLAPSED_PRESET_INDEX: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum SettingsListStoryAction {
    UpdateField,
    SetQuery,
    ToggleSection,
    ResetField,
    FocusField,
    HoverField,
    KeyboardNext,
    Scroll,
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<SettingsListStoryAction> {
    if state.selected_page != PAGE {
        return None;
    }
    if !preview_detail::component_action_hit_rect(PAGE).contains(x, y) {
        return None;
    }
    Some(match state.preset_index {
        QUERY_PRESET_INDEX => SettingsListStoryAction::SetQuery,
        RESET_PRESET_INDEX => SettingsListStoryAction::ResetField,
        DEFAULT_COLLAPSED_PRESET_INDEX => SettingsListStoryAction::ToggleSection,
        _ => SettingsListStoryAction::UpdateField,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_maps_query_reset_and_collapsed_presets() {
        let rect = preview_detail::component_action_hit_rect(PAGE);
        for (preset_index, expected) in [
            (QUERY_PRESET_INDEX, SettingsListStoryAction::SetQuery),
            (RESET_PRESET_INDEX, SettingsListStoryAction::ResetField),
            (
                DEFAULT_COLLAPSED_PRESET_INDEX,
                SettingsListStoryAction::ToggleSection,
            ),
        ] {
            let state = StorybookWindowState {
                selected_page: PAGE,
                preset_index,
                ..StorybookWindowState::default()
            };
            assert_eq!(Some(expected), operation_at(&state, rect.x, rect.y));
        }
    }
}
