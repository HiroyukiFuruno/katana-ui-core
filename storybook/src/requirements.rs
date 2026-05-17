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

pub fn required_pages() -> &'static [&'static str] {
    REQUIRED_PAGES
}

pub fn minimum_nodes_for(page: &str) -> usize {
    match page {
        "text" | "icon" | "divider" | "spacer" | "key-cap" | "loading-dots" | "spinner"
        | "progress-bar" | "color-swatch" | "toggle" | "slide-control" => 1,
        "button" | "text-button" | "svg-button" | "icon-text-button" | "text-input"
        | "checkbox" | "radio" | "badge" => 1,
        "card" | "tooltip" | "modal" | "popover" | "row" | "column" | "stack" | "grid"
        | "scroll-area" | "split-pane" | "align-center" => 2,
        "list" | "menu" | "tabs" | "toolbar" | "form-field" | "breadcrumb" | "accordion"
        | "code-diff" | "color-picker-rgba" | "combo-box" | "dynamic-array-editor"
        | "menu-button" | "modal-overlay" | "notification-toast" | "search-box"
        | "segmented-toggle" | "select-box" | "selection-list" | "side-menu" | "status-bar"
        | "tree-view" | "theme-tokens" => 3,
        "command-palette" => 4,
        _ => 2,
    }
}
