use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaRowMeasurement {
    pub rows: u16,
    pub internal_scroll: bool,
}

pub(super) fn measure_rows(
    value: &str,
    min_rows: u16,
    max_rows: u16,
    auto_grow: bool,
) -> TextAreaRowMeasurement {
    let lower_bound = min_rows.max(1);
    let upper_bound = max_rows.max(lower_bound);
    let content_rows = value.split('\n').count().max(1) as u16;
    let rows = if auto_grow {
        content_rows.clamp(lower_bound, upper_bound)
    } else {
        lower_bound
    };

    TextAreaRowMeasurement {
        rows,
        internal_scroll: content_rows > rows,
    }
}
