from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement
from storybook_ui_harness_public_options_molecules import MoleculePublicOptionRequirements


class AdditionalPublicOptionRequirements:
    ATTACHMENT_CHIP_SOURCE = "crates/katana-ui-core/src/molecule/attachment_chip/model.rs"
    ATTACHMENT_CHIP_TYPES_SOURCE = "crates/katana-ui-core/src/molecule/attachment_chip/types.rs"
    CHIP_SOURCE = "crates/katana-ui-core/src/atom/chip/model.rs"
    CHIP_GROUP_SOURCE = "crates/katana-ui-core/src/molecule/chip_group/model.rs"
    SHORTCUT_COMBO_SOURCE = "crates/katana-ui-core/src/atom/shortcut_combo.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.attachment_chip(),
            *cls.chip(),
            *cls.chip_group(),
            *cls.shortcut_combo(),
            *MoleculePublicOptionRequirements.all(),
        )

    @classmethod
    def attachment_chip(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            (cls.ATTACHMENT_CHIP_TYPES_SOURCE, "pub enum AttachmentKind", "attachment.kind"),
            (
                cls.ATTACHMENT_CHIP_SOURCE,
                "pub fn new(kind: AttachmentKind, name:",
                "attachment.name",
            ),
            (cls.ATTACHMENT_CHIP_SOURCE, "pub fn meta", "attachment.meta"),
            (cls.ATTACHMENT_CHIP_SOURCE, "pub fn thumbnail", "attachment.thumbnail"),
            (cls.ATTACHMENT_CHIP_SOURCE, "pub fn status", "attachment.status"),
            (cls.ATTACHMENT_CHIP_SOURCE, "pub fn progress", "attachment.progress"),
            (
                cls.ATTACHMENT_CHIP_SOURCE,
                "pub fn retry_action_label",
                "attachment.retry",
            ),
        )
        return cls.for_page_sources("attachment-chip", mappings)

    @classmethod
    def chip(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn new(label:", "chip.label"),
            ("pub fn leading_icon", "chip.leading_icon"),
            ("pub fn trailing_icon", "chip.trailing_icon"),
            ("pub fn variant", "chip.variant"),
            ("pub fn tone", "chip.tone"),
            ("pub fn size", "chip.size"),
            ("pub fn interactive", "chip.interactive"),
            ("pub fn selected", "chip.selected"),
            ("pub fn disabled", "chip.disabled"),
            ("pub fn dismissible", "chip.dismissible"),
            ("pub fn accessibility_label", "chip.a11y_label"),
            ("pub fn focused", "chip.focused"),
        )
        return cls.for_page("chip", cls.CHIP_SOURCE, mappings)

    @classmethod
    def chip_group(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn new", "chip_group.label"),
            ("pub fn chip", "chip_group.chip_count"),
            ("pub const fn wrap", "chip_group.wrap"),
            ("pub const fn overflow", "chip_group.overflow"),
            ("pub const fn reorder", "chip_group.reorder"),
            ("pub const fn gap", "chip_group.gap"),
            ("pub const fn available_width", "chip_group.available_width"),
            ("pub const fn available_width", "chip_group.hidden_count"),
            ("pub const fn overflow_trigger_width", "chip_group.overflow_trigger_width"),
        )
        return cls.for_page("chip-group", cls.CHIP_GROUP_SOURCE, mappings)

    @classmethod
    def shortcut_combo(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn platform_display", "shortcut_combo.platform_display"),
            ("pub fn separator", "shortcut_combo.separator"),
            ("pub fn size", "shortcut_combo.size"),
            ("pub fn tone", "shortcut_combo.tone"),
            ("pub fn accessibility_label", "shortcut_combo.a11y_label"),
        )
        return cls.for_page("shortcut-combo", cls.SHORTCUT_COMBO_SOURCE, mappings)

    @staticmethod
    def for_page(
        page: str,
        source: str,
        mappings: tuple[tuple[str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            PublicOptionRequirement(page, source, token, setting)
            for token, setting in mappings
        )

    @staticmethod
    def for_page_sources(
        page: str,
        mappings: tuple[tuple[str, str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            PublicOptionRequirement(page, source, token, setting)
            for source, token, setting in mappings
        )
