/// Visual style variant for Card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Plain,
    Elevated,
    Outlined,
}

/// Inner padding size for Card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardPadding {
    None,
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `Card`.
pub struct CardProps {
    pub variant: CardVariant,
    pub padding: CardPadding,
    pub interactive: bool,
    pub header: Option<Box<dyn floem::View>>,
    pub body: Option<Box<dyn floem::View>>,
    pub footer: Option<Box<dyn floem::View>>,
    pub actions: Option<Box<dyn floem::View>>,
    pub content: Option<Box<dyn floem::View>>,
    pub on_click: Option<std::rc::Rc<dyn Fn()>>,
}
