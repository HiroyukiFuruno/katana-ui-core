pub(super) const ROW_MAX_CHARS: usize = 37;
const CLIP_SUFFIX: &str = "...";

pub(super) fn row_value(value: String) -> String {
    if value.chars().count() <= ROW_MAX_CHARS {
        return value;
    }
    let keep = ROW_MAX_CHARS - CLIP_SUFFIX.len();
    let clipped: String = value.chars().take(keep).collect();
    format!("{clipped}{CLIP_SUFFIX}")
}
