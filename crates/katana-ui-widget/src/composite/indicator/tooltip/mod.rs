mod interaction;
mod interaction_arrow;
mod interaction_events;
mod interaction_overlay;
mod types;
mod view;

pub use types::{FreePlacement, Placement, ResolvedTooltip, Tooltip, TooltipProps};

use crate::theme::Theme;
use floem::IntoView;

impl Tooltip {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            props: TooltipProps {
                label: label.into(),
                placement: Placement::Top,
                delay_ms: default_delay_ms(),
                max_width: default_max_width(),
                dismiss_on_pointer_leave: true,
                dismiss_on_focus_loss: true,
                show_arrow: true,
                visible: false,
            },
        }
    }

    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.props.placement = placement;
        self
    }

    #[must_use]
    pub fn delay_ms(mut self, ms: u32) -> Self {
        self.props.delay_ms = ms;
        self
    }

    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.props.max_width = width;
        self
    }

    #[must_use]
    pub fn dismiss_on_pointer_leave(mut self, v: bool) -> Self {
        self.props.dismiss_on_pointer_leave = v;
        self
    }

    #[must_use]
    pub fn dismiss_on_focus_loss(mut self, v: bool) -> Self {
        self.props.dismiss_on_focus_loss = v;
        self
    }

    #[must_use]
    pub fn show_arrow(mut self, v: bool) -> Self {
        self.props.show_arrow = v;
        self
    }

    #[must_use]
    pub fn visible(mut self, v: bool) -> Self {
        self.props.visible = v;
        self
    }

    #[must_use]
    pub fn visible_after_hover(&self, elapsed_ms: u32) -> bool {
        view::hover_visible(elapsed_ms, self.props.delay_ms)
    }

    #[must_use]
    pub fn visible_on_focus(&self) -> bool {
        view::focus_visible()
    }

    #[must_use]
    pub fn flip_placement(
        placement: Placement,
        preferred_fits: bool,
        opposite_fits: bool,
    ) -> Placement {
        flip_placement(placement, preferred_fits, opposite_fits)
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedTooltip {
        let (pv, ph) = padding();
        ResolvedTooltip {
            label: self.props.label.clone(),
            placement: effective_placement(self.props.placement),
            delay_ms: self.props.delay_ms,
            max_width: self.props.max_width,
            font_size: font_size(),
            pad_v: pv,
            pad_h: ph,
            bg_color: bg_color(theme),
            text_color: text_color(theme),
            dismiss_on_pointer_leave: self.props.dismiss_on_pointer_leave,
            dismiss_on_focus_loss: self.props.dismiss_on_focus_loss,
            show_arrow: self.props.show_arrow,
            visible: self.props.visible,
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme, child: impl IntoView + 'static) -> impl IntoView {
        interaction::build_view(self.resolve(&theme), child)
    }
}

fn padding() -> (f32, f32) {
    view::padding()
}

fn font_size() -> f32 {
    view::font_size()
}

fn bg_color(theme: &Theme) -> crate::theme::color::Color {
    view::bg_color(theme)
}

fn text_color(theme: &Theme) -> crate::theme::color::Color {
    view::text_color(theme)
}

fn default_delay_ms() -> u32 {
    view::default_delay_ms()
}

fn default_max_width() -> f32 {
    view::default_max_width()
}

fn effective_placement(placement: Placement) -> Placement {
    view::effective_placement(placement)
}

fn flip_placement(placement: Placement, preferred_fits: bool, opposite_fits: bool) -> Placement {
    view::flip_placement(placement, preferred_fits, opposite_fits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn default_placement_is_top() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello").resolve(&theme);
        assert_eq!(r.placement, Placement::Top);
    }

    #[test]
    fn default_delay_is_400ms() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello").resolve(&theme);
        assert_eq!(r.delay_ms, 400);
    }

    #[test]
    fn arrow_is_enabled_by_default_and_can_be_disabled() {
        let theme = Theme::default_light();
        let default = Tooltip::new("Hello").resolve(&theme);
        assert!(default.show_arrow);

        let disabled = Tooltip::new("Hello").show_arrow(false).resolve(&theme);
        assert!(!disabled.show_arrow);
    }

    #[test]
    fn custom_placement_bottom() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello")
            .placement(Placement::Bottom)
            .resolve(&theme);
        assert_eq!(r.placement, Placement::Bottom);
    }

    #[test]
    fn custom_placement_auto_and_free() {
        let theme = Theme::default_light();
        let auto = Tooltip::new("Hello")
            .placement(Placement::Auto)
            .resolve(&theme);
        assert_eq!(auto.placement, Placement::Auto);

        let free = Tooltip::new("Hello")
            .placement(Placement::Free(FreePlacement::AnchorOffset {
                x: 4.0,
                y: 8.0,
            }))
            .resolve(&theme);
        assert_eq!(
            free.placement,
            Placement::Free(FreePlacement::AnchorOffset { x: 4.0, y: 8.0 })
        );
    }

    #[test]
    fn label_preserved() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Tooltip text").resolve(&theme);
        assert_eq!(r.label, "Tooltip text");
    }

    #[test]
    fn bg_is_text_color_for_contrast() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello").resolve(&theme);
        assert_eq!(r.bg_color, theme.color.text);
        assert_eq!(r.text_color, theme.color.bg);
    }

    #[test]
    fn hover_delay_controls_visibility() {
        let tooltip = Tooltip::new("Hello").delay_ms(400);
        assert!(!tooltip.visible_after_hover(399));
        assert!(tooltip.visible_after_hover(400));
    }

    #[test]
    fn hover_delay_controls_visibility_with_immediate_mode() {
        let tooltip = Tooltip::new("Hello").delay_ms(0);
        assert!(tooltip.visible_after_hover(0));
    }

    #[test]
    fn focus_shows_tooltip_immediately() {
        let tooltip = Tooltip::new("Hello");
        assert!(tooltip.visible_on_focus());
    }

    #[test]
    fn dismiss_flags_default_to_true() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello").resolve(&theme);
        assert!(r.dismiss_on_pointer_leave);
        assert!(r.dismiss_on_focus_loss);
    }

    #[test]
    fn dismiss_flags_can_disable_pointer_leave() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello")
            .dismiss_on_pointer_leave(false)
            .resolve(&theme);
        assert!(!r.dismiss_on_pointer_leave);
    }

    #[test]
    fn dismiss_flags_can_disable_focus_loss() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello")
            .dismiss_on_focus_loss(false)
            .resolve(&theme);
        assert!(!r.dismiss_on_focus_loss);
    }

    #[test]
    fn focus_loss_hides_when_dismiss_enabled() {
        assert!(!view::visible_after_focus_loss(true, true));
        assert!(view::visible_after_focus_loss(true, false));
    }

    #[test]
    fn placement_flips_when_preferred_side_does_not_fit() {
        assert_eq!(
            Tooltip::flip_placement(Placement::Top, false, true),
            Placement::Bottom
        );
        assert_eq!(
            Tooltip::flip_placement(Placement::End, false, true),
            Placement::Start
        );
    }

    #[test]
    fn placement_not_flipped_when_preferred_side_already_fits() {
        assert_eq!(
            Tooltip::flip_placement(Placement::Bottom, true, false),
            Placement::Bottom
        );
        assert_eq!(
            Tooltip::flip_placement(Placement::Top, true, false),
            Placement::Top
        );
    }
}
