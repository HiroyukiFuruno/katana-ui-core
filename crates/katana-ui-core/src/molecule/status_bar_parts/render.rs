use super::{ProgressMeterShape, StatusBar, StatusBarDensity, StatusBarMode, StatusBarSegment};
use crate::atom::Icon;
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiSize};

pub(super) fn segment_nodes(value: &StatusBar) -> Vec<UiNode> {
    if value.mode != StatusBarMode::MultiSegment {
        return Vec::new();
    }
    [
        super::StatusBarSegmentAlignment::Leading,
        super::StatusBarSegmentAlignment::Center,
        super::StatusBarSegmentAlignment::Trailing,
    ]
    .into_iter()
    .flat_map(|alignment| {
        value
            .segments
            .iter()
            .filter(move |segment| segment.alignment == alignment)
            .map(|segment| segment_node(segment, value.density))
    })
    .collect()
}

fn segment_node(segment: &StatusBarSegment, density: StatusBarDensity) -> UiNode {
    let mut node = UiNode::new(UiNodeKind::Row, segment.label.clone())
        .tone(segment.tone)
        .size(density.into())
        .focusable(segment.interactive)
        .accessibility_label(segment.accessibility_label.clone())
        .placeholder(segment.tooltip.clone().unwrap_or_default());
    if let Some(icon) = &segment.icon {
        node = node.child(Icon::new(icon.clone()));
    }
    if let Some(progress) = &segment.progress {
        node = node.child(
            UiNode::new(UiNodeKind::ProgressBar, progress.label.clone())
                .progress(true, progress.percent)
                .tone(progress.tone)
                .placeholder(progress.tooltip.clone())
                .interaction(UiInteractionState {
                    value: shape_name(progress.shape).to_string(),
                    ..UiInteractionState::default()
                }),
        );
    }
    node
}

fn shape_name(value: ProgressMeterShape) -> &'static str {
    match value {
        ProgressMeterShape::Linear => "Linear",
        ProgressMeterShape::Ring => "Ring",
        ProgressMeterShape::Pie => "Pie",
    }
}

impl From<StatusBarDensity> for UiSize {
    fn from(value: StatusBarDensity) -> Self {
        match value {
            StatusBarDensity::Compact => Self::Small,
            StatusBarDensity::Default => Self::Medium,
        }
    }
}
