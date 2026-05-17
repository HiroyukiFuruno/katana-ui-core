use super::{Accordion, render, view_helpers};
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::reactive::create_effect;
use floem::views::{Decorators, container, dyn_container, empty, v_stack_from_iter};
use std::rc::Rc;

use super::trigger::make_trigger_target;
use render::{HeaderRowStyle, make_body_view, make_header_row, make_trigger_wrapper};

use view_helpers::{HEADER_GAP, animate_open, make_open_state, open_state_get, open_state_set};

const ACCORDION_GAP: f32 = crate::floem_view::GAP_XS;
const ACCORDION_ROWS_OPEN: usize = 3;
const ACCORDION_ROWS_CLOSED: usize = 2;

impl Accordion {
    #[must_use]
    pub fn view<IV: IntoView + 'static>(
        self,
        theme: Theme,
        child: impl Fn() -> IV + Clone + 'static,
    ) -> impl IntoView {
        let props = self.props.clone();
        let open = make_open_state(&props);
        let open_ratio =
            floem::reactive::create_rw_signal(if open_state_get(&open) { 1.0 } else { 0.0 });
        let animation_token = floem::reactive::create_rw_signal(0_u32);

        let open_for_effect = open.clone();
        create_effect(move |_| {
            animate_open(
                open_ratio,
                animation_token,
                props.reduced_motion,
                props.animation_ms,
                open_state_get(&open_for_effect),
            );
        });

        let open_for_action = open.clone();
        let on_toggle = Rc::clone(&props.on_toggle);
        let disabled_for_action = props.disabled;

        let header_click: Rc<dyn Fn()> = Rc::new(move || {
            if disabled_for_action {
                return;
            }
            let next = !open_state_get(&open_for_action);
            open_state_set(&open_for_action, next);
            on_toggle(next);
        });

        let child = Rc::new(child);

        dyn_container(
            move || open_state_get(&open),
            move |is_open| {
                let mut next = props.clone();
                next.expanded = is_open;
                let resolved = Accordion { props: next }.resolve(&theme);

                let line_color = crate::floem_view::FloemColor::from_token(theme.color.border);
                let bg = crate::floem_view::FloemColor::from_token(
                    if resolved.tree_selected
                        && resolved.tree_mode == super::AccordionTreeMode::Enabled
                    {
                        theme.color.accent_muted
                    } else {
                        resolved.header_bg
                    },
                );
                let text_color = crate::floem_view::FloemColor::from_token(resolved.header_text);
                let border = crate::floem_view::FloemColor::from_token(resolved.border_color);
                let hover = crate::floem_view::FloemColor::from_token(theme.color.accent_muted);

                let icon: Option<&'static str> = match resolved.indicator {
                    super::IndicatorPosition::None => None,
                    _ => resolved.chevron,
                };

                let trigger_target = make_trigger_target(super::trigger::TriggerTargetConfig {
                    header: props.header_view.clone(),
                    header_font_size: resolved.header_font_size,
                    text_color,
                    icon,
                    indicator: resolved.indicator,
                    trigger_area: resolved.trigger_area,
                    disabled: resolved.disabled,
                    on_toggle: Rc::clone(&header_click),
                });

                let trigger = make_trigger_wrapper(
                    trigger_target,
                    resolved.tree_mode,
                    resolved.tree_depth,
                    resolved.tree_show_lines,
                    line_color,
                );

                let header_row = make_header_row(
                    trigger,
                    HeaderRowStyle {
                        header_bg: bg,
                        border_color: border,
                        pad_v: resolved.header_pad_v,
                        pad_h: resolved.header_pad_h,
                        disabled: resolved.disabled,
                        hover_color: hover,
                        gap: HEADER_GAP,
                    },
                );

                let body_view = make_body_view(
                    || child(),
                    open_ratio,
                    resolved.body_max_height,
                    resolved.header_pad_h,
                    resolved.header_pad_v,
                    resolved.body_border,
                    border,
                );

                let mut rows = Vec::<Box<dyn View>>::with_capacity(if is_open {
                    ACCORDION_ROWS_OPEN
                } else {
                    ACCORDION_ROWS_CLOSED
                });
                rows.push(header_row.into_any());
                if is_open {
                    rows.push(
                        container(empty())
                            .style(|style| style.height(ACCORDION_GAP))
                            .into_any(),
                    );
                }
                rows.push(body_view.into_any());
                v_stack_from_iter(rows)
            },
        )
    }
}

pub(super) use view_helpers::{
    animation_ms, body_max_height, border_color, chevron_symbol, header_bg, header_font_size,
    header_padding, header_text,
};
