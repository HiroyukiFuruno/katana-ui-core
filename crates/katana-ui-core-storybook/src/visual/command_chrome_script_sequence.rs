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

    let initial_frame = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        Vec::new(),
    );
    let mut frames = vec![initial_frame?];

    let target = center(floating_action(&frames)?);
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![egui::Event::PointerMoved(target)],
    );
    pushed?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    );
    pushed?;

    let target = outside_target(&frames)?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    );
    pushed?;

    let target = center(toolbar_action(&frames, "disabled")?);
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    );
    pushed?;

    let target = center(toolbar_action(&frames, "inline-bold")?);
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    );
    pushed?;

    let split = toolbar_secondary(&frames, "code-block")?;
    let target = outside_target(&frames)?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    );
    pushed?;

    for key in [
        egui::Key::ArrowDown,
        egui::Key::ArrowUp,
        egui::Key::Home,
        egui::Key::End,
        egui::Key::Space,
    ] {
        let pushed = push_events(
            &context,
            &mut frames,
            &mut adapter,
            &mut toolbar,
            &mut floating,
            &mut search,
            vec![key_event(key)],
        );
        pushed?;
    }

    let split = toolbar_secondary(&frames, "code-block")?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    );
    pushed?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![key_event(egui::Key::Enter)],
    );
    pushed?;

    let split = toolbar_secondary(&frames, "code-block")?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    );
    pushed?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(target),
    );
    pushed?;

    let split = toolbar_secondary(&frames, "code-block")?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        click_events(center(split)),
    );
    pushed?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![key_event(egui::Key::Escape)],
    );
    pushed?;

    floating = floating_fixture();
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        Vec::new(),
    );
    pushed?;
    let pushed = push_events(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
        vec![key_event(egui::Key::Escape)],
    );
    pushed?;

    let search_sequence = run_search_and_controlled_floating_sequence(
        &context,
        &mut frames,
        &mut adapter,
        &mut toolbar,
        &mut floating,
        &mut search,
    );
    search_sequence?;

    Ok(CommandChromeScriptResult { frames })
}
