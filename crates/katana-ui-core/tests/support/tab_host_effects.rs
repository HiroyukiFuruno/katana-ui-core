use std::sync::{Arc, Mutex};

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory,
    TabStripContextMenuPresentation, TabStripControlPresentation, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripProjection,
    TabStripProjectionLease, TabStripProposal, TabStripProposalOperation, TabStripProposalPort,
    TabStripProposalPortError, TabStripSwatchDescriptor, TabStripSwatchTarget,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

#[derive(Default, Debug)]
pub struct HostState {
    pub selected: Option<TabStripTabTarget>,
    pub closed: Vec<TabStripTabTarget>,
    pub group_order: Vec<TabStripGroupTarget>,
    pub pinned: Vec<TabStripTabTarget>,
    pub moved_tabs: Vec<(TabStripTabTarget, TabStripGroupTarget)>,
    pub removed_tabs: Vec<TabStripTabTarget>,
    pub collapsed_groups: Vec<(TabStripGroupTarget, bool)>,
    pub renamed_groups: Vec<(TabStripGroupTarget, String)>,
    pub recolored_groups: Vec<(TabStripGroupTarget, TabStripSwatchTarget)>,
    pub ungrouped: Vec<TabStripGroupTarget>,
    pub closed_groups: Vec<TabStripGroupTarget>,
    pub created_groups: Vec<TabStripTabTarget>,
    pub group_membership: Vec<(TabStripTabTarget, TabStripGroupTarget)>,
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
        let operation = proposal.into_operation();
        let mut host = self.0.lock().expect("host state mutex");
        host.operations += 1;
        match operation {
            TabStripProposalOperation::SelectTab(target) => host.selected = Some(target),
            TabStripProposalOperation::RequestClose(target)
            | TabStripProposalOperation::ConfirmClose(target) => {
                host.tab_order.retain(|value| value != &target);
                host.closed.push(target);
            }
            TabStripProposalOperation::SetPinned { tab, pinned } => {
                if pinned {
                    host.pinned.push(tab);
                } else {
                    host.pinned.retain(|value| value != &tab);
                }
            }
            TabStripProposalOperation::MoveTabToGroup { tab, group } => {
                host.moved_tabs.push((tab, group));
                let (tab, group) = host.moved_tabs.last().cloned().unwrap();
                host.group_membership.retain(|(value, _)| value != &tab);
                host.group_membership.push((tab, group));
            }
            TabStripProposalOperation::RemoveFromGroup(tab) => {
                host.removed_tabs.push(tab.clone());
                host.group_membership.retain(|(value, _)| value != &tab);
            }
            TabStripProposalOperation::SetGroupCollapsed { group, collapsed } => {
                host.collapsed_groups.retain(|(value, _)| value != &group);
                host.collapsed_groups.push((group, collapsed));
            }
            TabStripProposalOperation::RenameGroup { group, name } => {
                host.renamed_groups.retain(|(value, _)| value != &group);
                host.renamed_groups.push((group, name.as_str().to_owned()));
            }
            TabStripProposalOperation::RecolorGroup { group, swatch } => {
                host.recolored_groups.retain(|(value, _)| value != &group);
                host.recolored_groups.push((group, swatch));
            }
            TabStripProposalOperation::Ungroup(group) => {
                host.ungrouped.push(group.clone());
                host.group_order.retain(|value| value != &group);
                host.group_membership.retain(|(_, owner)| owner != &group);
            }
            TabStripProposalOperation::CloseGroup(group) => {
                host.closed_groups.push(group.clone());
                host.group_order.retain(|value| value != &group);
                let members = host
                    .group_membership
                    .iter()
                    .filter(|(_, owner)| owner == &group)
                    .map(|(tab, _)| tab.clone())
                    .collect::<Vec<_>>();
                host.closed.extend(members);
                let closed = host.closed.clone();
                host.tab_order
                    .retain(|tab| !closed.iter().any(|value| value == tab));
                host.group_membership.retain(|(_, owner)| owner != &group);
            }
            TabStripProposalOperation::CreateGroup(tab) => host.created_groups.push(tab),
            TabStripProposalOperation::StartDrag(tab) => host.active_drag = Some(tab),
            TabStripProposalOperation::CancelDrag => host.active_drag = None,
            TabStripProposalOperation::FinishDrag {
                committed: true,
                destination: Some(destination),
            } => {
                if let Some(source) = host.active_drag.take() {
                    let source_index = host
                        .tab_order
                        .iter()
                        .position(|tab| tab == &source)
                        .unwrap();
                    let value = host.tab_order.remove(source_index);
                    let index = match destination {
                        katana_ui_core::egui::text_command_surface::TabStripTabPlacement::Before(target) => host.tab_order.iter().position(|tab| tab == &target),
                        katana_ui_core::egui::text_command_surface::TabStripTabPlacement::After(target) => host.tab_order.iter().position(|tab| tab == &target).map(|index| index + 1),
                        katana_ui_core::egui::text_command_surface::TabStripTabPlacement::EndOfStrip => Some(host.tab_order.len()),
                        _ => None,
                    };
                    if let Some(index) = index {
                        let length = host.tab_order.len();
                        host.tab_order.insert(index.min(length), value);
                    }
                }
            }
            TabStripProposalOperation::FinishDrag { .. } => host.active_drag = None,
            TabStripProposalOperation::ReorderGroup {
                source,
                destination,
            } => {
                if let Some(index) = host.group_order.iter().position(|value| value == &source) {
                    let value = host.group_order.remove(index);
                    let destination_index = match destination {
                        katana_ui_core::egui::text_command_surface::TabStripGroupPlacement::Before(target) => host.group_order.iter().position(|item| item == &target),
                        katana_ui_core::egui::text_command_surface::TabStripGroupPlacement::After(target) => host.group_order.iter().position(|item| item == &target).map(|index| index + 1),
                        katana_ui_core::egui::text_command_surface::TabStripGroupPlacement::EndOfStrip => Some(host.group_order.len()),
                    };
                    if let Some(index) = destination_index {
                        let length = host.group_order.len();
                        host.group_order.insert(index.min(length), value);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

pub fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("host-effects").value("host"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"host-effects",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("host-effects-text")),
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
    group: TabStripGroupTarget,
) -> TabStripProjection {
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"host-effects"),
    )
    .tab(
        TabStripTabDescriptor::new(tab_a, TabStripText::new("同じ名前"))
            .capabilities(
                TabStripTabCapabilities::new()
                    .selectable(true)
                    .closeable(true),
            )
            .trailing_control(TabStripControlPresentation::new(
                TabStripText::new("閉じる"),
                TabStripText::new("閉じる"),
            )),
    )
    .tab(
        TabStripTabDescriptor::new(tab_b, TabStripText::new("同じ名前"))
            .capabilities(
                TabStripTabCapabilities::new()
                    .selectable(true)
                    .closeable(true),
            )
            .trailing_control(TabStripControlPresentation::new(
                TabStripText::new("閉じる"),
                TabStripText::new("閉じる"),
            )),
    )
    .group(
        TabStripGroupDescriptor::new(group, TabStripText::new("グループ")).capabilities(
            TabStripGroupCapabilities::new()
                .collapsible(true)
                .draggable(true),
        ),
    )
}

pub fn projection_with_group_menu(
    revision: u64,
    tab: TabStripTabTarget,
    group: TabStripGroupTarget,
) -> TabStripProjection {
    let menu = TabStripContextMenuPresentation::new()
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Pin"),
            TabStripText::new("Pin"),
            TabStripMenuOperation::SetPinned(true),
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Unpin"),
            TabStripText::new("Unpin"),
            TabStripMenuOperation::SetPinned(false),
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Create group"),
            TabStripText::new("Create group"),
            TabStripMenuOperation::CreateGroup,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Move to group"),
            TabStripText::new("Move to group"),
            TabStripMenuOperation::MoveToGroup(group.clone()),
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Remove from group"),
            TabStripText::new("Remove from group"),
            TabStripMenuOperation::RemoveFromGroup,
        ));
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"host-menu"),
    )
    .tab(
        TabStripTabDescriptor::new(tab, TabStripText::new("Menu target"))
            .capabilities(
                TabStripTabCapabilities::new()
                    .selectable(true)
                    .groupable(true)
                    .closeable(true),
            )
            .context_menu(menu),
    )
    .group(
        TabStripGroupDescriptor::new(group, TabStripText::new("Menu group"))
            .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
    )
}

