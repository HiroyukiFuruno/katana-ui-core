use super::group::SanitizedCommandGroup;
use crate::render_model::UiIconProps;
use sha2::{Digest, Sha256};

/// Generic command projection retained by the KUC root.
#[derive(Debug, Default)]
pub struct SanitizedCommandProjection {
    groups: Vec<SanitizedCommandGroup>,
}

impl SanitizedCommandProjection {
    #[must_use]
    pub fn new(groups: impl Into<Vec<SanitizedCommandGroup>>) -> Self {
        Self {
            groups: groups.into(),
        }
    }

    #[must_use]
    pub(crate) fn groups(&self) -> &[SanitizedCommandGroup] {
        &self.groups
    }

    pub(crate) fn stable_fingerprint(&self) -> String {
        let mut digest = Sha256::new();
        digest.update(b"kuc.sanitized-command-projection/v1\0");

        for group in &self.groups {
            digest.update(b"group");
            digest.update(group.order().to_le_bytes());
            hash_text(&mut digest, group.label().as_bytes());
            hash_optional_text(&mut digest, group.tooltip());
            hash_optional_text(&mut digest, group.accessibility_label());
            hash_optional_icon(&mut digest, group.icon());
            digest.update([u8::from(group.enabled())]);
            digest.update([u8::from(group.visible())]);

            for item in group.items() {
                digest.update(b"item");
                digest.update(item.order().to_le_bytes());
                digest.update(item.target().stable_fingerprint().as_bytes());
                hash_text(&mut digest, item.label().as_bytes());
                hash_optional_text(&mut digest, item.tooltip());
                hash_optional_text(&mut digest, item.accessibility_label());
                hash_optional_icon(&mut digest, item.icon());
                digest.update([u8::from(item.enabled())]);
                digest.update([u8::from(item.visible())]);

                for dropdown in item.dropdown_items() {
                    digest.update(b"dropdown");
                    digest.update(dropdown.order().to_le_bytes());
                    digest.update(dropdown.target().stable_fingerprint().as_bytes());
                    hash_text(&mut digest, dropdown.label().as_bytes());
                    hash_optional_text(&mut digest, dropdown.tooltip());
                    hash_optional_text(&mut digest, dropdown.accessibility_label());
                    hash_optional_icon(&mut digest, dropdown.icon());
                    digest.update([u8::from(dropdown.enabled())]);
                    digest.update([u8::from(dropdown.visible())]);
                }
            }
        }

        hex::encode(digest.finalize())
    }
}

fn hash_optional_icon(digest: &mut Sha256, icon: Option<&UiIconProps>) {
    match icon {
        Some(icon) => {
            digest.update(b"icon");
            let rendered = format!("{icon:?}");
            hash_text(digest, rendered.as_bytes());
        }
        None => {
            digest.update(b"icon-none");
        }
    }
}

fn hash_optional_text(digest: &mut Sha256, text: Option<&str>) {
    match text {
        Some(text) => {
            digest.update(b"some");
            hash_text(digest, text.as_bytes());
        }
        None => {
            digest.update(b"none");
        }
    }
}

fn hash_text(digest: &mut Sha256, value: &[u8]) {
    digest.update((value.len() as u64).to_le_bytes());
    digest.update(value);
}
