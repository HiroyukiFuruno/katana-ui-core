use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, button, dyn_container, h_stack, label, scroll, v_stack, v_stack_from_iter,
};
use floem::{IntoView, View};
use katana_ui_widget::layout::popover::{AnchorRect, AnchorRef, Placement, Popover};
use katana_ui_widget::theme::Theme;

fn placement_row(title: &'static str, placement: Placement, x: f32, y: f32) -> impl IntoView {
    let tag = match placement {
        Placement::Bottom => "Bottom",
        Placement::Top => "Top",
        Placement::Left => "Left",
        Placement::Right => "Right",
    };
    v_stack((
        label(move || title).style(|s| s.font_size(12.0)),
        h_stack((
            label(move || tag.to_string()).style(|s| s.font_size(11.0)),
            label(move || format!("({x:.0}, {y:.0})")).style(|s| s.font_size(11.0)),
        ))
        .style(|s| s.gap(4.0)),
    ))
}

fn anchor_presets() -> [AnchorRect; 2] {
    [
        AnchorRect {
            x: 200.0,
            y: 210.0,
            width: 120.0,
            height: 40.0,
        },
        AnchorRect {
            x: 700.0,
            y: 535.0,
            width: 100.0,
            height: 36.0,
        },
    ]
}

fn should_start_open() -> bool {
    let should_open = crate::interaction::requested("open");
    if should_open {
        crate::interaction::mark_supported("popover", "open");
    }
    should_open
}

fn bind_replay_open(is_open: floem::reactive::RwSignal<bool>) {
    if crate::interaction::requested("replay-open") {
        crate::interaction::mark_supported("popover", "replay-open");
        crate::interaction::schedule_replay(move || {
            is_open.set(true);
            crate::interaction::mark_exercised("popover", "replay-open", "signal-open");
        });
    }
}

