#[cfg(test)]
mod tests;
mod types;
mod view;

pub use types::{Breadcrumb, BreadcrumbCrumb, BreadcrumbProps};

use crate::theme::Theme;
use floem::IntoView;

impl Breadcrumb {
    #[must_use]
    pub fn new(crumbs: Vec<BreadcrumbCrumb>) -> Self {
        Self {
            props: BreadcrumbProps {
                crumbs,
                ..BreadcrumbProps::default()
            },
        }
    }

    #[must_use]
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.props.separator = separator.into();
        self
    }

    #[must_use]
    pub fn max_visible_crumbs(mut self, max_visible_crumbs: usize) -> Self {
        self.props.max_visible_crumbs = max_visible_crumbs;
        self
    }

    #[must_use]
    pub fn allow_last_click(mut self, allow_last_click: bool) -> Self {
        self.props.allow_last_click = allow_last_click;
        self
    }

    #[must_use]
    pub fn background(mut self, show_background: bool) -> Self {
        self.props.show_background = show_background;
        self
    }

    #[must_use]
    pub fn border(mut self, show_border: bool) -> Self {
        self.props.show_border = show_border;
        self
    }

    #[must_use]
    pub fn crumbs(mut self, crumbs: Vec<BreadcrumbCrumb>) -> Self {
        self.props.crumbs = crumbs;
        self
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        view::build_view(self, theme)
    }
}
