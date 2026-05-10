use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
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
    let origin_str: &'static str =
        Box::leak(format!("({:.0}, {:.0})", origin_x, origin_y).into_boxed_str());
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
            label(move || origin_str).style(|s| s.font_size(10.0).margin_left(4.0)),
        ))
        .style(|s| s.items_center().gap(2.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let anchor = AnchorRect { x: 200.0, y: 200.0, width: 100.0, height: 40.0 };
    let anchor_edge = AnchorRect { x: 700.0, y: 550.0, width: 80.0, height: 32.0 };

    let p_bottom = Popover::new().placement(Placement::Bottom).resolve(theme);
    let p_top = Popover::new().placement(Placement::Top).resolve(theme);
    let p_start = Popover::new().placement(Placement::Start).resolve(theme);
    let p_end = Popover::new().placement(Placement::End).resolve(theme);

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

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Popover Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Placements (anchor at 200,200)").style(|s| s.font_size(13.0).margin_bottom(4.0)),
            placement_row("Bottom", p_bottom.placement, o_bottom.x, o_bottom.y, p_bottom.popover_bg.r, p_bottom.popover_bg.g, p_bottom.popover_bg.b),
            placement_row("Top", p_top.placement, o_top.x, o_top.y, p_top.popover_bg.r, p_top.popover_bg.g, p_top.popover_bg.b),
            placement_row("Start", p_start.placement, o_start.x, o_start.y, p_start.popover_bg.r, p_start.popover_bg.g, p_start.popover_bg.b),
            placement_row("End", p_end.placement, o_end.x, o_end.y, p_end.popover_bg.r, p_end.popover_bg.g, p_end.popover_bg.b),
            label(|| "Auto-flip (anchor near bottom-right edge 700,550)").style(|s| s.font_size(13.0).margin_top(8.0).margin_bottom(4.0)),
            placement_row("Bottom→flipped", Placement::Bottom, o_flip_bottom.x, o_flip_bottom.y, p_bottom.popover_bg.r, p_bottom.popover_bg.g, p_bottom.popover_bg.b),
            placement_row("End→flipped", Placement::End, o_flip_end.x, o_flip_end.y, p_end.popover_bg.r, p_end.popover_bg.g, p_end.popover_bg.b),
            label(|| "dismiss_on_outside_click / dismiss_on_esc governed by props").style(|s| s.font_size(11.0).margin_top(8.0)),
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

pub fn popover_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Popover").style(|s| s.font_size(20.0)),
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
