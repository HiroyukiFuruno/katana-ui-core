use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, dyn_container, label, scroll, v_stack};
use katana_ui_widget::composite::selector::color_picker::{
    ColorPickerRgba, InlineColorPicker, LabeledColorPicker,
};
use katana_ui_widget::theme::Theme;
use katana_ui_widget::theme::color::Color;

fn sample_color() -> Color {
    Color {
        r: 80,
        g: 120,
        b: 220,
        a: 200,
    }
}

fn muted_color() -> Color {
    Color {
        r: 180,
        g: 96,
        b: 120,
        a: 128,
    }
}

fn rgba_text(color: Color) -> String {
    format!("rgba({}, {}, {}, {})", color.r, color.g, color.b, color.a)
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let selected = create_rw_signal(sample_color());

    scroll(
        v_stack((
            label(|| "ColorPicker Samples")
                .style(|style| style.font_size(16.0).margin_bottom(8.0)),
            label(|| "Labeled row: katana settings style").style(|style| style.font_size(13.0)),
            LabeledColorPicker::new("Accent", sample_color())
                .rgba(true)
                .on_change(move |color| selected.set(color))
                .view(theme.clone()),
            dyn_container(
                move || selected.get(),
                move |color| label(move || format!("selected: {}", rgba_text(color))),
            ),
            label(|| "Inline button: katana icon color column")
                .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            InlineColorPicker::new(muted_color(), "Inline icon color")
                .rgba(true)
                .view(theme.clone()),
            label(|| "RGB only").style(|style| {
                style
                    .font_size(16.0)
                    .margin_top(12.0)
                    .margin_bottom(8.0)
            }),
            LabeledColorPicker::new("Opaque color", sample_color()).view(theme.clone()),
            label(|| "Readonly / disabled").style(|style| {
                style
                    .font_size(16.0)
                    .margin_top(12.0)
                    .margin_bottom(8.0)
            }),
            LabeledColorPicker::new("Readonly", muted_color())
                .rgba(true)
                .readonly(true)
                .view(theme.clone()),
            LabeledColorPicker::new("Disabled", muted_color())
                .rgba(true)
                .disabled(true)
                .view(theme.clone()),
            label(|| "Compatibility entry").style(|style| {
                style
                    .font_size(16.0)
                    .margin_top(12.0)
                    .margin_bottom(8.0)
            }),
            ColorPickerRgba::new(sample_color(), "Legacy RGBA entry").view(theme.clone()),
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

pub fn color_picker_rgba_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
