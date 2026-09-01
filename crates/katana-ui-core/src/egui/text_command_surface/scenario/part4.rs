#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionMotionTransition {
    None,
    Begin,
    Advance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FindMotionTransition {
    None,
    Begin,
    Advance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DropdownMotionTransition {
    None,
    BeginTrigger,
    Advance,
    BeginItem,
}

const DEFAULT_SCENARIO_REVISION: u64 = 1;
const WORKSPACE_PREVIEW_DIMENSION: u32 = 2;
const WORKSPACE_PREVIEW_DISPLAY_DIMENSION: u32 = 0;
const WORKSPACE_PREVIEW_CONTENT_SCALE: u32 = 100;
const WORKSPACE_PREVIEW_RGBA: &[u8] = &[
    36, 42, 54, 255, 74, 85, 104, 255, 74, 85, 104, 255, 36, 42, 54, 255,
];

mod scenario_factory;

pub use scenario_factory::FullTextCommandSurfaceScenarioFactory;

pub(super) fn issue_lease<R>(
    id: FullTextCommandSurfaceScenarioId,
    presentation: EguiTextCommandSurfacePresentation,
    router: R,
) -> Result<EguiTextCommandSurfaceHostProjectionLease, FullTextCommandSurfaceScenarioError>
where
    R: super::KucRootEffectRouter + 'static,
{
    issue_lease_at_revision(id, presentation, DEFAULT_SCENARIO_REVISION, router)
}

pub(super) fn issue_lease_at_revision<R>(
    id: FullTextCommandSurfaceScenarioId,
    presentation: EguiTextCommandSurfacePresentation,
    revision: u64,
    router: R,
) -> Result<EguiTextCommandSurfaceHostProjectionLease, FullTextCommandSurfaceScenarioError>
where
    R: super::KucRootEffectRouter + 'static,
{
    let families = EguiTextCommandSurfaceCommandFamilyProjection::new(
        Some(
            crate::molecule::command_chrome::CommandChromeFamilyId::new(
                "kuc-scenario-primary",
            ),
        ),
        Some(
            crate::molecule::command_chrome::CommandChromeFamilyId::new(
                "kuc-scenario-floating",
            ),
        ),
    );
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        revision,
        format!("kuc-scenario-{id:?}"),
        presentation,
        scenario_style()?,
        families,
    )
    .map_err(|_| FullTextCommandSurfaceScenarioError::InvalidProjection)?;
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(token, router);
    let lease = match id {
        FullTextCommandSurfaceScenarioId::NavigationInput => {
            lease.with_source_address(navigation_input_lease())
        }
        FullTextCommandSurfaceScenarioId::WorkspaceTabs => lease
            .with_tab_strip(workspace_tabs_lease())
            .with_status_diagnostics(workspace_tabs_status_diagnostics_lease())
            .with_editor_viewport(workspace_tabs_editor_viewport_lease()),
        _ => lease,
    };
    Ok(lease)
}

struct NoopRouter;

impl super::KucRootEffectRouter for NoopRouter {
    fn route(
        &mut self,
        _context: super::KucRootEventBatchContext,
    ) -> Result<Option<super::KucOpaqueHostEffectBatch>, super::KucOpaqueHostEffectError> {
        Ok(None)
    }
}

/// KUC-only terminal sink for the deterministic generic navigation-input scenario.
///
/// It consumes the one-shot value without decoding or publishing it. Real host
/// projections supply their own private port; this scenario proves retained input
/// and opaque event transport only.
struct NavigationInputAcknowledgementPort;

impl SourceAddressSubmissionPort for NavigationInputAcknowledgementPort {
    fn forward_submission(
        &mut self,
        _submission: SourceAddressSubmission,
    ) -> Result<(), SourceAddressSubmissionPortError> {
        Ok(())
    }
}

fn navigation_input_lease() -> SourceAddressProjectionLease {
    SourceAddressProjectionLease::new(SourceAddressStrip::new(SourceAddressPresentation::new(
        "Navigation input",
        "Enter navigation text",
        "Navigation input",
    )))
    .with_submission_port(NavigationInputAcknowledgementPort)
}

