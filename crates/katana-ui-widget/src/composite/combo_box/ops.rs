use super::types::ComboBoxOption;

/// Compute next open state for trigger click.
#[must_use]
pub(super) fn toggle_open(current: bool, disabled: bool) -> bool {
    if disabled { false } else { !current }
}

#[must_use]
pub(super) fn normalize_query(raw: &str) -> String {
    raw.to_lowercase()
}

#[must_use]
pub(super) fn filtered_options<K: Clone + PartialEq>(
    query: &str,
    options: &[ComboBoxOption<K>],
) -> Vec<ComboBoxOption<K>> {
    let needle = normalize_query(query);
    if needle.is_empty() {
        return options.to_vec();
    }

    options
        .iter()
        .filter(|option| normalize_query(&option.label).contains(&needle))
        .cloned()
        .collect()
}
