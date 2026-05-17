use super::super::{StoryCatalog, StoryExample};
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
            "search-box",
            molecule::SearchBox::new("Search box")
                .value("query")
                .child(atom::Input::new("Query"))
                .child(atom::Button::new("Clear")),
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
    ]
}
