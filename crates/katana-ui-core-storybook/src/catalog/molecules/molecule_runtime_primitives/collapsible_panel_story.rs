use super::{
    CollapsiblePanelAction, CollapsiblePanelWidth, PANEL_DEFAULT_WIDTH, PANEL_MAX_WIDTH,
    PANEL_MIN_WIDTH, PANEL_RESIZED_WIDTH, PanelMode, PanelSide, StoryCatalog, StoryExample,
    UiCallbackLog, atom, molecule,
};

pub(super) fn collapsible_panel_story() -> StoryExample {
    let mut panel = molecule::CollapsiblePanel::new("Collapsible panel", panel_width())
        .side(PanelSide::Leading)
        .resize_handle(true)
        .expand_on_hover(true)
        .content(atom::Text::new(
            "Explorer panel: mode=Expanded width=240 pinned=true expand_on_hover=true resize_handle=true",
        ))
        .content(atom::Text::new(
            "Chat history panel: child slot shows recent threads and pinned sessions",
        ))
        .content(atom::Text::new(
            "TOC panel: child slot shows document headings and current section",
        ))
        .content(atom::Text::new(
            "Floating overlay: mode=FloatingOverlay layout_width=0 overlay_z=80",
        ))
        .content(atom::Text::new(
            "IconOnly: mode=IconOnly layout_width=56 child slot keeps icon navigation",
        ));
    let target = panel.state_id().clone();
    let resized = panel.apply_action(CollapsiblePanelAction::Resize(PANEL_RESIZED_WIDTH));
    let overlay = panel.apply_action(CollapsiblePanelAction::SetMode(PanelMode::FloatingOverlay));
    let icon_only = panel.apply_action(CollapsiblePanelAction::SetMode(PanelMode::IconOnly));
    let unpinned = panel.apply_action(CollapsiblePanelAction::Unpin);
    let hover_opened = panel.apply_action(CollapsiblePanelAction::HoverTrigger);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_resize",
            "width=240 resize_handle=true",
            format!("events={resized:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_overlay",
            "mode=Expanded pinned=true",
            format!("events={overlay:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_icon_only",
            "mode=FloatingOverlay width=320",
            format!("events={icon_only:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_hover",
            "mode=IconOnly expand_on_hover=true",
            format!("events={hover_opened:?}"),
        ),
        UiCallbackLog::new(
            target,
            "collapsible_panel_pin",
            "pinned=true",
            format!("events={unpinned:?}"),
        ),
    ];
    StoryCatalog::interactive_story("collapsible-panel", panel, logs)
}

fn panel_width() -> CollapsiblePanelWidth {
    CollapsiblePanelWidth::new(
        PANEL_MIN_WIDTH,
        PANEL_MAX_WIDTH,
        PANEL_DEFAULT_WIDTH,
        PANEL_DEFAULT_WIDTH,
        Some("storybook.panel.width"),
    )
}
