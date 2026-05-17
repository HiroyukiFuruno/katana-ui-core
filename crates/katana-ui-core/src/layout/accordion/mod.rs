mod group;
mod render;
mod resolved;
mod tests;
mod trigger;
mod types;
mod view;
mod view_helpers;

pub use group::{AccordionGroup, AccordionGroupItem};
pub use resolved::ResolvedAccordion;
use types::AccordionControlState;
pub use types::{
    AccordionHeaderView, AccordionProps, AccordionTreeMode, AccordionTriggerArea, IndicatorPosition,
};

use crate::theme::Theme;
use floem::IntoView;
use floem::reactive::{RwSignal, SignalGet};
use floem::views::label;
use std::rc::Rc;
use view::{
    animation_ms, body_max_height, border_color, chevron_symbol, header_bg, header_font_size,
    header_padding, header_text,
};

/// Builder for the Accordion layout widget.
#[derive(Clone)]
pub struct Accordion {
    props: AccordionProps,
}

impl Accordion {
    #[must_use]
    pub fn new(header: impl Into<String>) -> Self {
        let header = header.into();
        Self {
            props: AccordionProps {
                header: header.clone(),
                header_view: Self::text_header_view(header),
                expanded: false,
                disabled: false,
                indicator: IndicatorPosition::default(),
                trigger_area: AccordionTriggerArea::default(),
                tree_mode: AccordionTreeMode::default(),
                tree_depth: 0,
                tree_has_children: false,
                tree_selected: false,
                tree_show_lines: true,
                reduced_motion: false,
                animation_ms: animation_ms(),
                body_max_height: body_max_height(),
                body_border: false,
                on_toggle: Rc::new(|_| {}),
                control_state: Default::default(),
            },
        }
    }

    fn text_header_view(header: String) -> AccordionHeaderView {
        Rc::new(move || {
            let text = header.clone();
            label(move || text.clone()).into_any()
        })
    }

    #[must_use]
    pub fn header<IV: IntoView + 'static>(mut self, header: impl Fn() -> IV + 'static) -> Self {
        self.props.header_view = Rc::new(move || header().into_any());
        self
    }

    pub fn controlled(mut self, open: RwSignal<bool>) -> Self {
        self.props.control_state = AccordionControlState::Controlled(open);
        self
    }

    pub fn uncontrolled(mut self) -> Self {
        self.props.control_state = AccordionControlState::Uncontrolled;
        self
    }

    #[must_use]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.props.expanded = expanded;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn indicator(mut self, indicator: IndicatorPosition) -> Self {
        self.props.indicator = indicator;
        self
    }

    #[must_use]
    pub fn trigger_area(mut self, trigger_area: AccordionTriggerArea) -> Self {
        self.props.trigger_area = trigger_area;
        self
    }

    #[must_use]
    pub fn tree_mode(mut self, tree_mode: AccordionTreeMode) -> Self {
        self.props.tree_mode = tree_mode;
        self
    }

    #[must_use]
    pub fn tree_depth(mut self, tree_depth: usize) -> Self {
        self.props.tree_depth = tree_depth;
        self
    }

    #[must_use]
    pub fn tree_has_children(mut self, tree_has_children: bool) -> Self {
        self.props.tree_has_children = tree_has_children;
        self
    }

    #[must_use]
    pub fn tree_selected(mut self, tree_selected: bool) -> Self {
        self.props.tree_selected = tree_selected;
        self
    }

    #[must_use]
    pub fn tree_show_lines(mut self, tree_show_lines: bool) -> Self {
        self.props.tree_show_lines = tree_show_lines;
        self
    }

    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: bool) -> Self {
        self.props.reduced_motion = reduced_motion;
        self
    }

    #[must_use]
    pub fn animation_ms(mut self, animation_ms: u32) -> Self {
        self.props.animation_ms = animation_ms;
        self
    }

    #[must_use]
    pub fn body_max_height(mut self, body_max_height: f32) -> Self {
        self.props.body_max_height = body_max_height;
        self
    }

    #[must_use]
    pub fn body_border(mut self, body_border: bool) -> Self {
        self.props.body_border = body_border;
        self
    }

    #[must_use]
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) + 'static) -> Self {
        self.props.on_toggle = Rc::new(on_toggle);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedAccordion {
        let (pv, ph) = header_padding();
        let expanded = match &self.props.control_state {
            AccordionControlState::Controlled(open) => open.get(),
            AccordionControlState::Uncontrolled => self.props.expanded,
        };

        ResolvedAccordion {
            header: self.props.header.clone(),
            chevron: chevron_symbol(expanded, self.props.indicator),
            indicator: self.props.indicator,
            expanded,
            disabled: self.props.disabled,
            header_font_size: header_font_size(),
            header_pad_v: pv,
            header_pad_h: ph,
            header_bg: header_bg(self.props.disabled, theme),
            header_text: header_text(self.props.disabled, theme),
            border_color: border_color(theme),
            animation_ms: self.props.animation_ms,
            trigger_area: self.props.trigger_area,
            tree_mode: self.props.tree_mode,
            tree_depth: self.props.tree_depth,
            tree_has_children: self.props.tree_has_children,
            tree_selected: self.props.tree_selected,
            tree_show_lines: self.props.tree_show_lines,
            reduced_motion: self.props.reduced_motion,
            body_max_height: self.props.body_max_height,
            body_border: self.props.body_border,
            on_toggle: Rc::clone(&self.props.on_toggle),
        }
    }
}
