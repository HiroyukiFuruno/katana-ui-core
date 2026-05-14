use super::types::{AccordionTreeMode, AccordionTriggerArea, IndicatorPosition};
use crate::theme::color::Color;
use std::rc::Rc;

/// Resolved visual properties for `Accordion`.
#[derive(Clone)]
pub struct ResolvedAccordion {
    pub header: String,
    pub chevron: Option<&'static str>,
    pub indicator: IndicatorPosition,
    pub expanded: bool,
    pub disabled: bool,
    pub header_font_size: f32,
    pub header_pad_v: f32,
    pub header_pad_h: f32,
    pub header_bg: Color,
    pub header_text: Color,
    pub border_color: Color,
    pub animation_ms: u32,
    pub trigger_area: AccordionTriggerArea,
    pub tree_mode: AccordionTreeMode,
    pub tree_depth: usize,
    pub tree_has_children: bool,
    pub tree_selected: bool,
    pub tree_show_lines: bool,
    pub reduced_motion: bool,
    pub body_max_height: f32,
    pub body_border: bool,
    pub on_toggle: Rc<dyn Fn(bool)>,
}

impl ResolvedAccordion {
    pub fn toggle(&self) -> Option<bool> {
        if self.disabled {
            return None;
        }

        let next = !self.expanded;
        (self.on_toggle)(next);
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::theme::Theme;

    #[test]
    fn resolved_toggle_calls_handler() {
        let theme = Theme::default_light();
        let called = std::rc::Rc::new(std::cell::RefCell::new(None));
        let called_ref = std::rc::Rc::clone(&called);
        let resolved = Accordion::new("Section")
            .expanded(false)
            .on_toggle(move |expanded| {
                *called_ref.borrow_mut() = Some(expanded);
            })
            .resolve(&theme);

        assert_eq!(resolved.toggle(), Some(true));
        assert_eq!(*called.borrow(), Some(true));
    }
}
