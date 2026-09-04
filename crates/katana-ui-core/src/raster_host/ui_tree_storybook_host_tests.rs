use super::{Canvas, UiTreeRenderArea, UiTreeStorybookHost};
use crate::test_assert::KucTestExpect;
use crate::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::atom::{Text, Toggle};
use katana_ui_core::molecule::{SettingsControl, SettingsField, SettingsList, SettingsSection};
use katana_ui_core::render_model::{UiCursor, UiHostActionSpec, UiNode, UiNodeId};
use katana_ui_core::theme::ThemeSnapshot;

const TEST_AREA_WIDTH: usize = 240;
const SMALL_AREA_HEIGHT: usize = 80;
const SETTINGS_AREA_HEIGHT: usize = 140;

#[test]
fn host_returns_rendered_action_and_cursor_from_same_contract() {
    let root: UiNode = UiNode::from(Toggle::new("Dark").checked(true))
        .stable_node_id(UiNodeId::new("dark-toggle"))
        .host_action(UiHostActionSpec::command("ui.toggle.dark", "Toggle dark"));
    let host = UiTreeStorybookHost::new(ThemeSnapshot::dark());
    let area = small_area();
    let hit = host
        .host_action_hits(&root, area)
        .into_iter()
        .next()
        .kuc_expect("toggle host action hit");
    let (x, y) = hit.center_point();

    assert_eq!(
        Some(hit.clone()),
        host.host_action_hit_at(&root, area, x, y)
    );
    assert_eq!(UiCursor::Pointer, host.cursor_at(&root, area, x, y));
    assert_eq!(
        Some(UiNodeId::new("dark-toggle")),
        host.hovered_action_node_id_at(&root, area, x, y)
    );

    let target = host
        .interaction_target_at(&root, area, x, y)
        .kuc_expect("toggle interaction target");

    assert_eq!(UiNodeId::new("dark-toggle"), target.node_id);
    assert_eq!(UiCursor::Pointer, target.cursor);
    assert_eq!(
        Some("ui.toggle.dark"),
        target
            .action
            .as_ref()
            .map(|action| action.action_id.as_str())
    );
    assert_eq!(UiNodeId::new("dark-toggle"), target.hover_node_id());
}

#[test]
fn host_returns_node_interaction_target_when_no_action_exists() {
    let root: UiNode = UiNode::from(Text::new("Readable")).stable_node_id("readable-text");
    let host = UiTreeStorybookHost::new(ThemeSnapshot::light());
    let target = host
        .interaction_target_at(&root, small_area(), 4.0, 4.0)
        .kuc_expect("text node interaction target");

    assert_eq!(UiNodeId::new("readable-text"), target.node_id);
    assert!(target.action.is_none());
    assert_eq!(UiCursor::Default, target.cursor);
}

#[test]
fn host_renders_with_the_opt_in_first_candidate_face_policy() {
    let root: UiNode = UiNode::from(Text::new("KatanA Storybook text"));
    let host = UiTreeStorybookHost::with_text_raster_config(
        ThemeSnapshot::light(),
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::FirstCandidate,
    );
    let mut canvas = Canvas::new(TEST_AREA_WIDTH, SMALL_AREA_HEIGHT, 0);

    host.render(&mut canvas, &root, small_area());

    assert!(canvas.non_background_pixels(0) > 0);
}

#[test]
fn host_returns_same_settings_row_target_for_label_and_control() {
    let root: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let host = UiTreeStorybookHost::new(ThemeSnapshot::dark());
    let area = settings_area();

    let label_target = host
        .interaction_target_at(&root, area, 24.0, 80.0)
        .kuc_expect("settings label interaction target");
    let control_target = host
        .interaction_target_at(&root, area, 128.0, 80.0)
        .kuc_expect("settings control interaction target");

    for target in [&label_target, &control_target] {
        assert_eq!(UiNodeId::new("settings-field:dark"), target.node_id);
        assert_eq!(UiNodeId::new("settings-field:dark"), target.hover_node_id());
        assert_eq!(UiCursor::Pointer, target.cursor);
        assert_eq!(
            Some("dark"),
            target
                .action
                .as_ref()
                .and_then(|action| action.settings_field_control_target())
                .map(|target| target.field_id)
                .as_deref()
        );
    }
    assert_eq!(label_target.rect, control_target.rect);
}

const fn small_area() -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: TEST_AREA_WIDTH,
        height: SMALL_AREA_HEIGHT,
        scroll_y: 0.0,
    }
}

const fn settings_area() -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: TEST_AREA_WIDTH,
        height: SETTINGS_AREA_HEIGHT,
        scroll_y: 0.0,
    }
}
