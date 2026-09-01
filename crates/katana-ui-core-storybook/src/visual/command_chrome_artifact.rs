use katana_ui_core::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core::egui::command_chrome::CommandChromePaintPlan;
use katana_ui_core::render_model::UiRect;

pub(super) const RGBA_CHANNELS: usize = 4;
const ALPHA_CHANNEL: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CommandChromePlanPixels {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgba: Vec<u8>,
    pub(super) paint_plan_hash: String,
    pub(super) pixel_hash: String,
}

pub(super) fn render_command_chrome_plan(
    plan: &CommandChromePaintPlan,
    canvas: UiRect,
) -> Result<CommandChromePlanPixels, String> {
    let plans = [ArtifactPaintPlanRef::CommandChrome(plan)];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(canvas),
        plans: &plans,
    })
    .map_err(|error| error.to_string())?;
    Ok(CommandChromePlanPixels {
        width: canvas.width,
        height: canvas.height,
        rgba: frame.rgba_pixels,
        paint_plan_hash: frame.paint_plan_hash,
        pixel_hash: frame.pixel_hash,
    })
}

pub(super) fn paint_plan_has_star_variation_selector(plan: &CommandChromePaintPlan) -> bool {
    plan.operations.iter().any(|operation| {
        matches!(
            &operation.kind,
            katana_ui_core::egui::command_chrome::CommandChromePaintOperationKind::Texture { texture, .. }
                if texture.identity.contains("⭐️")
        )
    })
}

pub(super) fn paint_plan_has_colored_star_texture(plan: &CommandChromePaintPlan) -> bool {
    plan.operations.iter().any(|operation| {
        let katana_ui_core::egui::command_chrome::CommandChromePaintOperationKind::Texture {
            texture,
            ..
        } = &operation.kind
        else {
            return false;
        };
        texture.identity.contains("⭐️")
            && texture
                .rgba_pixels
                .as_chunks::<RGBA_CHANNELS>()
                .0
                .iter()
                .any(|rgba| rgba[ALPHA_CHANNEL] > 0 && (rgba[0] != rgba[1] || rgba[1] != rgba[2]))
    })
}

pub(super) fn texture_identities(plan: &CommandChromePaintPlan) -> Vec<String> {
    plan.operations.iter().filter_map(|operation| match &operation.kind {
        katana_ui_core::egui::command_chrome::CommandChromePaintOperationKind::Texture { texture, .. } => Some(texture.identity.clone()),
        katana_ui_core::egui::command_chrome::CommandChromePaintOperationKind::Fill { .. }
        | katana_ui_core::egui::command_chrome::CommandChromePaintOperationKind::RoundedFill { .. } => None,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        RGBA_CHANNELS, paint_plan_has_colored_star_texture, paint_plan_has_star_variation_selector,
        render_command_chrome_plan, texture_identities,
    };
    use katana_ui_core::egui::command_chrome::{
        CommandChromePaintOperation, CommandChromePaintOperationKind::Fill,
        CommandChromePaintOperationKind::Texture, CommandChromePaintPlan,
        CommandChromePaintTexture, EguiCommandChromeDrawLayer,
    };
    use katana_ui_core::render_model::UiRect;

    const CANVAS: UiRect = UiRect::new(0, 0, 16, 16);

    #[test]
    fn paint_plan_feature_checks_detect_star_variation_states() {
        let plain = solid_fill_plan([1, 2, 3, 4]);
        let uncolored = textured_plan("⭐️", [10, 10, 10, 255]);
        let colored = textured_plan("⭐️", [10, 11, 12, 255]);

        assert!(!paint_plan_has_star_variation_selector(&plain));
        assert!(paint_plan_has_star_variation_selector(&uncolored));
        assert!(paint_plan_has_star_variation_selector(&colored));
        assert!(!paint_plan_has_colored_star_texture(&uncolored));
        assert!(paint_plan_has_colored_star_texture(&colored));
    }

    #[test]
    fn command_chrome_plan_renders_and_records_hashes() -> Result<(), String> {
        let plan = solid_fill_plan([255, 0, 0, 255]);
        let pixels = render_command_chrome_plan(&plan, CANVAS)?;

        assert_eq!(CANVAS.width, pixels.width);
        assert_eq!(CANVAS.height, pixels.height);
        assert!(!pixels.rgba.is_empty());
        assert!(!pixels.pixel_hash.is_empty());
        assert!(!pixels.paint_plan_hash.is_empty());
        assert_eq!(
            pixels.rgba.len(),
            (CANVAS.width as usize) * (CANVAS.height as usize) * RGBA_CHANNELS
        );
        Ok(())
    }

    #[test]
    fn render_command_chrome_plan_rejects_invalid_texture_pixels() {
        let invalid = textured_plan_with_invalid_pixels();
        assert!(render_command_chrome_plan(&invalid, CANVAS).is_err());
    }

    #[test]
    fn texture_identities_collects_only_texture_operations() {
        let plan = CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![
                CommandChromePaintOperation {
                    layer: EguiCommandChromeDrawLayer::PanelFill,
                    clip_bounds: CANVAS,
                    kind: Fill {
                        bounds: CANVAS,
                        color_rgba: [1, 2, 3, 4],
                    },
                },
                CommandChromePaintOperation {
                    layer: EguiCommandChromeDrawLayer::PanelFill,
                    clip_bounds: CANVAS,
                    kind: Texture {
                        bounds: CANVAS,
                        texture: CommandChromePaintTexture {
                            identity: "keep-me".into(),
                            width: 1,
                            height: 1,
                            rgba_pixels: vec![1, 2, 3, 4],
                        },
                    },
                },
            ],
        };
        assert_eq!(texture_identities(&plan), vec!["keep-me".to_string()]);
    }

    fn solid_fill_plan(color: [u8; RGBA_CHANNELS]) -> CommandChromePaintPlan {
        CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![CommandChromePaintOperation {
                layer: EguiCommandChromeDrawLayer::PanelFill,
                clip_bounds: UiRect::new(0, 0, CANVAS.width, CANVAS.height),
                kind: Fill {
                    bounds: UiRect::new(0, 0, CANVAS.width, CANVAS.height),
                    color_rgba: color,
                },
            }],
        }
    }

    fn textured_plan(identity: &str, color: [u8; RGBA_CHANNELS]) -> CommandChromePaintPlan {
        CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![CommandChromePaintOperation {
                layer: EguiCommandChromeDrawLayer::IconTexture,
                clip_bounds: UiRect::new(0, 0, CANVAS.width, CANVAS.height),
                kind: Texture {
                    bounds: UiRect::new(0, 0, 1, 1),
                    texture: CommandChromePaintTexture {
                        identity: identity.to_string(),
                        width: 1,
                        height: 1,
                        rgba_pixels: color.to_vec(),
                    },
                },
            }],
        }
    }

    fn textured_plan_with_invalid_pixels() -> CommandChromePaintPlan {
        CommandChromePaintPlan {
            surface_bounds: CANVAS,
            operations: vec![CommandChromePaintOperation {
                layer: EguiCommandChromeDrawLayer::IconTexture,
                clip_bounds: UiRect::new(0, 0, CANVAS.width, CANVAS.height),
                kind: Texture {
                    bounds: UiRect::new(0, 0, 2, 2),
                    texture: CommandChromePaintTexture {
                        identity: "broken".into(),
                        width: 2,
                        height: 2,
                        rgba_pixels: vec![0; 3],
                    },
                },
            }],
        }
    }
}
