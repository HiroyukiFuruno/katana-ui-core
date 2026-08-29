use super::command_chrome_artifact_types::{
    CommandChromeArtifactFrame, StorybookCommandChromeManifest,
    StorybookCommandChromeManifestFrame, StorybookCommandChromeTypedEvent,
};
use crate::visual::command_chrome_artifact::{
    paint_plan_has_colored_star_texture, paint_plan_has_star_variation_selector, texture_identities,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::render_model::UiRect;

impl CommandChromeArtifactFrame {
    pub(crate) fn manifest_entry(&self) -> StorybookCommandChromeManifestFrame {
        let dropdown_close_reason =
            self.toolbar
                .events
                .iter()
                .rev()
                .find_map(|event| match event {
                    CommandChromeToolbarEvent::DropdownClosed { reason, .. } => Some(*reason),
                    _ => None,
                });
        let floating_close_reason = self.floating.as_ref().and_then(|floating| {
            floating.events.iter().rev().find_map(|event| match event {
                FloatingCommandToolbarEvent::Closed { reason } => Some(*reason),
                _ => None,
            })
        });
        let typed_events = self
            .toolbar
            .events
            .iter()
            .cloned()
            .map(StorybookCommandChromeTypedEvent::Toolbar)
            .chain(
                self.floating
                    .as_ref()
                    .into_iter()
                    .flat_map(|floating| floating.events.iter().cloned())
                    .map(StorybookCommandChromeTypedEvent::Floating),
            )
            .chain(
                self.search
                    .events
                    .iter()
                    .cloned()
                    .map(StorybookCommandChromeTypedEvent::Search),
            )
            .chain(
                self.search
                    .text_events
                    .iter()
                    .cloned()
                    .map(StorybookCommandChromeTypedEvent::SearchText),
            )
            .collect();
        let mut palette_identities = texture_identities(&self.toolbar.artifact.paint_plan);
        if let Some(floating) = self
            .floating
            .as_ref()
            .and_then(|floating| floating.artifact.as_ref())
        {
            palette_identities.extend(texture_identities(&floating.paint_plan));
        }
        palette_identities.extend(texture_identities(&self.search.artifact.paint_plan));
        let star_variation_selector_present =
            paint_plan_has_star_variation_selector(&self.toolbar.artifact.paint_plan)
                || self
                    .floating
                    .as_ref()
                    .and_then(|floating| floating.artifact.as_ref())
                    .is_some_and(|floating| {
                        paint_plan_has_star_variation_selector(&floating.paint_plan)
                    })
                || paint_plan_has_star_variation_selector(&self.search.artifact.paint_plan);
        let color_emoji_texture_present =
            paint_plan_has_colored_star_texture(&self.toolbar.artifact.paint_plan)
                || self
                    .floating
                    .as_ref()
                    .and_then(|floating| floating.artifact.as_ref())
                    .is_some_and(|floating| {
                        paint_plan_has_colored_star_texture(&floating.paint_plan)
                    })
                || paint_plan_has_colored_star_texture(&self.search.artifact.paint_plan);
        let mut accessibility_labels = self.accesskit_labels.clone();
        accessibility_labels.sort();
        accessibility_labels.dedup();

        StorybookCommandChromeManifestFrame {
            index: self.index,
            name: self.name.clone(),
            png: format!("{:02}-{}.png", self.index, self.name),
            frame_bounds: UiRect::new(0, 0, self.frame_width, self.frame_height),
            toolbar_bounds: self.toolbar.record.bounds,
            floating_bounds: self
                .floating
                .as_ref()
                .and_then(|value| value.record.as_ref().map(|record| record.panel_bounds)),
            search_bounds: self.search.record.bounds,
            toolbar_frame_record_hash: self.toolbar.artifact.frame_record_hash.clone(),
            toolbar_paint_plan_hash: self.toolbar.artifact.paint_plan_hash.clone(),
            toolbar_pixel_hash: self.toolbar_pixels.pixel_hash.clone(),
            floating_frame_record_hash: self
                .floating
                .as_ref()
                .and_then(|value| value.artifact.as_ref())
                .map(|value| value.frame_record_hash.clone()),
            floating_paint_plan_hash: self
                .floating
                .as_ref()
                .and_then(|value| value.artifact.as_ref())
                .map(|value| value.paint_plan_hash.clone()),
            floating_pixel_hash: self
                .floating_pixels
                .as_ref()
                .map(|value| value.pixel_hash.clone()),
            search_frame_record_hash: self.search.artifact.frame_record_hash.clone(),
            search_paint_plan_hash: self.search.artifact.paint_plan_hash.clone(),
            search_pixel_hash: self.search_pixels.pixel_hash.clone(),
            composite_pixel_hash: self.composite_pixels.pixel_hash.clone(),
            focused_action_id: self.toolbar.record.focused_action_id.clone(),
            search_focused_target: self.search.record.focused_target.clone(),
            dropdown_open: self.toolbar.record.dropdown.is_some(),
            dropdown_item_count: self
                .toolbar
                .record
                .dropdown
                .as_ref()
                .map_or(0, |dropdown| dropdown.items.len()),
            floating_tooltip_bounds: self.floating.as_ref().and_then(|value| {
                value
                    .record
                    .as_ref()
                    .and_then(|record| record.tooltip_bounds)
            }),
            toolbar_dropdown_close_reason: dropdown_close_reason,
            floating_close_reason,
            accessibility_labels,
            typed_events,
            star_variation_selector_present,
            color_emoji_texture_present,
            palette_identities,
        }
    }
}

impl StorybookCommandChromeManifest {
    pub(crate) fn from_frames(frames: Vec<StorybookCommandChromeManifestFrame>) -> Self {
        Self {
            schema: "kuc.command-chrome-storybook.v1",
            input_origin: "actual-egui-raw-input",
            artifact_encoder: "adapter-paint-plan-only",
            frames,
        }
    }
}

impl StorybookCommandChromeManifestFrame {
    pub(crate) fn validate_contract(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("manifest frame name must not be empty".to_string());
        }
        if self.toolbar_pixel_hash.is_empty() || self.search_pixel_hash.is_empty() {
            return Err(format!(
                "frame {} missing required pixel hashes",
                self.index
            ));
        }
        if self.composite_pixel_hash.is_empty() {
            return Err(format!("frame {} missing composite hash", self.index));
        }
        Ok(())
    }
}
