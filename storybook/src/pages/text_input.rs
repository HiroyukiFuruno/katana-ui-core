use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::input::text::{InputSize, TextInput, TrailingSlot};
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;

const ICON_SVG: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><circle cx=\"8\" cy=\"8\" r=\"6\" fill=\"currentColor\"/></svg>";

fn input_row(
    heading: &'static str,
    display_text: &'static str,
    r: u8,
    g: u8,
    b: u8,
    border_r: u8,
    border_g: u8,
    border_b: u8,
    font_sz: f32,
) -> impl IntoView {
    let text_color = PenikoColor::rgb8(r, g, b);
    let border_color = PenikoColor::rgb8(border_r, border_g, border_b);
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        label(move || display_text).style(move |s| {
            s.font_size(font_sz)
                .color(text_color)
                .border(1.0)
                .border_color(border_color)
                .padding(6.0)
                .min_width(200.0)
        }),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_plain = TextInput::new("Name").placeholder("Enter your name").resolve(theme);
    let r_value = TextInput::new("Email").value("user@example.com").resolve(theme);
    let r_icon = TextInput::new("Search")
        .leading_icon(IconSource::SvgBytes(ICON_SVG))
        .placeholder("Search…")
        .resolve(theme);
    let r_clear = TextInput::new("Clearable")
        .value("some text")
        .trailing(TrailingSlot::ClearButton)
        .resolve(theme);
    let r_spinner = TextInput::new("Loading")
        .value("processing…")
        .trailing(TrailingSlot::Spinner)
        .resolve(theme);
    let r_sm = TextInput::new("Small").size(InputSize::Sm).placeholder("Small").resolve(theme);
    let r_lg = TextInput::new("Large").size(InputSize::Lg).placeholder("Large").resolve(theme);
    let r_invalid = TextInput::new("Email invalid").value("not-an-email").invalid(true).resolve(theme);
    let r_disabled = TextInput::new("Disabled").disabled(true).placeholder("Disabled").resolve(theme);
    let r_readonly = TextInput::new("Readonly").value("read only value").readonly(true).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let to_static = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };

    let t0 = to_static(r_plain.placeholder.clone().unwrap_or_default());
    let t1 = to_static(r_value.value.clone());
    let t2 = to_static(r_icon.placeholder.clone().unwrap_or("Search…".into()));
    let t3 = to_static(r_clear.value.clone());
    let t4 = to_static(r_spinner.value.clone());
    let t5 = to_static(r_sm.placeholder.clone().unwrap_or_default());
    let t6 = to_static(r_lg.placeholder.clone().unwrap_or_default());
    let t7 = to_static(r_invalid.value.clone());
    let t8 = to_static(r_disabled.placeholder.clone().unwrap_or_default());
    let t9 = to_static(r_readonly.value.clone());

    scroll(
        v_stack((
            label(|| "TextInput Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            input_row("Placeholder only", t0, r_plain.text_color.r, r_plain.text_color.g, r_plain.text_color.b, r_plain.border_color.r, r_plain.border_color.g, r_plain.border_color.b, r_plain.font_size),
            input_row("With value", t1, r_value.text_color.r, r_value.text_color.g, r_value.text_color.b, r_value.border_color.r, r_value.border_color.g, r_value.border_color.b, r_value.font_size),
            input_row("Leading icon [○]", t2, r_icon.text_color.r, r_icon.text_color.g, r_icon.text_color.b, r_icon.border_color.r, r_icon.border_color.g, r_icon.border_color.b, r_icon.font_size),
            input_row("Trailing: ClearButton [×]", t3, r_clear.text_color.r, r_clear.text_color.g, r_clear.text_color.b, r_clear.border_color.r, r_clear.border_color.g, r_clear.border_color.b, r_clear.font_size),
            input_row("Trailing: Spinner [⟳]", t4, r_spinner.text_color.r, r_spinner.text_color.g, r_spinner.text_color.b, r_spinner.border_color.r, r_spinner.border_color.g, r_spinner.border_color.b, r_spinner.font_size),
            input_row("Size Sm", t5, r_sm.text_color.r, r_sm.text_color.g, r_sm.text_color.b, r_sm.border_color.r, r_sm.border_color.g, r_sm.border_color.b, r_sm.font_size),
            input_row("Size Lg", t6, r_lg.text_color.r, r_lg.text_color.g, r_lg.text_color.b, r_lg.border_color.r, r_lg.border_color.g, r_lg.border_color.b, r_lg.font_size),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            input_row("Invalid (danger border)", t7, r_invalid.text_color.r, r_invalid.text_color.g, r_invalid.text_color.b, r_invalid.border_color.r, r_invalid.border_color.g, r_invalid.border_color.b, r_invalid.font_size),
            input_row("Disabled", t8, r_disabled.text_color.r, r_disabled.text_color.g, r_disabled.text_color.b, r_disabled.border_color.r, r_disabled.border_color.g, r_disabled.border_color.b, r_disabled.font_size),
            input_row("Readonly", t9, r_readonly.text_color.r, r_readonly.text_color.g, r_readonly.text_color.b, r_readonly.border_color.r, r_readonly.border_color.g, r_readonly.border_color.b, r_readonly.font_size),
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

pub fn text_input_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "TextInput").style(|s| s.font_size(20.0)),
            label(move || if is_dark.get() { "Dark" } else { "Light" }),
            toggle_button(move || is_dark.get()).on_toggle(move |v| is_dark.set(v)),
        ))
        .style(|s| s.gap(12.0).items_center().padding(12.0)),
        dyn_container(
            move || is_dark.get(),
            move |dark| {
                let theme = if dark {
                    Theme::default_dark()
                } else {
                    Theme::default_light()
                };
                page_content(&theme)
            },
        ),
    ))
}
