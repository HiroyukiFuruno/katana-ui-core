use floem::peniko::Color as PenikoColor;
use floem::views::{
    button, container, dyn_container, empty, h_stack, label, scroll, v_stack, v_stack_from_iter,
    Decorators,
};
use floem::IntoView;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use katana_ui_widget::layout::popover::{AnchorRect, Placement, Popover};
use katana_ui_widget::theme::Theme;

fn placement_row(
    label_text: &'static str,
    placement: Placement,
    origin_x: f32,
    origin_y: f32,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
) -> impl IntoView {
    let origin_str = format!("({:.0}, {:.0})", origin_x, origin_y);
    let placement_tag: &'static str = match placement {
        Placement::Bottom => "[Bottom]",
        Placement::Top => "[Top]",
        Placement::Start => "[Start]",
        Placement::End => "[End]",
    };
    let bg = PenikoColor::rgb8(bg_r, bg_g, bg_b);
    v_stack((
        label(move || label_text).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            label(move || placement_tag).style(|s| s.font_size(10.0).margin_right(4.0)),
            label(move || "")
                .style(move |s| s.width(60.0).height(16.0).background(bg).border(0.5)),
            label(move || origin_str.clone()).style(|s| s.font_size(10.0).margin_left(4.0)),
        ))
        .style(|s| s.items_center().gap(2.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let anchor = AnchorRect { x: 200.0, y: 200.0, width: 100.0, height: 40.0 };
    let anchor_edge = AnchorRect { x: 700.0, y: 550.0, width: 80.0, height: 32.0 };
    let is_open = create_rw_signal(false);
    let dismiss_outside = create_rw_signal(true);
    let dismiss_esc = create_rw_signal(true);
    let placement = create_rw_signal(Placement::Bottom);
    let log = create_rw_signal("待機中".to_string());
    let page_theme = theme.clone();
    let interactive_theme = page_theme.clone();

    let p_bottom = Popover::new().placement(Placement::Bottom).resolve(&page_theme);
    let p_top = Popover::new().placement(Placement::Top).resolve(&page_theme);
    let p_start = Popover::new().placement(Placement::Start).resolve(&page_theme);
    let p_end = Popover::new().placement(Placement::End).resolve(&page_theme);

    let o_bottom =
        Popover::new().placement(Placement::Bottom).compute_origin(anchor, 120.0, 60.0, 800.0, 600.0);
    let o_top =
        Popover::new().placement(Placement::Top).compute_origin(anchor, 120.0, 60.0, 800.0, 600.0);
    let o_start =
        Popover::new().placement(Placement::Start).compute_origin(anchor, 120.0, 60.0, 800.0, 600.0);
    let o_end =
        Popover::new().placement(Placement::End).compute_origin(anchor, 120.0, 60.0, 800.0, 600.0);

    let o_flip_bottom =
        Popover::new().placement(Placement::Bottom).compute_origin(anchor_edge, 120.0, 60.0, 800.0, 600.0);
    let o_flip_end =
        Popover::new().placement(Placement::End).compute_origin(anchor_edge, 120.0, 60.0, 800.0, 600.0);

    let bg = PenikoColor::rgb8(page_theme.color.bg.r, page_theme.color.bg.g, page_theme.color.bg.b);
    let text_col = PenikoColor::rgb8(page_theme.color.text.r, page_theme.color.text.g, page_theme.color.text.b);

    let interactive = dyn_container(
        move || (is_open.get(), dismiss_outside.get(), dismiss_esc.get(), placement.get()),
        move |(open_now, _, _, placement_now)| {
            let pop = Popover::new()
                .open(open_now)
                .placement(placement_now)
                .dismiss_on_outside_click(dismiss_outside.get())
                .dismiss_on_esc(dismiss_esc.get())
                .on_close({
                    let is_open = is_open;
                    let log = log;
                    move || {
                        is_open.set(false);
                        log.set("closed".to_string());
                    }
                })
                .resolve(&interactive_theme);
            let origin = Popover::new()
                .placement(placement_now)
                .open(open_now)
                .compute_origin(anchor, 120.0, 60.0, 800.0, 600.0);
            let pop_for_outside = pop.clone();
            let pop_for_esc = pop.clone();
            let overlay = pop.overlay_layout(220.0, 120.0, 800.0, 600.0);
            let menu_button_colors = (pop.popover_bg.r, pop.popover_bg.g, pop.popover_bg.b);
            let pop_border = PenikoColor::rgb8(
                pop.popover_border.r,
                pop.popover_border.g,
                pop.popover_border.b,
            );
    let menu_rows = if open_now {
        ["開く", "編集", "移動", "削除", "共有"]
            .into_iter()
            .map(|text| {
                let text = text.to_string();
                let label_text = text.clone();
                let action_text = text;
                button(label(move || label_text.clone()))
                    .action({
                        let log = log;
                        move || {
                                    log.set(format!("menu clicked: {action_text}"));
                                }
                            })
                            .style({
                                let menu_button_colors = menu_button_colors;
                                move |s| {
                                    s.padding(4.0)
                                        .width(140.0)
                                        .background(PenikoColor::rgb8(
                                            menu_button_colors.0,
                                            menu_button_colors.1,
                                            menu_button_colors.2,
                                        ))
                                        .border(1.0)
                                        .border_color(pop_border)
                                }
                            })
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let overlay_box = if let Some(layer) = overlay {
                let overlay_bg = PenikoColor::rgb8(layer.popover_bg.r, layer.popover_bg.g, layer.popover_bg.b);
                let overlay_border = PenikoColor::rgb8(
                    layer.popover_border.r,
                    layer.popover_border.g,
                    layer.popover_border.b,
                );
                container(
                    v_stack((
                        label(move || "メニュー項目".to_string()).style(|s| s.font_size(11.0)),
                        v_stack_from_iter(menu_rows),
                    ))
                    .style(move |s| {
                        s.padding(8.0)
                            .background(overlay_bg)
                            .border(1.0)
                            .border_color(overlay_border)
                            .border_radius(layer.corner_radius)
                            .width(layer.width)
                    }),
                )
                .style(|s| s.width(220.0).height(120.0))
            } else {
                container(empty()).style(|s| s.width(0.0).height(0.0))
            };
            let overlay_status = match overlay {
                Some(layer) => format!(
                    "overlay=({}, {}), size=({}, {}), placement={:?}",
                    layer.x as i32, layer.y as i32, layer.width as i32, layer.height as i32, layer.placement
                ),
                None => "overlay=hidden".to_string(),
            };
            let state = if open_now { "open" } else { "closed" };
            v_stack((
                label(|| "Live state").style(|s| s.font_size(13.0)),
                h_stack((
                    button(label(|| "Open")).action({
                        let is_open = is_open;
                        let log = log;
                        move || {
                            is_open.set(true);
                            log.set("opened".to_string());
                        }
                    }),
                    button(label(|| "Close (outside)")).action({
                        let is_open = is_open;
                        let log = log;
                        let pop = pop_for_outside;
                        move || {
                            if pop.close_with_outside_click() {
                                is_open.set(false);
                                log.set("outside_click: closed".to_string());
                            } else {
                                log.set("outside_click: ignored".to_string());
                            }
                        }
                    }),
                    button(label(|| "Close (Esc)")).action({
                        let is_open = is_open;
                        let log = log;
                        let pop = pop_for_esc;
                        move || {
                            if pop.close_with_esc() {
                                is_open.set(false);
                                log.set("esc: closed".to_string());
                            } else {
                                log.set("esc: ignored".to_string());
                            }
                        }
                    }),
                ))
                .style(|s| s.gap(6.0)),
                label(move || overlay_status.clone()).style(|s| s.font_size(11.0)),
                h_stack((
                    button(label(|| "Bottom")).action({
                        let placement = placement;
                        let log = log;
                        move || {
                            placement.set(Placement::Bottom);
                            log.set("placement = Bottom".to_string());
                        }
                    }),
                    button(label(|| "Top")).action({
                        let placement = placement;
                        let log = log;
                        move || {
                            placement.set(Placement::Top);
                            log.set("placement = Top".to_string());
                        }
                    }),
                    button(label(|| "Start")).action({
                        let placement = placement;
                        let log = log;
                        move || {
                            placement.set(Placement::Start);
                            log.set("placement = Start".to_string());
                        }
                    }),
                    button(label(|| "End")).action({
                        let placement = placement;
                        let log = log;
                        move || {
                            placement.set(Placement::End);
                            log.set("placement = End".to_string());
                        }
                    }),
                ))
                .style(|s| s.gap(6.0)),
                label(move || format!("state={state}, placement={:?}", placement_now)).style(|s| s.font_size(11.0)),
                overlay_box,
                label(move || {
                    format!(
                        "origin=({}, {}), bg=({}, {}, {}), dismiss=outside({}) esc({})",
                        origin.x as i32, origin.y as i32, pop.popover_bg.r, pop.popover_bg.g, pop.popover_bg.b, pop.dismiss_on_outside_click, pop.dismiss_on_esc
                    )
                })
                .style(|s| s.font_size(11.0)),
                label(move || format!("log: {}", log.get()))
                    .style(|s| s.font_size(11.0)),
            ))
            .style(|s| s.gap(8.0))
        },
    );

    scroll(
        v_stack((
            label(|| "Popover Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            Popover::new()
                .children("Popover::view の内容")
                .view(theme.clone(), "Toggle popover"),
            label(|| "Placements (anchor at 200,200)").style(|s| s.font_size(13.0).margin_bottom(4.0)),
            placement_row("Bottom", p_bottom.placement, o_bottom.x, o_bottom.y, p_bottom.popover_bg.r, p_bottom.popover_bg.g, p_bottom.popover_bg.b),
            placement_row("Top", p_top.placement, o_top.x, o_top.y, p_top.popover_bg.r, p_top.popover_bg.g, p_top.popover_bg.b),
            placement_row("Start", p_start.placement, o_start.x, o_start.y, p_start.popover_bg.r, p_start.popover_bg.g, p_start.popover_bg.b),
            placement_row("End", p_end.placement, o_end.x, o_end.y, p_end.popover_bg.r, p_end.popover_bg.g, p_end.popover_bg.b),
            label(|| "Auto-flip (anchor near bottom-right edge 700,550)").style(|s| s.font_size(13.0).margin_top(8.0).margin_bottom(4.0)),
            placement_row("Bottom→flipped", Placement::Bottom, o_flip_bottom.x, o_flip_bottom.y, p_bottom.popover_bg.r, p_bottom.popover_bg.g, p_bottom.popover_bg.b),
            placement_row("End→flipped", Placement::End, o_flip_end.x, o_flip_end.y, p_end.popover_bg.r, p_end.popover_bg.g, p_end.popover_bg.b),
            label(|| "dismiss_on_outside_click / dismiss_on_esc governed by props").style(|s| s.font_size(11.0).margin_top(8.0)),
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

pub fn popover_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
