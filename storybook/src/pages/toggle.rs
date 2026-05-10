use floem::peniko::Color as PenikoColor;
use floem::views::{label, scroll, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::selector::toggle::{Toggle, ToggleSize};
use katana_ui_widget::theme::Theme;

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Toggle Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            Toggle::new("Live toggle").view(theme.clone()),
            label(|| "Readonly display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            Toggle::new("Off").disabled(true).view(theme.clone()),
            Toggle::new("On").value(true).disabled(true).view(theme.clone()),
            label(|| "Disabled display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            Toggle::new("Disabled off").disabled(true).view(theme.clone()),
            Toggle::new("Disabled on")
                .disabled(true)
                .value(true)
                .view(theme.clone()),
            label(|| "Size display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            Toggle::new("Small").size(ToggleSize::Sm).disabled(true).view(theme.clone()),
            Toggle::new("Large")
                .size(ToggleSize::Lg)
                .disabled(true)
                .value(true)
                .view(theme.clone()),
        ))
        .style(move |s| {
            s.gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn toggle_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
