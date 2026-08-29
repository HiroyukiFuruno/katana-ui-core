use super::super::command_chrome_fixture::{FRAME_HEIGHT, FRAME_WIDTH};
use super::super::text_surface_fixture::{
    paint_style as text_paint_style, raster_style as text_raster_style,
};
use super::{context_menu_paint_style, context_menu_raster_style};
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use katana_ui_core_egui_adapter::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPaintOperationKind, EguiContextMenuAdapter, EguiContextMenuFrameRecord,
    EguiContextMenuOutput,
};
use katana_ui_core_egui_adapter::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceOutput, TextSurfaceContextTargetAnchor,
};
use std::io;

const TEXT_TARGET_X: f32 = 940.0;
const TEXT_TARGET_Y: f32 = 260.0;
const ALPHA_CHANNEL: usize = RGBA_CHANNEL_COUNT - 1;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ContextMenuEvidence {
    pub(super) pointer_clamped: bool,
    pub(super) composite_hash: String,
    pub(super) plan_hash: String,
    pub(super) frame_hash: String,
    pub(super) colored_star_texture: bool,
    pub(super) accesskit_labels: Vec<String>,
}

pub(super) fn pointer_anchor(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut katana_ui_core::text_surface::TextSurface,
) -> Result<TextSurfaceContextTargetAnchor, Box<dyn std::error::Error>> {
    let _ = run_text_frame(context, adapter, surface, Vec::new())?;
    let primary_press = run_text_frame(
        context,
        adapter,
        surface,
        vec![pointer_event(
            TEXT_TARGET_X,
            TEXT_TARGET_Y,
            egui::PointerButton::Primary,
            true,
        )],
    );
    let _ = primary_press?;
    let primary_release = run_text_frame(
        context,
        adapter,
        surface,
        vec![pointer_event(
            TEXT_TARGET_X,
            TEXT_TARGET_Y,
            egui::PointerButton::Primary,
            false,
        )],
    );
    let _ = primary_release?;
    let secondary_press = run_text_frame(
        context,
        adapter,
        surface,
        vec![pointer_event(
            TEXT_TARGET_X,
            TEXT_TARGET_Y,
            egui::PointerButton::Secondary,
            true,
        )],
    );
    let _ = secondary_press?;
    let secondary_release = run_text_frame(
        context,
        adapter,
        surface,
        vec![pointer_event(
            TEXT_TARGET_X,
            TEXT_TARGET_Y,
            egui::PointerButton::Secondary,
            false,
        )],
    );
    let output = secondary_release?;
    let anchor = output.0.context_target.ok_or(io::Error::other(
        "actual secondary-click did not produce an anchor",
    ));
    Ok(anchor?)
}

pub(super) fn run_text_frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut katana_ui_core::text_surface::TextSurface,
    events: Vec<egui::Event>,
) -> Result<(EguiTextSurfaceOutput, egui::FullOutput), Box<dyn std::error::Error>> {
    let mut output = None;
    let mut full = context.run_ui(raw_input(events), |ui| {
        output = Some(adapter.show(ui, surface, &text_raster_style(), &text_paint_style()));
    });
    full.textures_delta.clear();
    let output = output.ok_or(io::Error::other(
        "egui did not execute the TextSurface UI closure",
    ));
    let output = output?;
    Ok((output?, full))
}

pub(super) fn run_combined_frame(
    context: &egui::Context,
    text_adapter: &mut EguiTextSurfaceAdapter,
    text_surface: &mut katana_ui_core::text_surface::TextSurface,
    menu_adapter: &mut EguiContextMenuAdapter,
    events: Vec<egui::Event>,
) -> Result<
    (
        EguiTextSurfaceOutput,
        EguiContextMenuOutput,
        egui::FullOutput,
    ),
    Box<dyn std::error::Error>,
