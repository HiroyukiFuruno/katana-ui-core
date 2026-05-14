use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::selector::segmented::{Segment, SegmentedSize, SegmentedToggle};
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

fn view_mode_options() -> Vec<(ViewMode, Segment)> {
    vec![
        (ViewMode::List, Segment::Label("List".into())),
        (ViewMode::Grid, Segment::Label("Grid".into())),
    ]
}

fn text_filter_options() -> Vec<(String, Segment)> {
    vec![
        ("All".into(), Segment::Label("All".into())),
        ("Active".into(), Segment::Label("Active".into())),
        ("Done".into(), Segment::Label("Done".into())),
    ]
}

fn status_options() -> Vec<(Filter, Segment)> {
    vec![
        (Filter::All, Segment::Label("All".into())),
        (Filter::Active, Segment::Label("Active".into())),
        (Filter::Done, Segment::Label("Done".into())),
        (Filter::Archived, Segment::Label("Archived".into())),
        (Filter::Flagged, Segment::Label("Flagged".into())),
    ]
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let selected = create_rw_signal(ViewMode::List);
    let log = create_rw_signal("on_change: なし".to_string());

    crate::interaction::replay("select-grid", "segmented-toggle", "value-grid", {
        let selected = selected;
        let log = log;
        move || {
            selected.set(ViewMode::Grid);
            log.set("on_change: Grid".to_string());
        }
    });

    scroll(
        v_stack((
            label(|| "SegmentedToggle Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            SegmentedToggle::new(selected.get(), view_mode_options(), "Live view mode")
                .on_change({
                    let selected = selected;
                    let log = log;
                    move |value| {
                        selected.set(value.clone());
                        let mode = match value {
                            ViewMode::List => "List",
                            ViewMode::Grid => "Grid",
                        };
                        log.set(format!("on_change: {mode}"));
                    }
                })
                .view(theme.clone()),
            label(move || format!("callback log: {}", log.get())).style(|s| s.font_size(12.0)),
            label(|| "Readonly display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            SegmentedToggle::new(ViewMode::Grid, view_mode_options(), "View mode")
                .disabled(true)
                .view(theme.clone()),
            SegmentedToggle::new("Active".to_owned(), text_filter_options(), "Filter")
                .disabled(true)
                .view(theme.clone()),
            SegmentedToggle::new(Filter::Archived, status_options(), "Status")
                .disabled(true)
                .view(theme.clone()),
            label(|| "Size display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            SegmentedToggle::new(ViewMode::Grid, view_mode_options(), "Small")
                .size(SegmentedSize::Sm)
                .disabled(true)
                .view(theme.clone()),
            SegmentedToggle::new(ViewMode::Grid, view_mode_options(), "Large")
                .size(SegmentedSize::Lg)
                .disabled(true)
                .view(theme.clone()),
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

pub fn segmented_toggle_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
