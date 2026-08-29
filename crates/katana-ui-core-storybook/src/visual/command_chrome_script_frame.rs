use super::super::command_chrome_fixture::{FRAME_HEIGHT, FRAME_WIDTH};
use super::super::command_chrome_surface::show_command_chrome;
use super::{CommandChromeScriptError, CommandChromeScriptFrame};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchStrip, CommandChromeToolbar, FloatingCommandToolbar,
};
use katana_ui_core::render_model::UiRect;
use katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeAdapter;

const CENTER_DIVISOR: f32 = 2.0;

pub(super) fn push(
    frames: &mut Vec<CommandChromeScriptFrame>,
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    floating: &mut FloatingCommandToolbar,
    search: &mut CommandChromeSearchStrip,
    events: Vec<egui::Event>,
) -> Result<(), CommandChromeScriptError> {
    frames.push(run_frame(
        context, adapter, toolbar, floating, search, events,
    )?);
    Ok(())
}

pub(super) fn push_events(
    context: &egui::Context,
    frames: &mut Vec<CommandChromeScriptFrame>,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    floating: &mut FloatingCommandToolbar,
    search: &mut CommandChromeSearchStrip,
    events: Vec<egui::Event>,
) -> Result<(), CommandChromeScriptError> {
    push(frames, context, adapter, toolbar, floating, search, events)
}

pub(super) fn toolbar_action(
    frames: &[CommandChromeScriptFrame],
    id: &str,
) -> Result<UiRect, CommandChromeScriptError> {
    last(frames)?
        .toolbar
        .record
        .actions
        .iter()
        .find(|action| action.action_id == id)
        .map(|action| action.bounds)
        .ok_or_else(|| {
            CommandChromeScriptError::message(format!(
                "toolbar action `{id}` did not expose bounds"
            ))
        })
}

pub(super) fn toolbar_secondary(
    frames: &[CommandChromeScriptFrame],
    id: &str,
) -> Result<UiRect, CommandChromeScriptError> {
    last(frames)?
        .toolbar
        .record
        .actions
        .iter()
        .find(|action| action.action_id == id)
        .and_then(|action| action.secondary_trigger_bounds)
        .ok_or_else(|| {
            CommandChromeScriptError::message(format!(
                "split action `{id}` did not expose secondary bounds"
            ))
        })
}

pub(super) fn floating_action(
    frames: &[CommandChromeScriptFrame],
) -> Result<UiRect, CommandChromeScriptError> {
    last(frames)?
        .floating
        .record
        .as_ref()
        .and_then(|record| record.toolbar.actions.first())
        .map(|action| action.bounds)
        .ok_or_else(|| {
            CommandChromeScriptError::message(
                "visible floating toolbar did not expose action bounds",
            )
        })
}

pub(super) fn query_bounds(
    frames: &[CommandChromeScriptFrame],
) -> Result<UiRect, CommandChromeScriptError> {
    Ok(last(frames)?.search.record.query.frame.content_bounds)
}

pub(super) fn replace_bounds(
    frames: &[CommandChromeScriptFrame],
) -> Result<UiRect, CommandChromeScriptError> {
    last(frames)?
        .search
        .record
        .replace
        .as_ref()
        .map(|record| record.frame.content_bounds)
        .ok_or_else(|| CommandChromeScriptError::message("replace input did not expose bounds"))
}

pub(super) fn search_control(
    frames: &[CommandChromeScriptFrame],
    suffix: &str,
) -> Result<UiRect, CommandChromeScriptError> {
    last(frames)?
        .search
        .record
        .controls
        .iter()
        .find(|control| control.control_id.ends_with(suffix))
        .map(|control| control.bounds)
        .ok_or_else(|| {
            CommandChromeScriptError::message(format!(
                "search control `{suffix}` did not expose bounds"
            ))
        })
}

pub(super) fn outside_target(
    frames: &[CommandChromeScriptFrame],
) -> Result<egui::Pos2, CommandChromeScriptError> {
    Ok(center(query_bounds(frames)?))
}

pub(super) fn center(bounds: UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / CENTER_DIVISOR,
        bounds.y as f32 + bounds.height as f32 / CENTER_DIVISOR,
    )
}

pub(super) fn click_events(position: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

pub(super) fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

pub(super) fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    floating: &mut FloatingCommandToolbar,
    search: &mut CommandChromeSearchStrip,
    events: Vec<egui::Event>,
) -> Result<CommandChromeScriptFrame, CommandChromeScriptError> {
    let mut result = None;
    let full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(FRAME_WIDTH, FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| result = Some(show_command_chrome(ui, adapter, toolbar, floating, search)),
    );
    let surface_frame = result
        .ok_or_else(|| CommandChromeScriptError::message("actual egui frame was not produced"))?
        .map_err(|error| CommandChromeScriptError::message(error.to_string()))?;
    let mut accesskit_labels: Vec<String> = full_output
        .platform_output
        .accesskit_update
        .into_iter()
        .flat_map(|update| update.nodes)
        .flat_map(|(_, node)| {
            [node.label(), node.placeholder()]
                .into_iter()
                .flatten()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .collect();
    accesskit_labels.sort();
    accesskit_labels.dedup();
    Ok(CommandChromeScriptFrame {
        toolbar: surface_frame.toolbar,
        floating: surface_frame.floating,
        search: surface_frame.search,
        accesskit_labels,
    })
}

fn last(
    frames: &[CommandChromeScriptFrame],
) -> Result<&CommandChromeScriptFrame, CommandChromeScriptError> {
    frames
        .last()
        .ok_or_else(|| CommandChromeScriptError::message("script has no frame"))
}
