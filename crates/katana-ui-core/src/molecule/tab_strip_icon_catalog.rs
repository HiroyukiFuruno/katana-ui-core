use crate::render_model::{UiIconProps, UiSvgPaintPolicy};

const TAB_STRIP_ICON_COUNT: usize = 7;

/// Generic icons used by a tab strip without imposing host-specific semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStripIcon {
    Previous,
    Next,
    Overflow,
    Close,
    Pin,
    DisclosureExpanded,
    DisclosureCollapsed,
}

impl TabStripIcon {
    #[must_use]
    pub const fn all() -> [Self; TAB_STRIP_ICON_COUNT] {
        [
            Self::Previous,
            Self::Next,
            Self::Overflow,
            Self::Close,
            Self::Pin,
            Self::DisclosureExpanded,
            Self::DisclosureCollapsed,
        ]
    }

    #[must_use]
    pub fn icon_props(self) -> UiIconProps {
        let (path, role, path_summary) = match self {
            Self::Previous => (
                "M10.5 3.5 6 8l4.5 4.5-1.5 1.5L3 8l6-6 1.5 1.5Z",
                "tab-strip.icon.previous",
                "left chevron",
            ),
            Self::Next => (
                "m5.5 2 6 6-6 6L4 12.5 8.5 8 4 3.5 5.5 2Z",
                "tab-strip.icon.next",
                "right chevron",
            ),
            Self::Overflow => (
                "M3 6h2v2H3V6Zm4 0h2v2H7V6Zm4 0h2v2h-2V6Z",
                "tab-strip.icon.overflow",
                "horizontal overflow dots",
            ),
            Self::Close => (
                "m4.4 3 3.6 3.6L11.6 3 13 4.4 9.4 8l3.6 3.6-1.4 1.4L8 9.4 4.4 13 3 11.6 6.6 8 3 4.4 4.4 3Z",
                "tab-strip.icon.close",
                "close cross",
            ),
            Self::Pin => (
                "m6 2 4 4 1.5-1.5L13 6l-2 2v3l1.5 1.5L11 14l-3-3H6l-2 2-1-1 2-2V8L3 6l1.5-1.5L6 6V2Z",
                "tab-strip.icon.pin",
                "push pin",
            ),
            Self::DisclosureExpanded => (
                "m3 5 5 5 5-5 1.5 1.5L8 13 1.5 6.5 3 5Z",
                "tab-strip.icon.disclosure-expanded",
                "down disclosure chevron",
            ),
            Self::DisclosureCollapsed => (
                "m5 3 5 5-5 5-1.5-1.5L7 8 3.5 4.5 5 3Z",
                "tab-strip.icon.disclosure-collapsed",
                "right disclosure chevron",
            ),
        };

        UiIconProps::new(format!(
            r#"<svg><path fill="currentColor" d="{path}"/></svg>"#
        ))
        .view_box("0 0 16 16")
        .path_summary(path_summary)
        .paint_policy(UiSvgPaintPolicy::CurrentColor)
        .role(role)
    }
}
