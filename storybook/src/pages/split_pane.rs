use floem::peniko::Color as PenikoColor;
use floem::views::{h_stack, label, scroll, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::layout::split::{Direction, SplitPane};
use katana_ui_widget::theme::Theme;

const DEMO_WIDTH: f32 = 360.0;
const DEMO_HEIGHT: f32 = 82.0;
const NESTED_WIDTH: f32 = 360.0;
const NESTED_HEIGHT: f32 = 138.0;

fn ratio_to_label(ratio: f32) -> String {
    format!("{:.0}%", ratio * 100.0)
}

fn panel_box(
    title: &'static str,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    fg_r: u8,
    fg_g: u8,
    fg_b: u8,
    width: f32,
    height: f32,
) -> impl IntoView {
    let bg = PenikoColor::rgb8(bg_r, bg_g, bg_b);
    let fg = PenikoColor::rgb8(fg_r, fg_g, fg_b);

    label(move || title).style(move |s| {
        s.width(width)
            .height(height)
            .padding(8.0)
            .border(0.5)
            .background(bg)
            .color(fg)
            .font_size(11.0)
    })
}

fn handle_strip(
    direction: Direction,
    main: f32,
    thickness: f32,
    color_r: u8,
    color_g: u8,
    color_b: u8,
) -> impl IntoView {
    let color = PenikoColor::rgb8(color_r, color_g, color_b);
    match direction {
        Direction::Horizontal => {
            label(move || "")
                .style(move |s| s.width(thickness).height(main).background(color).border(0.5))
        }
        Direction::Vertical => {
            label(move || "")
                .style(move |s| s.width(main).height(thickness).background(color).border(0.5))
        }
    }
}

fn split_row(
    heading: &'static str,
    direction: Direction,
    resolved: katana_ui_widget::layout::split::ResolvedSplitPane,
    width: f32,
    height: f32,
    text_color: (u8, u8, u8),
) -> impl IntoView {
    let ratio = resolved.ratio.clamp(0.0, 1.0);
    let (first_size, second_size) = match direction {
        Direction::Horizontal => {
            let first = (width - resolved.handle_thickness) * ratio;
            let first = first.clamp(1.0, (width - resolved.handle_thickness - 1.0).max(1.0));
            (first, (width - resolved.handle_thickness - first).max(1.0))
        }
        Direction::Vertical => {
            let first = (height - resolved.handle_thickness) * ratio;
            let first = first.clamp(1.0, (height - resolved.handle_thickness - 1.0).max(1.0));
            (first, (height - resolved.handle_thickness - first).max(1.0))
        }
    };

    let split = match direction {
        Direction::Horizontal => {
            h_stack((
                panel_box(
                    "Pane A",
                    resolved.handle_color.r,
                    resolved.handle_color.g,
                    resolved.handle_color.b,
                    text_color.0,
                    text_color.1,
                    text_color.2,
                    first_size,
                    height,
                ),
                handle_strip(
                    direction,
                    height,
                    resolved.handle_thickness,
                    resolved.handle_color.r,
                    resolved.handle_color.g,
                    resolved.handle_color.b,
                ),
                panel_box(
                    "Pane B",
                    resolved.handle_hover_color.r,
                    resolved.handle_hover_color.g,
                    resolved.handle_hover_color.b,
                    text_color.0,
                    text_color.1,
                    text_color.2,
                    second_size,
                    height,
                ),
            ))
            .style(move |s| s.width(width).height(height))
        }
        Direction::Vertical => {
            v_stack((
                panel_box(
                    "Pane A",
                    resolved.handle_color.r,
                    resolved.handle_color.g,
                    resolved.handle_color.b,
                    text_color.0,
                    text_color.1,
                    text_color.2,
                    width,
                    first_size,
                ),
                handle_strip(
                    direction,
                    width,
                    resolved.handle_thickness,
                    resolved.handle_color.r,
                    resolved.handle_color.g,
                    resolved.handle_color.b,
                ),
                panel_box(
                    "Pane B",
                    resolved.handle_hover_color.r,
                    resolved.handle_hover_color.g,
                    resolved.handle_hover_color.b,
                    text_color.0,
                    text_color.1,
                    text_color.2,
                    width,
                    second_size,
                ),
            ))
            .style(move |s| s.width(width).height(height))
        }
    };

    let cursor = match direction {
        Direction::Horizontal => "col-resize",
        Direction::Vertical => "row-resize",
    };

    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        label(move || {
            format!(
                "ratio {} / handle {}px / cursor {}",
                ratio_to_label(ratio),
                resolved.handle_thickness,
                cursor
            )
        })
            .style(|s| s.font_size(10.0)),
        split,
    ))
    .style(|s| s.gap(4.0))
}

