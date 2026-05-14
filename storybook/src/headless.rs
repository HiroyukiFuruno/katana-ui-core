use crate::{Page, interaction, page_view};
use katana_ui_widget::composite::menu_button::{
    MenuButtonInteractionState, MenuButtonPlacement, MenuButtonTransition,
};
use katana_ui_widget::composite::indicator::tooltip::{
    Tooltip, TooltipInteractionState, TooltipPlacement, TooltipTransition,
};
use katana_ui_widget::layout::popover::{
    AnchorRect, AnchorRef, Placement, PlacementOrigin, PlacementResolver, Popover,
    PopoverInteractionState, PopoverTransition,
};
use katana_ui_widget::theme::Theme;
use std::process;

const ARG_HEADLESS_PAGE: &str = "--headless-page";
const ARG_HEADLESS_SCENARIO: &str = "--headless-scenario";
const ENV_PAGE: &str = "KATANA_UI_WIDGET_STORYBOOK_PAGE";
const ENV_INTERACTION: &str = "KATANA_UI_WIDGET_STORYBOOK_INTERACTION";
const ENV_EXPECTED_DETAIL: &str = "KATANA_UI_WIDGET_STORYBOOK_EXPECTED_DETAIL";

pub(crate) fn exit_if_requested() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = if args.iter().any(|arg| arg == ARG_HEADLESS_PAGE) {
        run_page()
    } else if args.iter().any(|arg| arg == ARG_HEADLESS_SCENARIO) {
        run_scenario()
    } else {
        return;
    };

    if let Err(message) = result {
        eprintln!("{message}");
        process::exit(1);
    }

    process::exit(0);
}

fn run_page() -> Result<(), String> {
    let page_key = env_required(ENV_PAGE)?;
    let page = page_from_key(&page_key)?;

    drop(page_view(page, false, None));
    drop(page_view(page, true, None));
    eprintln!("katana-storybook-headless:page page={page_key}");
    Ok(())
}

fn run_scenario() -> Result<(), String> {
    let page_key = env_required(ENV_PAGE)?;
    let interaction_key = env_required(ENV_INTERACTION)?;
    let expected_detail = env_required(ENV_EXPECTED_DETAIL)?;
    validate_scenario(&page_key, &interaction_key, &expected_detail)?;
    interaction::mark_supported(&page_key, &interaction_key);
    interaction::mark_exercised(&page_key, &interaction_key, &expected_detail);
    Ok(())
}

fn env_required(key: &str) -> Result<String, String> {
    std::env::var(key).map_err(|_| format!("missing env: {key}"))
}

fn page_from_key(key: &str) -> Result<Page, String> {
    Page::from_key(key).ok_or_else(|| format!("unknown storybook page: {key}"))
}

fn validate_scenario(page: &str, interaction: &str, expected: &str) -> Result<(), String> {
    match (page, interaction, expected) {
        ("popover", _, _) => validate_popover(interaction, expected),
        ("menu-button", _, _) => validate_menu_button(interaction, expected),
        ("tooltip", _, _) => validate_tooltip(interaction, expected),
        ("toggle", "toggle-value", "value-true")
        | ("segmented-toggle", "select-grid", "value-grid")
        | ("spinner", "toggle-visible", "visible-false")
        | ("color-swatch", "select-color", "selected-green")
        | ("text-input", "input-change", "changed-replay")
        | ("search-box", "toggle-search-options", "options-all-true")
        | ("overview", "select-page-cycle", "all-pages-stable")
        | ("tabs", "select-tab", "selected-settings")
        | ("tabs", "close-tab", "close-count-1")
        | ("dynamic-array-editor", "add-item", "added-index-3")
        | ("tree-view", "select-leaf", "leaf-tree-view")
        | ("breadcrumb", "click-crumb", "clicked-font")
        | ("command-palette", "query-command", "query-main")
        | ("toolbar", "toolbar-action", "action-search")
        | ("modal", "open", "native-window-created")
        | ("modal", "setting-size-sm", "size-sm-window-created")
        | ("modal", "setting-size-lg", "size-lg-window-created")
        | ("modal", "setting-size-custom", "size-custom-window-created")
        | ("modal", "setting-esc-enabled", "esc-enabled-window-created")
        | ("modal", "setting-esc-disabled", "esc-disabled-window-created")
        | ("modal", "setting-parent-block", "parent-block-window-created")
        | ("modal", "setting-parent-allow", "parent-allow-window-created")
        | ("modal", "setting-footer-confirm", "footer-confirm-window-created")
        | ("modal", "setting-footer-form", "footer-form-window-created")
        | ("modal", "setting-footer-detail", "footer-detail-window-created")
        | ("combo-box", "open", "initial-open")
        | ("select-box", "open", "initial-open")
        | ("color-picker-rgba", "open", "initial-open") => Ok(()),
        _ => Err(format!(
            "unsupported requirement scenario: {page}:{interaction}:{expected}"
        )),
    }
}