> {
    let mut output = None;
    let mut full = context.run_ui(raw_input(events), |ui| {
        output = Some((
            text_adapter.show(ui, text_surface, &text_raster_style(), &text_paint_style()),
            menu_adapter.show(
                ui,
                &context_menu_raster_style(),
                &context_menu_paint_style(),
            ),
        ));
    });
    full.textures_delta.clear();
    let output = output.ok_or(io::Error::other(
        "egui did not execute the combined UI closure",
    ));
    let (text, menu) = output?;
    Ok((text?, menu?, full))
}

pub(super) fn compose_evidence(
    output: &(
        EguiTextSurfaceOutput,
        EguiContextMenuOutput,
        egui::FullOutput,
    ),
    record: &EguiContextMenuFrameRecord,
) -> Result<ContextMenuEvidence, Box<dyn std::error::Error>> {
    let menu = output
        .1
        .artifact
        .as_ref()
        .ok_or(io::Error::other("ContextMenu artifact was absent"))?;
    let plans = [
        ArtifactPaintPlanRef::TextSurface(&output.0.artifact.paint_plan),
        ArtifactPaintPlanRef::ContextMenu(&menu.paint_plan),
    ];
    let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            0,
            0,
            FRAME_WIDTH as u32,
            FRAME_HEIGHT as u32,
        )),
        plans: &plans,
    })?;
    Ok(ContextMenuEvidence {
        pointer_clamped: record.bounds.x < TEXT_TARGET_X as i32
            && record.bounds.y < TEXT_TARGET_Y as i32,
        composite_hash: composite.pixel_hash,
        plan_hash: menu.paint_plan_hash.clone(),
        frame_hash: menu.frame_record_hash.clone(),
        colored_star_texture: menu.paint_plan.operations.iter().any(|operation| {
            let ContextMenuPaintOperationKind::Texture { texture, .. } = &operation.kind else {
                return false;
            };
            texture.identity.contains("⭐️")
                && texture
                    .rgba_pixels
                    .as_chunks::<RGBA_CHANNEL_COUNT>()
                    .0
                    .iter()
                    .any(|pixel| {
                        pixel[ALPHA_CHANNEL] > 0 && (pixel[0] != pixel[1] || pixel[1] != pixel[2])
                    })
        }),
        accesskit_labels: accesskit_labels(&output.2),
    })
}

pub(super) fn item_bounds(
    record: &EguiContextMenuFrameRecord,
    id: &str,
) -> Result<UiRect, io::Error> {
    record
        .items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.bounds)
        .ok_or_else(|| io::Error::other(format!("actual menu item `{id}` was absent")))
}

pub(super) fn text_root_id(
    output: &egui::FullOutput,
) -> Result<egui::accesskit::NodeId, io::Error> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .into_iter()
        .flat_map(|update| update.nodes.iter())
        .find_map(|(id, node)| {
            (node.role() == egui::accesskit::Role::MultilineTextInput).then_some(*id)
        })
        .ok_or(io::Error::other("TextSurface AccessKit root was absent"))
}

pub(super) fn pointer_from_bounds(bounds: UiRect, pressed: bool) -> egui::Event {
    pointer_event(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
        egui::PointerButton::Primary,
        pressed,
    )
}

pub(super) fn pointer_event(
    x: f32,
    y: f32,
    button: egui::PointerButton,
    pressed: bool,
) -> egui::Event {
    egui::Event::PointerButton {
        pos: egui::pos2(x, y),
        button,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

pub(super) fn shift_f10_event() -> egui::Event {
    egui::Event::Key {
        key: egui::Key::F10,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers {
            shift: true,
            ..egui::Modifiers::default()
        },
    }
}

pub(super) fn accesskit_context_event(root_id: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::ShowContextMenu,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: root_id,
        data: None,
    })
}

