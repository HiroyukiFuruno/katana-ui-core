//! Artifact composition mapping for text-command surface output.

use crate::artifact_compositor::ArtifactPaintPlanRef;

use super::types::{EguiTextCommandSurfaceChild, EguiTextCommandSurfaceOutput};

/// Typed rejection for an artifact order that does not match its child outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceArtifactError {
    MissingToolbar,
    MissingSearch,
    MissingFloating,
    MissingFloatingPaintPlan,
    MissingContextMenu,
    MissingContextMenuPaintPlan,
}

impl std::fmt::Display for EguiTextCommandSurfaceArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::MissingToolbar => "toolbar child requires a toolbar plan",
            Self::MissingSearch => "search child requires a search plan",
            Self::MissingFloating => "floating child requires a floating output",
            Self::MissingFloatingPaintPlan => "floating child requires a floating paint plan",
            Self::MissingContextMenu => "context-menu child requires a context-menu output",
            Self::MissingContextMenuPaintPlan => {
                "context-menu child requires a context-menu paint plan"
            }
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for EguiTextCommandSurfaceArtifactError {}

pub(super) fn artifact_order_for_root(
    toolbar: bool,
    search: bool,
    floating_open: bool,
    context_menu_open: bool,
) -> Vec<EguiTextCommandSurfaceChild> {
    let mut order = Vec::new();
    if toolbar {
        order.push(EguiTextCommandSurfaceChild::Toolbar);
    }
    if search {
        order.push(EguiTextCommandSurfaceChild::Search);
    }
    order.push(EguiTextCommandSurfaceChild::Text);
    if floating_open {
        order.push(EguiTextCommandSurfaceChild::Floating);
    }
    if context_menu_open {
        order.push(EguiTextCommandSurfaceChild::ContextMenu);
    }
    order
}

impl EguiTextCommandSurfaceOutput {
    /// Keep artifact output order in lockstep with visible children.
    pub fn artifact_paint_plans(
        &self,
    ) -> Result<Vec<ArtifactPaintPlanRef<'_>>, EguiTextCommandSurfaceArtifactError> {
        let mut plans = Vec::with_capacity(self.artifact_order().len());
        for child in self.artifact_order() {
            plans.push(match child {
                EguiTextCommandSurfaceChild::Text => {
                    ArtifactPaintPlanRef::TextSurface(&self.text.artifact.paint_plan)
                }
                EguiTextCommandSurfaceChild::Toolbar => ArtifactPaintPlanRef::CommandChrome(
                    &self
                        .toolbar
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingToolbar)?
                        .artifact
                        .paint_plan,
                ),
                EguiTextCommandSurfaceChild::Search => ArtifactPaintPlanRef::CommandChrome(
                    &self
                        .search
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingSearch)?
                        .artifact
                        .paint_plan,
                ),
                EguiTextCommandSurfaceChild::Floating => {
                    let floating = self
                        .floating
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingFloating)?;
                    let artifact = floating
                        .artifact
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingFloatingPaintPlan)?;
                    ArtifactPaintPlanRef::CommandChrome(&artifact.paint_plan)
                }
                EguiTextCommandSurfaceChild::ContextMenu => {
                    let context_menu = self
                        .context_menu
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingContextMenu)?;
                    let artifact = context_menu
                        .artifact
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingContextMenuPaintPlan)?;
                    ArtifactPaintPlanRef::ContextMenu(&artifact.paint_plan)
                }
            });
        }
        Ok(plans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_error_display_covers_each_missing_child_fact() {
        let cases = [
            EguiTextCommandSurfaceArtifactError::MissingToolbar,
            EguiTextCommandSurfaceArtifactError::MissingSearch,
            EguiTextCommandSurfaceArtifactError::MissingFloating,
            EguiTextCommandSurfaceArtifactError::MissingFloatingPaintPlan,
            EguiTextCommandSurfaceArtifactError::MissingContextMenu,
            EguiTextCommandSurfaceArtifactError::MissingContextMenuPaintPlan,
        ];
        for error in cases {
            assert!(error.to_string().contains("requires"));
            let _: &dyn std::error::Error = &error;
        }
    }
}