fn popover_content(
    offset: f32,
    width: f32,
    focus_state: floem::reactive::RwSignal<String>,
    log: floem::reactive::RwSignal<String>,
) -> impl Fn() -> Box<dyn View> + 'static {
    move || {
        let menu = v_stack_from_iter(
            ["新規", "編集", "削除", "共有"]
                .into_iter()
                .map(|text| {
                    let text_label = text.to_string();
                    let log_label = text.to_string();
                    let log = log;
                    button(label(move || text_label.clone())).action({
                        let log = log;
                        let log_label = log_label;
                        let focus_state = focus_state;
                        move || {
                            log.set(format!("menu clicked: {log_label}"));
                            focus_state.set("menu clicked".to_string());
                        }
                    })
                })
                .collect::<Vec<_>>(),
        );
        let content = v_stack((
            label(move || format!("offset={offset}, width={width}")),
            label(|| "menu".to_string()).style(|s| s.font_size(12.0)),
            menu,
            label(|| "card".to_string()).style(|s| s.font_size(12.0)),
            label(|| "form".to_string()).style(|s| s.font_size(12.0)),
        ))
        .style(|s| s.gap(6.0).padding(8.0));

        Box::new(content)
    }
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let is_open = create_rw_signal(should_start_open());
    bind_replay_open(is_open);
    let dismiss_outside = create_rw_signal(true);
    let dismiss_esc = create_rw_signal(true);
    let placement = create_rw_signal(Placement::Bottom);
    let offset = create_rw_signal(4.0);
    let width = create_rw_signal(220.0);
    let anchor_index = create_rw_signal(0usize);
    let focus_state = create_rw_signal("未フォーカス".to_string());
    let log = create_rw_signal("待機中".to_string());
    let page_theme = theme.clone();
    let anchors = anchor_presets();

    let live = dyn_container(
        move || {
            (
                is_open.get(),
                dismiss_outside.get(),
                dismiss_esc.get(),
                placement.get(),
                offset.get(),
                width.get(),
                anchor_index.get(),
            )
        },
        move |(_, _, _, _, _, _, active_anchor)| {
            if is_open.get() && crate::interaction::requested("replay-open") {
                crate::interaction::mark_exercised("popover", "replay-open", "render-open");
            }
            if is_open.get() && crate::interaction::requested("open") {
                crate::interaction::mark_exercised("popover", "open", "render-open");
            }
            let anchor = anchors[active_anchor];
            let resolved = Popover::new()
                .open(is_open.get())
                .placement(placement.get())
                .offset(offset.get())
                .width(width.get())
                .anchor(AnchorRef::new(anchor))
                .children(popover_content(offset.get(), width.get(), focus_state, log))
                .dismiss_on_outside_click(dismiss_outside.get())
                .dismiss_on_esc(dismiss_esc.get())
                .on_focus_in({
                    let focus_state = focus_state;
                    move || focus_state.set("popover focus in".to_string())
                })
                .on_focus_out({
                    let focus_state = focus_state;
                    move || focus_state.set("popover focus out".to_string())
                })
                .on_close({
                    let is_open = is_open;
                    let focus_state = focus_state;
                    let log = log;
                    move || {
                        is_open.set(false);
                        focus_state.set("closed".to_string());
                        log.set("on_close".to_string());
                    }
                })
                .resolve(&page_theme);

            let close_by_outside = resolved.clone();
            let close_by_esc = resolved.clone();

            let origin = resolved.overlay_layout(width.get(), 140.0, 1024.0, 768.0);
            let overlay_text = match origin {
                Some(layer) => format!(
                    "overlay=({:.0}, {:.0}) size=({:.0}, {:.0}) {:?}",
                    layer.x, layer.y, layer.width, layer.height, layer.placement
                ),
                None => "overlay=hidden".to_string(),
            };
            let computed_origin = Popover::new()
                .placement(placement.get())
                .offset(offset.get())
                .compute_origin(anchor, width.get(), 140.0, 1024.0, 768.0);

            let placement_controls = h_stack((
                button(label(|| "Bottom")).action({
                    let placement = placement;
                    let log = log;
                    move || {
                        placement.set(Placement::Bottom);
                        log.set("placement: Bottom".to_string());
                    }
                }),
                button(label(|| "Top")).action({
                    let placement = placement;
                    let log = log;
                    move || {
                        placement.set(Placement::Top);
                        log.set("placement: Top".to_string());
                    }
                }),
                button(label(|| "Left")).action({
                    let placement = placement;
                    let log = log;
                    move || {
                        placement.set(Placement::Left);
                        log.set("placement: Left".to_string());
                    }
                }),
                button(label(|| "Right")).action({
                    let placement = placement;
                    let log = log;
                    move || {
                        placement.set(Placement::Right);
                        log.set("placement: Right".to_string());
                    }
                }),
            ))
            .style(|s| s.gap(4.0));

            let geom_controls = h_stack((
                button(label(|| "offset-")).action({
                    let offset = offset;
                    move || offset.set((offset.get() - 4.0).max(-20.0))
                }),
                button(label(|| "offset+")).action({
                    let offset = offset;
                    move || offset.set((offset.get() + 4.0).min(40.0))
                }),
                button(label(|| "offset=0")).action({
                    let offset = offset;
                    move || offset.set(0.0)
                }),
            ))
            .style(|s| s.gap(4.0));

            let width_controls = h_stack((
                button(label(|| "width-")).action({
                    let width = width;
                    move || width.set((width.get() - 40.0).max(160.0))
                }),
                button(label(|| "width+")).action({
                    let width = width;
                    move || width.set((width.get() + 40.0).min(360.0))
                }),
            ))
            .style(|s| s.gap(4.0));

            let dismiss_controls = h_stack((
                button(label(|| "outside on")).action({
                    let dismiss_outside = dismiss_outside;
                    let log = log;
                    move || {
                        dismiss_outside.set(true);
                        log.set("dismiss outside on".to_string());
                    }
                }),
                button(label(|| "outside off")).action({
                    let dismiss_outside = dismiss_outside;
                    let log = log;
                    move || {
                        dismiss_outside.set(false);
                        log.set("dismiss outside off".to_string());
                    }
                }),
                button(label(|| "esc on")).action({
                    let dismiss_esc = dismiss_esc;
                    let log = log;
                    move || {
                        dismiss_esc.set(true);
                        log.set("dismiss esc on".to_string());
                    }
                }),
                button(label(|| "esc off")).action({
                    let dismiss_esc = dismiss_esc;
                    let log = log;
                    move || {
                        dismiss_esc.set(false);
                        log.set("dismiss esc off".to_string());
                    }
                }),
            ))
            .style(|s| s.gap(4.0));

            let close_controls = h_stack((
                button(label(|| "Open")).action({
                    let is_open = is_open;
                    let log = log;
                    move || {
                        is_open.set(true);
                        log.set("open".to_string());
                    }
                }),
                button(label(|| "Close via outside API")).action({
                    let is_open = is_open;
                    let log = log;
                    move || {
                        if close_by_outside.close_with_outside_click() {
                            is_open.set(false);
                            log.set("outside click close".to_string());
                        } else {
                            log.set("outside click ignored".to_string());
                        }
                    }
                }),
                button(label(|| "Close via Esc API")).action({
                    let is_open = is_open;
                    let log = log;
                    move || {
                        if close_by_esc.close_with_esc() {
                            is_open.set(false);
                            log.set("esc close".to_string());
                        } else {
                            log.set("esc ignored".to_string());
                        }
                    }
                }),
            ))
            .style(|s| s.gap(4.0));

            v_stack((
                label(move || {
                    format!(
                        "state open={} anchor={}",
                        is_open.get(),
                        if active_anchor == 0 { "center" } else { "edge" }
                    )
                })
                .style(|s| s.font_size(11.0)),
                Popover::new()
                    .open(is_open.get())
                    .placement(placement.get())
                    .offset(offset.get())
                    .width(width.get())
                    .anchor(AnchorRef::new(anchor))
                    .children(popover_content(offset.get(), width.get(), focus_state, log))
                    .dismiss_on_outside_click(dismiss_outside.get())
                    .dismiss_on_esc(dismiss_esc.get())
                    .on_focus_in({
                        let focus_state = focus_state;
                        move || focus_state.set("popover focus in".to_string())
                    })
                    .on_focus_out({
                        let focus_state = focus_state;
                        move || focus_state.set("popover focus out".to_string())
                    })
                    .on_close({
                        let is_open = is_open;
                        let focus_state = focus_state;
                        let log = log;
                        move || {
                            is_open.set(false);
                            focus_state.set("closed".to_string());
                            log.set("on_close".to_string());
                        }
                    })
                    .view(
                        page_theme.clone(),
                        if active_anchor == 0 {
                            "anchor center"
                        } else {
                            "anchor edge"
                        },
                    ),
                close_controls,
                placement_controls,
                geom_controls,
                width_controls,
                dismiss_controls,
                label(move || {
                    format!(
                        "computed origin ({:.0}, {:.0}) / {}",
                        computed_origin.x, computed_origin.y, overlay_text
                    )
                })
                .style(|s| s.font_size(11.0)),
                label(move || format!("focus: {}", focus_state.get())).style(|s| s.font_size(11.0)),
                label(move || format!("log: {}", log.get())).style(|s| s.font_size(11.0)),
            ))
            .style(|s| s.gap(8.0))
        },
    );

    let base_anchor = anchors[0];
    let o_bottom = Popover::new().compute_origin(base_anchor, 120.0, 60.0, 800.0, 600.0);
    let o_top = Popover::new().placement(Placement::Top).compute_origin(
        base_anchor,
        120.0,
        60.0,
        800.0,
        600.0,
    );
    let o_start = Popover::new().placement(Placement::Left).compute_origin(
        base_anchor,
        120.0,
        60.0,
        800.0,
        600.0,
    );
    let o_end = Popover::new().placement(Placement::Right).compute_origin(
        base_anchor,
        120.0,
        60.0,
        800.0,
        600.0,
    );
    let anchor_buttons = h_stack((
        button(label(|| "anchor center")).action({
            let anchor_index = anchor_index;
            let log = log;
            move || {
                anchor_index.set(0);
                log.set("anchor: center".to_string());
            }
        }),
        button(label(|| "anchor edge")).action({
            let anchor_index = anchor_index;
            let log = log;
            move || {
                anchor_index.set(1);
                log.set("anchor: edge".to_string());
            }
        }),
    ))
    .style(|s| s.gap(4.0));

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Popover").style(|s| s.font_size(16.0)),
            label(|| "Live sample").style(|s| s.font_size(13.0)),
            anchor_buttons,
            live,
            label(|| "Placement").style(|s| s.font_size(13.0)),
            placement_row("Bottom", Placement::Bottom, o_bottom.x, o_bottom.y),
            placement_row("Top", Placement::Top, o_top.x, o_top.y),
            placement_row("Left", Placement::Left, o_start.x, o_start.y),
            placement_row("Right", Placement::Right, o_end.x, o_end.y),
        ))
        .style(move |s| {
            s.gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .min_width_full()
        }),
    )
}

pub fn popover_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
