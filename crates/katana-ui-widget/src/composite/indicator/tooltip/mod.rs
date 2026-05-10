mod types;
mod view;

pub use types::{Placement, ResolvedTooltip, Tooltip, TooltipProps};

use crate::theme::Theme;
use floem::IntoView;
use floem::action::exec_after;
use floem::event::EventListener;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, dyn_container, empty, h_stack, label, v_stack};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use view::{
    bg_color, default_delay_ms, default_max_width, effective_placement, font_size, padding,
    text_color,
};

const TOOLTIP_RADIUS: f32 = crate::floem_view::CORNER_RADIUS_SM;
const TOOLTIP_GAP: f32 = crate::floem_view::GAP_XS;
const TOOLTIP_EMPTY_SIZE: f32 = crate::floem_view::EMPTY_SIZE;

impl Tooltip {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            props: TooltipProps {
                label: label.into(),
                placement: Placement::default(),
                delay_ms: default_delay_ms(),
                max_width: default_max_width(),
                dismiss_on_pointer_leave: true,
                dismiss_on_focus_loss: true,
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
        view::flip_placement(placement, preferred_fits, opposite_fits)
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
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme, child: impl IntoView + 'static) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let visible = create_rw_signal(false);
        let hover_ready = Rc::new(Cell::new(false));
        let focus_ready = Rc::new(Cell::new(false));
        let hover_token = Rc::new(Cell::new(0_u64));
        let placement = resolved.placement;
        let tooltip_label = resolved.label.clone();
        let tooltip_bg = crate::floem_view::FloemColor::from_token(resolved.bg_color);
        let tooltip_text = crate::floem_view::FloemColor::from_token(resolved.text_color);
        let delay_ms = resolved.delay_ms;
        let dismiss_on_pointer_leave = resolved.dismiss_on_pointer_leave;
        let dismiss_on_focus_loss = resolved.dismiss_on_focus_loss;
        let tooltip = move || {
            let tooltip_label = tooltip_label.clone();
            dyn_container(
                move || visible.get(),
                move |is_visible| {
                    let tooltip_label = tooltip_label.clone();
                    if is_visible {
                        label(move || tooltip_label.clone())
                            .style({
                                let tooltip_bg = tooltip_bg;
                                let tooltip_text = tooltip_text;
                                move |style| {
                                    style
                                        .font_size(resolved.font_size)
                                        .background(tooltip_bg)
                                        .color(tooltip_text)
                                        .padding_vert(resolved.pad_v)
                                        .padding_horiz(resolved.pad_h)
                                        .max_width(resolved.max_width)
                                        .border_radius(TOOLTIP_RADIUS)
                                }
                            })
                            .into_any()
                    } else {
                        container(empty())
                            .style(|style| {
                                style.width(TOOLTIP_EMPTY_SIZE).height(TOOLTIP_EMPTY_SIZE)
                            })
                            .into_any()
                    }
                },
            )
            .into_any()
        };

        let interactive_child = child
            .on_event_cont(EventListener::PointerEnter, {
                let hover_ready = Rc::clone(&hover_ready);
                let hover_token = Rc::clone(&hover_token);
                move |_| {
                    hover_ready.set(false);
                    let token = hover_token.get().wrapping_add(1);
                    hover_token.set(token);
                    if delay_ms == 0 {
                        hover_ready.set(true);
                        let _ = visible.try_update(|value| *value = true);
                        return;
                    }
                    let hover_token = Rc::clone(&hover_token);
                    let hover_ready = Rc::clone(&hover_ready);
                    exec_after(Duration::from_millis(u64::from(delay_ms)), move |_| {
                        if hover_token.get() != token {
                            return;
                        }
                        hover_ready.set(true);
                        let _ = visible.try_update(|value| *value = true);
                    });
                }
            })
            .on_event_cont(EventListener::PointerLeave, {
                let hover_ready = Rc::clone(&hover_ready);
                let hover_token = Rc::clone(&hover_token);
                let focus_ready = Rc::clone(&focus_ready);
                move |_| {
                    hover_token.set(hover_token.get().wrapping_add(1));
                    hover_ready.set(false);
                    if !dismiss_on_pointer_leave {
                        return;
                    }
                    let should_show = focus_ready.get();
                    let _ = visible.try_update(|value| *value = should_show);
                }
            })
            .on_event_cont(EventListener::FocusGained, {
                let focus_ready = Rc::clone(&focus_ready);
                move |_| {
                    focus_ready.set(true);
                    let _ = visible.try_update(|value| *value = true);
                }
            })
            .on_event_cont(EventListener::FocusLost, {
                let focus_ready = Rc::clone(&focus_ready);
                let hover_ready = Rc::clone(&hover_ready);
                move |_| {
                    focus_ready.set(false);
                    if !dismiss_on_focus_loss {
                        return;
                    }
                    let should_show = hover_ready.get();
                    let _ = visible.try_update(|value| *value = should_show);
                }
            })
            .into_any();

        let tooltip_node = move || tooltip();
        let tooltip_first = matches!(placement, Placement::Top | Placement::Start);
        let horizontal = matches!(placement, Placement::Start | Placement::End);

        if horizontal {
            let row = if tooltip_first {
                (tooltip_node(), interactive_child)
            } else {
                (interactive_child, tooltip_node())
            };
            h_stack(row).style(|style| style.gap(TOOLTIP_GAP))
        } else {
            let stack = if tooltip_first {
                (tooltip_node(), interactive_child)
            } else {
                (interactive_child, tooltip_node())
            };
            v_stack(stack).style(|style| style.gap(TOOLTIP_GAP))
        }
    }
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
    fn label_preserved() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Tooltip text").resolve(&theme);
        assert_eq!(r.label, "Tooltip text");
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
}
