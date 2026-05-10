use floem::peniko::Color as PenikoColor;
use floem::views::{label, scroll, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::selector::segmented::{
    Segment, SegmentedSize, SegmentedToggle,
};
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

    scroll(
        v_stack((
            label(|| "SegmentedToggle Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            SegmentedToggle::new(ViewMode::List, view_mode_options(), "Live view mode")
                .view(theme.clone()),
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
