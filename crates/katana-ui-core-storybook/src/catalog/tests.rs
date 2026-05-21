use super::StoryPageContract;
use super::{StoryCatalog, StoryDetailContent, StoryPresetLabels};
use katana_ui_core::render_model::{
    UiNode, UiNodeKind, UiPopoverFocusManagement, UiPopoverPlacement, UiVisualRole,
};
use katana_ui_core::{atom, render_model::UiTree};

mod app_primitive_contracts;
mod atom_and_basic_contracts;
mod collection_feedback_contracts;
mod interaction_surface_contracts;
mod modal_and_runtime_contracts;
mod popover_and_accordion_contracts;
mod settings_and_panel_contracts;
mod window_and_hover_contracts;

fn page_children(examples: &[super::StoryExample], page: &str) -> Option<Vec<String>> {
    examples.iter().find(|it| it.page == page).map(|it| {
        it.tree
            .root()
            .children()
            .iter()
            .map(|child| child.props().label.clone())
            .collect()
    })
}

fn page_descendant_labels(examples: &[super::StoryExample], page: &str) -> Option<Vec<String>> {
    examples.iter().find(|it| it.page == page).map(|it| {
        let mut labels = Vec::new();
        collect_labels(it.tree.root(), &mut labels);
        labels
    })
}

fn collect_labels(node: &UiNode, labels: &mut Vec<String>) {
    labels.push(node.props().label.clone());
    for child in node.children() {
        collect_labels(child, labels);
    }
}

fn is_atom_kind(kind: UiNodeKind) -> bool {
    matches!(
        kind,
        UiNodeKind::Text
            | UiNodeKind::Icon
            | UiNodeKind::Button
            | UiNodeKind::Input
            | UiNodeKind::Checkbox
            | UiNodeKind::Radio
            | UiNodeKind::Badge
            | UiNodeKind::Divider
            | UiNodeKind::Spacer
            | UiNodeKind::KeyCap
            | UiNodeKind::LoadingDots
            | UiNodeKind::Spinner
            | UiNodeKind::ProgressBar
            | UiNodeKind::ColorSwatch
            | UiNodeKind::Chip
            | UiNodeKind::Toggle
            | UiNodeKind::SlideControl
            | UiNodeKind::SvgButton
            | UiNodeKind::TextButton
            | UiNodeKind::IconTextButton
    )
}
