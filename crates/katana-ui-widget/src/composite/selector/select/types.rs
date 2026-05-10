use std::rc::Rc;

/// Size of the select box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `SelectBox<K>`.
#[derive(Clone)]
pub struct SelectBoxProps<K> {
    pub value: Option<K>,
    pub options: Vec<(K, String)>,
    pub placeholder: String,
    pub size: SelectSize,
    pub disabled: bool,
    pub is_open: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(K)>,
}
