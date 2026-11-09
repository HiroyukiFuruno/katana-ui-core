use super::super::super::text_command_surface_style_factory::INVALID_INPUT_ROLE;
use super::{
    DialogInput, EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, EguiTextSurfaceKey,
    SelectionDialogRetainedState, TextArea, TextAreaEvent, TextAreaWrapPolicy,
    TextCommandSurfaceStyle, TextSurface, TextSurfaceEvent, TextSurfacePaintOperation,
    TextSurfaceProps, TextSurfaceViewport,
};

use crate::egui::text_surface::EguiTextSurfaceDrawLayer;
use crate::render_model::UiRect;

const INPUT_INITIAL_WIDTH: u32 = 400;
const INPUT_HEIGHT: u32 = 34;
const INVALID_UNDERLINE_HEIGHT: f32 = 2.0;

pub(super) struct InputPresentation<'a> {
    pub(super) focus: bool,
    pub(super) placeholder: &'a str,
    pub(super) invalid: bool,
}

impl SelectionDialogRetainedState {
    pub(super) fn input_presented(
        &mut self,
        ui: &mut egui::Ui,
        key: String,
        label: &str,
        value: &mut String,
        style: &TextCommandSurfaceStyle,
        field_presentation: InputPresentation<'_>,
        operations: &mut Vec<TextSurfacePaintOperation>,
    ) -> Result<bool, String> {
        self.input_ordered_presented(
            ui,
            key,
            label,
            value,
            style,
            field_presentation,
            operations,
            None,
        )
    }

    pub(super) fn input_ordered_presented(
        &mut self,
        ui: &mut egui::Ui,
        key: String,
        label: &str,
        value: &mut String,
        style: &TextCommandSurfaceStyle,
        field_presentation: InputPresentation<'_>,
        operations: &mut Vec<TextSurfacePaintOperation>,
        ordered: Option<&mut dyn crate::egui::text_surface::OrderedKeyboardInput>,
    ) -> Result<bool, String> {
        let resources = &self.resources;
        let input = self.inputs.entry(key).or_insert_with_key(|key| {
            let area = TextArea::new(label)
                .stable_state_id(key.as_str())
                .value(value.as_str())
                .placeholder(field_presentation.placeholder)
                .invalid(field_presentation.invalid)
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
        input.surface.synchronize_field_presentation(
            field_presentation.placeholder,
            field_presentation.invalid,
        );
        let mut presentation =
            crate::text_surface::TextSurfacePresentation::from_props(input.surface.props());
        presentation.readonly = self.lease.readonly;
        presentation.disabled = self.lease.disabled;
        presentation.accessibility_label = label.to_owned();
        input.surface.synchronize_presentation(presentation);
        if field_presentation.focus {
            input.adapter.request_focus_for_next_frame(true);
        }
        let policy = EguiTextSurfaceInputPolicy::default()
            .suppress(EguiTextSurfaceKey::ArrowUp)
            .suppress(EguiTextSurfaceKey::ArrowDown)
            .suppress(EguiTextSurfaceKey::Enter)
            .suppress(EguiTextSurfaceKey::Escape)
            .without_context_target();
        let output = if let Some(ordered) = ordered {
            input.adapter.show_with_ordered_input(
                ui,
                &mut input.surface,
                &style.text_raster,
                &style.text_paint,
                &policy,
                ordered,
            )
        } else {
            input.adapter.show_with_input_policy(
                ui,
                &mut input.surface,
                &style.text_raster,
                &style.text_paint,
                &policy,
            )
        }
        .map_err(|error| error.to_string())?;
        let field_bounds = output.artifact.paint_plan.surface_bounds;
        operations.extend(output.artifact.paint_plan.operations);
        if field_presentation.invalid {
            operations.push(paint_invalid_field(ui, field_bounds, style));
        }
        let changed = output
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::TextArea(TextAreaEvent::Change(_))));
        *value = input.surface.state().text_area.value.clone();
        Ok(changed)
    }
}

fn paint_invalid_field(
    ui: &egui::Ui,
    field: UiRect,
    style: &TextCommandSurfaceStyle,
) -> TextSurfacePaintOperation {
    let field_bounds = egui::Rect::from_min_size(
        egui::pos2(field.x as f32, field.y as f32),
        egui::vec2(field.width as f32, field.height as f32),
    );
    let bounds = egui::Rect::from_min_max(
        egui::pos2(
            field_bounds.min.x,
            (field_bounds.max.y - INVALID_UNDERLINE_HEIGHT).max(field_bounds.min.y),
        ),
        field_bounds.max,
    );
    let clip = field_bounds.intersect(ui.clip_rect());
    let [r, g, b, a] = style
        .search
        .input_paint
        .annotation_color(INVALID_INPUT_ROLE);
    ui.painter().with_clip_rect(clip).rect_filled(
        bounds,
        0.0,
        egui::Color32::from_rgba_unmultiplied(r, g, b, a),
    );
    let mut operation = super::fill(bounds, clip, [r, g, b, a]);
    operation.layer = EguiTextSurfaceDrawLayer::Annotation;
    operation
}
