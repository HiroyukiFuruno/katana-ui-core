use super::item::SanitizedContextMenuItem;
use sha2::{Digest, Sha256};

/// Generic context-menu projection retained by a future KUC root.
#[derive(Default)]
pub struct SanitizedContextMenuProjection {
    items: Vec<SanitizedContextMenuItem>,
}

impl std::fmt::Debug for SanitizedContextMenuProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedContextMenuProjection")
            .field("item_count", &self.items.len())
            .field("fingerprint", &self.stable_fingerprint())
            .finish()
    }
}

impl SanitizedContextMenuProjection {
    #[must_use]
    pub fn new(items: impl Into<Vec<SanitizedContextMenuItem>>) -> Self {
        Self {
            items: items.into(),
        }
    }

    #[must_use]
    pub(crate) fn items(&self) -> &[SanitizedContextMenuItem] {
        &self.items
    }

    pub(crate) fn same_as(&self, other: &Self) -> bool {
        self.stable_fingerprint() == other.stable_fingerprint()
    }

    pub(crate) fn stable_fingerprint(&self) -> String {
        let mut digest = Sha256::new();
        digest.update((self.items.len() as u64).to_le_bytes());
        for item in &self.items {
            item.update_fingerprint(&mut digest);
        }
        hex::encode(digest.finalize())
    }
}
