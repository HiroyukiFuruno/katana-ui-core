use crate::accessibility::{AccessibilityLabel, AccessibilityRole};
use crate::render_model::UiRect;
use crate::text_selection::UiTextSelectionRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceAccessibilityActionKind {
    Copy,
    Cut,
    Paste,
    Undo,
    Redo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAccessibilityLabels {
    pub copy: Option<AccessibilityLabel>,
    pub cut: Option<AccessibilityLabel>,
    pub paste: Option<AccessibilityLabel>,
    pub undo: Option<AccessibilityLabel>,
    pub redo: Option<AccessibilityLabel>,
}

impl TextSurfaceAccessibilityLabels {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            copy: None,
            cut: None,
            paste: None,
            undo: None,
            redo: None,
        }
    }

    #[must_use]
    pub fn with_label(
        mut self,
        action: TextSurfaceAccessibilityActionKind,
        label: impl Into<String>,
    ) -> Self {
        let label = AccessibilityLabel::new(label);
        match action {
            TextSurfaceAccessibilityActionKind::Copy => self.copy = Some(label),
            TextSurfaceAccessibilityActionKind::Cut => self.cut = Some(label),
            TextSurfaceAccessibilityActionKind::Paste => self.paste = Some(label),
            TextSurfaceAccessibilityActionKind::Undo => self.undo = Some(label),
            TextSurfaceAccessibilityActionKind::Redo => self.redo = Some(label),
        }
        self
    }

    #[must_use]
    pub const fn label_for(
        &self,
        action: TextSurfaceAccessibilityActionKind,
    ) -> Option<&AccessibilityLabel> {
        match action {
            TextSurfaceAccessibilityActionKind::Copy => self.copy.as_ref(),
            TextSurfaceAccessibilityActionKind::Cut => self.cut.as_ref(),
            TextSurfaceAccessibilityActionKind::Paste => self.paste.as_ref(),
            TextSurfaceAccessibilityActionKind::Undo => self.undo.as_ref(),
            TextSurfaceAccessibilityActionKind::Redo => self.redo.as_ref(),
        }
    }
}

impl Default for TextSurfaceAccessibilityLabels {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceAccessibilityTarget {
    Surface,
    GutterRow {
        logical_row: usize,
    },
    GutterMarker {
        logical_row: usize,
        marker_id: String,
    },
    ContextSelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAccessibilityNode {
    pub target: TextSurfaceAccessibilityTarget,
    pub role: AccessibilityRole,
    pub label: AccessibilityLabel,
    pub bounds: UiRect,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub hovered: bool,
    pub focused: bool,
    pub editable: bool,
    pub readonly: bool,
    pub disabled: bool,
    pub disabled_reason: Option<String>,
    pub description: Option<String>,
    pub selection: Option<UiTextSelectionRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAccessibilityAction {
    pub kind: TextSurfaceAccessibilityActionKind,
    pub label: AccessibilityLabel,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceAccessibilityTree {
    pub root: TextSurfaceAccessibilityNode,
    pub gutter_targets: Vec<TextSurfaceAccessibilityNode>,
    pub context_target: Option<TextSurfaceAccessibilityNode>,
    pub actions: Vec<TextSurfaceAccessibilityAction>,
}
