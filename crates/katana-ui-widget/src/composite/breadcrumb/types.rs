use crate::primitive::icon::IconSource;
use std::rc::Rc;

/// One breadcrumb segment.
#[derive(Clone)]
pub struct BreadcrumbCrumb {
    pub label: String,
    pub icon: Option<IconSource>,
    pub disabled: bool,
    pub on_click: Option<Rc<dyn Fn()>>,
    pub children: Vec<BreadcrumbCrumb>,
}

impl BreadcrumbCrumb {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            disabled: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: IconSource) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn on_click(mut self, on_click: impl Fn() + 'static) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    #[must_use]
    pub fn children(mut self, children: Vec<BreadcrumbCrumb>) -> Self {
        self.children = children;
        self
    }
}

/// Properties for the breadcrumb list.
#[derive(Clone)]
pub struct BreadcrumbProps {
    pub crumbs: Vec<BreadcrumbCrumb>,
    pub separator: String,
    pub max_visible_crumbs: usize,
    pub allow_last_click: bool,
    pub show_background: bool,
    pub show_border: bool,
}

impl Default for BreadcrumbProps {
    fn default() -> Self {
        Self {
            crumbs: Vec::new(),
            separator: " > ".to_string(),
            max_visible_crumbs: 0,
            allow_last_click: false,
            show_background: false,
            show_border: false,
        }
    }
}

/// Builder object for breadcrumb.
#[derive(Clone)]
pub struct Breadcrumb {
    pub props: BreadcrumbProps,
}
