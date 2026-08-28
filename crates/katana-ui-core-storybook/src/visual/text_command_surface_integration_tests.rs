#[path = "text_command_surface_integration_tests/assertions.rs"]
mod assertions;
#[path = "text_command_surface_integration_tests/facts.rs"]
mod facts;
#[path = "text_command_surface_integration_tests/harness.rs"]
mod harness;
#[path = "text_command_surface_integration_tests/scenario.rs"]
mod scenario;

/* WHY: EguiTextCommandSurfaceAdapter egui::RawInput actual_egui_text_command_surface_keeps_all_children_inside_root_repeatably ⭐️ assert_artifact_output_contract expected_artifact_order assert_inside */

#[test]
fn actual_egui_text_command_surface_keeps_all_children_inside_root_repeatably()
-> Result<(), Box<dyn std::error::Error>> {
    scenario::TextCommandSurfaceIntegrationScenario::actual_egui_text_command_surface_keeps_all_children_inside_root_repeatably()
}
