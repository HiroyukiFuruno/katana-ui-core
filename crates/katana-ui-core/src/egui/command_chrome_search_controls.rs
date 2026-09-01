use super::command_chrome_interaction::publish_labeled_button_accesskit;
use super::command_chrome_search_paint::{SearchControlPaintSource, SearchControlPaintState};
use super::command_chrome_types::{
    CommandChromeRasterStyle, EguiCommandChromeAdapter, EguiCommandChromeError,
    EguiCommandChromeSearchControlFrame, EguiCommandChromeSearchStyle, RenderedRaster,
};
use crate::molecule::command_chrome::{
    CommandChromeSearchAction, CommandChromeSearchEvent, CommandChromeSearchStrip,
    CommandChromeText, SearchControlIconSlot,
};
use crate::molecule::structured::{
    ReplaceMode, SearchControlStripAction, SearchNavigationDirection, SearchOptionKind,
    SearchReplaceScope,
};
use crate::render_model::{UiIconProps, UiRect};

type SearchControlsOutput = (
    Vec<EguiCommandChromeSearchControlFrame>,
    Vec<CommandChromeSearchEvent>,
    Vec<SearchControlPaintSource>,
);

pub(super) fn show_controls(
    adapter: &mut EguiCommandChromeAdapter,
    ui: &mut egui::Ui,
    strip: &mut CommandChromeSearchStrip,
    raster_style: &CommandChromeRasterStyle,
    search_style: &EguiCommandChromeSearchStyle,
) -> Result<SearchControlsOutput, EguiCommandChromeError> {
    let specs = specs(strip);
    let state_id = strip.state_id_model().as_str().to_string();
    let mut frames = Vec::new();
    let mut events = Vec::new();
    let mut paint_sources = Vec::new();
    let mut failure = None;
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.x = search_style.gap_px as f32;
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            for spec in specs {
                if failure.is_some() {
                    break;
                }
                match show_control(adapter, ui, &state_id, &spec, raster_style, search_style) {
                    Ok((frame, clicked, paint_source)) => {
                        if clicked && let Some(action) = spec.action {
                            events.extend(strip.apply_action(action));
                        }
                        frames.push(frame);
                        paint_sources.push(paint_source);
                    }
                    Err(error) => failure = Some(error),
                }
            }
        });
    });
    failure.map_or_else(|| Ok((frames, events, paint_sources)), Err)
}

#[derive(Clone)]
struct ControlSpec {
    id: &'static str,
    text: CommandChromeText,
    icon: Option<UiIconProps>,
    enabled: bool,
    active: bool,
    action: Option<CommandChromeSearchAction>,
}

fn specs(strip: &CommandChromeSearchStrip) -> Vec<ControlSpec> {
    let strings = strip.strings_model().clone();
    let icons = strip.icons_model().clone();
    let options = *strip.options_model();
    let navigation = strip.capabilities_model().navigation.is_available()
        && strip.result_count_model().unwrap_or_default() > 0;
    let replace_visible = strip.replace_mode_model() != ReplaceMode::Hidden;
    let replace = strip.replace_mode_model() == ReplaceMode::Visible
        && strip.capabilities_model().replace.is_available();
    let mut values = vec![
        ControlSpec::action(
            "match-case",
            strings.match_case,
            icons.icon_for(SearchControlIconSlot::MatchCase).cloned(),
            true,
            options.match_case,
            SearchControlStripAction::ToggleSearchOption(SearchOptionKind::MatchCase),
        ),
        ControlSpec::action(
            "whole-word",
            strings.whole_word,
            icons.icon_for(SearchControlIconSlot::WholeWord).cloned(),
            true,
            options.whole_word,
            SearchControlStripAction::ToggleSearchOption(SearchOptionKind::WholeWord),
        ),
        ControlSpec::action(
            "use-regex",
            strings.use_regex,
            icons.icon_for(SearchControlIconSlot::UseRegex).cloned(),
            strip.capabilities_model().regex.is_available(),
            options.use_regex,
            SearchControlStripAction::ToggleSearchOption(SearchOptionKind::UseRegex),
        ),
        ControlSpec::action(
            "previous",
            strings.previous,
            icons.icon_for(SearchControlIconSlot::Previous).cloned(),
            navigation,
            false,
            SearchControlStripAction::Navigate(SearchNavigationDirection::Previous),
        ),
        ControlSpec::action(
            "next",
            strings.next,
            icons.icon_for(SearchControlIconSlot::Next).cloned(),
            navigation,
            false,
            SearchControlStripAction::Navigate(SearchNavigationDirection::Next),
        ),
        ControlSpec::summary("result-summary", strip.result_summary_model()),
    ];
    if replace_visible {
        values.extend([
            ControlSpec::action(
                "replace-one",
                strings.replace_one,
                icons.icon_for(SearchControlIconSlot::ReplaceOne).cloned(),
                replace,
                false,
                SearchControlStripAction::Replace(SearchReplaceScope::One),
            ),
            ControlSpec::action(
                "replace-all",
                strings.replace_all,
                icons.icon_for(SearchControlIconSlot::ReplaceAll).cloned(),
                replace,
                false,
                SearchControlStripAction::Replace(SearchReplaceScope::All),
            ),
        ]);
    }
    values.push(ControlSpec {
        id: "close",
        text: strings.close,
        icon: icons.icon_for(SearchControlIconSlot::Close).cloned(),
        enabled: strip.capabilities_model().close.is_available(),
        active: false,
        action: Some(CommandChromeSearchAction::RequestClose),
    });
    values
}

