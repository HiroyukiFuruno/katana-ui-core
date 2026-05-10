use floem::views::{label, v_stack};
use floem::IntoView;

pub fn welcome_page() -> impl IntoView {
    v_stack((
        label(|| "katana-ui-widget Storybook"),
        label(|| "Select a widget from the sidebar to get started."),
    ))
}
