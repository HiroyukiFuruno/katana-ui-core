use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::selector::select::SelectBox;
use katana_ui_widget::theme::Theme;

fn short_options() -> Vec<(u8, String)> {
    vec![
        (1, "Apple".into()),
        (2, "Banana".into()),
        (3, "Cherry".into()),
    ]
}

fn long_options() -> Vec<(u8, String)> {
    (1u8..=10)
        .map(|index| (index, format!("Option {index}")))
        .collect()
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "SelectBox Samples").style(|style| style.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|style| style.font_size(13.0)),
            SelectBox::new(short_options(), "Live fruit")
                .placeholder("Pick a fruit")
                .view(theme.clone()),
            label(|| "Readonly display")
                .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            SelectBox::new(short_options(), "Placeholder")
                .placeholder("Pick a fruit")
                .disabled(true)
                .view(theme.clone()),
            SelectBox::new(short_options(), "Selected")
                .value(2u8)
                .disabled(true)
                .view(theme.clone()),
            SelectBox::new(short_options(), "Open")
                .value(2u8)
                .open(true)
                .disabled(true)
                .view(theme.clone()),
            SelectBox::new(long_options(), "Long list")
                .value(5u8)
                .disabled(true)
                .view(theme.clone()),
        ))
        .style(move |style| {
            style
                .gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn select_box_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