impl ControlSpec {
    fn action(
        id: &'static str,
        text: CommandChromeText,
        icon: Option<UiIconProps>,
        enabled: bool,
        active: bool,
        action: SearchControlStripAction,
    ) -> Self {
        Self {
            id,
            text,
            icon,
            enabled,
            active,
            action: Some(CommandChromeSearchAction::Strip { action }),
        }
    }

    fn summary(id: &'static str, text: String) -> Self {
        Self {
            id,
            text: CommandChromeText::new(text.clone(), String::new(), text),
            icon: None,
            enabled: false,
            active: false,
            action: None,
        }
    }
}

fn show_control(
    adapter: &mut EguiCommandChromeAdapter,
    ui: &mut egui::Ui,
    state_id: &str,
    spec: &ControlSpec,
    raster_style: &CommandChromeRasterStyle,
    search_style: &EguiCommandChromeSearchStyle,
) -> Result<
    (
        EguiCommandChromeSearchControlFrame,
        bool,
        SearchControlPaintSource,
    ),
    EguiCommandChromeError,
> {
    let raster = raster(adapter, ui, spec, raster_style)?;
    let size = egui::vec2(
        (raster.width + search_style.control_padding_px.saturating_mul(2)) as f32,
        (raster.height + search_style.control_padding_px.saturating_mul(2)) as f32,
    );
    let control_id = format!("{state_id}:{}", spec.id);
    let (rect, response) = ui
        .push_id(&control_id, |ui| {
            ui.allocate_exact_size(size, egui::Sense::click())
        })
        .inner;
    let bounds = ui_rect(rect);
    if spec.action.is_some() {
        publish_labeled_button_accesskit(
            ui,
            response.id,
            &spec.text.accessibility_label,
            !spec.enabled,
            bounds,
            control_id.as_str(),
            crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::SearchControl,
        );
    } else {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Label);
            node.set_value(spec.text.accessibility_label.as_str());
        });
    }
    Ok((
        EguiCommandChromeSearchControlFrame {
            control_id,
            bounds,
            raster_identity: raster.identity.clone(),
            disabled: !spec.enabled,
            active: spec.active,
        },
        response.clicked() && spec.enabled && spec.action.is_some(),
        SearchControlPaintSource::new(
            bounds,
            raster,
            SearchControlPaintState {
                icon: spec.icon.is_some(),
                action: spec.action.is_some(),
                disabled: !spec.enabled,
                active: spec.active,
                active_rgba: search_style.active_control_rgba,
                hovered: response.hovered(),
                padding_px: search_style.control_padding_px,
            },
        ),
    ))
}

fn raster(
    adapter: &mut EguiCommandChromeAdapter,
    ui: &egui::Ui,
    spec: &ControlSpec,
    style: &CommandChromeRasterStyle,
) -> Result<RenderedRaster, EguiCommandChromeError> {
    match spec.icon.as_ref() {
        Some(icon) => adapter.raster_icon(icon, style, ui.ctx().pixels_per_point()),
        None => adapter.raster_label(&spec.text.visible, style, ui.ctx().pixels_per_point()),
    }
}

fn ui_rect(rect: egui::Rect) -> UiRect {
    UiRect::new(
        rect.min.x.round() as i32,
        rect.min.y.round() as i32,
        rect.width().round().max(0.0) as u32,
        rect.height().round().max(0.0) as u32,
    )
}
