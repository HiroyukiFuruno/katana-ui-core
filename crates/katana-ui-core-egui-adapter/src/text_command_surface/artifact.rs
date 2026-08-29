//! Artifact composition mapping for text-command surface output.

use crate::artifact_compositor::ArtifactPaintPlanRef;

use super::types::{EguiTextCommandSurfaceChild, EguiTextCommandSurfaceOutput};

/// Typed rejection for an artifact order that does not match its child outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceArtifactError {
    MissingToolbar,
    MissingSearch,
    MissingSourceAddress,
    MissingTabStrip,
    MissingTabStripOverlay,
    MissingFloating,
    MissingFloatingPaintPlan,
    MissingContextMenu,
    MissingContextMenuPaintPlan,
    MissingStatusBar,
    MissingDiagnosticsList,
}

impl std::fmt::Display for EguiTextCommandSurfaceArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::MissingToolbar => "toolbar child requires a toolbar plan",
            Self::MissingSearch => "search child requires a search plan",
            Self::MissingSourceAddress => "source-address child requires a source-address plan",
            Self::MissingTabStrip => "tab-strip child requires a tab-strip plan",
            Self::MissingTabStripOverlay => "tab-strip overlay requires an overlay plan",
            Self::MissingFloating => "floating child requires a floating output",
            Self::MissingFloatingPaintPlan => "floating child requires a floating paint plan",
            Self::MissingContextMenu => "context-menu child requires a context-menu output",
            Self::MissingContextMenuPaintPlan => {
                "context-menu child requires a context-menu paint plan"
            }
            Self::MissingStatusBar => "status-bar child requires a status-bar paint plan",
            Self::MissingDiagnosticsList => {
                "diagnostics-list child requires a diagnostics-list paint plan"
            }
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for EguiTextCommandSurfaceArtifactError {}

pub(super) struct RootArtifactChildren {
    pub tab_strip: bool,
    pub tab_strip_overlay: bool,
    pub source_address: bool,
    pub toolbar: bool,
    pub toolbar_dropdown_open: bool,
    pub search: bool,
    pub floating_open: bool,
    pub context_menu_open: bool,
    pub status_bar: bool,
    pub diagnostics_list: bool,
}

pub(super) fn artifact_order_for_root(
    children: RootArtifactChildren,
) -> Vec<EguiTextCommandSurfaceChild> {
    let mut order = Vec::new();
    if children.tab_strip {
        order.push(EguiTextCommandSurfaceChild::TabStrip);
    }
    if children.source_address {
        order.push(EguiTextCommandSurfaceChild::SourceAddress);
    }
    if children.toolbar && !children.toolbar_dropdown_open {
        order.push(EguiTextCommandSurfaceChild::Toolbar);
    }
    if children.search {
        order.push(EguiTextCommandSurfaceChild::Search);
    }
    order.push(EguiTextCommandSurfaceChild::Text);
    if children.diagnostics_list {
        order.push(EguiTextCommandSurfaceChild::DiagnosticsList);
    }
    if children.status_bar {
        order.push(EguiTextCommandSurfaceChild::StatusBar);
    }
    if children.toolbar && children.toolbar_dropdown_open {
        /* WHY: the actual dropdown is rendered through egui's foreground Area and must remain
        above the text surface in the root-owned deterministic composite as well. */
        order.push(EguiTextCommandSurfaceChild::Toolbar);
    }
    if children.floating_open {
        order.push(EguiTextCommandSurfaceChild::Floating);
    }
    if children.context_menu_open {
        order.push(EguiTextCommandSurfaceChild::ContextMenu);
    }
    if children.tab_strip_overlay {
        order.push(EguiTextCommandSurfaceChild::TabStripOverlay);
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
                EguiTextCommandSurfaceChild::TabStrip => ArtifactPaintPlanRef::TabStrip(
                    &self
                        .tab_strip
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingTabStrip)?
                        .paint_plan,
                ),
                EguiTextCommandSurfaceChild::TabStripOverlay => ArtifactPaintPlanRef::TabStrip(
                    self.tab_strip
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingTabStrip)?
                        .overlay_paint_plan
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingTabStripOverlay)?,
                ),
                EguiTextCommandSurfaceChild::SourceAddress => ArtifactPaintPlanRef::SourceAddress(
                    &self
                        .source_address
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingSourceAddress)?
                        .paint_plan,
                ),
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
                EguiTextCommandSurfaceChild::StatusBar => ArtifactPaintPlanRef::StatusBar(
                    &self
                        .status_bar
                        .as_ref()
                        .ok_or(EguiTextCommandSurfaceArtifactError::MissingStatusBar)?
                        .paint_plan,
                ),
                EguiTextCommandSurfaceChild::DiagnosticsList => {
                    ArtifactPaintPlanRef::DiagnosticsList(
                        &self
                            .diagnostics_list
                            .as_ref()
                            .ok_or(EguiTextCommandSurfaceArtifactError::MissingDiagnosticsList)?
                            .paint_plan,
                    )
                }
            });
        }
        Ok(plans)
    }
}
