mod ops;
mod types;
mod view;

pub use ops::PopoverOrigin;
pub use types::{AnchorRect, Placement, PopoverProps};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{corner_radius, default_offset, popover_bg, popover_border, shadow_color};

/// Resolved visual properties for `Popover`.
#[derive(Debug, Clone)]
pub struct ResolvedPopover {
    pub open: bool,
    pub placement: Placement,
    pub offset: f32,
    pub dismiss_on_outside_click: bool,
    pub dismiss_on_esc: bool,
    pub popover_bg: Color,
    pub popover_border: Color,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

/// Builder for the Popover layout widget.
#[derive(Debug, Clone)]
pub struct Popover {
    props: PopoverProps,
}

impl Popover {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: PopoverProps {
                open: false,
                placement: Placement::default(),
                offset: default_offset(),
                dismiss_on_outside_click: true,
                dismiss_on_esc: true,
            },
        }
    }

    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }

    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.props.placement = placement;
        self
    }

    #[must_use]
    pub fn offset(mut self, offset: f32) -> Self {
        self.props.offset = offset;
        self
    }

    #[must_use]
    pub fn dismiss_on_outside_click(mut self, v: bool) -> Self {
        self.props.dismiss_on_outside_click = v;
        self
    }

    #[must_use]
    pub fn dismiss_on_esc(mut self, v: bool) -> Self {
        self.props.dismiss_on_esc = v;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedPopover {
        ResolvedPopover {
            open: self.props.open,
            placement: self.props.placement,
            offset: self.props.offset,
            dismiss_on_outside_click: ops::should_dismiss_on_outside_click(
                self.props.dismiss_on_outside_click,
            ),
            dismiss_on_esc: ops::should_dismiss_on_esc(self.props.dismiss_on_esc),
            popover_bg: popover_bg(theme),
            popover_border: popover_border(theme),
            shadow_color: shadow_color(theme),
            corner_radius: corner_radius(),
        }
    }

    /// Convenience: compute placement origin given anchor rect and viewport dimensions.
    #[must_use]
    pub fn compute_origin(
        &self,
        anchor: AnchorRect,
        popover_width: f32,
        popover_height: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> PopoverOrigin {
        ops::compute_origin(
            anchor,
            self.props.placement,
            self.props.offset,
            popover_width,
            popover_height,
            viewport_width,
            viewport_height,
        )
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    fn anchor() -> AnchorRect {
        AnchorRect {
            x: 200.0,
            y: 200.0,
            width: 100.0,
            height: 40.0,
        }
    }

    #[test]
    fn dismiss_defaults_true() {
        let theme = Theme::default_light();
        let r = Popover::new().resolve(&theme);
        assert!(r.dismiss_on_outside_click);
        assert!(r.dismiss_on_esc);
    }

    #[test]
    fn dismiss_can_be_disabled() {
        let theme = Theme::default_light();
        let r = Popover::new()
            .dismiss_on_outside_click(false)
            .dismiss_on_esc(false)
            .resolve(&theme);
        assert!(!r.dismiss_on_outside_click);
        assert!(!r.dismiss_on_esc);
    }

    #[test]
    fn default_placement_is_bottom() {
        let theme = Theme::default_light();
        let r = Popover::new().resolve(&theme);
        assert_eq!(r.placement, Placement::Bottom);
    }

    #[test]
    fn placement_set_correctly() {
        let theme = Theme::default_light();
        let r = Popover::new().placement(Placement::Top).resolve(&theme);
        assert_eq!(r.placement, Placement::Top);
    }

    #[test]
    fn compute_origin_bottom() {
        let p = Popover::new().offset(4.0);
        let o = p.compute_origin(anchor(), 120.0, 60.0, 800.0, 600.0);
        assert!((o.y - (200.0 + 40.0 + 4.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn light_and_dark_both_resolve() {
        let light = Popover::new().resolve(&Theme::default_light());
        let dark = Popover::new().resolve(&Theme::default_dark());
        assert_ne!(light.popover_bg.r, dark.popover_bg.r);
    }
}
