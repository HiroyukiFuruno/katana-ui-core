use super::SanitizedSearchProjection;
use super::sanitized_command_projection::SanitizedCommandProjection;
use super::sanitized_context_projection::SanitizedContextMenuProjection;
use super::sanitized_document_root_style::SanitizedDocumentRootStyleKey;
use super::sanitized_tab_projection::SanitizedTabProjection;
/// Opaque stable identity supplied by the host.
pub struct SanitizedDocumentRootIdentity {
    bytes: Box<[u8]>,
}

impl SanitizedDocumentRootIdentity {
    /// Creates an identity without assigning meaning to its bytes.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into().into_boxed_slice(),
        }
    }

    pub(crate) fn same_identity(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }

    pub(crate) fn stable_fingerprint(&self) -> String {
        SanitizedProjectionHash::sha256_signature_hex(&self.bytes)
    }
}

impl std::fmt::Debug for SanitizedDocumentRootIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SanitizedDocumentRootIdentity(..)")
    }
}

/// The complete public input for one retained document root.
pub struct SanitizedDocumentRootInput {
    pub(super) revision: u64,
    pub(super) identity: SanitizedDocumentRootIdentity,
    pub(super) snapshot: String,
    pub(super) readonly: bool,
    pub(super) style: SanitizedDocumentRootStyleKey,
    pub(super) command_projection: Option<SanitizedCommandProjection>,
    pub(super) floating_command_projection: Option<SanitizedCommandProjection>,
    pub(super) search_projection: Option<SanitizedSearchProjection>,
    pub(super) context_projection: Option<SanitizedContextMenuProjection>,
    pub(super) tab_projection: Option<SanitizedTabProjection>,
    pub(super) search_projection_fingerprint: Option<[u8; SHA256_BYTES]>,
}

impl SanitizedDocumentRootInput {
    /// Creates a document input from a valid UTF-8 snapshot.
    #[must_use]
    pub fn new(
        revision: u64,
        identity: SanitizedDocumentRootIdentity,
        snapshot: impl Into<String>,
        style: SanitizedDocumentRootStyleKey,
    ) -> Self {
        Self {
            revision,
            identity,
            snapshot: snapshot.into(),
            readonly: false,
            style,
            command_projection: None,
            floating_command_projection: None,
            search_projection: None,
            context_projection: None,
            tab_projection: None,
            search_projection_fingerprint: None,
        }
    }

    #[must_use]
    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    #[must_use]
    pub fn with_command_projection(
        mut self,
        command_projection: SanitizedCommandProjection,
    ) -> Self {
        self.command_projection = Some(command_projection);
        self
    }

    #[must_use]
    pub fn with_floating_command_projection(
        mut self,
        floating_command_projection: SanitizedCommandProjection,
    ) -> Self {
        self.floating_command_projection = Some(floating_command_projection);
        self
    }

    #[must_use]
    pub fn with_search_projection(mut self, search_projection: SanitizedSearchProjection) -> Self {
        self.search_projection_fingerprint = Some(search_projection.stable_fingerprint());
        self.search_projection = Some(search_projection);
        self
    }

    #[must_use]
    pub fn with_context_projection(
        mut self,
        context_projection: SanitizedContextMenuProjection,
    ) -> Self {
        self.context_projection = Some(context_projection);
        self
    }

    #[must_use]
    pub fn with_tab_projection(mut self, tab_projection: SanitizedTabProjection) -> Self {
        self.tab_projection = Some(tab_projection);
        self
    }

    pub(crate) fn same_command_projection_as(&self, other: &Self) -> bool {
        let commands_match = match (&self.command_projection, &other.command_projection) {
            (None, None) => true,
            (Some(left), Some(right)) => left.stable_fingerprint() == right.stable_fingerprint(),
            _ => false,
        };
        let floating_commands_match = match (
            &self.floating_command_projection,
            &other.floating_command_projection,
        ) {
            (None, None) => true,
            (Some(left), Some(right)) => left.stable_fingerprint() == right.stable_fingerprint(),
            _ => false,
        };
        commands_match
            && floating_commands_match
            && self.readonly == other.readonly
            && self.same_search_projection_as(other)
            && self.same_context_projection_as(other)
            && self.same_tab_projection_as(other)
    }

    pub(crate) fn same_search_projection_as(&self, other: &Self) -> bool {
        self.search_projection_fingerprint == other.search_projection_fingerprint
    }

    pub(crate) fn same_context_projection_as(&self, other: &Self) -> bool {
        match (&self.context_projection, &other.context_projection) {
            (None, None) => true,
            (Some(left), Some(right)) => left.same_as(right),
            _ => false,
        }
    }

    pub(crate) fn same_tab_projection_as(&self, other: &Self) -> bool {
        match (&self.tab_projection, &other.tab_projection) {
            (None, None) => true,
            (Some(left), Some(right)) => left.same_as(right),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SanitizedDocumentRootIdentity, SanitizedDocumentRootInput};
    use crate::text_command_surface::sanitized_document_root::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget, SanitizedContextMenuItem, SanitizedContextMenuProjectionBuilder,
        SanitizedContextMenuTarget, SanitizedDocumentRootStyleKey,
        SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
        SanitizedSearchOperationPresentation, SanitizedSearchProjectionBuilder,
        SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget,
        SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation, SanitizedTab,
        SanitizedTabGroup, SanitizedTabProjection, SanitizedTabTarget,
    };

    #[test]
    fn root_identity_debug_is_opaque() {
        let identity = SanitizedDocumentRootIdentity::from_opaque_bytes(b"secret".to_vec());
        let debug = format!("{identity:?}");
        assert_eq!(debug, "SanitizedDocumentRootIdentity(..)");
        assert!(!debug.contains("secret"));
    }

