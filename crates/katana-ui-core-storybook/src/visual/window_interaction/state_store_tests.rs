use super::state_store::{DEFAULT_INSTANCE_ID, StorybookScreenStateStore};
use crate::requirements::StoryRequirements;
use crate::visual::screen_state::StorybookScreenState;

#[test]
fn every_required_page_keeps_screen_state_instances_separate() {
    for &page in StoryRequirements::required_pages() {
        let mut store = StorybookScreenStateStore::default();
        let primary = screen_state("primary_action", "primary_event", "primary_state");
        let secondary = screen_state("secondary_action", "secondary_event", "secondary_state");

        store.save_instance(page, 0, "primary", primary.clone());
        store.save_instance(page, 0, "secondary", secondary.clone());

        assert_eq!(
            primary,
            store.restore_instance(page, 0, "primary"),
            "{page} primary instance"
        );
        assert_eq!(
            secondary,
            store.restore_instance(page, 0, "secondary"),
            "{page} secondary instance"
        );
        assert_default_interaction_state(
            store.restore_instance(page, 0, DEFAULT_INSTANCE_ID),
            page,
        );
    }
}

#[test]
fn screen_state_store_removes_default_instance_key_only_for_required_pages() {
    for &page in StoryRequirements::required_pages() {
        let mut store = StorybookScreenStateStore::default();
        let primary = screen_state("primary_action", "primary_event", "primary_state");
        let secondary = screen_state("secondary_action", "secondary_event", "secondary_state");

        store.save_instance(page, 0, "primary", primary);
        store.save_instance(page, 0, "secondary", secondary.clone());
        store.save_instance(page, 0, "primary", StorybookScreenState::default());

        assert_default_interaction_state(store.restore_instance(page, 0, "primary"), page);
        assert_eq!(
            secondary,
            store.restore_instance(page, 0, "secondary"),
            "{page} secondary instance"
        );
    }
}

fn assert_default_interaction_state(state: StorybookScreenState, page: &str) {
    assert_eq!(0, state.action_count, "{page} action count");
    assert_eq!(0, state.settings_revision, "{page} settings revision");
    assert_eq!("none", state.last_action, "{page} action");
    assert_eq!("none", state.last_event, "{page} event");
    assert_eq!("idle", state.state_label, "{page} state");
}

fn screen_state(
    last_action: &'static str,
    last_event: &'static str,
    state_label: &'static str,
) -> StorybookScreenState {
    StorybookScreenState {
        action_count: 1,
        last_action,
        last_event,
        state_label,
        ..Default::default()
    }
}
