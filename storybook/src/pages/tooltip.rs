use floem::peniko::Color as PenikoColor;
use floem::views::{
    button, container, dyn_container, empty, h_stack, label, scroll, v_stack, Decorators,
};
use floem::IntoView;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use katana_ui_widget::composite::indicator::tooltip::{Placement as TooltipPlacement, Tooltip};
use katana_ui_widget::layout::popover::{AnchorRef, AnchorRect, Placement as PopoverPlacement, Popover};
use katana_ui_widget::theme::Theme;

fn tooltip_row(
    heading: &'static str,
    lbl: String,
    placement_tag: &'static str,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    text_r: u8,
    text_g: u8,
    text_b: u8,
    font_sz: f32,
) -> impl IntoView {
    let bg = PenikoColor::rgb8(bg_r, bg_g, bg_b);
    let text_color = PenikoColor::rgb8(text_r, text_g, text_b);
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            label(move || placement_tag).style(|s| s.font_size(10.0).margin_right(4.0)),
            label(move || lbl.clone())
                .style(move |s| s.background(bg).color(text_color).font_size(font_sz).padding(4.0)),
        ))
        .style(|s| s.items_center()),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_top = Tooltip::new("Top tooltip").placement(TooltipPlacement::Top).resolve(theme);
    let r_bottom = Tooltip::new("Bottom tooltip").placement(TooltipPlacement::Bottom).resolve(theme);
    let r_start =
        Tooltip::new("Start (left) tooltip").placement(TooltipPlacement::Start).resolve(theme);
    let r_end = Tooltip::new("End (right) tooltip").placement(TooltipPlacement::End).resolve(theme);
    let r_long = Tooltip::new("This is a longer tooltip text that may need wrapping at max width").resolve(theme);
    let r_fast = Tooltip::new("Fast tooltip").delay_ms(0).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let placement_str = |p: TooltipPlacement| match p {
        TooltipPlacement::Top => "[▲ Top]",
        TooltipPlacement::Bottom => "[▼ Bottom]",
        TooltipPlacement::Start => "[◀ Start]",
        TooltipPlacement::End => "[▶ End]",
    };

    let tooltip_open = create_rw_signal(false);
    let popover_placement = create_rw_signal(PopoverPlacement::Bottom);
    let tooltip_log = create_rw_signal("ready".to_string());
    let theme = theme.clone();
    let popover_theme = theme.clone();
    let anchor = AnchorRect {
        x: 210.0,
        y: 200.0,
        width: 160.0,
        height: 32.0,
    };

    let to_row =
        |heading: &'static str, r: katana_ui_widget::composite::indicator::tooltip::ResolvedTooltip| {
            let ptag = placement_str(r.placement);
            tooltip_row(
                heading,
                r.label,
                ptag,
                r.bg_color.r,
                r.bg_color.g,
                r.bg_color.b,
                r.text_color.r,
                r.text_color.g,
                r.text_color.b,
                r.font_size,
            )
        };

    let interactive = dyn_container(
        move || (tooltip_open.get(), popover_placement.get()),
        move |(open_now, placement_now)| {
            let pop = Popover::new()
                .open(open_now)
                .placement(placement_now)
                .anchor(AnchorRef::new(anchor))
                .children("説明文を表示")
                .on_close({
                    let open = tooltip_open;
                    let log = tooltip_log;
                    move || {
                        open.set(false);
                        log.set("closed".to_string());
                    }
                })
                .resolve(&popover_theme);

            let overlay = pop.overlay_layout(240.0, 58.0, 800.0, 600.0);
            let pop_bg = PenikoColor::rgb8(pop.popover_bg.r, pop.popover_bg.g, pop.popover_bg.b);
            let pop_border = PenikoColor::rgb8(
                pop.popover_border.r,
                pop.popover_border.g,
                pop.popover_border.b,
            );
            let pop_for_outside = pop.clone();
            let pop_for_esc = pop.clone();

            let overlay_box = if let Some(layer) = overlay {
                container(
                    label(|| "説明文"
                        .to_string())
                        .style(move |s| {
                            s.width(layer.width)
                                .height(layer.height)
                                .padding(6.0)
                                .background(pop_bg)
                                .border(1.0)
                                .border_color(pop_border)
                                .border_radius(layer.corner_radius)
                                .color(PenikoColor::rgb8(
                                    popover_theme.color.text.r,
                                    popover_theme.color.text.g,
                                    popover_theme.color.text.b,
                                ))
                                .font_size(11.0)
                        }),
                )
                .style(|s| s.width(240.0).height(58.0))
            } else {
                container(empty()).style(|s| s.width(0.0).height(0.0))
            };

            let overlay_status = match overlay {
                Some(layer) => format!(
                    "placement={:?}, pos=({}, {}), size=({}, {})",
                    layer.placement, layer.x as i32, layer.y as i32, layer.width as i32, layer.height as i32
                ),
                None => "closed".to_string(),
            };

            v_stack((
                label(|| "Interactive tooltip sample".to_string())
                    .style(|s| s.font_size(12.0).margin_bottom(2.0)),
                h_stack((
                    button(label(|| "Open"))
                        .action({
                            let open = tooltip_open;
                            move || open.update(|is_open| *is_open = true)
                        }),
                    button(label(|| "Close (outside)"))
                        .action({
                            let open = tooltip_open;
                            let log = tooltip_log;
                            let pop = pop_for_outside;
                            move || {
                                if pop.close_with_outside_click() {
                                    open.set(false);
                                    log.set("outside close".to_string());
                                } else {
                                    log.set("outside ignored".to_string());
                                }
                            }
                        }),
                    button(label(|| "Close (esc)"))
                        .action({
                            let open = tooltip_open;
                            let log = tooltip_log;
                            let pop = pop_for_esc;
                            move || {
                                if pop.close_with_esc() {
                                    open.set(false);
                                    log.set("esc close".to_string());
                                } else {
                                    log.set("esc ignored".to_string());
                                }
                            }
                        }),
                ))
                .style(|s| s.gap(6.0)),
                h_stack((
                    button(label(|| "Bottom")).action({
                        let placement = popover_placement;
                        move || placement.set(PopoverPlacement::Bottom)
                    }),
                    button(label(|| "Top")).action({
                        let placement = popover_placement;
                        move || placement.set(PopoverPlacement::Top)
                    }),
                    button(label(|| "Start")).action({
                        let placement = popover_placement;
                        move || placement.set(PopoverPlacement::Start)
                    }),
                    button(label(|| "End")).action({
                        let placement = popover_placement;
                        move || placement.set(PopoverPlacement::End)
                    }),
                ))
                .style(|s| s.gap(6.0)),
                label(move || overlay_status.clone()).style(|s| s.font_size(11.0)),
                label(move || format!("log: {}", tooltip_log.get())).style(|s| s.font_size(11.0)),
                overlay_box,
            ))
            .style(|s| s.gap(4.0))
        },
    );

    scroll(
        v_stack((
            label(|| "Tooltip Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget: hover/focus the label below").style(|s| s.font_size(13.0)),
            Tooltip::new("Live tooltip").view(
                theme.clone(),
                label(|| "Hover me").style(|s| s.padding(6.0).border(1.0)),
            ),
            to_row("Top placement", r_top),
            to_row("Bottom placement", r_bottom),
            to_row("Start placement", r_start),
            to_row("End placement", r_end),
            to_row("Long text (max_width wrap)", r_long),
            to_row("Fast (delay=0ms)", r_fast),
            interactive,
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

pub fn tooltip_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