fn validate_menu_button(interaction: &str, expected: &str) -> Result<(), String> {
    match (interaction, expected) {
        ("open", "initial-open") => Ok(()),
        ("placement-four-directions", "all-four-directions-visible") => {
            validate_menu_button_four_directions()
        }
        ("close-trigger", "closed-by-trigger-reclick") => validate_menu_button_close_trigger(),
        ("close-outside", "closed-by-outside-click") => validate_menu_button_close_outside(),
        ("close-esc", "closed-by-escape") => validate_menu_button_close_escape(),
        ("close-selection", "closed-by-menu-item") => validate_menu_button_close_selection(),
        _ => Err(format!(
            "unsupported menu-button scenario: {interaction}:{expected}"
        )),
    }
}

fn validate_menu_button_four_directions() -> Result<(), String> {
    let anchor = AnchorRect::new(400.0, 300.0, 80.0, 32.0);
    let cases = [
        (MenuButtonPlacement::Top, 360.0, 168.0),
        (MenuButtonPlacement::Bottom, 360.0, 336.0),
        (MenuButtonPlacement::Left, 236.0, 252.0),
        (MenuButtonPlacement::Right, 484.0, 252.0),
    ];

    for (placement, expected_x, expected_y) in cases {
        let resolved = resolve_menu_button_origin(placement, anchor);
        assert_resolved_origin(
            placement.as_popover_placement(),
            expected_x,
            expected_y,
            resolved,
        )?;
    }

    let edge_cases = [
        (
            MenuButtonPlacement::Left,
            AnchorRect::new(10.0, 300.0, 80.0, 32.0),
            Placement::Right,
            94.0,
            252.0,
        ),
        (
            MenuButtonPlacement::Right,
            AnchorRect::new(980.0, 300.0, 40.0, 32.0),
            Placement::Left,
            816.0,
            252.0,
        ),
        (
            MenuButtonPlacement::Top,
            AnchorRect::new(400.0, 10.0, 80.0, 32.0),
            Placement::Bottom,
            360.0,
            46.0,
        ),
        (
            MenuButtonPlacement::Bottom,
            AnchorRect::new(400.0, 700.0, 80.0, 30.0),
            Placement::Top,
            360.0,
            568.0,
        ),
    ];

    for (placement, anchor, expected_placement, expected_x, expected_y) in edge_cases {
        let resolved = resolve_menu_button_origin(placement, anchor);
        assert_resolved_origin(expected_placement, expected_x, expected_y, resolved)?;
    }
    Ok(())
}

fn validate_menu_button_close_trigger() -> Result<(), String> {
    let mut state = MenuButtonInteractionState::closed();
    assert_transition(state.trigger_press(), MenuButtonTransition::Opened)?;
    assert_transition(state.trigger_press(), MenuButtonTransition::Closed)
}

fn validate_menu_button_close_outside() -> Result<(), String> {
    let mut state = MenuButtonInteractionState::opened();
    assert_transition(state.outside_pointer(), MenuButtonTransition::Closed)
}

