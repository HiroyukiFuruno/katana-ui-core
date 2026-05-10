use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::input::text::{InputSize, TextInput, TrailingSlot};
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;

const ICON_SVG: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><circle cx=\"8\" cy=\"8\" r=\"6\" fill=\"currentColor\"/></svg>";

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "TextInput Samples").style(|style| style.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|style| style.font_size(13.0)),
            TextInput::new("Live name")
                .placeholder("Type here")
                .view(theme.clone()),
            label(|| "Readonly display")
                .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            TextInput::new("Placeholder")
                .placeholder("Enter your name")
                .readonly(true)
                .view(theme.clone()),
            TextInput::new("With value")
                .value("user@example.com")
                .readonly(true)
                .view(theme.clone()),
            TextInput::new("Leading icon")
                .leading_icon(IconSource::SvgBytes(ICON_SVG))
                .placeholder("Search...")
                .readonly(true)
                .view(theme.clone()),
            TextInput::new("Clearable")
                .value("some text")
                .trailing(TrailingSlot::ClearButton)
                .readonly(true)
                .view(theme.clone()),
            TextInput::new("Loading")
                .value("processing...")
                .trailing(TrailingSlot::Spinner)
                .readonly(true)
                .view(theme.clone()),
            label(|| "Size display")
                .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            TextInput::new("Small")
                .size(InputSize::Sm)
                .placeholder("Small")
                .readonly(true)
                .view(theme.clone()),
            TextInput::new("Large")
                .size(InputSize::Lg)
                .placeholder("Large")
                .readonly(true)
                .view(theme.clone()),
            label(|| "State display")
                .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            TextInput::new("Invalid")
                .value("not-an-email")
                .invalid(true)
                .readonly(true)
                .view(theme.clone()),
            TextInput::new("Disabled")
                .placeholder("Disabled")
                .disabled(true)
                .view(theme.clone()),
        ))
        .style(move |style| {
            style
                .gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn text_input_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
