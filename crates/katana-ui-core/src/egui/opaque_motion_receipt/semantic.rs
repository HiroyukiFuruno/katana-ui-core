use crate::egui::text_command_surface::accesskit_projection::AccessKitTextInputNode;
use crate::egui::text_command_surface::{
    CONTROL_STAR_TEXT, EguiTextCommandSurfaceHostRootFrame, IME_PREEDIT_TEXT, STAR_TEXT,
};

const MOTION_IME_COMMIT_TEXT: &str = "入力";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MotionFrameSemanticEvidence {
    pub(crate) root_record_hash: String,
    pub(crate) star_scalar_sequence: Vec<u32>,
    pub(crate) star_chromatic_pixel_count: usize,
    pub(crate) control_star_chromatic_pixel_count: usize,
    pub(crate) star_hit_test_seen: bool,
    pub(crate) ime_preedit_event_seen: bool,
    pub(crate) ime_commit_event_seen: bool,
    pub(crate) expected_accesskit_text_input_value: String,
    pub(crate) accesskit_text_input_nodes: Vec<AccessKitTextInputNode>,
    pub(crate) accesskit_snapshot_hash: String,
}

pub(super) fn motion_semantics(
    frame: &EguiTextCommandSurfaceHostRootFrame,
) -> MotionFrameSemanticEvidence {
    let output = frame.artifact_output();
    let raster = &output.evidence_text.raster;
    let star = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| raster.text.get(bounds.byte_start..bounds.byte_end) == Some(STAR_TEXT));
    let control_star = raster.grapheme_bounds.iter().find(|bounds| {
        raster.text.get(bounds.byte_start..bounds.byte_end) == Some(CONTROL_STAR_TEXT)
    });
    let star_chromatic_pixel_count = star
        .and_then(|bounds| raster.grapheme_crop(bounds, 1.0))
        .map_or(0, |crop| crop.chromatic_pixel_count());
    let control_star_chromatic_pixel_count = control_star
        .and_then(|bounds| raster.grapheme_crop(bounds, 1.0))
        .map_or(0, |crop| crop.chromatic_pixel_count());
    let star_hit_test_seen = star.is_some_and(|bounds| {
        let query_x = bounds.x + bounds.width / 2.0;
        let query_y = bounds.y + bounds.height / 2.0;
        raster.hit_test(query_x, query_y).is_some_and(|hit| {
            hit.byte_start == bounds.byte_start && hit.byte_end == bounds.byte_end
        })
    });
    let ime_preedit_event_seen = output.evidence_text.events.iter().any(|event| {
        matches!(
            event,
            crate::text_surface::TextSurfaceEvent::TextArea(
                crate::atom::TextAreaEvent::ImeComposition(value)
            ) if value.preedit == IME_PREEDIT_TEXT
        )
    });
    let ime_commit_event_seen = output.evidence_text.events.iter().any(|event| {
        matches!(
            event,
            crate::text_surface::TextSurfaceEvent::TextArea(
                crate::atom::TextAreaEvent::ImeCommit(value)
            ) if value == MOTION_IME_COMMIT_TEXT
        )
    });

    MotionFrameSemanticEvidence {
        root_record_hash: frame.record().record_hash().to_owned(),
        star_scalar_sequence: star
            .map(|_| STAR_TEXT.chars().map(u32::from).collect())
            .unwrap_or_default(),
        star_chromatic_pixel_count,
        control_star_chromatic_pixel_count,
        star_hit_test_seen,
        ime_preedit_event_seen,
        ime_commit_event_seen,
        expected_accesskit_text_input_value: raster.text.clone(),
        accesskit_snapshot_hash: match output.accesskit_text_input_nodes.as_slice() {
            [node] => node.snapshot_hash(),
            _ => String::new(),
        },
        accesskit_text_input_nodes: output.accesskit_text_input_nodes.clone(),
    }
}
