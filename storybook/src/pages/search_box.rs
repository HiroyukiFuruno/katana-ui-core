use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::input::search::{
    SearchBox, SearchBoxIconMode, SearchBoxIconSlot,
};
use katana_ui_widget::composite::input::text::InputSize;
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;

const CUSTOM_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><path d=\"M8 2l2 4 4 .5-3 3 .8 4.5L8 12l-3.8 2 .8-4.5-3-3L6 6z\" fill=\"currentColor\"/></svg>";

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "SearchBox Samples").style(|style| style.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|style| style.font_size(13.0)),
            SearchBox::new("Live search")
                .search_icon(SearchBoxIconMode::Visible)
                .clear_icon(SearchBoxIconMode::Visible)
                .submit_icon(SearchBoxIconMode::Visible)
                .view(theme.clone()),
            label(|| "Readonly display")
                .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            label(|| "Default hidden icons").style(|style| style.font_size(12.0)),
            SearchBox::new("Empty search")
                .placeholder("Search...")
                .disabled(true)
                .view(theme.clone()),
            label(|| "Reserved icon space").style(|style| style.font_size(12.0)),
            SearchBox::new("With query")
                .value("floem widgets")
                .search_icon(SearchBoxIconMode::Reserved)
                .clear_icon(SearchBoxIconMode::Reserved)
                .submit_icon(SearchBoxIconMode::Reserved)
                .disabled(true)
                .view(theme.clone()),
            label(|| "Custom leading SVG").style(|style| style.font_size(12.0)),
            SearchBox::new("Custom search")
                .value("custom icon")
                .search_icon(SearchBoxIconMode::Visible)
                .icon_source(SearchBoxIconSlot::Leading, IconSource::SvgBytes(CUSTOM_ICON))
                .clear_icon(SearchBoxIconMode::Visible)
                .submit_icon(SearchBoxIconMode::Visible)
                .disabled(true)
                .view(theme.clone()),
            label(|| "Size display")
                .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            SearchBox::new("Small search")
                .size(InputSize::Sm)
                .value("small")
                .search_icon(SearchBoxIconMode::Visible)
                .clear_icon(SearchBoxIconMode::Visible)
                .submit_icon(SearchBoxIconMode::Visible)
                .disabled(true)
                .view(theme.clone()),
            SearchBox::new("Large search")
                .size(InputSize::Lg)
                .placeholder("Large")
                .search_icon(SearchBoxIconMode::Visible)
                .clear_icon(SearchBoxIconMode::Visible)
                .submit_icon(SearchBoxIconMode::Visible)
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

pub fn search_box_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
