use super::{HoverCard, PopoverSlots};

impl HoverCard {
    #[must_use]
    pub fn slots(mut self, value: PopoverSlots) -> Self {
        self.slots = value;
        self
    }

    #[must_use]
    pub fn slots_model(&self) -> &PopoverSlots {
        &self.slots
    }
}
