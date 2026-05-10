/// Compute the next `is_open` state on trigger click.
pub(super) fn toggle_open(current: bool, disabled: bool) -> bool {
    if disabled { false } else { !current }
}

/// Compute the next `is_open` state after an option is selected.
pub(super) fn close_on_select() -> bool {
    false
}
