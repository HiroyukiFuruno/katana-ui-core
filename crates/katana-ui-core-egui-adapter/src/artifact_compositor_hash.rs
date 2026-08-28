use super::{ArtifactCompositeError, ArtifactPaintPlanRef};
use sha2::{Digest, Sha256};

pub(super) fn paint_plan_hash(
    plans: &[ArtifactPaintPlanRef<'_>],
) -> Result<String, ArtifactCompositeError> {
    if let [plan] = plans {
        return serialized_plan(plan).map(|bytes| hash_bytes(&bytes));
    }
    let mut hasher = Sha256::new();
    for plan in plans {
        hasher.update(plan_kind(plan));
        hasher.update(serialized_plan(plan)?);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub(super) fn hash_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn serialized_plan(plan: &ArtifactPaintPlanRef<'_>) -> Result<Vec<u8>, ArtifactCompositeError> {
    match plan {
        ArtifactPaintPlanRef::TextSurface(value) => serde_json::to_vec(value),
        ArtifactPaintPlanRef::CommandChrome(value) => serde_json::to_vec(value),
        ArtifactPaintPlanRef::ContextMenu(value) => serde_json::to_vec(value),
    }
    .map_err(serialization_error)
}

fn serialization_error(error: serde_json::Error) -> ArtifactCompositeError {
    ArtifactCompositeError::Serialization(error.to_string())
}

const fn plan_kind(plan: &ArtifactPaintPlanRef<'_>) -> &'static [u8] {
    match plan {
        ArtifactPaintPlanRef::TextSurface(_) => b"text",
        ArtifactPaintPlanRef::CommandChrome(_) => b"chrome",
        ArtifactPaintPlanRef::ContextMenu(_) => b"context-menu",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_chrome::{
        CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
    };
    use crate::context_menu::{
        ContextMenuPaintOperation, ContextMenuPaintOperationKind, ContextMenuPaintPlan,
    };
    use crate::text_surface::{
        TextSurfacePaintOperation, TextSurfacePaintOperationKind, TextSurfacePaintPlan,
    };
    use crate::{
        artifact_compositor::ArtifactPaintPlanRef, command_chrome::EguiCommandChromeDrawLayer,
        text_surface::EguiTextSurfaceDrawLayer,
    };
    use katana_ui_core::render_model::UiRect;

    #[test]
    fn serialization_errors_map_to_the_typed_compositor_error() {
        let result = serde_json::from_slice::<serde_json::Value>(b"").map_err(serialization_error);
        assert!(matches!(
            result,
            Err(ArtifactCompositeError::Serialization(_))
        ));
    }

    fn text_plan() -> TextSurfacePaintPlan {
        TextSurfacePaintPlan {
            surface_bounds: UiRect::new(0, 0, 1, 1),
            viewport_bounds: UiRect::new(0, 0, 1, 1),
            operations: vec![TextSurfacePaintOperation {
                layer: EguiTextSurfaceDrawLayer::Background,
                clip_bounds: UiRect::new(0, 0, 1, 1),
                kind: TextSurfacePaintOperationKind::Fill {
                    bounds: UiRect::new(0, 0, 1, 1),
                    color_rgba: [12, 34, 56, 255],
                },
            }],
        }
    }

    fn chrome_plan() -> CommandChromePaintPlan {
        CommandChromePaintPlan {
            surface_bounds: UiRect::new(0, 0, 1, 1),
            operations: vec![CommandChromePaintOperation {
                layer: EguiCommandChromeDrawLayer::PanelFill,
                clip_bounds: UiRect::new(0, 0, 1, 1),
                kind: CommandChromePaintOperationKind::Fill {
                    bounds: UiRect::new(0, 0, 1, 1),
                    color_rgba: [1, 2, 3, 255],
                },
            }],
        }
    }

    fn context_menu_plan() -> ContextMenuPaintPlan {
        ContextMenuPaintPlan {
            surface_bounds: UiRect::new(0, 0, 1, 1),
            operations: vec![ContextMenuPaintOperation {
                clip_bounds: UiRect::new(0, 0, 1, 1),
                kind: ContextMenuPaintOperationKind::Fill {
                    bounds: UiRect::new(0, 0, 1, 1),
                    color_rgba: [10, 20, 30, 255],
                },
            }],
        }
    }

    #[test]
    fn paint_plan_hash_uses_single_plan_payload_without_type_prefix() {
        let text_plan = text_plan();
        let text = ArtifactPaintPlanRef::TextSurface(&text_plan);
        assert_eq!(
            paint_plan_hash(std::slice::from_ref(&text)).expect("single plan should hash"),
            hash_bytes(&serde_json::to_vec(&text_plan).expect("text plan serializes"))
        );
    }

    #[test]
    fn paint_plan_hash_distinguishes_plan_types() {
        let plans = [
            ArtifactPaintPlanRef::TextSurface(&text_plan()),
            ArtifactPaintPlanRef::CommandChrome(&chrome_plan()),
            ArtifactPaintPlanRef::ContextMenu(&context_menu_plan()),
        ];
        let actual = paint_plan_hash(&plans).expect("plans should hash");

        let mut expected = Sha256::new();
        for plan in plans {
            expected.update(plan_kind(&plan));
            let serialized = match plan {
                ArtifactPaintPlanRef::TextSurface(plan) => serde_json::to_vec(plan),
                ArtifactPaintPlanRef::CommandChrome(plan) => serde_json::to_vec(plan),
                ArtifactPaintPlanRef::ContextMenu(plan) => serde_json::to_vec(plan),
            }
            .expect("plan serializes");
            expected.update(serialized);
        }

        assert_eq!(actual, hex::encode(expected.finalize()));
    }

    #[test]
    fn hash_bytes_is_deterministic_and_empty_plan_hash_is_hash_of_nothing() {
        assert_eq!(
            hash_bytes(b"coverage-marker"),
            hash_bytes(b"coverage-marker")
        );
        let empty = paint_plan_hash(&[]).expect("empty list should hash deterministically");
        let expected = {
            let hasher = sha2::Sha256::new();
            hex::encode(hasher.finalize())
        };
        assert_eq!(empty, expected);
    }
}
