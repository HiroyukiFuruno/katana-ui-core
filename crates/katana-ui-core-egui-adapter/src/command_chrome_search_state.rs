use super::command_chrome_types::{EguiCommandChromeSearchStyle, SearchSurfaceState};
use katana_ui_core::atom::{TextArea, TextAreaNewlineKey, TextAreaSubmitKey};
use katana_ui_core::molecule::command_chrome::{CommandChromeSearchStrip, CommandChromeText};
use katana_ui_core::molecule::structured::ReplaceMode;
use katana_ui_core::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

impl SearchSurfaceState {
    pub(super) fn new(
        strip: &CommandChromeSearchStrip,
        style: &EguiCommandChromeSearchStyle,
    ) -> Self {
        let replace_disabled = replace_disabled(strip);
        Self {
            strip_state_id: strip.state_id_model().as_str().to_string(),
            query: surface(
                strip,
                "query",
                strip.strings_model().query.clone(),
                strip.query_model(),
                style,
                None,
            ),
            replace: surface(
                strip,
                "replace",
                strip.strings_model().replace.clone(),
                strip.replace_value_model(),
                style,
                replace_disabled_reason(strip, replace_disabled),
            ),
            query_presentation: strip.strings_model().query.clone(),
            replace_presentation: strip.strings_model().replace.clone(),
            input_width_px: style.input_width_px,
            input_height_px: style.input_height_px,
            replace_disabled,
        }
    }

    pub(super) fn synchronize(
        &mut self,
        strip: &CommandChromeSearchStrip,
        style: &EguiCommandChromeSearchStyle,
    ) {
        if self.strip_state_id != strip.state_id_model().as_str() {
            *self = Self::new(strip, style);
            return;
        }
        if self.query_presentation != strip.strings_model().query || self.input_size_changed(style)
        {
            self.query = surface(
                strip,
                "query",
                strip.strings_model().query.clone(),
                strip.query_model(),
                style,
                None,
            );
            self.query_presentation = strip.strings_model().query.clone();
        } else {
            self.query.synchronize_value(strip.query_model());
        }
        let disabled = replace_disabled(strip);
        if self.replace_presentation != strip.strings_model().replace
            || self.replace_disabled != disabled
            || self.input_size_changed(style)
        {
            self.replace = surface(
                strip,
                "replace",
                strip.strings_model().replace.clone(),
                strip.replace_value_model(),
                style,
                replace_disabled_reason(strip, disabled),
            );
            self.replace_presentation = strip.strings_model().replace.clone();
            self.replace_disabled = disabled;
        } else {
            self.replace.synchronize_value(strip.replace_value_model());
        }
        self.input_width_px = style.input_width_px;
        self.input_height_px = style.input_height_px;
    }

    fn input_size_changed(&self, style: &EguiCommandChromeSearchStyle) -> bool {
        self.input_width_px != style.input_width_px || self.input_height_px != style.input_height_px
    }
}

fn surface(
    strip: &CommandChromeSearchStrip,
    slot: &str,
    text: CommandChromeText,
    value: &str,
    style: &EguiCommandChromeSearchStyle,
    disabled_reason: Option<&str>,
) -> TextSurface {
    let text_area = TextArea::new(text.visible.clone())
        .stable_state_id(format!("{}:{slot}", strip.state_id_model().as_str()))
        .value(value)
        .placeholder(text.visible.clone())
        .min_rows(1)
        .max_rows(1)
        .auto_grow(false)
        .submit_key(TextAreaSubmitKey::Disabled)
        .newline_key(TextAreaNewlineKey::Disabled)
        .ime_enabled(true);
    let text_area = if disabled_reason.is_some() {
        text_area.disabled(true)
    } else {
        text_area
    };
    let props = TextSurfaceProps::new(
        text_area,
        Vec::new(),
        TextSurfaceViewport::new(0, 0, style.input_width_px, style.input_height_px),
    )
    .accessibility_label(text.accessibility_label);
    let props = disabled_reason.map_or(props.clone(), |reason| props.disabled_reason(reason));
    TextSurface::new(props)
}

fn replace_disabled(strip: &CommandChromeSearchStrip) -> bool {
    strip.replace_mode_model() == ReplaceMode::Disabled
        || !strip.capabilities_model().replace.is_available()
}

