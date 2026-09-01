use crate::molecule::structured::{
    CloseableTab, CloseableTabClosePresentation, CloseableTabGroup, CloseableTabStrip,
    CloseableTabStripAction, CloseableTabStripEvent, CloseableTabStripIntent,
};

pub(super) struct TabStripItems {
    tabs: Vec<CloseableTab>,
    groups: Vec<CloseableTabGroup>,
    pub(super) pinned_tabs: Vec<CloseableTab>,
    pub(super) root_groups: Vec<CloseableTabGroup>,
    pub(super) unknown_group_tabs: Vec<CloseableTab>,
    pub(super) ungrouped_tabs: Vec<CloseableTab>,
}

impl TabStripItems {
    pub(super) fn from_strip(strip: &CloseableTabStrip) -> Self {
        let options = strip.options();
        let tabs = options.tabs.clone();
        let groups = options.groups.clone();
        let pinned_tabs = options
            .tabs
            .iter()
            .filter(|tab| tab.pinned)
            .cloned()
            .collect();
        let root_groups = options
            .groups
            .iter()
            .filter(|group| group.parent_group_id.is_none())
            .cloned()
            .collect();
        let unknown_group_tabs = options
            .tabs
            .iter()
            .filter(|tab| {
                !tab.pinned
                    && tab.group_id.as_ref().is_some_and(|group_id| {
                        options.groups.iter().all(|group| group.id != *group_id)
                    })
            })
            .cloned()
            .collect();
        let ungrouped_tabs = options
            .tabs
            .iter()
            .filter(|tab| !tab.pinned && tab.group_id.is_none())
            .cloned()
            .collect();

        Self {
            tabs,
            groups,
            pinned_tabs,
            root_groups,
            unknown_group_tabs,
            ungrouped_tabs,
        }
    }

    fn tabs_for_group<'a>(
        &'a self,
        group_id: &crate::molecule::structured::CloseableTabGroupId,
    ) -> impl Iterator<Item = &'a CloseableTab> {
        self.tabs
            .iter()
            .filter(move |tab| tab.group_id.as_ref() == Some(group_id))
    }

    fn groups_for_parent<'a>(
        &'a self,
        parent_group_id: &crate::molecule::structured::CloseableTabGroupId,
    ) -> impl Iterator<Item = &'a CloseableTabGroup> {
        self.groups
            .iter()
            .filter(move |group| group.parent_group_id.as_ref() == Some(parent_group_id))
    }
}

pub(super) fn render_group(
    ui: &mut egui::Ui,
    group: &CloseableTabGroup,
    items: &TabStripItems,
    strip: &mut CloseableTabStrip,
    tab_rects: &mut Vec<(String, egui::Rect)>,
    #[cfg(test)] close_rects: &mut Vec<(String, egui::Rect)>,
    group_rects: &mut Vec<(String, egui::Rect)>,
    events: &mut Vec<CloseableTabStripEvent>,
) {
    let rendered = ui.group(|ui| {
        let header = ui
            .add(egui::Button::selectable(group.collapsed, &group.label).frame_when_inactive(true));
        if header.clicked()
            || header.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                })
        {
            events.extend(
                strip.apply_action(CloseableTabStripAction::ToggleGroupCollapse {
                    group_id: group.id.clone(),
                }),
            );
        }
        if !group.collapsed {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                for tab in items.tabs_for_group(&group.id).filter(|tab| !tab.pinned) {
                    render_tab(
                        ui,
                        tab,
                        strip,
                        tab_rects,
                        #[cfg(test)]
                        close_rects,
                        events,
                    );
                }
                for child in items.groups_for_parent(&group.id) {
                    render_group(
                        ui,
                        child,
                        items,
                        strip,
                        tab_rects,
                        #[cfg(test)]
                        close_rects,
                        group_rects,
                        events,
                    );
                }
            });
        }
    });
    group_rects.push((group.id.as_str().to_owned(), rendered.response.rect));
}

pub(super) fn render_tab(
    ui: &mut egui::Ui,
    tab: &CloseableTab,
    strip: &mut CloseableTabStrip,
    rects: &mut Vec<(String, egui::Rect)>,
    #[cfg(test)] close_rects: &mut Vec<(String, egui::Rect)>,
    events: &mut Vec<CloseableTabStripEvent>,
) {
    let mut label = egui::RichText::new(&tab.title);
    if tab.dirty {
        label = label.italics();
    }
    if tab.pinned {
        label = label.strong();
    }
    let close_presentation = tab.close_presentation.as_ref().filter(|presentation| {
        tab.closeable && !tab.pinned && !presentation.visible_label.is_empty()
    });
    let (response, close_response) = ui
        .with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            let response = ui
                .add(
                    egui::Button::selectable(
                        strip.state().active_tab_id.as_ref() == Some(&tab.id),
                        label,
                    )
                    .frame_when_inactive(true),
                )
                .on_hover_text(tab.tooltip.as_deref().unwrap_or(&tab.title));
            if response.clicked()
                || response.has_focus()
                    && ui.input(|input| {
                        input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                    })
            {
                events.extend(strip.apply_action(CloseableTabStripAction::SelectTab {
                    tab_id: tab.id.clone(),
                }));
            }
            let close_response = close_presentation.map(|presentation| {
                let response = ui
                    .button(presentation.visible_label.as_str())
                    .on_hover_text(presentation.tooltip.as_str());
                publish_close_accessibility(ui, &response, presentation);
                response
            });
            (response, close_response)
        })
        .inner;
    rects.push((tab.id.as_str().to_owned(), response.rect));
    if let Some(close_response) = close_response {
        #[cfg(test)]
        close_rects.push((tab.id.as_str().to_owned(), close_response.rect));
        if close_response.clicked()
            || ui.input(|input| {
                input
                    .has_accesskit_action_request(close_response.id, egui::accesskit::Action::Click)
            })
            || close_response.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                })
        {
            events.extend(
                strip.apply_intent(CloseableTabStripIntent::RequestTabClose {
                    tab_id: tab.id.clone(),
                }),
            );
        }
    }
}

fn publish_close_accessibility(
    ui: &egui::Ui,
    response: &egui::Response,
    presentation: &CloseableTabClosePresentation,
) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Button);
        node.set_label(presentation.accessibility_label.as_str());
        node.add_action(egui::accesskit::Action::Click);
    });
}
