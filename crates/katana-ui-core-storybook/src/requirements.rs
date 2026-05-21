const REQUIRED_PAGES: &[&str] = &[
    "text",
    "icon",
    "chip",
    "button",
    "text-button",
    "svg-button",
    "icon-text-button",
    "text-input",
    "text-area",
    "checkbox",
    "radio",
    "badge",
    "divider",
    "spacer",
    "key-cap",
    "skeleton",
    "loading-dots",
    "spinner",
    "progress-bar",
    "color-swatch",
    "toggle",
    "slide-control",
    "card",
    "list",
    "menu",
    "context-menu",
    "banner",
    "toast-stack-manager",
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
    "hover-card",
    "search-box",
    "search-control-strip",
    "segmented-toggle",
    "select-box",
    "selection-list",
    "side-menu",
    "status-bar",
    "shortcut-combo",
    "shortcut-cheatsheet",
    "settings-list",
    "collapsible-panel",
    "virtualization",
    "skeleton-cluster",
    "motion",
    "window-control-button-group",
    "startup-state-panel",
    "attachment-chip",
    "chip-group",
    "diagnostics-list",
    "empty-state",
    "tree-view",
    "drag-and-drop",
    "closeable-tab-strip",
    "panel",
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
            | "progress-bar" | "color-swatch" | "toggle" | "slide-control" | "chip"
            | "shortcut-combo" | "skeleton" | "virtualization" | "motion" => MIN_SINGLE_NODE,
            "button" | "text-button" | "svg-button" | "icon-text-button" | "text-input"
            | "text-area" | "checkbox" | "radio" | "badge" => MIN_SINGLE_NODE,
            "card"
            | "tooltip"
            | "modal"
            | "popover"
            | "hover-card"
            | "empty-state"
            | "row"
            | "column"
            | "stack"
            | "grid"
            | "scroll-area"
            | "split-pane"
            | "align-center"
            | "banner"
            | "shortcut-cheatsheet"
            | "skeleton-cluster"
            | "collapsible-panel"
            | "startup-state-panel" => MIN_CHILDREN_NODE,
            "list"
            | "menu"
            | "context-menu"
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
            | "search-control-strip"
            | "segmented-toggle"
            | "select-box"
            | "selection-list"
            | "side-menu"
            | "status-bar"
            | "toast-stack-manager"
            | "settings-list"
            | "window-control-button-group"
            | "attachment-chip"
            | "chip-group"
            | "diagnostics-list"
            | "tree-view"
            | "drag-and-drop"
            | "closeable-tab-strip"
            | "panel"
            | "theme-tokens" => MIN_COMPOSITE_NODE,
            "command-palette" => MIN_COMMAND_PALETTE_NODE,
            _ => MIN_CHILDREN_NODE,
        }
    }
}
