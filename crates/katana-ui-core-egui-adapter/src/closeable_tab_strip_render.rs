use egui::{Align, Button, Layout};
use katana_ui_core::molecule::structured::{
    CloseableTab, CloseableTabClosePresentation, CloseableTabStripAction, CloseableTabStripEvent,
    CloseableTabStripIntent,
};

use super::closeable_tab_strip_data::TabStripItems;

pub(super) struct RenderedCloseableTabStripLayout {
    pub(super) widget_rect: egui::Rect,
    pub(super) tab_rects: Vec<(String, egui::Rect)>,
    #[cfg(test)]
    pub(super) close_rects: Vec<(String, egui::Rect)>,
    pub(super) group_rects: Vec<(String, egui::Rect)>,
}

pub(super) fn render_tab_strip(
    ui: &mut egui::Ui,
    items: &TabStripItems,
    strip: &mut katana_ui_core::molecule::structured::CloseableTabStrip,
    events: &mut Vec<CloseableTabStripEvent>,
) -> RenderedCloseableTabStripLayout {
    let mut tabs = Vec::new();
    #[cfg(test)]
    let mut close_rects = Vec::new();
    let mut groups = Vec::new();

    let widget_rect = ui
        .scope(|ui| {
            ui.horizontal_wrapped(|ui| {
                for tab in &items.pinned_tabs {
                    render_tab(
                        ui,
                        tab,
                        strip,
                        &mut tabs,
                        #[cfg(test)]
                        &mut close_rects,
                        events,
                    );
                }
                for group in &items.root_groups {
                    render_group(
                        ui,
                        group,
                        items,
                        strip,
                        &mut tabs,
                        #[cfg(test)]
                        &mut close_rects,
                        &mut groups,
                        events,
                    );
                }
                for tab in &items.unknown_group_tabs {
                    render_tab(
                        ui,
                        tab,
                        strip,
                        &mut tabs,
                        #[cfg(test)]
                        &mut close_rects,
                        events,
                    );
                }
                for tab in &items.ungrouped_tabs {
                    render_tab(
                        ui,
                        tab,
                        strip,
                        &mut tabs,
                        #[cfg(test)]
                        &mut close_rects,
                        events,
                    );
                }
            });
        })
        .response
        .rect;

    RenderedCloseableTabStripLayout {
        widget_rect,
        tab_rects: tabs,
        #[cfg(test)]
        close_rects,
        group_rects: groups,
    }
}

fn render_group(
    ui: &mut egui::Ui,
    group: &katana_ui_core::molecule::structured::CloseableTabGroup,
    items: &TabStripItems,
    strip: &mut katana_ui_core::molecule::structured::CloseableTabStrip,
    tab_rects: &mut Vec<(String, egui::Rect)>,
    #[cfg(test)] close_rects: &mut Vec<(String, egui::Rect)>,
    group_rects: &mut Vec<(String, egui::Rect)>,
    events: &mut Vec<CloseableTabStripEvent>,
) {
    let rendered = ui.group(|ui| {
        let header =
            ui.add(Button::selectable(group.collapsed, &group.label).frame_when_inactive(true));
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
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
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

fn render_tab(
    ui: &mut egui::Ui,
    tab: &CloseableTab,
    strip: &mut katana_ui_core::molecule::structured::CloseableTabStrip,
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
        .with_layout(Layout::left_to_right(Align::Center), |ui| {
            let response = ui
                .add(
                    Button::selectable(
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
