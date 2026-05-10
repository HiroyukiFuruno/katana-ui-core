use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{container, dyn_container, empty, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::primitive::spinner::{Spinner, SpinnerSize};
use katana_ui_widget::theme::Theme;

fn spinner_row(desc: &'static str, spinner: impl IntoView + 'static) -> impl IntoView {
    h_stack((
        label(move || desc).style(|s| s.width(160.0).font_size(11.0)),
        spinner,
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn page_content(theme: Theme, reduced_motion: bool, is_visible: bool) -> impl IntoView + use<> {
    let sizes = [
        ("Sm", SpinnerSize::Sm),
        ("Md", SpinnerSize::Md),
        ("Lg", SpinnerSize::Lg),
        ("Xl", SpinnerSize::Xl),
    ];

    let sm = Spinner::new().size(SpinnerSize::Sm).reduced_motion(reduced_motion).view(theme.clone());
    let md = Spinner::new().size(SpinnerSize::Md).reduced_motion(reduced_motion).view(theme.clone());
    let lg = Spinner::new().size(SpinnerSize::Lg).reduced_motion(reduced_motion).view(theme.clone());
    let xl = Spinner::new().size(SpinnerSize::Xl).reduced_motion(reduced_motion).view(theme.clone());

    let accent_spinner = Spinner::new().size(SpinnerSize::Lg).reduced_motion(reduced_motion).view(theme.clone());
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
            .view(theme.clone())
    };

    let _ = sizes;

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Spinner Sizes (widget owns rotation)").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            spinner_row("Sm (8px)", sm),
            spinner_row("Md (12px)", md),
            spinner_row("Lg (16px)", lg),
            spinner_row("Xl (24px)", xl),
            label(|| "Color overrides").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            spinner_row("accent color", accent_spinner),
            spinner_row("danger color", danger_spinner),
            label(|| "Visibility control").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            if is_visible {
                container(Spinner::new().size(SpinnerSize::Lg).view(theme.clone())).into_any()
            } else {
                container(empty()).style(|s| s.width(16.0).height(16.0)).into_any()
            },
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

pub fn spinner_page(theme: Theme) -> impl IntoView {
    let reduced = create_rw_signal(false);
    let visible = create_rw_signal(true);

    v_stack((
        h_stack((
            label(|| "Spinner Primitive").style(|s| s.font_size(20.0)),
            label(|| "Reduced motion"),
            toggle_button(move || reduced.try_get().unwrap_or(false))
                .on_toggle(move |v| reduced.set(v)),
            label(|| "Visible"),
            toggle_button(move || visible.try_get().unwrap_or(false))
                .on_toggle(move |v| visible.set(v)),
        ))
        .style(|s| s.gap(12.0).items_center().padding(12.0)),
        dyn_container(
            move || {
                (
                    reduced.try_get().unwrap_or(false),
                    visible.try_get().unwrap_or(false),
                )
            },
            move |(reduce, is_visible)| page_content(theme.clone(), reduce, is_visible),
        ),
    ))
}
