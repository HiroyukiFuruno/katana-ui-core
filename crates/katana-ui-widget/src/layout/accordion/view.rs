use super::Accordion;
use super::types::{AccordionProps, IndicatorPosition};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, container, dyn_container, empty, label, v_stack};
use std::rc::Rc;

const HEADER_FONT_SIZE: f32 = 13.0;
const HEADER_PAD_V: f32 = 8.0;
const HEADER_PAD_H: f32 = 12.0;
const ANIMATION_MS: u32 = 180;
const ACCORDION_GAP: f32 = crate::floem_view::GAP_XS;

pub(super) fn header_font_size() -> f32 {
    HEADER_FONT_SIZE
}

pub(super) fn header_padding() -> (f32, f32) {
    (HEADER_PAD_V, HEADER_PAD_H)
}

pub(super) fn animation_ms() -> u32 {
    ANIMATION_MS
}

pub(super) fn chevron_symbol(expanded: bool, position: IndicatorPosition) -> Option<&'static str> {
    match position {
        IndicatorPosition::None => None,
        _ => Some(if expanded { "▲" } else { "▼" }),
    }
}

pub(super) fn header_bg(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.surface
    } else {
        theme.color.bg
    }
}

pub(super) fn header_text(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.text
    }
}

pub(super) fn border_color(theme: &Theme) -> Color {
    theme.color.border
}

impl Accordion {
    #[must_use]
    pub fn view<IV: IntoView + 'static>(
        self,
        theme: Theme,
        child: impl Fn() -> IV + Clone + 'static,
    ) -> impl IntoView {
        let expanded = create_rw_signal(self.props.expanded);
        let disabled = self.props.disabled;
        let on_toggle = Rc::clone(&self.props.on_toggle);
        let header = self.props.header.clone();
        let indicator = self.props.indicator;

        dyn_container(
            move || expanded.get(),
            move |is_expanded| {
                let probe = Accordion {
                    props: AccordionProps {
                        header: header.clone(),
                        expanded: is_expanded,
                        disabled,
                        indicator,
                        on_toggle: Rc::clone(&on_toggle),
                    },
                };
                let resolved = probe.resolve(&theme);
                let bg = crate::floem_view::FloemColor::from_token(resolved.header_bg);
                let text = crate::floem_view::FloemColor::from_token(resolved.header_text);
                let border = crate::floem_view::FloemColor::from_token(resolved.border_color);
                let chevron = resolved.chevron.unwrap_or("");
                let on_toggle_for_action = Rc::clone(&on_toggle);
                let body = if is_expanded {
                    child().into_any()
                } else {
                    container(empty())
                        .style(|style| {
                            style
                                .width(crate::floem_view::EMPTY_SIZE)
                                .height(crate::floem_view::EMPTY_SIZE)
                        })
                        .into_any()
                };

                v_stack((
                    button(
                        label(move || format!("{chevron} {}", resolved.header)).style(
                            move |style| style.font_size(resolved.header_font_size).color(text),
                        ),
                    )
                    .action(move || {
                        if !disabled {
                            let next = !expanded.get_untracked();
                            expanded.set(next);
                            on_toggle_for_action(next);
                        }
                    })
                    .style(move |style| {
                        style
                            .background(bg)
                            .border(1.0)
                            .border_color(border)
                            .padding_vert(resolved.header_pad_v)
                            .padding_horiz(resolved.header_pad_h)
                    }),
                    body,
                ))
                .style(|style| style.gap(ACCORDION_GAP))
            },
        )
    }
}
