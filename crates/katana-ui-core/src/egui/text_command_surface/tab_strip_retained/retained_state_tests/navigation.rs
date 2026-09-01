use super::*;

#[test]
fn navigation_controls_propagate_real_missing_port_for_previous_and_next_frames() {
    for label in ["previous", "next"] {
        let projection = build_projection_with_navigation();
        let mut state = build_state_with_projection(build_projection_with_navigation());
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut first_result = None;
        let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
            first_result = Some(state.show_projection(ui, &projection, &mut false, 0.0));
        });
        first_result
            .expect("initial navigation frame should execute")
            .expect("initial navigation frame should render");
        let node = accesskit_button(&output, label);
        output.textures_delta.clear();

        let mut observed = None;
        let mut output = context.run_ui(retained_input(vec![accesskit_click(node)]), |ui| {
            observed = Some(state.show_projection(ui, &projection, &mut false, 0.0));
        });
        output.textures_delta.clear();

        assert!(matches!(
            observed.expect("navigation activation frame should execute"),
            Err(TabStripRetainedError::MissingPort)
        ));
    }
}
