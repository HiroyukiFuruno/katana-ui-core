mod pages;

use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{button, h_stack, label, scroll, v_stack, Decorators};
use floem::{Application, IntoView};
use pages::icon::icon_page;
use pages::spinner::spinner_page;
use pages::svg_button::svg_button_page;
use pages::text::text_page;
use pages::text_button::text_button_page;
use pages::theme_tokens::theme_tokens_page;
use pages::welcome::welcome_page;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Page {
    Welcome,
    ThemeTokens,
    Text,
    Icon,
    Spinner,
    SvgButton,
    TextButton,
}

fn app_view() -> impl IntoView {
    let current_page = create_rw_signal(Page::Welcome);

    let sidebar = scroll(
        v_stack((
            label(|| "Widgets").style(|s| s.font_size(14.0).margin_bottom(8.0)),
            button(label(|| "Welcome")).action(move || current_page.set(Page::Welcome)),
            button(label(|| "Theme Tokens"))
                .action(move || current_page.set(Page::ThemeTokens)),
            button(label(|| "Text")).action(move || current_page.set(Page::Text)),
            button(label(|| "Icon")).action(move || current_page.set(Page::Icon)),
            button(label(|| "Spinner")).action(move || current_page.set(Page::Spinner)),
            button(label(|| "SvgButton")).action(move || current_page.set(Page::SvgButton)),
            button(label(|| "TextButton")).action(move || current_page.set(Page::TextButton)),
        ))
        .style(|s| s.padding(8.0).gap(4.0)),
    )
    .style(|s| s.width(160.0).min_height_full());

    let content = floem::views::dyn_container(
        move || current_page.get(),
        move |page| match page {
            Page::Welcome => welcome_page().into_any(),
            Page::ThemeTokens => theme_tokens_page().into_any(),
            Page::Text => text_page().into_any(),
            Page::Icon => icon_page().into_any(),
            Page::Spinner => spinner_page().into_any(),
            Page::SvgButton => svg_button_page().into_any(),
            Page::TextButton => text_button_page().into_any(),
        },
    )
    .style(|s| s.flex_grow(1.0));

    h_stack((sidebar, content)).style(|s| s.min_height_full())
}

fn main() {
    Application::new().window(|_| app_view(), None).run();
}
