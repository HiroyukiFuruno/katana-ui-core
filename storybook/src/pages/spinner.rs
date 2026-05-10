use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, svg, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::primitive::spinner::{Spinner, SpinnerSize};
use katana_ui_widget::theme::Theme;

fn spinner_row(desc: &'static str, svg_content: String, size_px: f32) -> impl IntoView {
    h_stack((
        label(move || desc).style(|s| s.width(160.0).font_size(11.0)),
        svg(svg_content).style(move |s| s.width(size_px).height(size_px)),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn page_content(theme: &Theme, reduced_motion: bool) -> impl IntoView + use<> {
    let sizes = [
        ("Sm", SpinnerSize::Sm),
        ("Md", SpinnerSize::Md),
        ("Lg", SpinnerSize::Lg),
        ("Xl", SpinnerSize::Xl),
    ];

    let sm = Spinner::new().size(SpinnerSize::Sm).reduced_motion(reduced_motion).resolve(theme, 45.0);
    let md = Spinner::new().size(SpinnerSize::Md).reduced_motion(reduced_motion).resolve(theme, 45.0);
    let lg = Spinner::new().size(SpinnerSize::Lg).reduced_motion(reduced_motion).resolve(theme, 45.0);
    let xl = Spinner::new().size(SpinnerSize::Xl).reduced_motion(reduced_motion).resolve(theme, 45.0);

    let accent_spinner = Spinner::new().size(SpinnerSize::Lg).reduced_motion(reduced_motion).resolve(theme, 90.0);
    let danger_spinner = {
        use katana_ui_widget::theme::color::Color;
        Spinner::new()
            .size(SpinnerSize::Lg)
            .color_override(Color {
                r: theme.color.danger.r,
                g: theme.color.danger.g,
                b: theme.color.danger.b,
                a: 255,
            })
            .reduced_motion(reduced_motion)
            .resolve(theme, 135.0)
    };

    let _ = sizes;

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Spinner Sizes (static snapshot, animation via timer)").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            spinner_row("Sm (8px)", sm.svg_content, sm.size_px),
            spinner_row("Md (12px)", md.svg_content, md.size_px),
            spinner_row("Lg (16px)", lg.svg_content, lg.size_px),
            spinner_row("Xl (24px)", xl.svg_content, xl.size_px),
            label(|| "Color overrides").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            spinner_row("accent color", accent_spinner.svg_content, accent_spinner.size_px),
            spinner_row("danger color", danger_spinner.svg_content, danger_spinner.size_px),
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

pub fn spinner_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);
    let reduced = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Spinner Primitive").style(|s| s.font_size(20.0)),
            label(move || if is_dark.get() { "Dark" } else { "Light" }),
            toggle_button(move || is_dark.get()).on_toggle(move |v| is_dark.set(v)),
            label(|| "Reduced motion"),
            toggle_button(move || reduced.get()).on_toggle(move |v| reduced.set(v)),
        ))
        .style(|s| s.gap(12.0).items_center().padding(12.0)),
        dyn_container(
            move || (is_dark.get(), reduced.get()),
            move |(dark, reduce)| {
                let theme = if dark {
                    Theme::default_dark()
                } else {
                    Theme::default_light()
                };
                page_content(&theme, reduce)
            },
        ),
    ))
}
