use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, button, container, dyn_container, empty, h_stack, label, scroll, v_stack,
};
use katana_ui_widget::primitive::spinner::{Spinner, SpinnerSize};
use katana_ui_widget::theme::Theme;

fn spinner_row(desc: &'static str, spinner: impl IntoView + 'static) -> impl IntoView {
    h_stack((
        label(move || desc).style(|s| s.width(160.0).font_size(11.0)),
        spinner,
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn state_button(
    label_text: &'static str,
    value: bool,
    on_click: impl Fn() + 'static,
) -> impl IntoView {
    button(label(move || {
        if value {
            format!("{label_text}: on")
        } else {
            format!("{label_text}: off")
        }
    }))
    .action(on_click)
    .style(|s| s.min_width(96.0))
}

fn page_content(
    theme: Theme,
    reduced: floem::reactive::RwSignal<bool>,
    visible: floem::reactive::RwSignal<bool>,
) -> impl IntoView + use<> {
    let reduced_motion = reduced.try_get().unwrap_or(false);
    let is_visible = visible.try_get().unwrap_or(false);
    let sizes = [
        ("Sm", SpinnerSize::Sm),
        ("Md", SpinnerSize::Md),
        ("Lg", SpinnerSize::Lg),
        ("Xl", SpinnerSize::Xl),
    ];

    let sm = Spinner::new()
        .size(SpinnerSize::Sm)
        .reduced_motion(reduced_motion)
        .view(theme.clone());
    let md = Spinner::new()
        .size(SpinnerSize::Md)
        .reduced_motion(reduced_motion)
        .view(theme.clone());
    let lg = Spinner::new()
        .size(SpinnerSize::Lg)
        .reduced_motion(reduced_motion)
        .view(theme.clone());
    let xl = Spinner::new()
        .size(SpinnerSize::Xl)
        .reduced_motion(reduced_motion)
        .view(theme.clone());

    let accent_spinner = Spinner::new()
        .size(SpinnerSize::Lg)
        .reduced_motion(reduced_motion)
        .view(theme.clone());
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
    let slow_spinner = Spinner::new()
        .size(SpinnerSize::Lg)
        .speed_rps(0.35)
        .reduced_motion(reduced_motion)
        .view(theme.clone());
    let normal_spinner = Spinner::new()
        .size(SpinnerSize::Lg)
        .speed_rps(1.0)
        .reduced_motion(reduced_motion)
        .view(theme.clone());
    let fast_spinner = Spinner::new()
        .size(SpinnerSize::Lg)
        .speed_rps(2.0)
        .reduced_motion(reduced_motion)
        .view(theme.clone());

    let _ = sizes;

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Spinner Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            h_stack((
                label(|| "Live controls").style(|s| s.font_size(13.0)),
                state_button("Reduced motion", reduced_motion, {
                    let reduced = reduced;
                    move || reduced.set(!reduced.try_get().unwrap_or(false))
                }),
                state_button("Visible", is_visible, {
                    let visible = visible;
                    move || visible.set(!visible.try_get().unwrap_or(false))
                }),
            ))
            .style(|s| s.gap(8.0).items_center()),
            label(|| "Size display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            spinner_row("Sm (8px)", sm),
            spinner_row("Md (12px)", md),
            spinner_row("Lg (16px)", lg),
            spinner_row("Xl (24px)", xl),
            label(|| "Color overrides")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            spinner_row("accent color", accent_spinner),
            spinner_row("danger color", danger_spinner),
            label(|| "Speed").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            spinner_row("slow speed_rps=0.35", slow_spinner),
            spinner_row("normal speed_rps=1.0", normal_spinner),
            spinner_row("fast speed_rps=2.0", fast_spinner),
            label(|| "Visibility control")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            if is_visible {
                container(Spinner::new().size(SpinnerSize::Lg).view(theme.clone())).into_any()
            } else {
                container(empty())
                    .style(|s| s.width(16.0).height(16.0))
                    .into_any()
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

    crate::interaction::replay("toggle-visible", "spinner", "visible-false", {
        let visible = visible;
        move || {
            visible.set(false);
        }
    });

    dyn_container(
        move || (reduced.get(), visible.get()),
        move |_| page_content(theme.clone(), reduced, visible),
    )
}
