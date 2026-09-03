use sha2::Digest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AccessKitTextInputNode {
    pub(crate) role: AccessKitTextInputRole,
    pub(crate) value: Option<String>,
    pub(crate) scalar_sequence: Vec<u32>,
    pub(crate) bounds: Option<AccessKitTextInputBounds>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessKitTextInputRole {
    TextInput,
    MultilineTextInput,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AccessKitTextInputBounds {
    pub(crate) x0_bits: u64,
    pub(crate) y0_bits: u64,
    pub(crate) x1_bits: u64,
    pub(crate) y1_bits: u64,
}

impl AccessKitTextInputNode {
    pub(crate) fn from_accesskit_node(node: &egui::accesskit::Node) -> Self {
        let value = node.value().map(ToOwned::to_owned);
        let scalar_sequence = value
            .as_deref()
            .map(|text| text.chars().map(u32::from).collect())
            .unwrap_or_default();
        Self {
            role: match node.role() {
                egui::accesskit::Role::TextInput => AccessKitTextInputRole::TextInput,
                egui::accesskit::Role::MultilineTextInput => {
                    AccessKitTextInputRole::MultilineTextInput
                }
                _ => AccessKitTextInputRole::Other,
            },
            value,
            scalar_sequence,
            bounds: node.bounds().map(AccessKitTextInputBounds::from),
        }
    }

    pub(crate) const fn is_text_input(&self) -> bool {
        matches!(
            self.role,
            AccessKitTextInputRole::TextInput | AccessKitTextInputRole::MultilineTextInput
        )
    }

    pub(crate) fn snapshot_hash(&self) -> String {
        let bounds = self.bounds.map(|bounds| {
            serde_json::json!({
                "x0_bits": bounds.x0_bits,
                "y0_bits": bounds.y0_bits,
                "x1_bits": bounds.x1_bits,
                "y1_bits": bounds.y1_bits,
            })
        });
        let material = serde_json::json!({
            "role": role_name(self.role),
            "value": self.value.as_deref(),
            "scalar_sequence": self.scalar_sequence,
            "bounds": bounds,
        });
        hex::encode(sha2::Sha256::digest(material.to_string()))
    }
}

impl From<egui::accesskit::Rect> for AccessKitTextInputBounds {
    fn from(bounds: egui::accesskit::Rect) -> Self {
        Self {
            x0_bits: bounds.x0.to_bits(),
            y0_bits: bounds.y0.to_bits(),
            x1_bits: bounds.x1.to_bits(),
            y1_bits: bounds.y1.to_bits(),
        }
    }
}

fn role_name(role: AccessKitTextInputRole) -> &'static str {
    match role {
        AccessKitTextInputRole::TextInput => "text-input",
        AccessKitTextInputRole::MultilineTextInput => "multiline-text-input",
        AccessKitTextInputRole::Other => "other",
    }
}

#[cfg(test)]
#[path = "accesskit_projection_tests.rs"]
mod tests;
