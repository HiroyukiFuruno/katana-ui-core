use std::rc::Rc;

use floem::IntoView;
use floem::View;
use floem::reactive::SignalUpdate;
use floem::reactive::create_rw_signal;
use floem::views::{Decorators, v_stack_from_iter};

use crate::theme::Theme;

use super::Accordion;
use super::types::{AccordionTreeMode, AccordionTriggerArea, IndicatorPosition};

const DEFAULT_ANIMATION_MS: u32 = 180;
const DEFAULT_BODY_MAX_HEIGHT: f32 = 240.0;

/// Item model for grouped accordion rendering.
#[derive(Clone)]
pub struct AccordionGroupItem<IV: IntoView + 'static> {
    pub header: String,
    pub body: Rc<dyn Fn() -> IV>,
    pub expanded: bool,
    pub disabled: bool,
    pub indicator: IndicatorPosition,
    pub trigger_area: AccordionTriggerArea,
    pub tree_mode: AccordionTreeMode,
    pub tree_depth: usize,
    pub tree_has_children: bool,
    pub tree_selected: bool,
    pub tree_show_lines: bool,
    pub reduced_motion: bool,
    pub animation_ms: u32,
    pub body_max_height: f32,
    pub on_toggle: Rc<dyn Fn(bool)>,
}

impl<IV: IntoView + 'static> AccordionGroupItem<IV> {
    #[must_use]
    pub fn new(header: impl Into<String>, body: impl Fn() -> IV + 'static) -> Self {
        Self {
            header: header.into(),
            body: Rc::new(body),
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
            animation_ms: DEFAULT_ANIMATION_MS,
            body_max_height: DEFAULT_BODY_MAX_HEIGHT,
            on_toggle: Rc::new(|_| {}),
        }
    }

    #[must_use]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn indicator(mut self, indicator: IndicatorPosition) -> Self {
        self.indicator = indicator;
        self
    }

    #[must_use]
    pub fn trigger_area(mut self, trigger_area: AccordionTriggerArea) -> Self {
        self.trigger_area = trigger_area;
        self
    }

    #[must_use]
    pub fn tree_mode(mut self, tree_mode: AccordionTreeMode) -> Self {
        self.tree_mode = tree_mode;
        self
    }

    #[must_use]
    pub fn tree_depth(mut self, tree_depth: usize) -> Self {
        self.tree_depth = tree_depth;
        self
    }

    #[must_use]
    pub fn tree_has_children(mut self, tree_has_children: bool) -> Self {
        self.tree_has_children = tree_has_children;
        self
    }

    #[must_use]
    pub fn tree_selected(mut self, tree_selected: bool) -> Self {
        self.tree_selected = tree_selected;
        self
    }

    #[must_use]
    pub fn tree_show_lines(mut self, tree_show_lines: bool) -> Self {
        self.tree_show_lines = tree_show_lines;
        self
    }

    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: bool) -> Self {
        self.reduced_motion = reduced_motion;
        self
    }

    #[must_use]
    pub fn animation_ms(mut self, animation_ms: u32) -> Self {
        self.animation_ms = animation_ms;
        self
    }

    #[must_use]
    pub fn body_max_height(mut self, body_max_height: f32) -> Self {
        self.body_max_height = body_max_height;
        self
    }

    #[must_use]
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) + 'static) -> Self {
        self.on_toggle = Rc::new(on_toggle);
        self
    }
}

/// Grouped accordion builder to control single/open-multiple behavior.
#[derive(Clone)]
pub struct AccordionGroup<IV: IntoView + 'static> {
    allow_multiple: bool,
    items: Vec<AccordionGroupItem<IV>>,
}

impl<IV: IntoView + 'static> AccordionGroup<IV> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            allow_multiple: true,
            items: Vec::new(),
        }
    }

    #[must_use]
    pub fn allow_multiple(mut self, allow_multiple: bool) -> Self {
        self.allow_multiple = allow_multiple;
        self
    }

    #[must_use]
    pub fn push(mut self, item: AccordionGroupItem<IV>) -> Self {
        self.items.push(item);
        self
    }

    pub fn view(self, theme: Theme) -> impl IntoView {
        if self.items.is_empty() {
            return v_stack_from_iter(Vec::<Box<dyn View>>::new())
                .style(|style| style.width_full())
                .into_any();
        }

        let allow_multiple = self.allow_multiple;
        let row_signals = Rc::new(
            self.items
                .iter()
                .map(|item| create_rw_signal(item.expanded))
                .collect::<Vec<_>>(),
        );

        let rows = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let row_signals = Rc::clone(&row_signals);
                let signal = row_signals[index];
                let on_item_toggle = Rc::clone(&item.on_toggle);
                let child = Rc::clone(&item.body);
                let item_theme = theme.clone();
                let selected = item.tree_selected;
                let show_lines = item.tree_show_lines;
                let depth = item.tree_depth;
                let has_children = item.tree_has_children;
                let on_toggle = Rc::new(move |next: bool| {
                    if !allow_multiple && next {
                        for (other_index, sibling_signal) in row_signals.iter().enumerate() {
                            if other_index != index {
                                sibling_signal.set(false);
                            }
                        }
                    }
                    on_item_toggle(next);
                });

                Accordion::new(item.header)
                    .controlled(signal)
                    .expanded(item.expanded)
                    .disabled(item.disabled)
                    .indicator(item.indicator)
                    .trigger_area(item.trigger_area)
                    .tree_mode(item.tree_mode)
                    .tree_depth(depth)
                    .tree_has_children(has_children)
                    .tree_selected(selected)
                    .tree_show_lines(show_lines)
                    .reduced_motion(item.reduced_motion)
                    .animation_ms(item.animation_ms)
                    .body_max_height(item.body_max_height)
                    .on_toggle(move |next| on_toggle(next))
                    .view(item_theme, move || child())
                    .into_any()
            })
            .collect::<Vec<_>>();

        v_stack_from_iter(rows)
            .style(move |style| style.width_full().gap(1.0))
            .into_any()
    }
}

impl<IV: IntoView + 'static> Default for AccordionGroup<IV> {
    fn default() -> Self {
        Self::new()
    }
}
