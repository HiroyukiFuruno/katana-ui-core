/// Position of the expand/collapse indicator chevron.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndicatorPosition {
    Leading,
    #[default]
    Trailing,
    None,
}

/// Properties for `Accordion`.
#[derive(Debug, Clone)]
pub struct AccordionProps {
    pub header: String,
    pub expanded: bool,
    pub disabled: bool,
    pub indicator: IndicatorPosition,
}
