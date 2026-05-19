use super::super::{StorybookWindowState, apply_click};
use crate::requirements::StoryRequirements;
use crate::visual::button_options::{StorybookButtonOptionControl, control_rect, is_button_page};
use crate::visual::interaction_spec::StorybookInteractionSpec;
use crate::visual::{layout_metrics, preview_detail};

#[test]
fn every_required_page_has_screen_action_and_settings_paths() {
    for page in StoryRequirements::required_pages() {
        assert_required_page_has_screen_action_and_setting_path(page);
    }
}

fn assert_required_page_has_screen_action_and_setting_path(page: &'static str) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect(page);

    assert!(target.width > 0, "{page} lacks preview action target");
    assert!(
        apply_click(&mut state, target.x + 1, target.y + 1),
        "{page} preview action did not mutate state"
    );
    assert_eq!(1, state.screen_state.action_count, "{page} action count");
    let spec = StorybookInteractionSpec::for_page(page);
    assert_eq!(spec.action, state.screen_state.last_action, "{page} action");
    assert_eq!(spec.event, state.screen_state.last_event, "{page} event");
    assert_eq!(spec.state, state.screen_state.state_label, "{page} state");

    let setting = setting_target_for_page(page);
    assert!(
        apply_click(&mut state, setting.x + 1, setting.y + 1),
        "{page} setting click did not mutate state"
    );
    assert_settings_result(page, spec, state);
}

fn setting_target_for_page(page: &str) -> layout_metrics::LayoutRect {
    if is_button_page(page) {
        return control_rect(StorybookButtonOptionControl::Label);
    }
    layout_metrics::button_setting_hit_rect()
}

fn assert_settings_result(page: &str, spec: StorybookInteractionSpec, state: StorybookWindowState) {
    assert_eq!(1, state.screen_state.settings_revision, "{page} setting");
    if is_button_page(page) {
        assert_eq!(
            "label", state.screen_state.last_setting,
            "{page} setting option"
        );
        assert_eq!(
            "保存する", state.screen_state.last_setting_value,
            "{page} setting value"
        );
        return;
    }
    assert_eq!(
        spec.option, state.screen_state.last_setting,
        "{page} setting option"
    );
    assert_eq!(
        spec.after, state.screen_state.last_setting_value,
        "{page} setting value"
    );
}
