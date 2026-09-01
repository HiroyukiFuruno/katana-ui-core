use crate::molecule::structured::{CloseableTabStrip, CloseableTabStripEvent};

#[cfg(test)]
#[path = "closeable_tab_strip_adapter_tests.rs"]
mod closeable_tab_strip_adapter_tests;
#[path = "closeable_tab_strip_data.rs"]
mod closeable_tab_strip_data;
#[path = "closeable_tab_strip_render.rs"]
mod closeable_tab_strip_render;

pub(crate) struct CloseableTabStripAdapter;

pub(crate) struct CloseableTabStripRender {
    widget_rect: egui::Rect,
    tab_rects: Vec<(String, egui::Rect)>,
    #[cfg(test)]
    close_rects: Vec<(String, egui::Rect)>,
    group_rects: Vec<(String, egui::Rect)>,
    events: Vec<CloseableTabStripEvent>,
}

impl CloseableTabStripRender {
    pub(crate) const fn widget_rect(&self) -> egui::Rect {
        self.widget_rect
    }

    pub(crate) fn tab_rects(&self) -> &[(String, egui::Rect)] {
        &self.tab_rects
    }

    #[cfg(test)]
    pub(crate) fn close_rects(&self) -> &[(String, egui::Rect)] {
        &self.close_rects
    }

    pub(crate) fn group_rects(&self) -> &[(String, egui::Rect)] {
        &self.group_rects
    }

    pub(crate) fn events(&self) -> &[CloseableTabStripEvent] {
        &self.events
    }
}

impl CloseableTabStripAdapter {
    pub(crate) fn show(
        &self,
        ui: &mut egui::Ui,
        strip: &mut CloseableTabStrip,
    ) -> CloseableTabStripRender {
        let items = closeable_tab_strip_data::TabStripItems::from_strip(strip);
        let mut events = Vec::new();

        let layout = closeable_tab_strip_render::render_tab_strip(ui, &items, strip, &mut events);

        CloseableTabStripRender {
            widget_rect: layout.widget_rect,
            tab_rects: layout.tab_rects,
            #[cfg(test)]
            close_rects: layout.close_rects,
            group_rects: layout.group_rects,
            events,
        }
    }
}
