use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, container, dyn_container, h_stack, label, scroll, v_stack};
use katana_ui_widget::composite::indicator::tooltip::{Tooltip, TooltipPlacement};
use katana_ui_widget::theme::Theme;

fn placement_label(placement: TooltipPlacement) -> &'static str {
    match placement {
        TooltipPlacement::Top => "Top",
        TooltipPlacement::Bottom => "Bottom",
        TooltipPlacement::Left => "Left",
        TooltipPlacement::Right => "Right",
    }
}

fn should_start_open() -> bool {
    let should_open = crate::interaction::requested("open");
    if should_open {
        crate::interaction::mark_supported("tooltip", "open");
        crate::interaction::mark_exercised("tooltip", "open", "initial-visible");
    }
    should_open
}

fn placement_button(
    text: &'static str,
    next: TooltipPlacement,
    placement: RwSignal<TooltipPlacement>,
) -> impl IntoView {
    button(label(move || text)).action(move || placement.set(next))
}

fn placement_controls(placement: RwSignal<TooltipPlacement>) -> impl IntoView {
    v_stack((
        h_stack((
            placement_button("Top", TooltipPlacement::Top, placement),
            placement_button("Bottom", TooltipPlacement::Bottom, placement),
            placement_button("Left", TooltipPlacement::Left, placement),
            placement_button("Right", TooltipPlacement::Right, placement),
        ))
        .style(|style| style.gap(8.0)),
    ))
    .style(|style| style.gap(8.0))
}

fn tooltip_target(
    theme: Theme,
    placement: RwSignal<TooltipPlacement>,
    show_arrow: RwSignal<bool>,
) -> impl IntoView {
    let sample_theme = theme.clone();

    dyn_container(
        move || (placement.get(), show_arrow.get()),
        move |(current_placement, arrow)| {
            Tooltip::new(format!(
                "{} に表示します",
                placement_label(current_placement)
            ))
            .placement(current_placement)
            .delay_ms(0)
            .max_width(220.0)
            .show_arrow(arrow)
            .visible(should_start_open())
            .view(sample_theme.clone(), button(label(|| "対象ボタン")))
        },
    )
}

pub fn tooltip_page(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let border = PenikoColor::rgb8(
        theme.color.border.r,
        theme.color.border.g,
        theme.color.border.b,
    );
    let muted = PenikoColor::rgb8(
        theme.color.text_muted.r,
        theme.color.text_muted.g,
        theme.color.text_muted.b,
    );

    let placement = create_rw_signal(TooltipPlacement::Top);
    let show_arrow = create_rw_signal(true);
    let sample = tooltip_target(theme.clone(), placement, show_arrow);

    scroll(
        v_stack((
            label(|| "Tooltip").style(|style| style.font_size(20.0).margin_bottom(8.0)),
            label(|| "対象ボタンにマウスを乗せるか、入力対象にすると説明が出ます。")
                .style(move |style| style.font_size(12.0).color(muted)),
            label(|| "表示位置").style(|style| style.font_size(14.0).margin_top(8.0)),
            placement_controls(placement),
            h_stack((
                button(label(move || {
                    if show_arrow.get() {
                        "矢印あり"
                    } else {
                        "矢印なし"
                    }
                }))
                .action(move || show_arrow.set(!show_arrow.get())),
                label(move || format!("現在: {}", placement_label(placement.get())))
                    .style(|style| style.font_size(12.0)),
            ))
            .style(|style| style.gap(10.0).items_center()),
            container(
                v_stack((
                    label(|| "確認エリア").style(|style| style.font_size(12.0)),
                    sample,
                    button(label(|| "blur 確認用ボタン")),
                ))
                .style(|style| style.gap(18.0).padding(18.0)),
            )
            .style(move |style| {
                style
                    .width(560.0)
                    .height(260.0)
                    .border(1.0)
                    .border_color(border)
                    .border_radius(4.0)
            }),
        ))
        .style(move |style| {
            style
                .gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .min_width_full()
        }),
    )
    .style(|style| style.width_full().flex_grow(1.0))
}
