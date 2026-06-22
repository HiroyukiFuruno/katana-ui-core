use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn collects_settings_list_field_action_rect_from_rendered_row() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let root: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 240,
        height: 140,
        scroll_y: 0.0,
    };
    let hits = UiTreeHostActionHitCollector::collect(&root, area);
    let hit = hits
        .iter()
        .find(|hit| {
            hit.action.settings_field_control_target().is_some()
                && hit.action.target.as_str() == "settings-field:dark"
        })
        .kuc_expect("settings field host action missing");

    assert_eq!(UiCursor::Pointer, hit.cursor);
    assert_eq!(
        UiTreeHitRect {
            x: 8,
            y: 66,
            width: 232,
            height: 22,
        },
        hit.rect
    );

    let mut canvas = Canvas::new(240, 140, palette.background);
    UiTreeCanvasRenderer::new(theme).render(&mut canvas, &root, area);
    let track_center = (hit.rect.y + hit.rect.height / 2) * canvas.width() + 128;
    assert_eq!(palette.visual.accent, canvas.pixels()[track_center]);
}

#[test]
fn host_action_hit_at_returns_rendered_settings_row_action_for_label_pointer() {
    let theme = ThemeSnapshot::dark();
    let root: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 240,
        height: 140,
        scroll_y: 0.0,
    };
    let hit = UiTreeCanvasRenderer::new(theme)
        .host_action_hit_at(&root, area, 24.0, 80.0)
        .kuc_expect("rendered settings row label should be hit");

    assert!(hit.action.settings_field_control_target().is_some());
    assert_eq!("settings-field:dark", hit.action.target.as_str());
    assert_eq!(UiCursor::Pointer, hit.cursor);
}

#[test]
fn host_action_hit_at_returns_rendered_settings_row_action_for_control_pointer() {
    let theme = ThemeSnapshot::dark();
    let root: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 240,
        height: 140,
        scroll_y: 0.0,
    };
    let hit = UiTreeCanvasRenderer::new(theme)
        .host_action_hit_at(&root, area, 128.0, 80.0)
        .kuc_expect("rendered settings row control should be hit");

    assert!(hit.action.settings_field_control_target().is_some());
    assert_eq!("settings-field:dark", hit.action.target.as_str());
    assert_eq!(UiCursor::Pointer, hit.cursor);
}

#[test]
fn collects_tree_view_row_action_rect_from_rendered_row() {
    let root: UiNode = TreeView::new("Files")
        .icons_visible(true)
        .item(
            TreeNode::new("katana", "katana", 0)
                .directory()
                .expanded(true),
        )
        .item(TreeNode::new("katana/sample.md", "sample.md", 1).file())
        .into();

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 10,
            y: 4,
            width: 240,
            height: 120,
            scroll_y: 0.0,
        },
    );
    let file_hit = hits
        .iter()
        .find(|hit| {
            hit.action
                .tree_row_action_target()
                .is_some_and(|target| target.node_id == "katana/sample.md")
        })
        .kuc_expect("tree file row host action missing");

    assert_eq!(UiCursor::Pointer, file_hit.cursor);
    assert_eq!(
        UiTreeHitRect {
            x: 10,
            y: 48,
            width: 240,
            height: 22,
        },
        file_hit.rect
    );
}
