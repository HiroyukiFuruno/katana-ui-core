use crate::composite::breadcrumb::BreadcrumbCrumb;

#[derive(Clone)]
pub(crate) enum RenderSegment {
    Crumb(usize, BreadcrumbCrumb),
    Ellipsis,
}

pub(crate) struct BreadcrumbSegments;

impl BreadcrumbSegments {
    pub(crate) fn apply_ellipsis(
        crumbs: &[BreadcrumbCrumb],
        max_visible_crumbs: usize,
    ) -> Vec<RenderSegment> {
        if crumbs.is_empty() {
            return Vec::new();
        }

        if max_visible_crumbs == 0 || crumbs.len() <= max_visible_crumbs {
            return crumbs
                .iter()
                .enumerate()
                .map(|(index, crumb)| RenderSegment::Crumb(index, crumb.clone()))
                .collect();
        }

        match max_visible_crumbs {
            1 => vec![RenderSegment::Crumb(
                crumbs.len() - 1,
                crumbs[crumbs.len() - 1].clone(),
            )],
            2 => vec![
                RenderSegment::Crumb(0, crumbs[0].clone()),
                RenderSegment::Crumb(crumbs.len() - 1, crumbs[crumbs.len() - 1].clone()),
            ],
            _ => ellipsis_with_tail(crumbs, max_visible_crumbs),
        }
    }
}

fn ellipsis_with_tail(crumbs: &[BreadcrumbCrumb], max_visible_crumbs: usize) -> Vec<RenderSegment> {
    let keep_tail = max_visible_crumbs.saturating_sub(2);
    let start = crumbs.len() - keep_tail;
    let mut segments = Vec::with_capacity(2 + keep_tail);
    segments.push(RenderSegment::Crumb(0, crumbs[0].clone()));
    segments.push(RenderSegment::Ellipsis);
    segments.extend(
        crumbs
            .iter()
            .enumerate()
            .skip(start)
            .map(|(index, crumb)| RenderSegment::Crumb(index, crumb.clone())),
    );
    segments
}
