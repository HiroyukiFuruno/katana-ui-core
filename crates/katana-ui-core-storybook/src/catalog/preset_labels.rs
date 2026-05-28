pub(crate) struct StoryPresetLabels;

impl StoryPresetLabels {
    pub(crate) fn for_page(page: &str) -> &'static [&'static str] {
        match page {
            "panel" => &[
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
            "text" => &["role grid", "mixed script", "empty text", "theme color"],
            "icon" => &["svg grid", "accent icon", "custom SVG", "muted icon"],
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
            ],
            "text-area" => &[
                "chat composer",
                "search multiline",
                "long text",
                "resize handle",
                "auto grow",
                "vertical scroll",
                "horizontal scroll",
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
            "skeleton" => &[
                "text lines",
                "avatar circle",
                "rect shimmer",
                "line wave",
                "reduced motion",
                "tone/radius",
            ],
            "card" => &["slots", "card click", "nested controls", "theme border"],
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
                "vertical scroll",
                "horizontal scroll",
                "scrollbar toggle",
                "nested panels"
            ],
            StoryPresetLabels::for_page("panel")
        );
    }
}
