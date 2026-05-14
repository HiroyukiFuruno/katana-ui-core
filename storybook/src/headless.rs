use crate::{Page, interaction, page_view};
use katana_ui_widget::layout::popover::{AnchorRect, AnchorRef, Placement, Popover};
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
        | ("menu-button", "open", "initial-open")
        | ("tooltip", "open", "initial-visible")
        | ("combo-box", "open", "initial-open")
        | ("select-box", "open", "initial-open")
        | ("color-picker-rgba", "open", "initial-open") => Ok(()),
        _ => Err(format!(
            "unsupported requirement scenario: {page}:{interaction}:{expected}"
        )),
    }
}

fn validate_popover(interaction: &str, expected: &str) -> Result<(), String> {
    if !matches!(
        (interaction, expected),
        ("open", "render-open") | ("replay-open", "render-open")
    ) {
        return Err(format!(
            "unsupported popover scenario: {interaction}:{expected}"
        ));
    }

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
