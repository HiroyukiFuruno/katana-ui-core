use crate::requirements::{minimum_nodes_for, required_pages};
use katana_ui_core::render_model::{UiNode, UiStateId, UiTree};
use katana_ui_core::{atom, layout, molecule};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryExample {
    pub page: &'static str,
    pub tree: UiTree,
    pub minimum_nodes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryCatalogReport {
    pub stories: usize,
    pub validated: usize,
    pub state_conflicts: usize,
    pub structure_failures: usize,
    pub missing_required_pages: usize,
    pub nodes: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StoryCatalog;

impl StoryCatalog {
    #[must_use]
    pub fn examples(self) -> Vec<StoryExample> {
        let mut examples = Vec::new();
        examples.extend(Self::atom_examples());
        examples.extend(Self::molecule_examples());
        examples.extend(Self::layout_examples());
        examples
    }

    fn atom_examples() -> Vec<StoryExample> {
        vec![
            Self::story("text", atom::Text::new("Text").accessibility_label("Text")),
            Self::story("icon", atom::Icon::new("Icon").accessibility_label("Icon")),
            Self::story("button", atom::Button::new("Button").focusable(true)),
            Self::story("text-button", atom::TextButton::new("Text button")),
            Self::story("svg-button", atom::SvgButton::new("Svg button")),
            Self::story("icon-text-button", atom::IconTextButton::new("Icon text")),
            Self::story(
                "text-input",
                atom::Input::new("Text input").focusable(true).value("typed"),
            ),
            Self::story("checkbox", atom::Checkbox::new("Checkbox")),
            Self::story("radio", atom::Radio::new("Radio")),
            Self::story("badge", atom::Badge::new("Badge").accessibility_label("Status badge")),
            Self::story("divider", atom::Divider::new("Divider")),
            Self::story("spacer", atom::Spacer::new("Spacer")),
            Self::story("key-cap", atom::KeyCap::new("Key cap").accessibility_label("Shortcut")),
            Self::story("loading-dots", atom::LoadingDots::new("Loading dots")),
            Self::story("spinner", atom::Spinner::new("Spinner").accessibility_label("Loading")),
            Self::story("progress-bar", atom::ProgressBar::new("Progress bar")),
            Self::story(
                "color-swatch",
                atom::ColorSwatch::new("Color swatch").value("rgba(64, 128, 255, 1)"),
            ),
            Self::story("toggle", atom::Toggle::new("Toggle").selected(true)),
            Self::story("slide-control", atom::SlideControl::new("Slide control")),
        ]
    }

    fn molecule_examples() -> Vec<StoryExample> {
        vec![
            Self::story("card", molecule::Card::new("Card").child(atom::Text::new("Body"))),
            Self::story("list", molecule::List::new("List").child(atom::Text::new("Row 1")).child(atom::Text::new("Row 2"))),
            Self::story("menu", molecule::Menu::new("Menu").child(atom::Button::new("Open")).child(atom::Button::new("Close"))),
            Self::story("tooltip", molecule::Tooltip::new("Tooltip").open(true).child(atom::Icon::new("Info")).child(atom::Text::new("Hint"))),
            Self::story("modal", molecule::Modal::new("Modal").open(true).child(atom::Text::new("Body")).child(atom::Button::new("Close"))),
            Self::story("tabs", molecule::Tabs::new("Tabs").child(atom::Text::new("Tab")).child(atom::Text::new("Panel"))),
            Self::story("toolbar", molecule::Toolbar::new("Toolbar").child(atom::Button::new("Save")).child(atom::Button::new("Undo"))),
            Self::story("form-field", molecule::FormField::new("Form field").child(atom::Text::new("Label")).child(atom::Input::new("Value"))),
            Self::story("breadcrumb", molecule::Breadcrumb::new("Breadcrumb").child(atom::Text::new("Root")).child(atom::Text::new("Leaf"))),
            Self::story("accordion", molecule::Accordion::new("Accordion").open(true).child(atom::Button::new("Toggle")).child(atom::Text::new("Panel"))),
            Self::story("code-diff", molecule::CodeDiff::new("Code diff").item_count(2).child(atom::Text::new("- old")).child(atom::Text::new("+ new"))),
            Self::story("color-picker-rgba", molecule::ColorPicker::new("Color picker").open(true).value("rgba(64, 128, 255, 1)").child(atom::ColorSwatch::new("Preview")).child(atom::SlideControl::new("Alpha"))),
            Self::story("combo-box", molecule::ComboBox::new("Combo box").open(true).selected_index(0).item_count(1).child(atom::Input::new("Search")).child(atom::Text::new("Option"))),
            Self::story("command-palette", molecule::CommandPalette::new("Command palette").open(true).selected_index(0).item_count(1).child(molecule::SearchBox::new("Search")).child(molecule::SelectionList::new("Commands")).child(atom::Text::new("Action"))),
            Self::story("dynamic-array-editor", molecule::DynamicArrayEditor::new("Dynamic array").item_count(1).child(atom::Button::new("Add")).child(atom::Text::new("Item"))),
            Self::story("menu-button", molecule::MenuButton::new("Menu button").open(true).item_count(1).child(atom::Button::new("Trigger")).child(molecule::Menu::new("Menu"))),
            Self::story("modal-overlay", molecule::ModalOverlay::new("Modal overlay").open(true).child(molecule::Modal::new("Dialog")).child(atom::Button::new("Dismiss"))),
            Self::story("notification-toast", molecule::NotificationToast::new("Notification").open(true).child(atom::Badge::new("Info")).child(atom::Text::new("Message"))),
            Self::story("popover", molecule::Popover::new("Popover").open(true).child(atom::Button::new("Anchor")).child(atom::Text::new("Content"))),
            Self::story("search-box", molecule::SearchBox::new("Search box").value("query").child(atom::Input::new("Query")).child(atom::Button::new("Clear"))),
            Self::story("segmented-toggle", molecule::SegmentedToggle::new("Segmented toggle").selected_index(1).item_count(2).child(atom::Toggle::new("A")).child(atom::Toggle::new("B"))),
            Self::story("select-box", molecule::SelectBox::new("Select box").selected_index(0).item_count(2).child(atom::Button::new("Trigger")).child(molecule::List::new("Options"))),
            Self::story("selection-list", molecule::SelectionList::new("Selection list").child(atom::Text::new("First")).child(atom::Text::new("Second"))),
            Self::story("side-menu", molecule::SideMenu::new("Side menu").child(atom::Button::new("Files")).child(atom::Button::new("Settings"))),
            Self::story("status-bar", molecule::StatusBar::new("Status bar").child(atom::Badge::new("Ready")).child(atom::Text::new("Ln 1"))),
            Self::story("tree-view", molecule::TreeView::new("Tree view").open(true).item_count(2).child(atom::Text::new("Parent")).child(atom::Text::new("Child"))),
        ]
    }

    fn layout_examples() -> Vec<StoryExample> {
        vec![
            Self::story("row", layout::Row::new().child(atom::Text::new("Row item"))),
            Self::story("column", layout::Column::new().child(atom::Text::new("Column item"))),
            Self::story("stack", layout::Stack::new().child(atom::Text::new("Stack item"))),
            Self::story("grid", layout::Grid::new().child(atom::Text::new("Grid item")).child(atom::Text::new("Grid item 2"))),
            Self::story("scroll-area", layout::ScrollArea::new().child(atom::Text::new("Scroll item"))),
            Self::story("split-pane", layout::SplitPane::new().value("0.5").child(atom::Text::new("Left")).child(atom::Text::new("Right"))),
            Self::story("align-center", layout::AlignCenter::new().child(atom::Text::new("Centered"))),
            Self::story("theme-tokens", molecule::Card::new("Theme tokens").child(atom::Badge::new("Light/Dark")).child(atom::ColorSwatch::new("Accent"))),
        ]
    }

    #[must_use]
    pub fn verify(self) -> StoryCatalogReport {
        let examples = self.examples();
        self.verify_examples(&examples)
    }

    #[must_use]
    pub fn verify_examples(self, examples: &[StoryExample]) -> StoryCatalogReport {
        let state_conflicts = examples.iter().filter(|it| Self::has_state_conflict(&it.tree)).count();
        let structure_failures = examples.iter().filter(|it| Self::node_count(it.tree.root()) < it.minimum_nodes).count();
        let present: BTreeSet<&str> = examples.iter().map(|it| it.page).collect();
        let missing_required_pages = required_pages().iter().filter(|it| !present.contains(**it)).count();
        StoryCatalogReport {
            stories: examples.len(),
            validated: examples.len() - state_conflicts - structure_failures,
            state_conflicts,
            structure_failures,
            missing_required_pages,
            nodes: examples.iter().map(|it| Self::node_count(it.tree.root())).sum(),
        }
    }

    fn story(page: &'static str, root: impl Into<UiNode>) -> StoryExample {
        StoryExample {
            page,
            tree: UiTree::new(root),
            minimum_nodes: minimum_nodes_for(page),
        }
    }

    fn has_state_conflict(tree: &UiTree) -> bool {
        let mut ids = Vec::new();
        Self::collect_state_ids(tree.root(), &mut ids);
        let unique: BTreeSet<&str> = ids.iter().map(UiStateId::as_str).collect();
        unique.len() != ids.len()
    }

    fn collect_state_ids(node: &UiNode, ids: &mut Vec<UiStateId>) {
        ids.push(node.props().state_id.clone());
        for child in node.children() {
            Self::collect_state_ids(child, ids);
        }
    }

    fn node_count(node: &UiNode) -> usize {
        1 + node.children().iter().map(Self::node_count).sum::<usize>()
    }
}

impl StoryCatalogReport {
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "stories={} validated={} state_conflicts={} structure_failures={} missing_required_pages={} nodes={}",
            self.stories,
            self.validated,
            self.state_conflicts,
            self.structure_failures,
            self.missing_required_pages,
            self.nodes
        )
    }
}

#[cfg(test)]
mod tests {
    use super::StoryCatalog;
    use crate::requirements::required_pages;
    use katana_ui_core::render_model::UiNodeKind;
    use std::collections::BTreeSet;

    #[test]
    fn catalog_covers_required_legacy_and_core_targets() {
        let examples = StoryCatalog.examples();
        let pages: BTreeSet<&str> = examples.iter().map(|it| it.page).collect();
        assert!(required_pages().iter().all(|it| pages.contains(*it)));
        assert!(examples.iter().any(|it| it.tree.root().kind() == UiNodeKind::CodeDiff));
        assert!(examples.iter().any(|it| it.tree.root().kind() == UiNodeKind::Grid));
    }

    #[test]
    fn core_catalog_validates_every_story_without_shared_state() {
        let report = StoryCatalog.verify();
        assert_eq!(report.stories, report.validated);
        assert_eq!(0, report.state_conflicts);
        assert_eq!(0, report.structure_failures);
        assert_eq!(0, report.missing_required_pages);
    }
}
