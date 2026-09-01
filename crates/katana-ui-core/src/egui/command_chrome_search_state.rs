use super::command_chrome_types::{EguiCommandChromeSearchStyle, SearchSurfaceState};
use crate::atom::{TextArea, TextAreaNewlineKey, TextAreaSubmitKey};
use crate::molecule::command_chrome::{CommandChromeSearchStrip, CommandChromeText};
use crate::molecule::structured::ReplaceMode;
use crate::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

#[cfg(test)]
#[path = "command_chrome_search_state_tests.rs"]
mod tests;

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
