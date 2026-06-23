use super::visual_interaction_test_support::require_some;
use super::{dedicated_closeable_tab_strip, screen_state_tabs::TabsScreenState};

const ACTIVE_FOLLOW_PRESET: usize = 6;

#[test]
fn closeable_tab_strip_active_follow_preset_scrolls_current_tab_into_strip() -> Result<(), String> {
    let state = TabsScreenState::for_preset(ACTIVE_FOLLOW_PRESET);
    let strip = dedicated_closeable_tab_strip::strip_rect_for_test();
    let active = require_some(
        dedicated_closeable_tab_strip::tab_rect_for_test(&state, "theme.rs"),
        "active follow tab rect",
    )?;

    assert_eq!("theme.rs", state.active_tab_id);
    assert!(dedicated_closeable_tab_strip::scroll_x_for_test(&state) > 0);
    assert!(active.x >= strip.x);
    assert!(active.right() <= strip.x + strip.width);
    Ok(())
}