/// KUC-only terminal sink for a generic tab-strip scenario.
///
/// The scenario owns the targets and never exposes a tab/group operation to a
/// Storybook or KLE consumer. Real hosts supply a private port that resolves
/// the same sealed proposal against their authoritative state.
struct WorkspaceTabsAcknowledgementPort;

impl TabStripProposalPort for WorkspaceTabsAcknowledgementPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        proposal.consume_for_port();
        Ok(())
    }
}

fn workspace_tabs_lease() -> TabStripProjectionLease {
    let selectable_drag_source = TabStripTabCapabilities::new()
        .selectable(true)
        .draggable(true);
    let accepting_tab = TabStripTabCapabilities::new()
        .selectable(true)
        .accepts_tab_drop(true);
    let projection = TabStripProjection::new(
        DEFAULT_SCENARIO_REVISION,
        TabStripCorrelation::from_opaque_bytes(b"kuc-scenario-workspace-tabs"),
    )
    .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(true))
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"kuc-scenario-workspace-source"),
            TabStripText::new("Guide ⭐️"),
        )
        .capabilities(selectable_drag_source),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"kuc-scenario-workspace-target"),
            TabStripText::new("Notes"),
        )
        .capabilities(accepting_tab),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"kuc-scenario-workspace-group"),
            TabStripText::new("Drafts"),
        )
        .capabilities(
            TabStripGroupCapabilities::new()
                .collapsible(true)
                .accepts_tab_drop(true),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"kuc-scenario-workspace-group-tab"),
                TabStripText::new("Outline"),
            )
            .capabilities(accepting_tab),
        ),
    );
    TabStripProjectionLease::new(projection).with_proposal_port(WorkspaceTabsAcknowledgementPort)
}

fn workspace_tabs_status_diagnostics_lease() -> StatusDiagnosticsProjectionLease {
    let status_bar = StatusBar::new("Workspace status")
        .mode(StatusBarMode::MultiSegment)
        .density(StatusBarDensity::Compact)
        .segment(StatusBarSegment::new("branch", "main"))
        .segment(StatusBarSegment::new("encoding", "UTF-8"));
    let diagnostics = DiagnosticsList::new("Diagnostics")
        .scope("all", "全件 ⭐️", "全件の診断 ⭐️")
        .scope("current", "現在の範囲", "現在の診断範囲")
        .item(
            crate::molecule::DiagnosticItem::new(
                "workspace-warning",
                DiagnosticSeverity::Warning,
                "日本語の診断 ⭐️",
                DiagnosticLocation::new("src/lib.rs", DIAGNOSTIC_LINE, DIAGNOSTIC_COLUMN),
            )
            .scopes(["all", "current"]),
        );
    StatusDiagnosticsProjectionLease::new()
        .with_status_bar(status_bar)
        .with_diagnostics_list(diagnostics)
}

fn workspace_tabs_editor_viewport_lease() -> EditorViewportProjectionLease {
    /* WHY: 固定fixtureは構築時点で検証済みのため、実行時に到達不能な失敗境界を持たせない。 */
    let preview = UiImageSurfaceProps {
        fingerprint: "generic-workspace-preview".to_string(),
        width: WORKSPACE_PREVIEW_DIMENSION,
        height: WORKSPACE_PREVIEW_DIMENSION,
        display_width: WORKSPACE_PREVIEW_DISPLAY_DIMENSION,
        display_height: WORKSPACE_PREVIEW_DISPLAY_DIMENSION,
        display_width_milli: WORKSPACE_PREVIEW_DISPLAY_DIMENSION,
        display_height_milli: WORKSPACE_PREVIEW_DISPLAY_DIMENSION,
        rgba: WORKSPACE_PREVIEW_RGBA.to_vec(),
        content_scale: WORKSPACE_PREVIEW_CONTENT_SCALE,
        fit: UiImageSurfaceFit::Contain,
        accessibility_label: "Generic preview".to_string(),
        selection_text: String::new(),
        highlight_rects: Vec::new(),
        transform: UiImageSurfaceTransform::default(),
    };
    EditorViewportProjectionLease::new(preview)
}
