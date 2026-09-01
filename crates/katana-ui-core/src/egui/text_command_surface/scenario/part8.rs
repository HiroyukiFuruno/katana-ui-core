fn stage(events: Vec<egui::Event>, pixels_per_point: f32) -> FullTextCommandSurfaceRawInputStage {
    stage_with_screen(events, pixels_per_point, egui::vec2(WIDTH, HEIGHT))
}

fn primary_pointer(position: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: position,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn key_press(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

fn stage_with_screen(
    events: Vec<egui::Event>,
    pixels_per_point: f32,
    size: egui::Vec2,
) -> FullTextCommandSurfaceRawInputStage {
    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
        events,
        ..egui::RawInput::default()
    };
    input.viewports.insert(
        egui::ViewportId::ROOT,
        egui::ViewportInfo {
            native_pixels_per_point: Some(pixels_per_point),
            ..egui::ViewportInfo::default()
        },
    );
    FullTextCommandSurfaceRawInputStage::new(input)
}