fn nested_split_sample(theme: Theme) -> impl IntoView {
    let text = theme.color.text;
    let surface = theme.color.surface;
    let bg = theme.color.bg;
    let text_muted = theme.color.text_muted;

    let outer = SplitPane::new()
        .direction(Direction::Horizontal)
        .ratio(0.36)
        .resolve(&theme);
    let inner = SplitPane::new()
        .direction(Direction::Vertical)
        .ratio(0.6)
        .resolve(&theme);

    let left = (NESTED_WIDTH - outer.handle_thickness) * outer.ratio;
    let left = left.clamp(60.0, (NESTED_WIDTH - outer.handle_thickness - 20.0).max(60.0));
    let right = (NESTED_WIDTH - outer.handle_thickness - left).max(1.0);

    let top = (NESTED_HEIGHT - inner.handle_thickness) * inner.ratio;
    let top = top.clamp(30.0, (NESTED_HEIGHT - inner.handle_thickness - 20.0).max(30.0));
    let bottom = (NESTED_HEIGHT - inner.handle_thickness - top).max(1.0);

    let left_panel = panel_box(
        "Pane A",
        outer.handle_color.r,
        outer.handle_color.g,
        outer.handle_color.b,
        text.r,
        text.g,
        text.b,
        left,
        NESTED_HEIGHT,
    );
    let right_top = panel_box(
        "Pane B",
        surface.r,
        surface.g,
        surface.b,
        text.r,
        text.g,
        text.b,
        right,
        top,
    );
    let right_bottom = panel_box(
        "Pane C",
        text_muted.r,
        text_muted.g,
        text_muted.b,
        bg.r,
        bg.g,
        bg.b,
        right,
        bottom,
    );
    let right_stack = v_stack((
        right_top,
        handle_strip(
            Direction::Vertical,
            right,
            inner.handle_thickness,
            inner.handle_color.r,
            inner.handle_color.g,
            inner.handle_color.b,
        ),
        right_bottom,
    ))
    .style(move |s| s.width(right).height(NESTED_HEIGHT));

    v_stack((
        label(|| "Nested 3-panes").style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            left_panel,
            handle_strip(
                Direction::Horizontal,
                NESTED_HEIGHT,
                outer.handle_thickness,
                outer.handle_color.r,
                outer.handle_color.g,
                outer.handle_color.b,
            ),
            right_stack,
        ))
        .style(|s| s.width(NESTED_WIDTH).height(NESTED_HEIGHT)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: Theme) -> impl IntoView + use<> {
    let h = SplitPane::new().direction(Direction::Horizontal).resolve(&theme);
    let v = SplitPane::new().direction(Direction::Vertical).resolve(&theme);
    let r_60 = SplitPane::new().ratio(0.6).resolve(&theme);
    let r_min = SplitPane::new()
        .ratio(0.05)
        .min_ratio(0.15)
        .resolve(&theme);
    let r_min_ratio = r_min.ratio;

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "SplitPane Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            SplitPane::new().direction(Direction::Horizontal).view(
                theme.clone(),
                label(|| "Left pane").style(|s| s.padding(8.0)),
                label(|| "Right pane").style(|s| s.padding(8.0)),
            ),
                split_row(
                "Horizontal sample",
                Direction::Horizontal,
                h,
                DEMO_WIDTH,
                DEMO_HEIGHT,
                (text.r, text.g, text.b),
            ),
            split_row(
                "Vertical sample",
                Direction::Vertical,
                v,
                DEMO_WIDTH,
                DEMO_HEIGHT,
                (text.r, text.g, text.b),
            ),
            split_row(
                "Nested composition",
                Direction::Vertical,
                r_60,
                DEMO_WIDTH,
                DEMO_HEIGHT,
                (text.r, text.g, text.b),
            ),
            nested_split_sample(theme),
            split_row(
                "Min constraint demo (5% -> 15% clamp)",
                Direction::Horizontal,
                r_min,
                DEMO_WIDTH,
                DEMO_HEIGHT,
                (text.r, text.g, text.b),
            ),
            label(move || "cursor: horizontal=col-resize, vertical=row-resize").style(|s| s.font_size(11.0)),
            label(move || format!("min raw 5% resolved {}", ratio_to_label(r_min_ratio)))
                .style(|s| s.font_size(11.0)),
            label(|| "Double-click handle resets to 50/50 (docs behavior)").style(|s| s.font_size(11.0)),
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

pub fn split_pane_page(theme: Theme) -> impl IntoView {
    page_content(theme)
}
