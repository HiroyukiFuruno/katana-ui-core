use floem::IntoView;
use floem::views::{label, v_stack};

pub fn welcome_page() -> impl IntoView {
    v_stack((
        label(|| "katana-ui-core Storybook"),
        label(|| "Use the sidebar to inspect components with the global Light/Dark theme."),
    ))
}
