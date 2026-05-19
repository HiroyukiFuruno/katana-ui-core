pub(crate) struct StoryPresetLabels;

impl StoryPresetLabels {
    pub(crate) fn for_page(page: &str) -> &'static [&'static str] {
        match page {
            "theme-tokens" => &[
                "token table",
                "theme switch",
                "missing token",
                "katana accent",
            ],
            "text" => &["role grid", "mixed script", "empty text", "theme color"],
            "icon" => &["svg grid", "accent icon", "missing label", "muted icon"],
            "checkbox" => &["unchecked", "checked", "disabled", "focus ring"],
            "radio" => &["unselected", "selected", "group", "focus ring"],
            "divider" => &["horizontal", "vertical", "inset", "theme line"],
            "spacer" => &["fixed gap", "flex gap", "dense", "theme gap"],
            "loading-dots" => &[
                "running dots",
                "phase tick",
                "reduced motion",
                "theme label",
            ],
            "spinner" => &["running spinner", "phase tick", "paused", "theme label"],
            "progress-bar" => &["determinate", "change", "empty", "theme track"],
            "button" | "text-button" | "svg-button" | "icon-text-button" => {
                &["modern", "classic", "basic", "dense"]
            }
            "toggle" => &["off", "on action", "disabled", "theme switch"],
            "slide-control" => &["track", "drag", "step", "theme knob"],
            "segmented-toggle" => &[
                "segments",
                "select action",
                "disabled segment",
                "theme marker",
            ],
            "select-box" => &["trigger", "select action", "long list", "theme panel"],
            "combo-box" => &["input list", "select row", "filter", "theme menu"],
            "menu-button" => &["trigger", "open menu", "disabled", "theme menu"],
            "color-swatch" => &["palette", "select color", "disabled", "theme ring"],
            "text-input" => &["value", "ime event", "invalid", "theme slot"],
            "search-box" => &["search icon", "submit action", "regex case", "theme clear"],
            "form-field" => &["label", "invalid", "helper", "theme field"],
            "tooltip" => &["anchor", "hover open", "placement edge", "theme overlay"],
            "badge" => &["tone grid", "dismiss action", "small size", "theme badge"],
            "key-cap" => &["single key", "combo", "non-macos", "theme key"],
            "card" => &["slots", "card click", "nested controls", "theme border"],
            "list" => &["rows", "selection", "empty", "theme list"],
            "menu" => &["menu items", "shortcut", "disabled", "theme menu"],
            "context-menu" => &[
                "編集器右クリック",
                "explorer 空領域",
                "tab bar",
                "message 行",
                "leading icon + shortcut",
            ],
            "tabs" => &["browser tab", "switch", "overflow", "theme line"],
            "toolbar" => &["icon tools", "toggle", "disabled", "theme bar"],
            "breadcrumb" => &["trail", "click", "overflow", "theme crumb"],
            "accordion" => &[
                "header body",
                "toggle action",
                "tree mode",
                "theme indicator",
            ],
            "split-pane" => &["horizontal", "drag action", "min clamp", "theme handle"],
            "modal" | "modal-overlay" => &["dialog", "escape close", "focus trap", "theme overlay"],
            "notification-toast" => &["toast", "dismiss", "stack", "theme toast"],
            "popover" => &["anchor", "outside close", "placement edge", "theme shadow"],
            "color-picker-rgba" => &["rgba panel", "drag plane", "alpha edge", "theme preview"],
            "code-diff" => &["inline", "mode switch", "collapsed", "theme japanese"],
            "command-palette" => &["commands", "execute", "empty", "theme modal"],
            "dynamic-array-editor" => &["rows", "add remove", "reorder", "theme row"],
            "selection-list" => &["items", "select", "multi", "theme list"],
            "side-menu" => &["nav tree", "select", "collapse", "theme side"],
            "status-bar" => &["segments", "update", "warning", "theme bar"],
            "tree-view" => &["folders", "toggle", "context", "theme tree"],
            "row" => &["row layout", "align", "overflow", "theme gap"],
            "column" => &["column", "align", "overflow", "theme gap"],
            "stack" => &["stacked", "z order", "overlay", "theme stack"],
            "grid" => &["grid cells", "span", "overflow", "theme grid"],
            "scroll-area" => &["viewport", "scroll", "scrollbar", "theme scroll"],
            "align-center" => &["center box", "baseline", "mixed text", "theme align"],
            _ => &["overview", "operate", "edge case", "theme view"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StoryPresetLabels;
    use crate::requirements::StoryRequirements;
    use std::collections::BTreeSet;

    const GENERIC_LABELS: &[&str] = &["default", "interactive", "edge", "theme"];

    #[test]
    fn required_pages_have_component_specific_preset_labels() {
        for page in StoryRequirements::required_pages() {
            let labels = StoryPresetLabels::for_page(page);
            let unique: BTreeSet<&str> = labels.iter().copied().collect();

            assert_eq!(labels.len(), unique.len(), "{page} has duplicate presets");
            assert!(
                labels.iter().all(|it| !it.is_empty()),
                "{page} has an empty preset label"
            );
            assert!(
                labels.iter().all(|it| !GENERIC_LABELS.contains(it)),
                "{page} still exposes generic preset labels: {labels:?}"
            );
        }
    }
}
