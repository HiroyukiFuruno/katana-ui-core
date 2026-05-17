use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::{atom, molecule};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story(
            "menu",
            molecule::Menu::new("Menu")
                .child(atom::Button::new("Open"))
                .child(atom::Button::new("Close")),
        ),
        StoryCatalog::story(
            "tooltip",
            molecule::Tooltip::new("Tooltip")
                .open(true)
                .child(atom::Icon::new("Info"))
                .child(atom::Text::new("Hint")),
        ),
        StoryCatalog::story(
            "modal",
            molecule::Modal::new("Modal")
                .open(true)
                .child(atom::Text::new("Body"))
                .child(atom::Button::new("Close")),
        ),
        StoryCatalog::story(
            "accordion",
            molecule::Accordion::new("Accordion")
                .open(true)
                .child(atom::Button::new("Toggle"))
                .child(atom::Text::new("Panel")),
        ),
        StoryCatalog::story(
            "combo-box",
            molecule::ComboBox::new("Combo box")
                .open(true)
                .selected_index(0)
                .item_count(1)
                .child(atom::Input::new("Search"))
                .child(atom::Text::new("Option")),
        ),
        StoryCatalog::story(
            "menu-button",
            molecule::MenuButton::new("Menu button")
                .open(true)
                .item_count(1)
                .child(atom::Button::new("Trigger"))
                .child(molecule::Menu::new("Menu")),
        ),
        StoryCatalog::story(
            "modal-overlay",
            molecule::ModalOverlay::new("Modal overlay")
                .open(true)
                .child(molecule::Modal::new("Dialog"))
                .child(atom::Button::new("Dismiss")),
        ),
        StoryCatalog::story(
            "notification-toast",
            molecule::NotificationToast::new("Notification")
                .open(true)
                .child(atom::Badge::new("Info"))
                .child(atom::Text::new("Message")),
        ),
        StoryCatalog::story(
            "popover",
            molecule::Popover::new("Popover")
                .open(true)
                .child(atom::Button::new("Anchor"))
                .child(atom::Text::new("Content")),
        ),
        StoryCatalog::story(
            "segmented-toggle",
            molecule::SegmentedToggle::new("Segmented toggle")
                .selected_index(1)
                .item_count(2)
                .child(atom::Toggle::new("A"))
                .child(atom::Toggle::new("B")),
        ),
        StoryCatalog::story(
            "select-box",
            molecule::SelectBox::new("Select box")
                .selected_index(0)
                .item_count(2)
                .child(atom::Button::new("Trigger"))
                .child(molecule::List::new("Options")),
        ),
    ]
}
