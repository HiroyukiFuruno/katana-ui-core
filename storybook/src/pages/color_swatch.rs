use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, dyn_container, h_stack, label, scroll, v_stack};
use katana_ui_core::composite::selector::color::{ColorSwatch, SwatchShape, SwatchSize};
use katana_ui_core::theme::Theme;
use katana_ui_core::theme::color::Color;

fn six_palette() -> Vec<Color> {
    vec![
        Color {
            r: 220,
            g: 50,
            b: 50,
            a: 255,
        },
        Color {
            r: 220,
            g: 140,
            b: 50,
            a: 255,
        },
        Color {
            r: 220,
            g: 220,
            b: 50,
            a: 255,
        },
        Color {
            r: 50,
            g: 180,
            b: 50,
            a: 255,
        },
        Color {
            r: 50,
            g: 100,
            b: 220,
            a: 255,
        },
        Color {
            r: 140,
            g: 50,
            b: 200,
            a: 255,
        },
    ]
}

fn twelve_palette() -> Vec<Color> {
    let mut p = six_palette();
    p.extend(vec![
        Color {
            r: 200,
            g: 80,
            b: 120,
            a: 255,
        },
        Color {
            r: 80,
            g: 200,
            b: 180,
            a: 255,
        },
        Color {
            r: 180,
            g: 120,
            b: 60,
            a: 255,
        },
        Color {
            r: 100,
            g: 100,
            b: 100,
            a: 255,
        },
        Color {
            r: 30,
            g: 30,
            b: 30,
            a: 255,
        },
        Color {
            r: 240,
            g: 240,
            b: 240,
            a: 255,
        },
    ]);
    p
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let six = six_palette();
    let twelve = twelve_palette();
    let selected = Color {
        r: 50,
        g: 100,
        b: 220,
        a: 255,
    };
    let live_selected = create_rw_signal(selected);

    crate::interaction::replay("select-color", "color-swatch", "selected-green", {
        let live_selected = live_selected;
        move || {
            live_selected.set(Color {
                r: 50,
                g: 180,
                b: 50,
                a: 255,
            });
        }
    });

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "ColorSwatch Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            ColorSwatch::new(selected, six_palette(), "Live palette")
                .on_change(move |color| {
                    live_selected.set(color);
                })
                .view(theme.clone()),
            dyn_container(
                move || live_selected.try_get().unwrap_or(selected),
                move |color| {
                    let fill = PenikoColor::rgba8(color.r, color.g, color.b, color.a);
                    h_stack((
                        container(label(|| "")).style(move |s| {
                            s.width(48.0).height(32.0).background(fill).border(1.0)
                        }),
                        label(move || {
                            format!(
                                "selected rgba({}, {}, {}, {})",
                                color.r, color.g, color.b, color.a
                            )
                        })
                        .style(|s| s.font_size(12.0)),
                    ))
                    .style(|s| s.gap(8.0).items_center())
                },
            ),
            label(|| "Readonly display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            ColorSwatch::new(selected, six, "6-color palette")
                .disabled(true)
                .view(theme.clone()),
            ColorSwatch::new(selected, twelve, "12-color palette")
                .disabled(true)
                .view(theme.clone()),
            label(|| "Size display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            ColorSwatch::new(selected, six_palette(), "Small swatches")
                .size(SwatchSize::Sm)
                .disabled(true)
                .view(theme.clone()),
            ColorSwatch::new(selected, six_palette(), "Large swatches")
                .size(SwatchSize::Lg)
                .disabled(true)
                .view(theme.clone()),
            label(|| "Shape display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            ColorSwatch::new(selected, six_palette(), "Circle swatches")
                .shape(SwatchShape::Circle)
                .disabled(true)
                .view(theme.clone()),
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

pub fn color_swatch_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
