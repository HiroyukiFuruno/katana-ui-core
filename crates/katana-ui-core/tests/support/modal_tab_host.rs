use std::sync::{Arc, Mutex};

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory, TabStripCorrelation,
    TabStripProjection, TabStripProjectionLease, TabStripProposal, TabStripProposalOperation,
    TabStripProposalPort, TabStripProposalPortError, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText, TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

#[derive(Default, Debug)]
pub struct HostState {
    pub selected: Option<TabStripTabTarget>,
    pub tab_order: Vec<TabStripTabTarget>,
    pub active_drag: Option<TabStripTabTarget>,
    pub operations: usize,
}

#[derive(Clone)]
pub struct HostPort(pub Arc<Mutex<HostState>>);

impl TabStripProposalPort for HostPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        let mut host = self.0.lock().expect("host state mutex");
        host.operations += 1;
        match proposal.into_operation() {
            TabStripProposalOperation::SelectTab(target) => host.selected = Some(target),
            TabStripProposalOperation::StartDrag(target) => host.active_drag = Some(target),
            TabStripProposalOperation::CancelDrag => host.active_drag = None,
            TabStripProposalOperation::FinishDrag {
                committed: true,
                destination: Some(destination),
            } => {
                if let Some(source) = host.active_drag.take()
                    && let Some(index) = host.tab_order.iter().position(|tab| tab == &source)
                {
                    let value = host.tab_order.remove(index);
                    let destination_index = match destination {
                            katana_ui_core::egui::text_command_surface::TabStripTabPlacement::Before(target) => {
                                host.tab_order.iter().position(|tab| tab == &target)
                            }
                            katana_ui_core::egui::text_command_surface::TabStripTabPlacement::After(target) => {
                                host.tab_order.iter().position(|tab| tab == &target).map(|index| index + 1)
                            }
                            katana_ui_core::egui::text_command_surface::TabStripTabPlacement::EndOfStrip => {
                                Some(host.tab_order.len())
                            }
                            _ => None,
                        };
                    if let Some(index) = destination_index {
                        let length = host.tab_order.len();
                        host.tab_order.insert(index.min(length), value);
                    }
                }
            }
            TabStripProposalOperation::FinishDrag { .. } => host.active_drag = None,
            _ => {}
        }
        Ok(())
    }
}

fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("modal-host").value("host"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"modal-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("modal-host-text")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap()
}

pub fn projection(
    revision: u64,
    tab_a: TabStripTabTarget,
    tab_b: TabStripTabTarget,
) -> TabStripProjection {
    let tab = |target| {
        TabStripTabDescriptor::new(target, TabStripText::new("同じ名前"))
            .capabilities(TabStripTabCapabilities::new().selectable(true))
    };
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"modal-host"),
    )
    .tab(tab(tab_a))
    .tab(tab(tab_b))
}

pub fn projection_with_draggable_tabs(
    revision: u64,
    first: TabStripTabTarget,
    second: TabStripTabTarget,
) -> TabStripProjection {
    let first_is_drag = first == TabStripTabTarget::from_opaque_bytes(b"drag-first");
    let first_label = if first_is_drag {
        "ドラッグタブ"
    } else {
        "移動先"
    };
    let second_label = if first_is_drag {
        "移動先"
    } else {
        "ドラッグタブ"
    };
    let tab = |target, label| {
        TabStripTabDescriptor::new(target, TabStripText::new(label)).capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .draggable(true)
                .accepts_tab_drop(true),
        )
    };
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"modal-drag"),
    )
    .tab(tab(first, first_label))
    .tab(tab(second, second_label))
}

pub fn root_with_host(
    projection: TabStripProjection,
    host: Arc<Mutex<HostState>>,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot {
    EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_context| Ok(None))
                .with_tab_strip(
                    TabStripProjectionLease::new(projection).with_proposal_port(HostPort(host)),
                ),
        )
        .unwrap()
}

pub fn button_bounds(output: &egui::FullOutput, label: &str) -> egui::Rect {
    button_bounds_all(output, label)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("missing AX button {label}"))
}

pub fn button_bounds_all(output: &egui::FullOutput, label: &str) -> Vec<egui::Rect> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .into_iter()
        .flat_map(|update| update.nodes.iter())
        .filter_map(|(_, node)| {
            (node.label() == Some(label))
                .then(|| node.bounds())
                .flatten()
                .map(|bounds| {
                    egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    )
                })
        })
        .collect()
}
