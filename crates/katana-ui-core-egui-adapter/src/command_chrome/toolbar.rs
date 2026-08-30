use super::command_chrome_artifact::CommandChromeArtifactFrame;
use super::command_chrome_dropdown::{self, dropdown_layout, show_dropdown};
use super::command_chrome_interaction::{keyboard_events, publish_button_accesskit};
use super::command_chrome_paint::{ActionPaintSource, build_toolbar_paint_plan};
use super::command_chrome_presentation::{frame_bounds, split_rects, toolbar_size};
use super::command_chrome_types::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeActionFrame,
    EguiCommandChromeAdapter, EguiCommandChromeDrawLayer, EguiCommandChromeError,
    EguiCommandChromeOutput,
};
use katana_ui_core::molecule::command_chrome::{CommandChromeToolbar, CommandChromeToolbarAction};

impl EguiCommandChromeAdapter {
    pub(super) fn show_toolbar_unpainted(
        &mut self,
        ui: &mut egui::Ui,
        toolbar: &mut CommandChromeToolbar,
        raster_style: &CommandChromeRasterStyle,
        paint_style: &CommandChromePaintStyle,
        target_class: crate::text_command_surface::accesskit_evidence::AccessKitTargetClass,
    ) -> Result<EguiCommandChromeOutput, EguiCommandChromeError> {
        let start = ui.cursor().min;
        let mut frames = Vec::new();
        let mut events = Vec::new();
        let mut toolbar_has_focus = false;
        let mut pointer_activation_consumed = false;
        let mut paint_sources = Vec::new();
        let mut primary_focus_targets = Vec::new();
        let mut prepared_dropdowns = Vec::new();
        let actions = toolbar.actions().to_vec();
        let display_mode = toolbar.display_mode_model();
        let rendered = actions
            .iter()
            .map(|action| self.render_action(ui, action, display_mode, raster_style))
            .collect::<Result<Vec<_>, _>>()?;
        let toolbar_size = toolbar_size(ui, &rendered);
        let render_result = ui.allocate_ui_with_layout(
            toolbar_size,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| -> Result<(), EguiCommandChromeError> {
                for (action, rendered) in actions.iter().zip(rendered) {
                    let size =
                        egui::vec2(rendered.bounds.width as f32, rendered.bounds.height as f32);
                    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
                    let bounds = super::ui_rect(rect);
                    let (primary_rect, secondary_rect) = split_rects(rect, action);
                    let primary_response = ui.interact(
                        primary_rect,
                        response.id.with("kuc-command-chrome-primary"),
                        egui::Sense::click(),
                    );
                    let secondary_response = secondary_rect.map(|secondary| {
                        ui.interact(
                            secondary,
                            response.id.with("kuc-command-chrome-secondary"),
                            egui::Sense::click(),
                        )
                    });
                    toolbar_has_focus |= primary_response.has_focus()
                        || secondary_response
                            .as_ref()
                            .is_some_and(egui::Response::has_focus);
                    let primary_bounds = super::ui_rect(primary_rect);
                    let primary_target_class = if action.dropdown_model().is_some_and(|dropdown| {
                        matches!(
                            dropdown.trigger_model(),
                            katana_ui_core::molecule::command_chrome::CommandChromeDropdownTrigger::Primary
                        )
                    }) {
                        crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DropdownTrigger
                    } else {
                        target_class
                    };
                    publish_button_accesskit(
                        ui,
                        primary_response.id,
                        action,
                        primary_bounds,
                        primary_target_class,
                    );
                    if let (Some(secondary_response), Some(secondary_rect)) =
                        (secondary_response.as_ref(), secondary_rect)
                    {
                        publish_button_accesskit(
                            ui,
                            secondary_response.id,
                            action,
                            super::ui_rect(secondary_rect),
                            crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DropdownTrigger,
                        );
                    }
                    if let Some(dropdown) = action.dropdown_model() {
                        let trigger_bounds = if matches!(
                            dropdown.trigger_model(),
                            katana_ui_core::molecule::command_chrome::CommandChromeDropdownTrigger::SplitSecondary
                        ) {
                            secondary_rect.map(super::ui_rect).unwrap_or(primary_bounds)
                        } else {
                            primary_bounds
                        };
                        let (layout, rendered_dropdown) =
                            dropdown_layout(self, ui, trigger_bounds, dropdown, raster_style)?;
                        prepared_dropdowns
                            .push((action.id().as_str().to_owned(), rendered_dropdown));
                        events.extend(toolbar.apply_action(
                            CommandChromeToolbarAction::update_dropdown_layout(
                                action.id().clone(),
                                layout,
                            ),
                        ));
                    }
                    let dropdown_keyboard_activation = toolbar.open_dropdown_model().is_some()
                        && command_chrome_dropdown::keyboard_dropdown_activation(ui);
                    if primary_response.clicked()
                        && !dropdown_keyboard_activation
                        && !action.disabled_model()
                    {
                        pointer_activation_consumed = true;
                        primary_response.request_focus();
                        events.extend(toolbar.apply_action(CommandChromeToolbarAction::activate(
                            action.id().clone(),
                        )));
                    }
                    primary_focus_targets
                        .push((action.id().as_str().to_owned(), primary_response.clone()));
                    if secondary_response
                        .as_ref()
                        .is_some_and(egui::Response::clicked)
                        && !action.disabled_model()
                    {
                        events.extend(toolbar.apply_action(
                            CommandChromeToolbarAction::open_split_dropdown(action.id().clone()),
                        ));
                    }
                    frames.push(EguiCommandChromeActionFrame {
                        action_id: action.id().as_str().to_string(),
                        bounds,
                        secondary_trigger_bounds: secondary_rect.map(super::ui_rect),
                        primary_dropdown_trigger: action.dropdown_model().is_some_and(|dropdown| matches!(dropdown.trigger_model(), katana_ui_core::molecule::command_chrome::CommandChromeDropdownTrigger::Primary)),
                        icon_raster_identity: rendered.icon_identity.clone(),
                        label_raster_identity: rendered.label_identity.clone(),
                        disabled: action.disabled_model(),
                    });
                    paint_sources.push(ActionPaintSource::new(
                        primary_bounds,
                        secondary_rect.map(super::ui_rect),
                        primary_response.hovered(),
                        secondary_response
                            .as_ref()
                            .is_some_and(egui::Response::hovered),
                        action.disabled_model(),
                        rendered,
                    ));
                }
                Ok(())
            },
        );
        render_result.inner?;
        let bounds = frame_bounds(start, &frames);
        let dropdown = show_dropdown(
            self,
            ui,
            toolbar,
            bounds,
            &prepared_dropdowns,
            paint_style,
            &mut events,
        );
        if toolbar_has_focus || toolbar.open_dropdown_model().is_some() {
            events.extend(keyboard_events(ui, toolbar, pointer_activation_consumed));
        }
        if let Some(action_id) = super::dropdown_focus_return_target(&events)
            && let Some((_, response)) = primary_focus_targets
                .iter()
                .find(|(candidate, _)| candidate == action_id)
        {
            response.request_focus();
        }
        let record = super::EguiCommandChromeFrameRecord {
            bounds,
            actions: frames,
            dropdown: dropdown.as_ref().map(|value| value.record.clone()),
            hidden_item_ids: if dropdown.is_none() {
                toolbar
                    .actions()
                    .iter()
                    .filter_map(|action| action.dropdown_model())
                    .flat_map(|dropdown| dropdown.items().iter())
                    .map(|item| item.id().as_str().to_owned())
                    .collect()
            } else {
                Vec::new()
            },
            focused_action_id: toolbar
                .focused_action_id_model()
                .map(|id| id.as_str().to_string()),
            layers: vec![
                EguiCommandChromeDrawLayer::ActionFill,
                EguiCommandChromeDrawLayer::IconTexture,
                EguiCommandChromeDrawLayer::TextTexture,
                EguiCommandChromeDrawLayer::FocusRing,
            ],
        };
        let paint_plan =
            build_toolbar_paint_plan(&record, &paint_sources, dropdown.as_ref(), paint_style);
        let artifact = CommandChromeArtifactFrame::new(record.clone(), paint_plan, events.clone())?;
        Ok(EguiCommandChromeOutput {
            record,
            events,
            artifact,
        })
    }
}
