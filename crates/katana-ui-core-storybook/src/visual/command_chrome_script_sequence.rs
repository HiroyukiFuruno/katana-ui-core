use super::super::{
    command_chrome_fixture::floating_fixture, command_chrome_surface::CommandChromeSurfaceFixture,
    command_chrome_surface::command_chrome_surface_fixture,
};
use super::command_chrome_script_frame::{
    center, click_events, floating_action, key_event, outside_target, push_events, run_frame,
    toolbar_action, toolbar_secondary,
};
use super::command_chrome_script_sequence_search::run_search_and_controlled_floating_sequence;
use super::{CommandChromeScriptError, CommandChromeScriptResult};

pub(super) fn run_scripted_sequence() -> Result<CommandChromeScriptResult, CommandChromeScriptError>
{
    let context = egui::Context::default();
    context.enable_accesskit();

    let CommandChromeSurfaceFixture {
        mut adapter,
        mut toolbar,
        mut floating,
        mut search,
    } = command_chrome_surface_fixture(true);

    let mut frames = vec![run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        Vec::new(),
    )?];

    let target = center(floating_action(&frames)?);
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![egui::Event::PointerMoved(target)],
    )?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    )?;

    let target = outside_target(&frames)?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    )?;

    let target = center(toolbar_action(&frames, "disabled")?);
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    )?;

    let target = center(toolbar_action(&frames, "inline-bold")?);
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    )?;

    let split = toolbar_secondary(&frames, "code-block")?;
    let target = outside_target(&frames)?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    )?;

    for key in [
        egui::Key::ArrowDown,
        egui::Key::ArrowUp,
        egui::Key::Home,
        egui::Key::End,
        egui::Key::Space,
    ] {
        push_events(
            &context,
            &mut frames,
            &mut adapter,
            &mut toolbar,
            &mut floating,
            &mut search,
            vec![key_event(key)],
        )?;
    }

    let split = toolbar_secondary(&frames, "code-block")?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    )?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![key_event(egui::Key::Enter)],
    )?;

    let split = toolbar_secondary(&frames, "code-block")?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    )?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    )?;

    let split = toolbar_secondary(&frames, "code-block")?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    )?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![key_event(egui::Key::Escape)],
    )?;

    floating = floating_fixture();
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        Vec::new(),
    )?;
    push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![key_event(egui::Key::Escape)],
    )?;

    run_search_and_controlled_floating_sequence(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
    )?;

    Ok(CommandChromeScriptResult { frames })
}
