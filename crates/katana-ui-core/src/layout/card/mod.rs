mod tests;
mod types;
mod view;

pub use types::{CardPadding, CardProps, CardVariant};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::event::{Event, EventListener};
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::style::CursorStyle;
use floem::views::{Decorators, container, empty, v_stack_from_iter};
use floem::{IntoView, View};
use std::rc::Rc;
use view::{active_bg, bg_color, border_color, corner_radius, has_shadow, hover_bg, padding_px};

const CARD_SECTION_GAP: f32 = 8.0;
const CARD_ACTIONS_GAP: f32 = 8.0;
const CARD_FOCUS_BORDER_WIDTH: f32 = 2.0;

#[derive(Debug, Clone)]
pub struct ResolvedCard {
    pub bg_color: Color,
    pub hover_bg_color: Color,
    pub active_bg_color: Color,
    pub border_color: Option<Color>,
    pub has_shadow: bool,
    pub corner_radius: f32,
    pub padding: f32,
    pub interactive: bool,
    pub has_on_click: bool,
    pub focus_ring_color: Color,
}

pub struct Card {
    props: CardProps,
}

impl Card {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: CardProps {
                variant: CardVariant::default(),
                padding: CardPadding::default(),
                interactive: false,
                header: None,
                body: None,
                footer: None,
                actions: None,
                content: None,
                on_click: None,
            },
        }
    }

    #[must_use]
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.props.variant = variant;
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: CardPadding) -> Self {
        self.props.padding = padding;
        self
    }

    #[must_use]
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.props.interactive = interactive;
        self
    }

    #[must_use]
    pub fn on_click(mut self, on_click: impl Fn() + 'static) -> Self {
        self.props.on_click = Some(Rc::new(on_click));
        self
    }

    #[must_use]
    pub fn header<V>(mut self, header: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.header = Some(header.into_any());
        self
    }

    #[must_use]
    pub fn body<V>(mut self, body: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.body = Some(body.into_any());
        self
    }

    #[must_use]
    pub fn content<V>(mut self, content: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.content = Some(content.into_any());
        self
    }

    #[must_use]
    pub fn actions<V>(mut self, actions: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.actions = Some(actions.into_any());
        self
    }

    #[must_use]
    pub fn footer<V>(mut self, footer: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.footer = Some(footer.into_any());
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedCard {
        ResolvedCard {
            bg_color: bg_color(self.props.variant, theme),
            hover_bg_color: hover_bg(self.props.variant, theme),
            active_bg_color: active_bg(self.props.variant, theme),
            border_color: border_color(self.props.variant, theme),
            has_shadow: has_shadow(self.props.variant),
            corner_radius: corner_radius(),
            padding: padding_px(self.props.padding, theme),
            interactive: self.props.interactive,
            has_on_click: self.props.on_click.is_some(),
            focus_ring_color: theme.color.accent,
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme, child: impl IntoView + 'static) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let focus_state = create_rw_signal(false);

        let focus_ring = crate::floem_view::FloemColor::from_token(resolved.focus_ring_color);
        let bg = crate::floem_view::FloemColor::from_token(resolved.bg_color);
        let hover_bg = crate::floem_view::FloemColor::from_token(resolved.hover_bg_color);
        let active_bg = crate::floem_view::FloemColor::from_token(resolved.active_bg_color);
        let border = resolved
            .border_color
            .map(crate::floem_view::FloemColor::from_token);

        let mut body_parts: Vec<Box<dyn View>> = Vec::new();

        if let Some(header) = self.props.header {
            body_parts.push(header);
        }
        if let Some(body) = self.props.body {
            body_parts.push(body);
        } else {
            body_parts.push(child.into_any());
        }
        if let Some(content) = self.props.content {
            body_parts.push(content);
        }
        if let Some(actions) = self.props.actions {
            let actions = container(actions)
                .style(move |style| style.gap(CARD_ACTIONS_GAP).justify_end())
                .into_any();
            body_parts.push(actions);
        }
        if let Some(footer) = self.props.footer {
            body_parts.push(footer);
        }

        let body = if body_parts.is_empty() {
            container(empty())
                .style(|style| style.height(0.0).width(0.0))
                .into_any()
        } else {
            v_stack_from_iter(body_parts)
                .style(|style| style.gap(CARD_SECTION_GAP))
                .into_any()
        };

        let mut card = container(body).style(move |style| {
            let mut style = style
                .background(bg)
                .border_radius(resolved.corner_radius)
                .padding(resolved.padding)
                .margin(0.0);

            if resolved.interactive {
                style = style
                    .cursor(CursorStyle::Pointer)
                    .hover(move |style| style.background(hover_bg))
                    .active(move |style| style.background(active_bg));
            }

            if focus_state.get() {
                style
                    .border(CARD_FOCUS_BORDER_WIDTH)
                    .border_color(focus_ring)
            } else if let Some(border) = border {
                style.border(1.0).border_color(border)
            } else {
                style
            }
        });

        if resolved.interactive {
            card = card
                .keyboard_navigable()
                .on_event_cont(EventListener::FocusGained, {
                    move |_| focus_state.set(true)
                });
            card = card.on_event_cont(EventListener::FocusLost, {
                move |_| focus_state.set(false)
            });

            if let Some(on_click) = self.props.on_click {
                card = card.on_event_cont(EventListener::PointerDown, move |event| match event {
                    Event::PointerDown(pointer_event) if pointer_event.button.is_primary() => {
                        (on_click)();
                    }
                    _ => (),
                });
            }
        }

        card
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}