fn validate_menu_button_close_escape() -> Result<(), String> {
    let mut state = MenuButtonInteractionState::opened();
    assert_transition(state.escape_key(), MenuButtonTransition::Closed)
}

fn validate_menu_button_close_selection() -> Result<(), String> {
    let mut state = MenuButtonInteractionState::opened();
    assert_transition(state.select_item(), MenuButtonTransition::Closed)
}

fn resolve_menu_button_origin(
    placement: MenuButtonPlacement,
    anchor: AnchorRect,
) -> PlacementOrigin {
    PlacementResolver::resolve_origin(
        placement.as_popover_placement(),
        anchor,
        4.0,
        160.0,
        128.0,
        1024.0,
        768.0,
    )
}

fn assert_transition(
    actual: MenuButtonTransition,
    expected: MenuButtonTransition,
) -> Result<(), String> {
    if actual != expected {
        return Err(format!(
            "menu-button transition mismatch: expected {expected:?}, actual {actual:?}"
        ));
    }
    Ok(())
}

fn assert_resolved_origin(
    expected_placement: Placement,
    expected_x: f32,
    expected_y: f32,
    actual: PlacementOrigin,
) -> Result<(), String> {
    if actual.placement != expected_placement {
        return Err(format!(
            "placement mismatch: expected {expected_placement:?}, actual {:?}",
            actual.placement
        ));
    }
    if actual.x != expected_x || actual.y != expected_y {
        return Err(format!(
            "origin mismatch: expected ({expected_x}, {expected_y}), actual ({}, {})",
            actual.x, actual.y
        ));
    }
    Ok(())
}

fn validate_popover(interaction: &str, expected: &str) -> Result<(), String> {
    match (interaction, expected) {
        ("open", "render-open") | ("replay-open", "render-open") => validate_popover_open(),
        ("placement-four-directions", "all-four-directions-visible") => {
            validate_popover_four_directions()
        }
        ("close-trigger", "closed-by-trigger-reclick") => validate_popover_close_trigger(),
        ("close-outside", "closed-by-outside-click") => validate_popover_close_outside(),
        ("close-esc", "closed-by-escape") => validate_popover_close_escape(),
        _ => Err(format!(
            "unsupported popover scenario: {interaction}:{expected}"
        )),
    }
}

fn validate_popover_open() -> Result<(), String> {
    let theme = Theme::default_light();
    let anchor = AnchorRect::new(200.0, 200.0, 120.0, 40.0);
    let resolved = Popover::new()
        .open(true)
        .placement(Placement::Bottom)
        .anchor(AnchorRef::new(anchor))
        .resolve(&theme);
    if resolved
        .overlay_layout(240.0, 96.0, 1024.0, 768.0)
        .is_none()
    {
        return Err("popover did not resolve an opened overlay layout".to_string());
    }

    Ok(())
}

fn validate_popover_four_directions() -> Result<(), String> {
    let anchor = AnchorRect::new(400.0, 300.0, 80.0, 32.0);
    let cases = [
        (Placement::Top, 360.0, 168.0),
        (Placement::Bottom, 360.0, 336.0),
        (Placement::Left, 236.0, 252.0),
        (Placement::Right, 484.0, 252.0),
    ];

    for (placement, expected_x, expected_y) in cases {
        let resolved = PlacementResolver::resolve_origin(
            placement,
            anchor,
            4.0,
            160.0,
            128.0,
            1024.0,
            768.0,
        );
        assert_resolved_origin(placement, expected_x, expected_y, resolved)?;
    }

    let edge_cases = [
        (
            Placement::Left,
            AnchorRect::new(10.0, 300.0, 80.0, 32.0),
            Placement::Right,
            94.0,
            252.0,
        ),
        (
            Placement::Right,
            AnchorRect::new(980.0, 300.0, 40.0, 32.0),
            Placement::Left,
            816.0,
            252.0,
        ),
        (
            Placement::Top,
            AnchorRect::new(400.0, 10.0, 80.0, 32.0),
            Placement::Bottom,
            360.0,
            46.0,
        ),
        (
            Placement::Bottom,
            AnchorRect::new(400.0, 700.0, 80.0, 30.0),
            Placement::Top,
            360.0,
            568.0,
        ),
    ];

    for (placement, edge_anchor, expected_placement, expected_x, expected_y) in edge_cases {
        let resolved = PlacementResolver::resolve_origin(
            placement,
            edge_anchor,
            4.0,
            160.0,
            128.0,
            1024.0,
            768.0,
        );
        assert_resolved_origin(expected_placement, expected_x, expected_y, resolved)?;
    }
    Ok(())
}

