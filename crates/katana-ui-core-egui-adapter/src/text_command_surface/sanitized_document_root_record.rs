use super::super::root::EguiTextCommandSurfaceRootOutput;

/// Closed root record containing only root-level opaque proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SanitizedDocumentRootRecord {
    identity: String,
    revision: u64,
    state_revision: u64,
    dimensions: SanitizedDocumentRootRecordDimensions,
    rgba_hash: String,
    paint_plan_hash: String,
    record_hash: String,
    accessibility_snapshot_hash: String,
}

impl SanitizedDocumentRootRecord {
    pub(super) fn from_output(revision: u64, output: &EguiTextCommandSurfaceRootOutput) -> Self {
        let frame = output.frame();
        let dimensions = frame.dimensions();
        Self {
            identity: frame.identity().to_owned(),
            revision,
            state_revision: frame.state_revision(),
            dimensions: SanitizedDocumentRootRecordDimensions {
                width: dimensions.width(),
                height: dimensions.height(),
            },
            rgba_hash: frame.rgba_hash().to_owned(),
            paint_plan_hash: frame.paint_plan_hash().to_owned(),
            record_hash: frame.record_hash().to_owned(),
            accessibility_snapshot_hash: frame.accessibility().snapshot_hash().to_owned(),
        }
    }

    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn presentation_revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    #[must_use]
    pub const fn dimensions(&self) -> SanitizedDocumentRootRecordDimensions {
        self.dimensions
    }

    #[must_use]
    pub fn rgba_hash(&self) -> &str {
        &self.rgba_hash
    }

    #[must_use]
    pub fn paint_plan_hash(&self) -> &str {
        &self.paint_plan_hash
    }

    #[must_use]
    pub fn record_hash(&self) -> &str {
        &self.record_hash
    }

    #[must_use]
    pub fn accessibility_snapshot_hash(&self) -> &str {
        &self.accessibility_snapshot_hash
    }
}

/// Root dimensions without child geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SanitizedDocumentRootRecordDimensions {
    width: u32,
    height: u32,
}

impl SanitizedDocumentRootRecordDimensions {
    #[must_use]
    pub const fn width(self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> u32 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn public_record_has_no_child_payload_or_pixel_storage() {
        let source = include_str!("sanitized_document_root_record.rs");
        let public = source
            .split_once("pub struct SanitizedDocumentRootRecord")
            .expect("record declaration exists")
            .1
            .split_once("impl SanitizedDocumentRootRecord")
            .expect("record implementation exists")
            .0;
        for forbidden in [
            "child_geometry",
            "payload",
            "rgba_pixels",
            "texture",
            "accesskit_nodes",
        ] {
            assert!(!public.contains(forbidden), "record leaked `{forbidden}`");
        }
    }
}
