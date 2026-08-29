mod repeatable_children;

pub(crate) struct TextCommandSurfaceIntegrationScenario;

impl TextCommandSurfaceIntegrationScenario {
    pub(crate) fn actual_egui_text_command_surface_keeps_all_children_inside_root_repeatably()
    -> Result<(), Box<dyn std::error::Error>> {
        repeatable_children::RepeatableChildrenScenario::run()
    }
}
