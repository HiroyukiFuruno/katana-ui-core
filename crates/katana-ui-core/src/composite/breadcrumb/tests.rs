use crate::primitive::icon::IconSource;

use super::{
    Breadcrumb, BreadcrumbCrumb,
    view::{BreadcrumbSegments, RenderSegment},
};

fn sample_crumb(label: &str) -> BreadcrumbCrumb {
    BreadcrumbCrumb::new(label).icon(IconSource::SvgString(format!("<svg />{label}")))
}

#[test]
fn visible_crumbs_respects_max_visible() {
    let crumbs = vec![
        sample_crumb("root"),
        sample_crumb("home"),
        sample_crumb("app"),
        sample_crumb("settings"),
        sample_crumb("user"),
        sample_crumb("profile"),
    ];

    let segments = BreadcrumbSegments::apply_ellipsis(&crumbs, 4);
    assert!(matches!(segments[0], RenderSegment::Crumb(0, _)));
    assert!(matches!(segments[1], RenderSegment::Ellipsis));
    assert!(matches!(segments[2], RenderSegment::Crumb(4, _)));
    assert!(matches!(segments[3], RenderSegment::Crumb(5, _)));
}

#[test]
fn background_and_border_default_to_false() {
    let breadcrumb = Breadcrumb::new(vec![sample_crumb("home")]);

    assert!(!breadcrumb.props.show_background);
    assert!(!breadcrumb.props.show_border);
}

#[test]
fn crumb_can_hold_recursive_children_for_hover_tree() {
    let root = BreadcrumbCrumb::new("home").children(vec![
        BreadcrumbCrumb::new("src").children(vec![BreadcrumbCrumb::new("main.rs")]),
    ]);

    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].label, "src");
    assert_eq!(root.children[0].children[0].label, "main.rs");
}
