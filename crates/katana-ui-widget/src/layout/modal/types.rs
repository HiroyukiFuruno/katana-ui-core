use std::rc::Rc;

/// Modal dialog size.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ModalSize {
    Sm,
    #[default]
    Md,
    Lg,
    Custom(f32),
}

/// Properties for `Modal`.
#[derive(Clone)]
pub struct ModalProps {
    pub open: bool,
    pub title: Option<String>,
    pub size: ModalSize,
    pub dismiss_on_backdrop: bool,
    pub dismiss_on_esc: bool,
    pub children: Option<String>,
    pub footer: Option<String>,
    pub on_close: Rc<dyn Fn()>,
    pub on_focus_return: Rc<dyn Fn()>,
}
