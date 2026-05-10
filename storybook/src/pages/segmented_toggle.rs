use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::selector::segmented::{Segment, SegmentedToggle};
use katana_ui_widget::theme::Theme;

#[derive(Debug, Clone, PartialEq)]
enum ViewMode {
    List,
    Grid,
}

#[derive(Debug, Clone, PartialEq)]
enum Filter {
    All,
    Active,
    Done,
    Archived,
    Flagged,
}

fn seg_row(
    heading: &'static str,
    segments: Vec<(String, u8, u8, u8, bool)>,
    font_sz: f32,
) -> impl IntoView {
    let cells: Vec<_> = segments
        .into_iter()
        .map(|(lbl, r, g, b, selected)| {
            let bg = PenikoColor::rgb8(r, g, b);
            let lbl: &'static str = Box::leak(lbl.into_boxed_str());
            let indicator = if selected { " ●" } else { "" };
            label(move || format!("{lbl}{indicator}"))
                .style(move |s| s.background(bg).padding(6.0).font_size(font_sz))
        })
        .collect();

    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack(cells).style(|s| s.gap(2.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let two_opts = vec![
        (ViewMode::List, Segment::Label("List".into())),
        (ViewMode::Grid, Segment::Label("Grid".into())),
    ];
    let three_opts = vec![
        ("All".into(), Segment::Label("All".into())),
        ("Active".into(), Segment::Label("Active".into())),
        ("Done".into(), Segment::Label("Done".into())),
    ];
    let five_opts = vec![
        (Filter::All, Segment::Label("All".into())),
        (Filter::Active, Segment::Label("Active".into())),
        (Filter::Done, Segment::Label("Done".into())),
        (Filter::Archived, Segment::Label("Archived".into())),
        (Filter::Flagged, Segment::Label("Flagged".into())),
    ];

    let r2 = SegmentedToggle::new(ViewMode::Grid, two_opts, "View mode").resolve(theme);
    let r3 = SegmentedToggle::new("Active".to_owned(), three_opts, "Filter").resolve(theme);
    let r5 = SegmentedToggle::new(Filter::Archived, five_opts.clone(), "Status").resolve(theme);
    let r_dis = SegmentedToggle::new(Filter::All, five_opts, "Status disabled")
        .disabled(true)
        .resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let to_row = |heading: &'static str, r: katana_ui_widget::composite::selector::segmented::ResolvedSegmentedToggle| {
        let segs = r
            .segments
            .iter()
            .map(|s| (s.label.clone(), s.bg_color.r, s.bg_color.g, s.bg_color.b, s.selected))
            .collect();
        seg_row(heading, segs, r.font_size)
    };

    scroll(
        v_stack((
            label(|| "SegmentedToggle Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            to_row("2 segments (List | Grid)", r2),
            to_row("3 segments (All | Active | Done)", r3),
            to_row("5 segments", r5),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            to_row("Disabled", r_dis),
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

pub fn segmented_toggle_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "SegmentedToggle").style(|s| s.font_size(20.0)),
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
