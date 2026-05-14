use floem::View;
use floem::reactive::RwSignal;
use std::rc::Rc;

pub type AccordionHeaderView = Rc<dyn Fn() -> Box<dyn View>>;

/// Position of the expand/collapse indicator chevron.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndicatorPosition {
    Leading,
    #[default]
    Trailing,
    None,
}

/// Click target area for triggering accordion open/close.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionTriggerArea {
    IconAndLabel,
    IconOnly,
    LabelOnly,
    /// Entire row is clickable.
    #[default]
    FullRow,
}

/// Tree-mode configuration for nested accordion rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionTreeMode {
    #[default]
    Disabled,
    Enabled,
}

#[derive(Clone, Default)]
pub(super) enum AccordionControlState {
    Controlled(RwSignal<bool>),
    #[default]
    Uncontrolled,
}

/// Properties for `Accordion`.
#[derive(Clone)]
pub struct AccordionProps {
    pub header: String,
    pub header_view: AccordionHeaderView,
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
    pub body_border: bool,
    pub on_toggle: Rc<dyn Fn(bool)>,
    pub(super) control_state: AccordionControlState,
}