pub fn projection_with_popup(
    revision: u64,
    group: TabStripGroupTarget,
    collapsed: bool,
    request_rename: bool,
) -> TabStripProjection {
    projection_with_popup_named(revision, group, collapsed, request_rename, "グループ")
}

pub fn projection_with_popup_named(
    revision: u64,
    group: TabStripGroupTarget,
    collapsed: bool,
    request_rename: bool,
    name: &str,
) -> TabStripProjection {
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"host-popup"),
    )
    .group(
        TabStripGroupDescriptor::new(group, TabStripText::new(name))
            .capabilities(
                TabStripGroupCapabilities::new()
                    .collapsed(collapsed)
                    .collapsible(true)
                    .menu_available(true)
                    .renamable(true)
                    .recolorable(true),
            )
            .swatch(
                TabStripSwatchDescriptor::new(
                    TabStripSwatchTarget::from_opaque_bytes(b"swatch-blue"),
                    katana_ui_core::molecule::RgbaColor::new(40, 120, 220, 255),
                )
                .accessibility_label(TabStripText::new("青")),
            )
            .popup(
                TabStripGroupPopupPresentation::new()
                    .rename_placeholder(TabStripText::new("グループ名"))
                    .request_inline_rename(request_rename),
            ),
    )
}

pub fn projection_with_group_actions(
    revision: u64,
    group: TabStripGroupTarget,
    member: TabStripTabTarget,
) -> TabStripProjection {
    let popup = TabStripGroupPopupPresentation::new()
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Ungroup"),
            TabStripText::new("Ungroup"),
            TabStripMenuOperation::Ungroup,
        ))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("Close group"),
            TabStripText::new("Close group"),
            TabStripMenuOperation::CloseGroup,
        ));
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"group-actions"),
    )
    .group(
        TabStripGroupDescriptor::new(group, TabStripText::new("操作グループ"))
            .capabilities(
                TabStripGroupCapabilities::new()
                    .collapsible(true)
                    .menu_available(true)
                    .ungroupable(true)
                    .closeable(true),
            )
            .tab(
                TabStripTabDescriptor::new(member, TabStripText::new("メンバー"))
                    .capabilities(TabStripTabCapabilities::new().selectable(true)),
            )
            .popup(popup),
    )
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
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"tab-drag"),
    )
    .tab(
        TabStripTabDescriptor::new(first, TabStripText::new(first_label)).capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .draggable(true)
                .accepts_tab_drop(true),
        ),
    )
    .tab(
        TabStripTabDescriptor::new(second, TabStripText::new(second_label)).capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .draggable(true)
                .accepts_tab_drop(true),
        ),
    )
}

