use super::{ROW_VALUE_MAX_CHARS, contract_rows, row_value, status_rows};
use crate::catalog::StoryCatalog;

#[test]
fn contract_rows_stay_within_visual_table_value_width() {
    let examples = StoryCatalog.examples();

    for example in &examples {
        let rows = contract_rows(example.tree.root(), example);
        for (label, value) in rows {
            assert!(
                value.chars().count() <= ROW_VALUE_MAX_CHARS,
                "{} {} row overflows: {}",
                example.page,
                label,
                value
            );
        }
    }
}

#[test]
fn status_rows_stay_within_visual_table_value_width() {
    let examples = StoryCatalog.examples();

    for example in &examples {
        let rows = status_rows(example);
        for (label, value) in rows {
            assert!(
                value.chars().count() <= ROW_VALUE_MAX_CHARS,
                "{} {} status overflows: {}",
                example.page,
                label,
                value
            );
        }
    }
}

#[test]
fn long_contract_value_is_clipped_before_it_can_escape_table() {
    let long_value = "x".repeat(ROW_VALUE_MAX_CHARS + 10);
    let clipped = row_value(long_value);

    assert_eq!(ROW_VALUE_MAX_CHARS, clipped.chars().count());
    assert!(clipped.ends_with("..."));
}
