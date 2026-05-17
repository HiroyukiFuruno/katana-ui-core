use super::{StoryCatalog, StoryExample};
use katana_ui_core::{atom, molecule};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story(
            "card",
            molecule::Card::new("Card").child(atom::Text::new("Body")),
        ),
        StoryCatalog::story(
            "list",
            molecule::List::new("List")
                .child(atom::Text::new("Row 1"))
                .child(atom::Text::new("Row 2")),
        ),
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
            "tabs",
            molecule::Tabs::new("Tabs")
                .child(atom::Text::new("Tab"))
                .child(atom::Text::new("Panel")),
        ),
        StoryCatalog::story(
            "toolbar",
            molecule::Toolbar::new("Toolbar")
                .child(atom::Button::new("Save"))
                .child(atom::Button::new("Undo")),
        ),
        StoryCatalog::story(
            "form-field",
            molecule::FormField::new("Form field")
                .child(atom::Text::new("Label"))
                .child(atom::Input::new("Value")),
        ),
        StoryCatalog::story(
            "breadcrumb",
            molecule::Breadcrumb::new("Breadcrumb")
                .child(atom::Text::new("Root"))
                .child(atom::Text::new("Leaf")),
        ),
        StoryCatalog::story(
            "accordion",
            molecule::Accordion::new("Accordion")
                .open(true)
                .child(atom::Button::new("Toggle"))
                .child(atom::Text::new("Panel")),
        ),
        StoryCatalog::story(
            "code-diff",
            molecule::CodeDiff::new("Code diff")
                .item_count(2)
                .child(atom::Text::new("- old"))
                .child(atom::Text::new("+ new")),
        ),
        StoryCatalog::story(
            "color-picker-rgba",
            molecule::ColorPicker::new("Color picker")
                .open(true)
                .value("rgba(64, 128, 255, 1)")
                .child(atom::ColorSwatch::new("Preview"))
                .child(atom::SlideControl::new("Alpha")),
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
            "command-palette",
            molecule::CommandPalette::new("Command palette")
                .open(true)
                .selected_index(0)
                .item_count(1)
                .child(molecule::SearchBox::new("Search"))
                .child(molecule::SelectionList::new("Commands"))
                .child(atom::Text::new("Action")),
        ),
        StoryCatalog::story(
            "dynamic-array-editor",
            molecule::DynamicArrayEditor::new("Dynamic array")
                .item_count(1)
                .child(atom::Button::new("Add"))
                .child(atom::Text::new("Item")),
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
            "search-box",
            molecule::SearchBox::new("Search box")
                .value("query")
                .child(atom::Input::new("Query"))
                .child(atom::Button::new("Clear")),
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
        StoryCatalog::story(
            "selection-list",
            molecule::SelectionList::new("Selection list")
                .child(atom::Text::new("First"))
                .child(atom::Text::new("Second")),
        ),
        StoryCatalog::story(
            "side-menu",
            molecule::SideMenu::new("Side menu")
                .child(atom::Button::new("Files"))
                .child(atom::Button::new("Settings")),
        ),
        StoryCatalog::story(
            "status-bar",
            molecule::StatusBar::new("Status bar")
                .child(atom::Badge::new("Ready"))
                .child(atom::Text::new("Ln 1")),
        ),
        StoryCatalog::story(
            "tree-view",
            molecule::TreeView::new("Tree view")
                .open(true)
                .item_count(2)
                .child(atom::Text::new("Parent"))
                .child(atom::Text::new("Child")),
        ),
    ]
}
