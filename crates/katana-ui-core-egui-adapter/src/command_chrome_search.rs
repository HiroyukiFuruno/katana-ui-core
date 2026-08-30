use super::command_chrome_artifact::EguiCommandChromeSearchArtifactFrame;
use super::command_chrome_paint::paint_command_chrome;
use super::command_chrome_search_controls::show_controls;
use super::command_chrome_search_interaction::{
    apply_text_events, bounds, query_input_policy, query_key_events,
};
use super::command_chrome_search_paint::build_search_paint_plan;
use super::command_chrome_types::{
    EguiCommandChromeAdapter, EguiCommandChromeError, EguiCommandChromeSearchFrameRecord,
    EguiCommandChromeSearchOutput, EguiCommandChromeSearchStyle, SearchSurfaceState,
};
use crate::text_surface::{EguiTextSurfaceInputPolicy, EguiTextSurfaceOutput};
use katana_ui_core::molecule::command_chrome::CommandChromeSearchStrip;
use katana_ui_core::molecule::structured::ReplaceMode;
use katana_ui_core::text_surface::TextSurface;

impl EguiCommandChromeAdapter {
    pub fn show_search_strip(
        &mut self,
        ui: &mut egui::Ui,
        strip: &mut CommandChromeSearchStrip,
        raster_style: &super::command_chrome_types::CommandChromeRasterStyle,
        paint_style: &super::command_chrome_types::CommandChromePaintStyle,
        search_style: &EguiCommandChromeSearchStyle,
    ) -> Result<EguiCommandChromeSearchOutput, EguiCommandChromeError> {
        let mut surfaces = self
            .search_surfaces
            .take()
            .unwrap_or_else(|| SearchSurfaceState::new(strip, search_style));
        surfaces.synchronize(strip, search_style);
        let result = self.show_search_strip_with_surfaces(
            ui,
            strip,
            raster_style,
            paint_style,
            search_style,
            &mut surfaces,
        );
        self.search_surfaces = Some(surfaces);
        result
    }

    fn show_search_strip_with_surfaces(
        &mut self,
        ui: &mut egui::Ui,
        strip: &mut CommandChromeSearchStrip,
        raster_style: &super::command_chrome_types::CommandChromeRasterStyle,
        paint_style: &super::command_chrome_types::CommandChromePaintStyle,
        search_style: &EguiCommandChromeSearchStyle,
        surfaces: &mut SearchSurfaceState,
    ) -> Result<EguiCommandChromeSearchOutput, EguiCommandChromeError> {
        let start = ui.cursor().min;
        let replace_visible = strip.replace_mode_model() != ReplaceMode::Hidden;
        let query_was_focused = surfaces.query.state().text_area.focused;
        let (query, replace, controls, control_events, control_paint_sources) = ui
            .scope(|ui| {
                ui.spacing_mut().item_spacing.x = search_style.gap_px as f32;
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| -> Result<_, EguiCommandChromeError> {
                        let query = show_input(
                            ui,
                            &mut self.text_surface_adapter,
                            &mut surfaces.query,
                            search_style,
                            &query_input_policy(),
                        )?;
                        let replace = replace_visible
                            .then(|| {
                                show_input(
                                    ui,
                                    &mut self.text_surface_adapter,
                                    &mut surfaces.replace,
                                    search_style,
                                    &EguiTextSurfaceInputPolicy::default().without_context_target(),
                                )
                            })
                            .transpose()?;
                        let (controls, events, control_paint_sources) =
                            show_controls(self, ui, strip, raster_style, search_style)?;
                        Ok((query, replace, controls, events, control_paint_sources))
                    },
                )
                .inner
            })
            .inner?;
        let mut text_events = query.events.clone();
        let mut events = apply_text_events(strip, &query.events, true);
        if let Some(replace) = &replace {
            text_events.extend(replace.events.clone());
            events.extend(apply_text_events(strip, &replace.events, false));
        }
        events.extend(control_events);
        events.extend(query_key_events(
            ui,
            strip,
            query_was_focused || surfaces.query.state().text_area.focused,
        ));
        let bounds = bounds(
            start,
            &query.record.frame.content_bounds,
            replace.as_ref(),
            &controls,
        );
        let focused_target = surfaces
            .query
            .state()
            .text_area
            .focused
            .then(|| {
                surfaces
                    .query
                    .state()
                    .text_area
                    .state_id
                    .as_str()
                    .to_string()
            })
            .or_else(|| {
                surfaces.replace.state().text_area.focused.then(|| {
                    surfaces
                        .replace
                        .state()
                        .text_area
                        .state_id
                        .as_str()
                        .to_string()
                })
            });
        let record = EguiCommandChromeSearchFrameRecord {
            bounds,
            query: query.record.clone(),
            replace: replace.as_ref().map(|output| output.record.clone()),
            controls,
            focused_target,
            layers: vec![
                super::command_chrome_types::EguiCommandChromeDrawLayer::PanelFill,
                super::command_chrome_types::EguiCommandChromeDrawLayer::ActionFill,
                super::command_chrome_types::EguiCommandChromeDrawLayer::IconTexture,
                super::command_chrome_types::EguiCommandChromeDrawLayer::TextTexture,
                super::command_chrome_types::EguiCommandChromeDrawLayer::FocusRing,
            ],
        };
        let paint_plan = build_search_paint_plan(
            &record,
            &query.artifact.paint_plan,
            replace.as_ref().map(|output| &output.artifact.paint_plan),
            &control_paint_sources,
            paint_style,
        );
        let artifact = EguiCommandChromeSearchArtifactFrame::new(
            record.clone(),
            paint_plan,
            events.clone(),
            text_events.clone(),
        )?;
        paint_command_chrome(ui, &mut self.textures, &artifact.paint_plan);
        Ok(EguiCommandChromeSearchOutput {
            record,
            events,
            text_events,
            artifact,
        })
    }
}

fn show_input(
    ui: &mut egui::Ui,
    adapter: &mut crate::text_surface::EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    style: &EguiCommandChromeSearchStyle,
    policy: &EguiTextSurfaceInputPolicy,
) -> Result<EguiTextSurfaceOutput, EguiCommandChromeError> {
    ui.allocate_ui_with_layout(
        egui::vec2(style.input_width_px as f32, style.input_height_px as f32),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            adapter.show_with_input_policy_unpainted(
                ui,
                surface,
                &style.input_raster,
                &style.input_paint,
                policy,
            )
        },
    )
    .inner
    .map_err(Into::into)
}
