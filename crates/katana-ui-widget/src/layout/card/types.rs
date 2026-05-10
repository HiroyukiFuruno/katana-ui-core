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
#[derive(Debug, Clone)]
pub struct CardProps {
    pub variant: CardVariant,
    pub padding: CardPadding,
    pub interactive: bool,
}