pub fn projection_with_selectable(
    revision: u64,
    correlation: TabStripCorrelation,
    tab: TabStripTabTarget,
    selectable: bool,
) -> TabStripProjection {
    TabStripProjection::new(revision, correlation).tab(
        TabStripTabDescriptor::new(tab, TabStripText::new("失効対象"))
            .capabilities(TabStripTabCapabilities::new().selectable(selectable)),
    )
}

pub fn input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1280.0, 720.0),
        )),
        events,
        ..Default::default()
    }
}

pub fn frame(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    let mut output = context.run_ui(input(events), |ui| {
        root.show(ui).unwrap();
    });
    output.textures_delta.clear();
    output
}

pub fn button_bounds(output: &egui::FullOutput, label: &str) -> egui::Rect {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
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
        })
        .unwrap_or_else(|| {
            let labels = output
                .platform_output
                .accesskit_update
                .as_ref()
                .map(|update| {
                    update
                        .nodes
                        .iter()
                        .filter_map(|(_, node)| node.label().map(str::to_owned))
                        .collect::<Vec<_>>()
                });
            panic!("missing AX button {label}; labels={labels:?}")
        })
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

pub fn root_with_port(
    projection: TabStripProjection,
    port: impl TabStripProposalPort + 'static,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot {
    EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_with_port(1, projection, port))
        .unwrap()
}

pub fn lease_for(
    revision: u64,
    projection: TabStripProjection,
    port: HostPort,
) -> EguiTextCommandSurfaceHostProjectionLease {
    EguiTextCommandSurfaceHostProjectionLease::new(token(revision), |_context| Ok(None))
        .with_tab_strip(TabStripProjectionLease::new(projection).with_proposal_port(port))
}

pub fn lease_with_port(
    revision: u64,
    projection: TabStripProjection,
    port: impl TabStripProposalPort + 'static,
) -> EguiTextCommandSurfaceHostProjectionLease {
    EguiTextCommandSurfaceHostProjectionLease::new(token(revision), |_context| Ok(None))
        .with_tab_strip(TabStripProjectionLease::new(projection).with_proposal_port(port))
}
