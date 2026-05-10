mod pages;

use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{
    button, h_stack, label, scroll, toggle_button, v_stack, v_stack_from_iter, Decorators,
};
use floem::{Application, IntoView};
use katana_ui_widget::theme::Theme;
use pages::accordion::accordion_page;
use pages::modal_overlay::modal_overlay_page;
use pages::popover::popover_page;
use pages::split_pane::split_pane_page;
use pages::badge::badge_page;
use pages::card::card_page;
use pages::color_picker_rgba::color_picker_rgba_page;
use pages::color_swatch::color_swatch_page;
use pages::icon::icon_page;
use pages::icon_text_button::icon_text_button_page;
use pages::key_cap::key_cap_page;
use pages::segmented_toggle::segmented_toggle_page;
use pages::search_box::search_box_page;
use pages::select_box::select_box_page;
use pages::text_input::text_input_page;
use pages::spinner::spinner_page;
use pages::toggle::toggle_page;
use pages::tooltip::tooltip_page;
use pages::svg_button::svg_button_page;
use pages::text::text_page;
use pages::text_button::text_button_page;
use pages::theme_tokens::theme_tokens_page;
use pages::welcome::welcome_page;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Page {
    Overview,
    ThemeTokens,
    Text,
    Icon,
    Spinner,
    SvgButton,
    TextButton,
    IconTextButton,
    Toggle,
    SegmentedToggle,
    SelectBox,
    ColorSwatch,
    ColorPickerRgba,
    TextInput,
    SearchBox,
    Tooltip,
    Badge,
    KeyCap,
    Card,
    Accordion,
    SplitPane,
    ModalOverlay,
    Popover,
}

fn app_view() -> impl IntoView {
    let current_page = create_rw_signal(Page::Overview);
    let is_dark = create_rw_signal(false);

    let sidebar_buttons: Vec<_> = [
        ("Overview", Some(Page::Overview)),
        ("Theme Tokens", Some(Page::ThemeTokens)),
        ("Text", Some(Page::Text)),
        ("Icon", Some(Page::Icon)),
        ("Spinner", Some(Page::Spinner)),
        ("SvgButton", Some(Page::SvgButton)),
        ("TextButton", Some(Page::TextButton)),
        ("IconTextButton", Some(Page::IconTextButton)),
        ("Toggle", Some(Page::Toggle)),
        ("SegmentedToggle", Some(Page::SegmentedToggle)),
        ("SelectBox", Some(Page::SelectBox)),
        ("ColorSwatch", Some(Page::ColorSwatch)),
        ("ColorPickerRgba", Some(Page::ColorPickerRgba)),
        ("TextInput", Some(Page::TextInput)),
        ("SearchBox", Some(Page::SearchBox)),
        ("Tooltip", Some(Page::Tooltip)),
        ("Badge", Some(Page::Badge)),
        ("KeyCap", Some(Page::KeyCap)),
        ("Card", Some(Page::Card)),
        ("Accordion", Some(Page::Accordion)),
        ("SplitPane", Some(Page::SplitPane)),
        ("ModalOverlay", Some(Page::ModalOverlay)),
        ("Popover", Some(Page::Popover)),
    ]
    .into_iter()
    .map(|(name, page)| {
        let current_page = current_page;
        let button_view = button(label(move || name));
        match page {
            None => button_view,
            Some(page) => button_view.action(move || current_page.set(page)),
        }
    })
    .collect();

    let theme_switch = h_stack((
        label(|| "Theme").style(|s| s.font_size(13.0).margin_left(4.0)),
        label(move || if is_dark.get() { "Dark" } else { "Light" }),
        toggle_button(move || is_dark.get()).on_toggle(move |v| is_dark.set(v)),
    ))
    .style(|s| s.gap(8.0).items_center().padding(8.0));

    let sidebar = scroll(v_stack((
        theme_switch,
        label(|| "Components").style(|s| {
            s.font_size(13.0)
                .margin_left(8.0)
                .margin_top(8.0)
                .margin_bottom(4.0)
        }),
        v_stack_from_iter(sidebar_buttons).style(|s| s.padding(4.0).gap(4.0)),
    )))
    .style(|s| s.width(180.0).min_height_full());

    let content = floem::views::dyn_container(
        move || (current_page.get(), is_dark.get()),
        move |(page, dark)| {
            let theme = if dark {
                Theme::default_dark()
            } else {
                Theme::default_light()
            };
            theme.clone().provide();

            match page {
                Page::Overview => welcome_page().into_any(),
                Page::ThemeTokens => theme_tokens_page(theme).into_any(),
                Page::Text => text_page(theme).into_any(),
                Page::Icon => icon_page(theme).into_any(),
                Page::Spinner => spinner_page(theme).into_any(),
                Page::SvgButton => svg_button_page(theme).into_any(),
                Page::TextButton => text_button_page(theme).into_any(),
                Page::IconTextButton => icon_text_button_page(theme).into_any(),
                Page::Toggle => toggle_page(theme).into_any(),
                Page::SegmentedToggle => segmented_toggle_page(theme).into_any(),
                Page::SelectBox => select_box_page(theme).into_any(),
                Page::ColorSwatch => color_swatch_page(theme).into_any(),
                Page::ColorPickerRgba => color_picker_rgba_page(theme).into_any(),
                Page::TextInput => text_input_page(theme).into_any(),
                Page::SearchBox => search_box_page(theme).into_any(),
                Page::Tooltip => tooltip_page(theme).into_any(),
                Page::Badge => badge_page(theme).into_any(),
                Page::KeyCap => key_cap_page(theme).into_any(),
                Page::Card => card_page(theme).into_any(),
                Page::Accordion => accordion_page(theme).into_any(),
                Page::SplitPane => split_pane_page(theme).into_any(),
                Page::ModalOverlay => modal_overlay_page(theme).into_any(),
                Page::Popover => popover_page(theme).into_any(),
            }
        },
    )
    .style(|s| s.flex_grow(1.0));

    h_stack((sidebar, content)).style(|s| s.min_height_full())
}

fn main() {
    Application::new().window(|_| app_view(), None).run();
}
