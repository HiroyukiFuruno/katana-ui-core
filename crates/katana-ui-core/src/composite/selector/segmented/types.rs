use crate::primitive::icon::IconSource;
use std::rc::Rc;

/// Content for a single segment.
#[derive(Debug, Clone)]
pub enum Segment {
    Label(String),
    Icon(IconSource, String),
}

/// Size of the segmented toggle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SegmentedSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `SegmentedToggle<K>`.
#[derive(Clone)]
pub struct SegmentedToggleProps<K> {
    pub value: K,
    pub options: Vec<(K, Segment)>,
    pub size: SegmentedSize,
    pub disabled: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(K)>,
}
