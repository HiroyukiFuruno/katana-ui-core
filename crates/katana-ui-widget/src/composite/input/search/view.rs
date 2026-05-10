/// Returns the value after Esc key: always clears the input.
pub(super) fn on_esc(_current: &str) -> String {
    String::new()
}

/// Returns whether clear button should be shown (value non-empty and not disabled).
pub(super) fn show_clear(value: &str, disabled: bool) -> bool {
    !value.is_empty() && !disabled
}
