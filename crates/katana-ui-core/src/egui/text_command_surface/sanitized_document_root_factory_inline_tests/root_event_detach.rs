use super::support::*;
use super::*;

#[test]
fn real_root_output_reports_each_already_detached_event_channel() {
    for channel in ["search", "command", "context menu"] {
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(input(1, b"detach-document", "本文 ⭐️"))
            .expect("retaining the real root succeeds");
        let context = egui::Context::default();
        let mut output = None;
        crate::egui::run_ui_discard(
            &context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    output = Some(root.process.show(ui).expect("real root output renders"));
                });
            },
        );
        let output = output.expect("real root output exists");

        match channel {
            "search" => {
                output
                    .events()
                    .detach_search_events_exclusively()
                    .expect("first search detach succeeds");
            }
            "command" => {
                output
                    .events()
                    .detach_command_events()
                    .expect("first command detach succeeds");
            }
            "context menu" => {
                output
                    .events()
                    .detach_context_menu_events()
                    .expect("first context-menu detach succeeds");
            }
            _ => unreachable!(),
        }

        let error = root
            .finish_output(output)
            .expect_err("the second detach must fail closed");
        let message = error.to_string();
        assert!(message.contains(channel), "unexpected error: {message}");
        assert!(
            message.contains("AlreadyDetached"),
            "unexpected error: {message}"
        );
    }
}
