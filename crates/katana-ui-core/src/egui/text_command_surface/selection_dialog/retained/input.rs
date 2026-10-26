use super::{
    DialogInput, EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, EguiTextSurfaceKey,
    SelectionDialogRetainedState, TextArea, TextAreaEvent, TextAreaWrapPolicy,
    TextCommandSurfaceStyle, TextSurface, TextSurfaceEvent, TextSurfacePaintOperation,
    TextSurfaceProps, TextSurfaceViewport,
};

const INPUT_INITIAL_WIDTH: u32 = 400;
const INPUT_HEIGHT: u32 = 34;

impl SelectionDialogRetainedState {
    pub(super) fn input(
        &mut self,
        ui: &mut egui::Ui,
        key: String,
        label: &str,
        value: &mut String,
        style: &TextCommandSurfaceStyle,
        focus: bool,
        operations: &mut Vec<TextSurfacePaintOperation>,
    ) -> Result<bool, String> {
        let resources = &self.resources;
        let input = self.inputs.entry(key).or_insert_with_key(|key| {
            let area = TextArea::new(label)
                .stable_state_id(key.as_str())
                .value(value.as_str())
                .max_rows(1)
                .min_rows(1)
                .auto_grow(false)
                .wrap_policy(TextAreaWrapPolicy::None)
                .ime_enabled(true);
            DialogInput {
                surface: TextSurface::new(TextSurfaceProps::new(
                    area,
                    Vec::new(),
                    TextSurfaceViewport::new(0, 0, INPUT_INITIAL_WIDTH, INPUT_HEIGHT),
                )),
                adapter: EguiTextSurfaceAdapter::with_resources_and_metrics(
                    resources,
                    std::rc::Rc::new(std::cell::RefCell::new(
                        crate::text_raster::PlatformTextMetricsFrame::new(),
                    )),
                ),
            }
        });
        input.surface.synchronize_value(value.as_str());
        let mut presentation =
            crate::text_surface::TextSurfacePresentation::from_props(input.surface.props());
        presentation.readonly = self.lease.readonly;
        presentation.disabled = self.lease.disabled;
        presentation.accessibility_label = label.to_owned();
        input.surface.synchronize_presentation(presentation);
        if focus {
            input.adapter.request_focus_for_next_frame(true);
        }
        let output = input
            .adapter
            .show_with_input_policy(
                ui,
                &mut input.surface,
                &style.text_raster,
                &style.text_paint,
                &EguiTextSurfaceInputPolicy::default()
                    .suppress(EguiTextSurfaceKey::ArrowUp)
                    .suppress(EguiTextSurfaceKey::ArrowDown)
                    .suppress(EguiTextSurfaceKey::Enter)
                    .suppress(EguiTextSurfaceKey::Escape)
                    .without_context_target(),
            )
            .map_err(|error| error.to_string())?;
        operations.extend(output.artifact.paint_plan.operations);
        let changed = output
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::TextArea(TextAreaEvent::Change(_))));
        *value = input.surface.state().text_area.value.clone();
        Ok(changed)
    }
}
