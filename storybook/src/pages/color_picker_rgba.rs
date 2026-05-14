use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, dyn_container, h_stack, label, scroll, v_stack};
use katana_ui_widget::composite::selector::color_picker::{
    ColorPickerRgba, ColorPickerTriggerSize, InlineColorPicker, LabeledColorPicker,
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

fn should_start_open() -> bool {
    crate::interaction::open_requested("color-picker-rgba", "initial-open")
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let selected = create_rw_signal(sample_color());
    let live_log = create_rw_signal(String::from("on_change: none"));
    let readonly_log = create_rw_signal(String::from("on_change: not expected"));
    let disabled_log = create_rw_signal(String::from("on_change: not expected"));

    scroll(
        v_stack((
            label(|| "ColorPicker Samples").style(|style| style.font_size(16.0).margin_bottom(8.0)),
            v_stack((
                label(|| "Labeled row: katana settings style").style(|style| style.font_size(13.0)),
                LabeledColorPicker::new("Accent", sample_color())
                    .rgba(true)
                    .title("アクセント色")
                    .on_change(move |color| selected.set(color))
                    .view(theme.clone()),
                dyn_container(
                    move || selected.get(),
                    move |color| label(move || format!("selected: {}", rgba_text(color))),
                ),
            ))
            .style(|style| style.gap(8.0)),
            v_stack((
                label(|| "Inline button: katana icon color column")
                    .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
                InlineColorPicker::new(muted_color(), "Inline icon color")
                    .rgba(true)
                    .title("アイコン色")
                    .open(should_start_open())
                    .on_change(move |color| {
                        live_log.set(format!("callback: {}", rgba_text(color)));
                        selected.set(color);
                    })
                    .view(theme.clone()),
                dyn_container(
                    move || live_log.get(),
                    move |value| {
                        let text = value.clone();
                        label(move || text.clone())
                    },
                ),
            ))
            .style(|style| style.gap(8.0)),
            v_stack((
                label(|| "RGB only")
                    .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
                LabeledColorPicker::new("Opaque color", sample_color()).view(theme.clone()),
            ))
            .style(|style| style.gap(8.0)),
            v_stack((
                label(|| "Readonly / disabled")
                    .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
                LabeledColorPicker::new("Readonly", muted_color())
                    .rgba(true)
                    .title("Readonly color")
                    .readonly(true)
                    .on_change(move |_| {
                        readonly_log.set(format!("on_change: {}", rgba_text(sample_color())));
                    })
                    .view(theme.clone()),
                dyn_container(
                    move || readonly_log.get(),
                    move |value| {
                        let text = value.clone();
                        label(move || text.clone())
                    },
                ),
                LabeledColorPicker::new("Disabled", muted_color())
                    .rgba(true)
                    .title("Disabled color")
                    .disabled(true)
                    .on_change(move |_| {
                        disabled_log.set(format!("on_change: {}", rgba_text(muted_color())));
                    })
                    .view(theme.clone()),
                dyn_container(
                    move || disabled_log.get(),
                    move |value| {
                        let text = value.clone();
                        label(move || text.clone())
                    },
                ),
            ))
            .style(|style| style.gap(8.0)),
            v_stack((
                label(|| "Compatibility entry")
                    .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
                ColorPickerRgba::new(sample_color(), "Legacy RGBA entry")
                    .title("Legacy RGBA")
                    .view(theme.clone()),
                label(|| "Custom size").style(|style| style.font_size(13.0).margin_top(8.0)),
                ColorPickerRgba::new(muted_color(), "Large RGBA entry")
                    .panel_scale(1.0)
                    .title("Custom size RGBA")
                    .view(theme.clone()),
                dyn_container(
                    move || selected.get(),
                    move |color| label(move || format!("current value: {}", rgba_text(color))),
                ),
                label(|| "Trigger size presets")
                    .style(|style| style.font_size(13.0).margin_top(8.0)),
                h_stack((
                    InlineColorPicker::new(sample_color(), "XS trigger")
                        .rgba(true)
                        .title("XS trigger")
                        .trigger_size(ColorPickerTriggerSize::Xs)
                        .view(theme.clone()),
                    InlineColorPicker::new(sample_color(), "SM trigger")
                        .rgba(true)
                        .title("SM trigger")
                        .trigger_size(ColorPickerTriggerSize::Sm)
                        .view(theme.clone()),
                    InlineColorPicker::new(sample_color(), "MID trigger")
                        .rgba(true)
                        .title("MID trigger")
                        .trigger_size(ColorPickerTriggerSize::Mid)
                        .view(theme.clone()),
                    InlineColorPicker::new(sample_color(), "Large trigger")
                        .rgba(true)
                        .title("Large trigger")
                        .trigger_size(ColorPickerTriggerSize::Large)
                        .view(theme.clone()),
                    InlineColorPicker::new(sample_color(), "XLarge trigger")
                        .rgba(true)
                        .title("XLarge trigger")
                        .trigger_size(ColorPickerTriggerSize::Xlarge)
                        .view(theme.clone()),
                ))
                .style(|style| style.items_center().gap(8.0)),
                label(|| "Borderless trigger").style(|style| style.font_size(13.0).margin_top(8.0)),
                InlineColorPicker::new(muted_color(), "Borderless RGBA trigger")
                    .rgba(true)
                    .title("Borderless RGBA")
                    .trigger_border(false)
                    .view(theme.clone()),
            ))
            .style(|style| style.gap(8.0)),
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
    .style(|style| style.min_width_full().flex_grow(1.0))
}

pub fn color_picker_rgba_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