pub(super) fn escape_event() -> egui::Event {
    egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

pub(super) fn accesskit_labels(output: &egui::FullOutput) -> Vec<String> {
    let mut labels = output
        .platform_output
        .accesskit_update
        .as_ref()
        .into_iter()
        .flat_map(|update| update.nodes.iter())
        .filter_map(|(_, node)| node.label().map(ToOwned::to_owned))
        .collect::<Vec<_>>();
    labels.sort();
    labels.dedup();
    labels
}

fn raw_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(FRAME_WIDTH, FRAME_HEIGHT),
        )),
        events,
        ..egui::RawInput::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::UiRect;

    #[test]
    fn item_bounds_rejects_unknown_item_id() {
        let record = katana_ui_core_egui_adapter::context_menu::EguiContextMenuFrameRecord {
            bounds: UiRect::new(0, 0, 64, 64),
            viewport_bounds: UiRect::new(0, 0, 64, 64),
            highlighted_path: Vec::new(),
            focused: false,
            items: Vec::new(),
        };

        let error = item_bounds(&record, "missing");
        assert_eq!(
            Some(io::ErrorKind::Other),
            error.as_ref().err().map(io::Error::kind)
        );
        assert!(error.is_err_and(|error| {
            error
                .to_string()
                .contains("actual menu item `missing` was absent")
        }));
    }

    #[test]
    fn item_bounds_returns_matching_bounds() -> Result<(), io::Error> {
        let record = katana_ui_core_egui_adapter::context_menu::EguiContextMenuFrameRecord {
            bounds: UiRect::new(0, 0, 64, 64),
            viewport_bounds: UiRect::new(0, 0, 64, 64),
            highlighted_path: Vec::new(),
            focused: false,
            items: vec![
                katana_ui_core_egui_adapter::context_menu::EguiContextMenuItemFrame {
                    id: "target".to_string(),
                    bounds: UiRect::new(4, 8, 16, 12),
                    disabled: false,
                    checked: false,
                },
            ],
        };
        let bounds = item_bounds(&record, "target")?;
        assert_eq!(UiRect::new(4, 8, 16, 12), bounds);
        Ok(())
    }

    #[test]
    fn accesskit_labels_returns_sorted_unique_labels() {
        let mut first = egui::accesskit::Node::new(egui::accesskit::Role::Button);
        first.set_label("一重");
        let mut second = egui::accesskit::Node::new(egui::accesskit::Role::Button);
        second.set_label("二重");
        let update = egui::accesskit::TreeUpdate {
            nodes: vec![
                (egui::accesskit::NodeId(2), second),
                (egui::accesskit::NodeId(1), first),
            ],
            tree: None,
            tree_id: egui::accesskit::TreeId::ROOT,
            focus: egui::accesskit::NodeId(1),
        };
        let mut output = egui::FullOutput::default();
        output.platform_output.accesskit_update = Some(update);

        let labels = accesskit_labels(&output);
        assert_eq!(vec!["一重".to_string(), "二重".to_string()], labels);
    }

    #[test]
    fn accesskit_labels_is_empty_when_update_is_absent() {
        let output = egui::FullOutput::default();
        assert!(accesskit_labels(&output).is_empty());
    }

    #[test]
    fn text_root_id_selects_multiline_text_input_node() -> Result<(), io::Error> {
        let update = egui::accesskit::TreeUpdate {
            nodes: vec![
                (
                    egui::accesskit::NodeId(1),
                    egui::accesskit::Node::new(egui::accesskit::Role::Button),
                ),
                (
                    egui::accesskit::NodeId(2),
                    egui::accesskit::Node::new(egui::accesskit::Role::MultilineTextInput),
                ),
            ],
            tree: None,
            tree_id: egui::accesskit::TreeId::ROOT,
            focus: egui::accesskit::NodeId(1),
        };

        let mut output = egui::FullOutput::default();
        output.platform_output.accesskit_update = Some(update);
        assert_eq!(egui::accesskit::NodeId(2), text_root_id(&output)?);
        Ok(())
    }
}
