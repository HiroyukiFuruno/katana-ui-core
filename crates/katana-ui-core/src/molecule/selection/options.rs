use super::choice::{Breadcrumb, ComboBox, MenuButton, SelectBox, SelectionList, SideMenu, Tabs};
use super::types::ChoiceItem;
use crate::interaction::{VirtualRange, VirtualizationConfig};
use crate::molecule::virtualization::MoleculeVirtualization;
use crate::render_model::UiStateId;

macro_rules! selection_options {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn state_id(&self) -> &UiStateId {
                &self.state.state_id
            }

            #[must_use]
            pub fn items(&self) -> &[ChoiceItem] {
                &self.items
            }
        }
    };
}

selection_options!(Breadcrumb);
selection_options!(ComboBox);
selection_options!(MenuButton);
selection_options!(SelectBox);
selection_options!(SelectionList);
selection_options!(SideMenu);
selection_options!(Tabs);

impl Tabs {
    #[must_use]
    pub fn icon_action(mut self, value: impl Into<String>) -> Self {
        self.model.icon_action = value.into();
        self
    }
}

impl SideMenu {
    #[must_use]
    pub fn hover_expansion(mut self, value: bool) -> Self {
        self.model.hover_expansion = value;
        self
    }
}

impl SelectionList {
    #[must_use]
    pub fn section(mut self, value: impl Into<String>) -> Self {
        self.model.section = value.into();
        self
    }

    #[must_use]
    pub fn marker(mut self, value: impl Into<String>) -> Self {
        self.model.marker = value.into();
        self
    }

    #[must_use]
    pub fn more_row(mut self, value: bool) -> Self {
        self.model.more_row = value;
        self
    }
}

impl SelectionList {
    #[must_use]
    pub fn virtualization(mut self, value: VirtualizationConfig) -> Self {
        self.model.virtualization = Some(value);
        self
    }

    #[must_use]
    pub fn virtual_range_model(&self) -> Option<VirtualRange> {
        MoleculeVirtualization::range(&self.model.virtualization, self.items.len())
    }
}
