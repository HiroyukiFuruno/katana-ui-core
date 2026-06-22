pub(crate) struct StoryPresetLabels;

impl StoryPresetLabels {
    pub(crate) fn for_page(page: &str) -> &'static [&'static str] {
        match page {
            "panel" => &[
                "active panel",
                "vertical scroll",
                "horizontal scroll",
                "scrollbar toggle",
                "nested panels",
            ],
            "theme-tokens" => &[
                "dark palette",
                "light palette",
                "contrast surface",
                "accent override",
            ],
            "text" => &[
                "role grid",
                "content state",
                "mixed script",
                "theme color",
                "color token",
                "line metrics",
                "vertical center",
                "rich spans",
                "wrap mode",
            ],
            "icon" => &[
                "content value",
                "visual role",
                "a11y label",
                "theme color",
                "custom SVG",
                "svg props",
                "view box",
                "path summary",
                "paint policy",
                "icon role",
                "color token",
                "theme token",
            ],
            "chip" => &[
                "filter label",
                "leading filter icon",
                "trailing dismiss icon",
                "filled chip variant",
                "danger chip tone",
                "large chip size",
                "interactive chip",
                "selected chip",
                "disabled chip",
                "dismissible chip",
                "accessible chip label",
                "chip focus ring",
            ],
            "checkbox" => &["unchecked", "checked", "disabled", "focus ring"],
            "radio" => &["unselected", "selected", "group", "focus ring"],
            "divider" => &["horizontal", "vertical", "inset", "theme line"],
            "spacer" => &["fixed gap", "flex gap", "dense", "theme gap"],
            "loading-dots" => &[
                "running dots",
                "phase tick",
                "reduced motion",
                "theme label",
                "speed",
                "dot count",
                "tone",
                "size",
            ],
            "spinner" => &[
                "running spinner",
                "phase tick",
                "paused",
                "theme label",
                "speed",
                "segment count",
                "tone",
                "size",
            ],
            "progress-bar" => &[
                "determinate",
                "change",
                "empty",
                "theme track",
                "speed",
                "segment count",
                "reduced motion",
                "tone",
                "size",
            ],
            "button" | "text-button" | "svg-button" | "icon-text-button" => &[
                "modern",
                "classic",
                "basic",
                "dense",
                "visible",
                "disabled",
                "focusable",
                "width",
                "height",
                "border",
                "label",
                "tab index",
                "z index",
                "command",
                "keyboard",
                "icon position",
                "layout preset",
                "svg source",
                "aria label",
            ],
            "toggle" => &["off", "on action", "disabled", "theme switch"],
            "slide-control" => &["track", "drag", "step", "theme knob"],
            "segmented-toggle" => &[
                "segments",
                "select action",
                "disabled segment",
                "theme marker",
            ],
            "select-box" => &["items", "open", "selected index", "placeholder", "disabled"],
            "combo-box" => &[
                "items",
                "open",
                "selected index",
                "value",
                "placeholder",
                "disabled",
                "readonly",
                "input value",
                "filter result",
                "free input",
                "keyboard nav",
                "placement",
                "highlight",
                "long list",
                "outside dismiss",
                "framed",
                "trigger summary",
                "select callback",
                "invalid",
            ],
            "menu-button" => &["trigger", "open menu", "disabled", "theme menu"],
            "color-swatch" => &["palette", "select color", "disabled", "theme ring"],
            "text-input" => &[
                "value",
                "ime event",
                "readonly",
                "placeholder",
                "icon slot",
                "search icon",
                "icon buttons",
                "invalid",
                "theme slot",
                "disabled",
                "font role",
                "trailing slot",
                "clear action",
                "submit enter",
                "emoji",
            ],
            "text-area" => &[
                "chat composer",
                "newline key",
                "wrap policy",
                "resize handle",
                "auto grow",
                "vertical scroll",
                "horizontal scroll",
                "tab behavior",
                "vertical scrollbar",
                "horizontal scrollbar",
                "leading svg",
                "icon callbacks",
                "clear action",
                "value",
                "placeholder",
                "font role",
                "disabled",
                "readonly",
                "invalid",
                "min rows",
                "max rows",
                "ime",
                "leading slot",
                "trailing slot",
            ],
            "search-box" => &["search icon", "submit action", "regex case", "theme clear"],
            "search-control-strip" => &[
                "query",
                "match case",
                "whole word",
                "regex",
                "replace mode",
                "result count",
                "active index",
            ],
            "form-field" => &["label", "invalid", "helper", "required", "theme field"],
            "tooltip" => &["anchor", "hover open", "placement edge", "theme overlay"],
            "badge" => &[
                "tone grid",
                "passive status",
                "small size",
                "theme badge",
                "leading icon",
                "filled variant",
            ],
            "key-cap" => &["single key", "combo", "non-macos", "theme key"],
            "skeleton" => &[
                "shape variant",
                "text lines",
                "last line ratio",
                "line thickness",
                "size fill",
                "animation wave",
                "tone accent",
                "radius round",
                "reduced motion",
                "a11y label",
                "aspect ratio",
            ],
            "card" => &[
                "slots",
                "card click",
                "nested controls",
                "theme border",
                "label",
                "header",
                "footer",
                "padding",
            ],
            "list" => &["rows", "selection", "empty", "theme list", "Virtualization"],
            "menu" => &["menu items", "shortcut", "disabled", "theme menu"],
            page => super::preset_label_extra::for_page(page).unwrap_or(&[
                "overview",
                "operate",
                "edge case",
                "theme view",
            ]),
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

    #[test]
    fn theme_tokens_presets_describe_token_surfaces() {
        assert_eq!(
            &[
                "dark palette",
                "light palette",
                "contrast surface",
                "accent override"
            ],
            StoryPresetLabels::for_page("theme-tokens")
        );
    }

    #[test]
    fn panel_presets_describe_actual_panel_controls() {
        assert_eq!(
            &[
                "active panel",
                "vertical scroll",
                "horizontal scroll",
                "scrollbar toggle",
                "nested panels"
            ],
            StoryPresetLabels::for_page("panel")
        );
    }
}
