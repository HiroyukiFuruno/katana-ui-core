use super::*;
use crate::molecule::RgbaColor;
use crate::molecule::command_chrome::{
    CommandChromeDropdown, CommandChromeDropdownItem, CommandChromeDropdownTrigger,
};
use crate::theme::{FontFamily, FontToken};

#[test]
fn outside_pointer_respects_enabled_item_exclusions_and_pressed_state() {
    let dropdown = UiRect::new(0, 0, 10, 10);
    let toolbar = UiRect::new(20, 0, 10, 10);
    let item_bounds = UiRect::new(40, 40, 10, 10);
    let enabled = item(item_bounds, false);

    assert!(!run_pointer_outside(
        egui::pos2(45.0, 45.0),
        true,
        dropdown,
        toolbar,
        &[enabled]
    ));
    assert!(run_pointer_outside(
        egui::pos2(45.0, 45.0),
        true,
        dropdown,
        toolbar,
        &[item(item_bounds, true)]
    ));
    assert!(!run_pointer_outside(
        egui::pos2(60.0, 60.0),
        false,
        dropdown,
        toolbar,
        &[]
    ));
}

fn item(bounds: UiRect, disabled: bool) -> EguiCommandChromeDropdownItemFrame {
    EguiCommandChromeDropdownItemFrame {
        item_id: "item".to_string(),
        bounds,
        icon_raster_identity: None,
        label_raster_identity: "label".to_string(),
        disabled,
        selected: false,
        focused: false,
    }
}

#[test]
fn render_dropdown_items_maps_each_item_to_rendered_output() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let style = CommandChromeRasterStyle {
        font: FontToken {
            name: "system-ui".to_string(),
            family: FontFamily::Proportional,
            size: 16.0,
            weight: 400,
        },
        text_color_rgba: [230, 230, 230, 255],
        icon_color: RgbaColor::new(230, 230, 230, 255),
        line_height_px: 24.0,
        icon_size_px: 16,
    };
    let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
        .item(CommandChromeDropdownItem::new("label-only", "Label"));

    let mut output = None;
    let mut frame_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(200.0, 50.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            output = Some(
                render_dropdown_items(&mut adapter, ui, &dropdown, &style)
                    .map_err(|error| error.to_string()),
            );
        },
    );
    frame_output.textures_delta.clear();

    let rendered = output.expect("egui must execute the dropdown render")?;
    assert_eq!(rendered.len(), 1);
    let first = &rendered[0];
    assert_eq!(first.item.id().as_str(), "label-only");
    assert!(first.rendered.label_identity.is_some());
    assert!(
        first
            .rendered
            .label_identity
            .as_deref()
            .expect("label identity should be set")
            .starts_with("command-label:Label:")
    );
    let label = first
        .rendered
        .label
        .as_ref()
        .expect("label raster should be available");
    assert!(label.width >= 1);
    assert!(label.height >= 1);
    Ok(())
}

fn run_pointer_outside(
    position: egui::Pos2,
    pressed: bool,
    dropdown: UiRect,
    toolbar: UiRect,
    items: &[EguiCommandChromeDropdownItemFrame],
) -> bool {
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(100.0, 100.0),
            )),
            events: vec![egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            }],
            ..egui::RawInput::default()
        },
        |ui| result = Some(pointer_pressed_outside(ui, dropdown, toolbar, items)),
    );
    output.textures_delta.clear();
    result.expect("egui must execute the pointer predicate")
}
