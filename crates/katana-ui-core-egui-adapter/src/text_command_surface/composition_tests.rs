use super::*;
use katana_ui_core::molecule::command_chrome::CommandChromeAction;

pub(super) fn inject_same_bounds_test_overlay(
    ui: &mut egui::Ui,
    toolbar: Option<&mut crate::command_chrome::EguiCommandChromeOutput>,
    chrome: &mut crate::command_chrome::EguiCommandChromeAdapter,
    style: &TextCommandSurfaceStyle,
) -> Result<(), EguiTextCommandSurfaceError> {
    let Some(toolbar) = toolbar else {
        return Ok(());
    };
    let bounds = toolbar.record.bounds;
    let rect = egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    );
    let mut render = |id: &str| -> Result<crate::command_chrome::EguiCommandChromeOutput, _> {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect));
        let mut overlay = katana_ui_core::molecule::command_chrome::CommandChromeToolbar::new();
        overlay =
            overlay.action(CommandChromeAction::new(id, "同一").accessibility_label("同一 ⭐️"));
        chrome.show_toolbar(
            &mut child,
            &mut overlay,
            &style.chrome_raster,
            &style.chrome_paint,
        )
    };
    let first = render("collision-left")?;
    let second = render("collision-right")?;
    toolbar.record.actions.extend(first.record.actions);
    toolbar.record.actions.extend(second.record.actions);
    toolbar.events.extend(first.events);
    toolbar.events.extend(second.events);
    Ok(())
}

#[test]
fn ui_rect_rounds_position_and_size_while_clamping_negative_size() {
    let rect = egui::Rect::from_min_max(egui::pos2(1.6, 2.4), egui::pos2(4.4, 1.9));
    let converted = ui_rect(rect);
    assert_eq!(converted.x, 2);
    assert_eq!(converted.y, 2);
    assert_eq!(converted.width, 3);
    assert_eq!(converted.height, 0);
}
