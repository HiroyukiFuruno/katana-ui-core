use super::*;

#[test]
fn no_actions_when_no_items() {
    let result = ToolbarKeyboardNavigator::apply(Some(2), 0, ToolbarKeyboardInput::ArrowRight);
    assert_eq!(result.focused_index(), None);
    assert_eq!(result.activated_index(), None);
}

#[test]
fn home_and_end_jump_to_extremes() {
    let first = ToolbarKeyboardNavigator::apply(Some(4), 3, ToolbarKeyboardInput::Home);
    let last = ToolbarKeyboardNavigator::apply(None, 5, ToolbarKeyboardInput::End);
    assert_eq!(first, ToolbarKeyboardResult::new(Some(0), None));
    assert_eq!(last, ToolbarKeyboardResult::new(Some(4), None));
}

#[test]
fn escape_preserves_the_current_focus_without_activation() {
    let result = ToolbarKeyboardNavigator::apply(Some(2), 4, ToolbarKeyboardInput::Escape);
    assert_eq!(result.focused_index(), Some(2));
    assert_eq!(result.activated_index(), None);
}