    fn search_text(value: &str) -> SanitizedSearchTextPresentation {
        SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
    }

    fn localized_search(next: &str) -> SanitizedSearchLocalizedPresentation {
        SanitizedSearchLocalizedPresentation::new(
            SanitizedSearchControlPresentation::new(
                search_text("検索"),
                search_text("検索語"),
                search_text("置換"),
                search_text("大文字小文字"),
                search_text("単語"),
                search_text("正規表現"),
            ),
            SanitizedSearchOperationPresentation::new(
                search_text("前へ"),
                search_text(next),
                search_text("置換"),
                search_text("すべて置換"),
                search_text("閉じる"),
            ),
            SanitizedSearchResultSummaryPresentation::new(
                "検索待機 ⭐️",
                "一致なし ⭐️",
                "一件 ⭐️",
                "位置 ⭐️",
                "件数 ⭐️",
            ),
            SanitizedSearchUnavailablePresentation::new(
                "正規表現は利用不可 ⭐️",
                "置換は利用不可 ⭐️",
                "移動は利用不可 ⭐️",
                "閉じる操作は利用不可 ⭐️",
            ),
        )
    }

    fn search_projection(target: u8) -> super::SanitizedSearchProjection {
        SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized_search("次へ"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([target]))
            .build()
            .expect("localized search projection is valid")
    }

    fn context_projection(target: u8) -> super::SanitizedContextMenuProjection {
        SanitizedContextMenuProjectionBuilder::new()
            .item(SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes([target]),
                1,
                "表示",
            ))
            .build()
    }

    #[test]
    fn identity_comparison_and_fingerprint_are_stable() {
        let first = SanitizedDocumentRootIdentity::from_opaque_bytes([1, 2, 3]);
        let same = SanitizedDocumentRootIdentity::from_opaque_bytes([1, 2, 3]);
        let different = SanitizedDocumentRootIdentity::from_opaque_bytes([1, 2, 4]);

        assert!(first.same_identity(&same));
        assert!(!first.same_identity(&different));
        assert_eq!(first.stable_fingerprint(), first.stable_fingerprint());
        assert_ne!(first.stable_fingerprint(), different.stable_fingerprint());
        assert_eq!(first.stable_fingerprint().len(), 64);
    }

    #[test]
    fn search_builder_is_retained_without_a_public_getter() {
        let base = |target| {
            SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
            .with_search_projection(search_projection(target))
        };

        assert!(base(1).same_search_projection_as(&base(1)));
        assert!(!base(1).same_search_projection_as(&base(2)));
    }

    #[test]
    fn context_projection_comparison_is_equal_for_equal_input() {
        let base = |target| {
            SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
            .with_context_projection(context_projection(target))
        };

        assert!(base(1).same_context_projection_as(&base(1)));
    }

    fn command_projection(target: u8) -> SanitizedCommandProjection {
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "操作").item(
            SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes([target]),
                1,
                "実行",
            ),
        )])
    }

    #[test]
    fn floating_command_projection_is_optional_and_compared_separately() {
        let base = |projection| {
            SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
            .with_floating_command_projection(projection)
        };

        let first_projection = command_projection(1);
        let same_projection = command_projection(1);
        let different_projection = command_projection(2);

        assert_eq!(
            first_projection.stable_fingerprint(),
            same_projection.stable_fingerprint()
        );
        assert_ne!(
            first_projection.stable_fingerprint(),
            different_projection.stable_fingerprint()
        );
        assert!(base(first_projection).same_command_projection_as(&base(same_projection)));
        assert!(
            !base(command_projection(1)).same_command_projection_as(&base(different_projection))
        );
        assert!(!base(command_projection(1)).same_command_projection_as(
            &SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
        ));
        assert!(!SanitizedDocumentRootInput::new(
            1,
            SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
            "本文",
            SanitizedDocumentRootStyleKey::default(),
        )
        .same_command_projection_as(&base(command_projection(1))));
        assert!(!SanitizedDocumentRootInput::new(
            1,
            SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
            "本文",
            SanitizedDocumentRootStyleKey::default(),
        )
        .with_command_projection(command_projection(1))
        .same_command_projection_as(
            &SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
            .with_floating_command_projection(command_projection(1)),
        ));
    }

    #[test]
    fn context_projection_comparison_rejects_different_or_missing_input() {
        let base = |target| {
            SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
            .with_context_projection(context_projection(target))
        };

        assert!(!base(1).same_context_projection_as(&base(2)));
        assert!(
            !base(1).same_context_projection_as(&SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            ))
        );
    }

    fn tab_projection(target: u8) -> SanitizedTabProjection {
        SanitizedTabProjection::new([SanitizedTabGroup::new(
            crate::text_command_surface::sanitized_document_root::sanitized_tab_projection::SanitizedTabGroupTarget::from_opaque_bytes([1]),
            1,
            "タブ",
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([target]),
            1,
            "編集",
        ))])
    }

    #[test]
    fn tab_projection_comparison_is_equal_for_equal_input() {
        let base = |target| {
            SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
            .with_tab_projection(tab_projection(target))
        };

        assert!(base(1).same_tab_projection_as(&base(1)));
    }

    #[test]
    fn tab_projection_comparison_rejects_different_or_missing_input() {
        let base = |target| {
            SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            )
            .with_tab_projection(tab_projection(target))
        };

        assert!(!base(1).same_tab_projection_as(&base(2)));
        assert!(
            !base(1).same_tab_projection_as(&SanitizedDocumentRootInput::new(
                1,
                SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
                "本文",
                SanitizedDocumentRootStyleKey::default(),
            ))
        );
    }
}
