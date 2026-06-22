from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class DisclosurePublicOptionRequirements:
    DISCLOSURE_MODEL_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/model.rs"
    DISCLOSURE_OPTIONS_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/options.rs"
    MODAL_OVERLAY_SOURCE = "crates/katana-ui-core/src/molecule/disclosure/modal_overlay.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.overlay_disclosures(),
            *cls.modal_overlay(),
        )

    @classmethod
    def overlay_disclosures(cls) -> tuple[PublicOptionRequirement, ...]:
        pages = ("tooltip", "popover", "modal")
        mappings = (
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn open", "open"),
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn placement", "placement"),
            (cls.DISCLOSURE_OPTIONS_SOURCE, "pub fn focus_handling", "focus"),
            (cls.DISCLOSURE_MODEL_SOURCE, "pub fn outside_click_dismiss", "dismiss"),
        )
        return cls.for_pages(pages, mappings)

    @classmethod
    def modal_overlay(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn open", "open"),
            ("pub fn placement", "placement"),
            ("pub fn focus_trap", "focus"),
            ("pub fn outside_click_dismiss", "dismiss"),
        )
        return cls.for_page("modal-overlay", cls.MODAL_OVERLAY_SOURCE, mappings)

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

    @classmethod
    def for_pages(
        cls,
        pages: tuple[str, ...],
        mappings: tuple[tuple[str, str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            PublicOptionRequirement(page, source, token, setting)
            for page in pages
            for source, token, setting in mappings
        )
