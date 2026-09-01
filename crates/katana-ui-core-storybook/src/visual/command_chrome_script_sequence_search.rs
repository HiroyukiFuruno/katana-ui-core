use super::CommandChromeScriptError;
use super::CommandChromeScriptFrame;
use super::command_chrome_script_frame::{
    center, click_events, push_events, query_bounds, replace_bounds, search_control,
};
use katana_ui_core::egui::command_chrome::EguiCommandChromeAdapter;
use katana_ui_core::interaction::placement::Rect;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchPresentation, CommandChromeSearchStrip, CommandChromeToolbar,
    FloatingCommandToolbar, FloatingCommandToolbarPresentation, FloatingCommandToolbarVisibility,
    SearchControlIcons,
};

const CONTROLLED_ANCHOR_X: i32 = 1_080;
const CONTROLLED_ANCHOR_Y: i32 = 620;
const CONTROLLED_ANCHOR_WIDTH: u32 = 32;
const CONTROLLED_ANCHOR_HEIGHT: u32 = 24;
const CONTROLLED_VIEWPORT_WIDTH: u32 = 1_280;
const CONTROLLED_VIEWPORT_HEIGHT: u32 = 720;

pub(super) fn run_search_and_controlled_floating_sequence(
    context: &egui::Context,
    frames: &mut Vec<CommandChromeScriptFrame>,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    floating: &mut FloatingCommandToolbar,
    search: &mut CommandChromeSearchStrip,
) -> Result<(), CommandChromeScriptError> {
    let query = query_bounds(frames)?;
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        click_events(center(query)),
    );
    pushed?;
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        vec![egui::Event::Text("日本語 ⭐️".to_string())],
    );
    pushed?;
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "ほし".to_string(),
            active_range_chars: None,
        })],
    );
    pushed?;
    let synchronized_search = CommandChromeSearchPresentation {
        query: "同期検索 ⭐️".to_string(),
        options: *search.options_model(),
        result_count: search.result_count_model(),
        active_index: search.active_index_model(),
        replace_mode: search.replace_mode_model(),
        replace_value: "同期置換 ⭐️".to_string(),
        strings: search.strings_model().clone(),
        capabilities: search.capabilities_model().clone(),
        icons: SearchControlIcons::default(),
    };
    let _ = search.synchronize_presentation(synchronized_search);
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        Vec::new(),
    );
    pushed?;
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );
    pushed?;

    for control in ["match-case", "whole-word"] {
        let target = center(search_control(frames, control)?);
        let pushed = push_events(
            context,
            frames,
            adapter,
            toolbar,
            floating,
            search,
            click_events(target),
        );
        pushed?;
    }

    let replace = replace_bounds(frames)?;
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        click_events(center(replace)),
    );
    pushed?;
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        vec![egui::Event::Text("置換 ⭐️".to_string())],
    );
    pushed?;

    for control in ["replace-one", "replace-all", "use-regex"] {
        let target = center(search_control(frames, control)?);
        let pushed = push_events(
            context,
            frames,
            adapter,
            toolbar,
            floating,
            search,
            click_events(target),
        );
        pushed?;
    }
    let target = center(search_control(frames, "close")?);
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        click_events(target),
    );
    pushed?;

    let _ = floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
        Rect::new(
            CONTROLLED_ANCHOR_X,
            CONTROLLED_ANCHOR_Y,
            CONTROLLED_ANCHOR_WIDTH,
            CONTROLLED_ANCHOR_HEIGHT,
        ),
        Rect::new(0, 0, CONTROLLED_VIEWPORT_WIDTH, CONTROLLED_VIEWPORT_HEIGHT),
        FloatingCommandToolbarVisibility::Visible,
    ));
    let pushed = push_events(
        context,
        frames,
        adapter,
        toolbar,
        floating,
        search,
        Vec::new(),
    );
    pushed?;
    Ok(())
}