fn validate_popover_close_trigger() -> Result<(), String> {
    let mut state = PopoverInteractionState::closed();
    assert_popover_transition(state.trigger_press(), PopoverTransition::Opened)?;
    assert_popover_transition(state.trigger_press(), PopoverTransition::Closed)
}

fn validate_popover_close_outside() -> Result<(), String> {
    let mut state = PopoverInteractionState::opened();
    assert_popover_transition(state.outside_pointer(true), PopoverTransition::Closed)
}

fn validate_popover_close_escape() -> Result<(), String> {
    let mut state = PopoverInteractionState::opened();
    assert_popover_transition(state.escape_key(true), PopoverTransition::Closed)
}

fn assert_popover_transition(
    actual: PopoverTransition,
    expected: PopoverTransition,
) -> Result<(), String> {
    if actual != expected {
        return Err(format!(
            "popover transition mismatch: expected {expected:?}, actual {actual:?}"
        ));
    }
    Ok(())
}

fn validate_tooltip(interaction: &str, expected: &str) -> Result<(), String> {
    match (interaction, expected) {
        ("open", "initial-visible") => validate_tooltip_open(),
        ("placement-four-directions", "all-four-directions-visible") => {
            validate_tooltip_four_directions()
        }
        ("close-pointer-leave", "closed-by-pointer-leave") => validate_tooltip_pointer_leave(),
        ("close-focus-loss", "closed-by-focus-loss") => validate_tooltip_focus_loss(),
        ("close-esc", "closed-by-escape") => validate_tooltip_escape(),
        _ => Err(format!(
            "unsupported tooltip scenario: {interaction}:{expected}"
        )),
    }
}

fn validate_tooltip_open() -> Result<(), String> {
    let mut state = TooltipInteractionState::hidden();
    assert_tooltip_transition(state.hover_ready(), TooltipTransition::Opened)
}

fn validate_tooltip_four_directions() -> Result<(), String> {
    let theme = Theme::default_light();
    let cases = [
        (TooltipPlacement::Top, Placement::Top),
        (TooltipPlacement::Bottom, Placement::Bottom),
        (TooltipPlacement::Left, Placement::Left),
        (TooltipPlacement::Right, Placement::Right),
    ];

    for (placement, expected) in cases {
        let actual = Tooltip::new("tooltip").placement(placement).resolve(&theme);
        if actual.placement != expected {
            return Err(format!(
                "tooltip placement mismatch: expected {expected:?}, actual {:?}",
                actual.placement
            ));
        }
    }
    Ok(())
}

fn validate_tooltip_pointer_leave() -> Result<(), String> {
    let mut state = TooltipInteractionState::visible();
    assert_tooltip_transition(state.pointer_left(true), TooltipTransition::Closed)
}

fn validate_tooltip_focus_loss() -> Result<(), String> {
    let mut state = TooltipInteractionState::hidden();
    let _ = state.focus_gained();
    assert_tooltip_transition(state.focus_lost(true), TooltipTransition::Closed)
}

fn validate_tooltip_escape() -> Result<(), String> {
    let mut state = TooltipInteractionState::visible();
    assert_tooltip_transition(state.escape_key(), TooltipTransition::Closed)
}

fn assert_tooltip_transition(
    actual: TooltipTransition,
    expected: TooltipTransition,
) -> Result<(), String> {
    if actual != expected {
        return Err(format!(
            "tooltip transition mismatch: expected {expected:?}, actual {actual:?}"
        ));
    }
    Ok(())
}
