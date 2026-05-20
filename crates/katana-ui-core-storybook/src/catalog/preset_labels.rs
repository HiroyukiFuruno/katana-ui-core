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
            "chip" => &["filter tag", "dismiss", "selected", "tone matrix"],
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
            "text-area" => &[
                "chat composer",
                "search multiline",
                "long text",
                "auto grow",
                "max rows",
                "ime input",
                "emoji input",
            ],
            "search-box" => &["search icon", "submit action", "regex case", "theme clear"],
            "search-control-strip" => &[
                "workspace search",
                "editor find",
                "editor replace",
                "viewer search",
                "history search",
            ],
            "form-field" => &["label", "invalid", "helper", "theme field"],
            "tooltip" => &["anchor", "hover open", "placement edge", "theme overlay"],
            "badge" => &["tone grid", "passive status", "small size", "theme badge"],
            "key-cap" => &["single key", "combo", "non-macos", "theme key"],
            "skeleton" => &["text line", "avatar", "reduced motion", "theme shimmer"],
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
            "banner" => &[
                "保存失敗",
                "vendor 未接続",
                "添付サイズ超過",
                "成功通知",
                "details 展開",
            ],
            "toast-stack-manager" => &[
                "位置 6 種類",
                "dedup ById",
                "pause_on_hover",
                "queue 上限超過",
                "action 付き toast",
            ],
            "tabs" => &["browser tab", "switch", "overflow", "theme line"],
            "toolbar" => &[
                "overflow menu",
                "split action",
                "display mode",
                "density",
                "accelerator",
            ],
            "breadcrumb" => &["trail", "click", "overflow", "theme crumb"],
            "accordion" => &[
                "header body",
                "toggle action",
                "tree mode",
                "theme indicator",
            ],
            "split-pane" => &["horizontal", "drag action", "min clamp", "theme handle"],
            "modal" | "modal-overlay" => &["dialog", "escape close", "focus trap", "theme overlay"],
            "notification-toast" => &["toast", "dismiss", "ToastStackManager", "theme toast"],
            "popover" => &["anchor", "arrow", "slots", "focus management"],
            "hover-card" => &[
                "delayed open",
                "pointer follow",
                "focus trigger",
                "rich content",
                "actions",
            ],
            "color-picker-rgba" => &[
                "rgba panel",
                "color trigger",
                "size presets",
                "borderless",
                "floating panel",
            ],
            "code-diff" => &[
                "split left-right",
                "split top-bottom",
                "inline",
                "collapsed",
                "japanese whitespace",
            ],
            "command-palette" => &[
                "command palette",
                "search results",
                "slash launcher",
                "disabled rows",
                "virtualized results",
            ],
            "dynamic-array-editor" => &["rows", "add remove", "reorder", "theme row"],
            "selection-list" => &["items", "select", "multi", "theme list"],
            "side-menu" => &["nav tree", "select", "collapse", "theme side"],
            "status-bar" => &[
                "editor status bar",
                "chat usage bar",
                "linter summary",
                "progress segment",
                "popover segment",
            ],
            "shortcut-combo" => &[
                "macOS",
                "Windows",
                "Linux",
                "custom separator",
                "a11y label",
            ],
            "shortcut-cheatsheet" => &[
                "cheatsheet sample",
                "カテゴリ filter",
                "two column",
                "one column",
                "select combo",
            ],
            "settings-list" => &[
                "app settings",
                "chat settings",
                "lint settings",
                "dirty 表示",
                "query filter",
                "reset",
            ],
            "collapsible-panel" => &["expanded", "icon only", "floating overlay", "resized width"],
            "virtualization" => &[
                "fixed rows",
                "variable rows",
                "focused sentinel",
                "measured correction",
            ],
            "skeleton-cluster" => &[
                "list loading",
                "card loading",
                "message loading",
                "reduced motion",
            ],
            "motion" => &[
                "slide transition",
                "fade transition",
                "reduced motion",
                "disabled context",
            ],
            "window-control-button-group" => &[
                "macos leading",
                "windows trailing",
                "fullscreen hover",
                "close only",
            ],
            "startup-state-panel" => &["app boot", "session init", "update install", "error retry"],
            "attachment-chip" => &[
                "file attachment",
                "image attachment",
                "url attachment",
                "uploading",
                "error retry",
            ],
            "chip-group" => &["wrap", "overflow menu", "horizontal scroll", "reorder"],
            "diagnostics-list" => &[
                "lint result",
                "editor inline",
                "tool result",
                "empty",
                "loading",
                "bulk fix",
            ],
            "empty-state" => &[
                "explorer empty",
                "search no result",
                "diagnostics clean",
                "history empty",
                "error fallback",
            ],
            "tree-view" => &["folders", "toggle", "context", "theme tree"],
            "drag-and-drop" => &[
                "reorder list",
                "file drop",
                "tab reorder",
                "attachment drop",
                "keyboard drag",
            ],
            "closeable-tab-strip" => &[
                "tab row default",
                "overflow",
                "pinned",
                "groups",
                "dirty",
                "dragging",
            ],
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
