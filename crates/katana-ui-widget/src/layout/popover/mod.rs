mod ops;
mod types;
mod view;

pub use ops::PopoverOrigin;
pub use types::{AnchorRect, AnchorRef, Placement, PopoverProps};
pub use view::PopoverOverlay;

use crate::theme::Theme;
use crate::theme::color::Color;
use std::rc::Rc;
use view::{default_offset, style};

/// Resolved visual properties for `Popover`.
#[derive(Clone)]
pub struct ResolvedPopover {
    pub open: bool,
    pub placement: Placement,
    pub offset: f32,
    pub anchor: Option<AnchorRect>,
    pub children: Option<String>,
    pub on_close: Rc<dyn Fn()>,
    pub dismiss_on_outside_click: bool,
    pub dismiss_on_esc: bool,
    pub popover_bg: Color,
    pub popover_border: Color,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

impl ResolvedPopover {
    /// Returns the resolved overlay layout.
    ///
    /// Returns `None` if popover is closed or anchor is not set.
    #[must_use]
    pub fn overlay_layout(
        &self,
        popover_width: f32,
        popover_height: f32,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Option<PopoverOverlay> {
        if !self.open {
            return None;
        }

        let anchor = self.anchor?;
        let placement = ops::resolve_placement(
            self.placement,
            anchor,
            self.offset,
            popover_width,
            popover_height,
            viewport_width,
            viewport_height,
        );
        let origin = Popover::new()
            .placement(placement)
            .offset(self.offset)
            .compute_origin(
                anchor,
                popover_width,
                popover_height,
                viewport_width,
                viewport_height,
            );

        Some(PopoverOverlay {
            x: origin.x,
            y: origin.y,
            width: popover_width,
            height: popover_height,
            placement,
            popover_bg: self.popover_bg,
            popover_border: self.popover_border,
            shadow_color: self.shadow_color,
            corner_radius: self.corner_radius,
        })
    }

    /// Returns whether outside click should close the popover.
    #[must_use]
    pub fn should_close_with_outside_click(&self) -> bool {
        self.open && self.dismiss_on_outside_click
    }

    /// Returns whether Esc key should close the popover.
    #[must_use]
    pub fn should_close_with_esc(&self) -> bool {
        self.open && self.dismiss_on_esc
    }

    /// Tries to close by outside click and returns whether close was executed.
    pub fn close_with_outside_click(&self) -> bool {
        if self.should_close_with_outside_click() {
            (self.on_close)();
            true
        } else {
            false
        }
    }

    /// Tries to close by Esc key and returns whether close was executed.
    pub fn close_with_esc(&self) -> bool {
        if self.should_close_with_esc() {
            (self.on_close)();
            true
        } else {
            false
        }
    }
}

fn noop_close() {}

/// Builder for the Popover layout widget.
#[derive(Clone)]
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
                anchor: None,
                children: None,
                on_close: Rc::new(noop_close),
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
    pub fn anchor(mut self, anchor: AnchorRef) -> Self {
        self.props.anchor = Some(anchor);
        self
    }

    #[must_use]
    pub fn children(mut self, children: impl Into<String>) -> Self {
        self.props.children = Some(children.into());
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
    pub fn on_close(mut self, on_close: impl Fn() + 'static) -> Self {
        self.props.on_close = Rc::new(on_close);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedPopover {
        let resolved_style = style(theme);
        ResolvedPopover {
            open: self.props.open,
            placement: self.props.placement,
            offset: self.props.offset,
            anchor: self.props.anchor.map(|anchor| anchor.rect),
            children: self.props.children.clone(),
            on_close: Rc::clone(&self.props.on_close),
            dismiss_on_outside_click: ops::should_dismiss_on_outside_click(&self.props),
            dismiss_on_esc: ops::should_dismiss_on_esc(&self.props),
            popover_bg: resolved_style.popover_bg,
            popover_border: resolved_style.popover_border,
            shadow_color: resolved_style.shadow_color,
            corner_radius: resolved_style.corner_radius,
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
mod tests;
