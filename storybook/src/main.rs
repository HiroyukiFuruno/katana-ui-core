mod pages;

use floem::views::{h_stack, label, scroll, v_stack, Decorators};
use floem::{Application, IntoView};
use pages::welcome::welcome_page;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum Page {
    Welcome,
}

fn app_view() -> impl IntoView {
    let sidebar = scroll(v_stack((label(|| "Widgets"),)).style(|s| s.padding(8.0)));

    let content = welcome_page();

    h_stack((sidebar, content))
}

fn main() {
    Application::new().window(|_| app_view(), None).run();
}
