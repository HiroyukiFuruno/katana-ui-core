const REQUIRED_PAGES: &[&str] = &[
    "text",
    "icon",
    "button",
    "text-button",
    "svg-button",
    "icon-text-button",
    "text-input",
    "checkbox",
    "radio",
    "badge",
    "divider",
    "spacer",
    "key-cap",
    "loading-dots",
    "spinner",
    "progress-bar",
    "color-swatch",
    "toggle",
    "slide-control",
    "card",
    "list",
    "menu",
    "tooltip",
    "modal",
    "tabs",
    "toolbar",
    "form-field",
    "breadcrumb",
    "accordion",
    "code-diff",
    "color-picker-rgba",
    "combo-box",
    "command-palette",
    "dynamic-array-editor",
    "menu-button",
    "modal-overlay",
    "notification-toast",
    "popover",
    "search-box",
    "segmented-toggle",
    "select-box",
    "selection-list",
    "side-menu",
    "status-bar",
    "tree-view",
    "row",
    "column",
    "stack",
    "grid",
    "scroll-area",
    "split-pane",
    "align-center",
    "theme-tokens",
];

const MIN_SINGLE_NODE: usize = 1;
const MIN_CHILDREN_NODE: usize = 2;
const MIN_COMPOSITE_NODE: usize = 3;
const MIN_COMMAND_PALETTE_NODE: usize = 4;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StoryRequirements;

impl StoryRequirements {
    pub(crate) fn required_pages() -> &'static [&'static str] {
        REQUIRED_PAGES
    }

    pub(crate) fn minimum_nodes_for(page: &str) -> usize {
        match page {
            "text" | "icon" | "divider" | "spacer" | "key-cap" | "loading-dots" | "spinner"
            | "progress-bar" | "color-swatch" | "toggle" | "slide-control" => MIN_SINGLE_NODE,
            "button" | "text-button" | "svg-button" | "icon-text-button" | "text-input"
            | "checkbox" | "radio" | "badge" => MIN_SINGLE_NODE,
            "card" | "tooltip" | "modal" | "popover" | "row" | "column" | "stack" | "grid"
            | "scroll-area" | "split-pane" | "align-center" => MIN_CHILDREN_NODE,
            "list"
            | "menu"
            | "tabs"
            | "toolbar"
            | "form-field"
            | "breadcrumb"
            | "accordion"
            | "code-diff"
            | "color-picker-rgba"
            | "combo-box"
            | "dynamic-array-editor"
            | "menu-button"
            | "modal-overlay"
            | "notification-toast"
            | "search-box"
            | "segmented-toggle"
            | "select-box"
            | "selection-list"
            | "side-menu"
            | "status-bar"
            | "tree-view"
            | "theme-tokens" => MIN_COMPOSITE_NODE,
            "command-palette" => MIN_COMMAND_PALETTE_NODE,
            _ => MIN_CHILDREN_NODE,
        }
    }
}
