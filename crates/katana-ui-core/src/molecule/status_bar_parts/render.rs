use super::{StatusBar, StatusBarMode};
use crate::render_model::{UiNode, UiNodeKind};

pub(super) fn segment_nodes(value: &StatusBar) -> Vec<UiNode> {
    if value.mode != StatusBarMode::MultiSegment {
        return Vec::new();
    }
    value
        .segments
        .iter()
        .map(|segment| {
            let mut node = UiNode::new(UiNodeKind::Row, segment.label.clone()).tone(segment.tone);
            if let Some(progress) = &segment.progress {
                node = node.child(
                    UiNode::new(UiNodeKind::ProgressBar, progress.label.clone())
                        .progress(true, progress.percent)
                        .tone(progress.tone),
                );
            }
            node
        })
        .collect()
}
