use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::selector::color::{ColorSwatch, SwatchSize};
use katana_ui_widget::theme::color::Color;
use katana_ui_widget::theme::Theme;

fn six_palette() -> Vec<Color> {
    vec![
        Color { r: 220, g: 50, b: 50, a: 255 },
        Color { r: 220, g: 140, b: 50, a: 255 },
        Color { r: 220, g: 220, b: 50, a: 255 },
        Color { r: 50, g: 180, b: 50, a: 255 },
        Color { r: 50, g: 100, b: 220, a: 255 },
        Color { r: 140, g: 50, b: 200, a: 255 },
    ]
}

fn twelve_palette() -> Vec<Color> {
    let mut p = six_palette();
    p.extend(vec![
        Color { r: 200, g: 80, b: 120, a: 255 },
        Color { r: 80, g: 200, b: 180, a: 255 },
        Color { r: 180, g: 120, b: 60, a: 255 },
        Color { r: 100, g: 100, b: 100, a: 255 },
        Color { r: 30, g: 30, b: 30, a: 255 },
        Color { r: 240, g: 240, b: 240, a: 255 },
    ]);
    p
}

fn swatch_row(
    heading: &'static str,
    cells: Vec<(u8, u8, u8, bool, f32)>,
) -> impl IntoView {
    let swatches: Vec<_> = cells
        .into_iter()
        .map(|(r, g, b, selected, sz)| {
            let fill = PenikoColor::rgb8(r, g, b);
            let border_w = if selected { 3.0_f32 } else { 0.5_f32 };
            label(|| "")
                .style(move |s| s.width(sz).height(sz).background(fill).border(border_w))
        })
        .collect();

    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack(swatches).style(|s| s.gap(4.0).flex_wrap(true)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let six = six_palette();
    let twelve = twelve_palette();
    let selected = Color { r: 50, g: 100, b: 220, a: 255 };

    let r6 = ColorSwatch::new(selected, six.clone(), "6-color palette").resolve(theme);
    let r12 = ColorSwatch::new(selected, twelve, "12-color palette").resolve(theme);
    let r_sm = ColorSwatch::new(selected, six.clone(), "Small swatches")
        .size(SwatchSize::Sm)
        .resolve(theme);
    let r_lg = ColorSwatch::new(selected, six.clone(), "Large swatches")
        .size(SwatchSize::Lg)
        .resolve(theme);
    let r_dis = ColorSwatch::new(selected, six, "Disabled").disabled(true).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let to_row = |heading: &'static str,
                  r: katana_ui_widget::composite::selector::color::ResolvedColorSwatch| {
        let cells = r
            .cells
            .iter()
            .map(|c| (c.color.r, c.color.g, c.color.b, c.selected, c.cell_size))
            .collect();
        swatch_row(heading, cells)
    };

    scroll(
        v_stack((
            label(|| "ColorSwatch Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            to_row("6-color palette", r6),
            to_row("12-color palette", r12),
            to_row("Small (Sm)", r_sm),
            to_row("Large (Lg)", r_lg),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            to_row("Disabled", r_dis),
        ))
        .style(move |s| {
            s.gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn color_swatch_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "ColorSwatch").style(|s| s.font_size(20.0)),
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