fn replace_disabled_reason(strip: &CommandChromeSearchStrip, disabled: bool) -> Option<&str> {
    disabled
        .then(|| strip.capabilities_model().replace.disabled_reason())
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_surface::{TextSurfacePaintStyle, TextSurfaceRasterStyle};
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeCapability, SearchControlCapabilities, SearchControlStrings,
        SearchResultSummaryTemplate,
    };
    use katana_ui_core::molecule::structured::SearchControlStrip;
    use katana_ui_core::theme::{FontFamily, FontToken};

    fn text(value: &str) -> CommandChromeText {
        CommandChromeText::new(value, value, value)
    }

    fn strings(query: &str, replace: &str) -> SearchControlStrings {
        SearchControlStrings {
            strip: text("search"),
            query: text(query),
            replace: text(replace),
            match_case: text("case"),
            whole_word: text("word"),
            use_regex: text("regex"),
            previous: text("previous"),
            next: text("next"),
            replace_one: text("replace one"),
            replace_all: text("replace all"),
            close: text("close"),
            result_summary: SearchResultSummaryTemplate {
                empty: String::new(),
                zero_results: "0".into(),
                single_result: "1".into(),
                indexed_result: "{active}/{count}".into(),
                count_results: "{count}".into(),
            },
        }
    }

    fn strip(
        state_id: &str,
        query: &str,
        replace: &str,
        query_label: &str,
        replace_label: &str,
        replace_mode: ReplaceMode,
        replace_capability: CommandChromeCapability,
    ) -> CommandChromeSearchStrip {
        CommandChromeSearchStrip::new(
            SearchControlStrip::new("search")
                .stable_state_id(state_id)
                .query(query)
                .replace_mode(replace_mode)
                .replace_value(replace),
            strings(query_label, replace_label),
        )
        .capabilities(SearchControlCapabilities {
            replace: replace_capability,
            ..SearchControlCapabilities::all_available()
        })
    }

    fn style(width: u32, height: u32) -> EguiCommandChromeSearchStyle {
        EguiCommandChromeSearchStyle {
            input_raster: TextSurfaceRasterStyle::new(
                FontToken {
                    name: "search-test".into(),
                    family: FontFamily::Monospace,
                    size: 14.0,
                    weight: 400,
                },
                [255; 4],
                18.0,
            ),
            input_paint: TextSurfacePaintStyle {
                background_rgba: [0; 4],
                gutter_background_rgba: [0; 4],
                gutter_paints: Vec::new(),
                selection_rgba: [0; 4],
                preedit_rgba: [0; 4],
                caret_rgba: [0; 4],
                annotation_paints: Vec::new(),
            },
            input_width_px: width,
            input_height_px: height,
            gap_px: 0,
            control_padding_px: 0,
            active_control_rgba: [0; 4],
        }
    }

    #[test]
    fn synchronize_covers_identity_value_presentation_size_and_disabled_transitions() {
        let available = CommandChromeCapability::available();
        let initial = strip(
            "search-a",
            "one",
            "first",
            "query",
            "replace",
            ReplaceMode::Visible,
            available.clone(),
        );
        let mut state = SearchSurfaceState::new(&initial, &style(100, 20));

        let value_only = strip(
            "search-a",
            "two",
            "second",
            "query",
            "replace",
            ReplaceMode::Visible,
            available.clone(),
        );
        state.synchronize(&value_only, &style(100, 20));
        assert_eq!(state.query.state().text_area.value, "two");
        assert_eq!(state.replace.state().text_area.value, "second");

        let presented = strip(
            "search-a",
            "three",
            "third",
            "find",
            "substitute",
            ReplaceMode::Visible,
            CommandChromeCapability::unavailable("read only"),
        );
        state.synchronize(&presented, &style(120, 24));
        assert_eq!(state.query_presentation.visible, "find");
        assert_eq!(state.replace_presentation.visible, "substitute");
        assert!(state.replace_disabled);
        assert_eq!(state.input_width_px, 120);
        assert_eq!(state.input_height_px, 24);

        let replacement = strip(
            "search-b",
            "four",
            "fourth",
            "query-b",
            "replace-b",
            ReplaceMode::Disabled,
            available,
        );
        state.synchronize(&replacement, &style(140, 28));
        assert_eq!(state.strip_state_id, "search-b");
        assert!(state.replace_disabled);
        assert_eq!(state.query.state().text_area.value, "four");
    }
}
