use super::{StoryCatalog, StoryExample};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core::{atom, molecule, panel};

const NAV_SCROLL_Y: u32 = 48;
const NAV_VIEWPORT_HEIGHT: u32 = 180;
const NAV_CONTENT_HEIGHT: u32 = 520;
const PREVIEW_SCROLL_Y: u32 = 0;
const PREVIEW_VIEWPORT_HEIGHT: u32 = 260;
const PREVIEW_CONTENT_HEIGHT: u32 = 260;
const PREVIEW_SCROLL_X: u32 = 96;
const PREVIEW_VIEWPORT_WIDTH: u32 = 420;
const PREVIEW_CONTENT_WIDTH: u32 = 900;
const DETAILS_SCROLL_Y: u32 = 0;
const DETAILS_VIEWPORT_HEIGHT: u32 = 220;
const DETAILS_CONTENT_HEIGHT: u32 = 220;
const ROOT_SCROLL_Y: u32 = 0;
const ROOT_VIEWPORT_HEIGHT: u32 = 600;
const ROOT_CONTENT_HEIGHT: u32 = 600;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![panel_story(), theme_tokens_story()]
}

fn panel_story() -> StoryExample {
    let theme = ThemeSnapshot::dark();
    let navigation = panel::Panel::new(
        "Navigation panel",
        panel::PanelRegion::Navigation,
        theme.clone(),
    )
    .vertical_scroll(NAV_SCROLL_Y, NAV_VIEWPORT_HEIGHT, NAV_CONTENT_HEIGHT, true)
    .child(atom::Text::new("Foundation"))
    .child(atom::Text::new("Atoms"))
    .child(atom::Text::new("Molecules"));
    let preview = panel::Panel::new("Preview panel", panel::PanelRegion::Preview, theme.clone())
        .vertical_scroll(
            PREVIEW_SCROLL_Y,
            PREVIEW_VIEWPORT_HEIGHT,
            PREVIEW_CONTENT_HEIGHT,
            false,
        )
        .horizontal_scroll(
            PREVIEW_SCROLL_X,
            PREVIEW_VIEWPORT_WIDTH,
            PREVIEW_CONTENT_WIDTH,
            true,
        )
        .child(atom::Text::new(
            "preview content is clipped to the parent panel",
        ))
        .child(atom::Text::new("horizontal overflow is local to preview"));
    let details = panel::Panel::new("Details panel", panel::PanelRegion::Details, theme.clone())
        .vertical_scroll(
            DETAILS_SCROLL_Y,
            DETAILS_VIEWPORT_HEIGHT,
            DETAILS_CONTENT_HEIGHT,
            false,
        )
        .child(atom::Text::new("settings"))
        .child(atom::Text::new("state / event / action"));
    let root = panel::Panel::new("Panel foundation", panel::PanelRegion::Root, theme)
        .vertical_scroll(
            ROOT_SCROLL_Y,
            ROOT_VIEWPORT_HEIGHT,
            ROOT_CONTENT_HEIGHT,
            false,
        )
        .child(navigation)
        .child(preview)
        .child(details)
        .child(atom::Text::new(
            "settings: region theme overflow clip vertical_scroll horizontal_scroll",
        ))
        .child(atom::Text::new(
            "state: parent and child panels keep independent scroll offsets",
        ))
        .child(atom::Text::new(
            "quality: clipping hit_target scrollbar_visibility nested_state",
        ));
    StoryCatalog::story("panel", root)
}

fn theme_tokens_story() -> StoryExample {
    StoryCatalog::story(
        "theme-tokens",
        molecule::Card::new("Theme tokens")
            .child(atom::Badge::new("Light/Dark"))
            .child(atom::ColorSwatch::new("Accent")),
    )
}
