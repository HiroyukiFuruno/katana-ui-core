from __future__ import annotations

from storybook_ui_harness_public_option_model import PublicOptionRequirement


class AtomPublicOptionRequirements:
    ACTION_POLICY_SOURCE = "crates/katana-ui-core/src/atom/action_policy.rs"
    ATOM_SOURCE = "crates/katana-ui-core/src/atom/mod.rs"
    ATOM_OPTIONS_SOURCE = "crates/katana-ui-core/src/atom/options.rs"
    ATOM_TYPED_SOURCE = "crates/katana-ui-core/src/atom/typed.rs"
    SKELETON_SOURCE = "crates/katana-ui-core/src/atom/skeleton/types.rs"

    @classmethod
    def all(cls) -> tuple[PublicOptionRequirement, ...]:
        return (
            *cls.text(),
            *cls.content_primitives(),
            *cls.icon(),
            *cls.primitives(),
            *cls.loading_indicators(),
            *cls.progress_bar(),
            *cls.badge(),
            *cls.skeleton(),
        )

    @classmethod
    def text(cls) -> tuple[PublicOptionRequirement, ...]:
        atom_mappings = (
            ("pub fn font_role", "text.role"),
            ("pub fn value", "text.content"),
            ("pub fn value", "text.script"),
            ("pub fn tone", "text.color"),
        )
        option_mappings = (
            ("pub fn text_role", "text.role"),
            ("pub fn text_color_token", "text.color_token"),
            ("pub fn line_metrics", "text.line_metrics"),
            ("pub fn vertical_centered", "text.vertical_centered"),
            ("pub fn text_spans", "text.spans"),
            ("pub fn wrap", "text.wrap"),
        )
        return (
            *cls.for_page("text", cls.ATOM_SOURCE, atom_mappings),
            *cls.for_page("text", cls.ATOM_OPTIONS_SOURCE, option_mappings),
        )

    @classmethod
    def content_primitives(cls) -> tuple[PublicOptionRequirement, ...]:
        pages = ("icon", "key-cap")
        mappings = (
            ("pub fn value", "content.value"),
            ("pub fn visual_role", "visual.role"),
            ("pub fn accessibility_label", "a11y.label"),
            ("pub fn tone", "theme.color"),
        )
        return cls.for_pages(pages, cls.ATOM_SOURCE, mappings)

    @classmethod
    def icon(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn svg_source", "icon.svg_source"),
            ("pub fn svg_icon", "icon.svg_icon"),
            ("pub fn icon_view_box", "icon.view_box"),
            ("pub fn icon_path_summary", "icon.path_summary"),
            ("pub fn icon_paint_policy", "icon.paint_policy"),
            ("pub fn icon_role", "icon.role"),
            ("pub fn icon_color_token", "icon.color_token"),
            ("pub fn icon_theme_token", "icon.theme_token"),
        )
        return cls.for_page("icon", cls.ATOM_TYPED_SOURCE, mappings)

    @classmethod
    def loading_indicators(cls) -> tuple[PublicOptionRequirement, ...]:
        pages = ("loading-dots", "spinner", "progress-bar")
        mappings = (
            ("pub fn animation_state", "loading.animation_state"),
            ("pub fn loading_label", "loading.label"),
            ("pub fn speed_ms", "loading.speed_ms"),
            ("pub fn dot_count", "loading.dot_count"),
            ("pub fn reduced_motion", "loading.reduced_motion"),
        )
        return cls.for_pages(pages, cls.ATOM_TYPED_SOURCE, mappings)

    @classmethod
    def progress_bar(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (("pub fn progress", "progress.percent"),)
        return cls.for_page("progress-bar", cls.ATOM_SOURCE, mappings)

    @classmethod
    def primitives(cls) -> tuple[PublicOptionRequirement, ...]:
        pages = (
            "divider",
            "spacer",
            "color-swatch",
            "slide-control",
        )
        mappings = (
            ("pub fn variant", "variant"),
            ("pub fn tone", "tone"),
            ("pub fn size", "size"),
            ("pub fn theme_slot", "theme.slot"),
        )
        shared_pages = (
            "loading-dots",
            "spinner",
            "progress-bar",
        )
        shared_mappings = (
            ("pub fn variant", "variant"),
            ("pub fn tone", "tone"),
            ("pub fn size", "size"),
        )
        return (
            *cls.for_pages(pages, cls.ATOM_SOURCE, mappings),
            *cls.for_pages(shared_pages, cls.ATOM_SOURCE, shared_mappings),
        )

    @classmethod
    def badge(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub fn severity", "status.severity"),
            ("pub fn tone", "tone"),
            ("pub fn variant", "variant"),
            ("pub fn size", "size"),
        )
        return (
            *cls.for_page("badge", cls.ATOM_SOURCE, mappings),
            *cls.for_page(
                "badge",
                cls.ATOM_TYPED_SOURCE,
                (("pub fn leading_icon", "badge.leading_icon"),),
            ),
            *cls.for_page(
                "badge",
                cls.ACTION_POLICY_SOURCE,
                (("fn is_passive_status_action", "badge.passive"),),
            ),
        )

    @classmethod
    def skeleton(cls) -> tuple[PublicOptionRequirement, ...]:
        mappings = (
            ("pub enum SkeletonShape", "skeleton.shape"),
            ("lines:", "skeleton.text_lines"),
            ("last_line_ratio", "skeleton.last_line_ratio"),
            ("thickness:", "skeleton.line_thickness"),
            ("pub fn size", "size"),
            ("pub fn animation", "skeleton.animation"),
            ("pub fn tone", "tone"),
            ("pub fn radius_px", "skeleton.radius_px"),
            ("pub fn reduced_motion", "skeleton.reduced_motion"),
            ("pub fn accessibility_label", "a11y.label"),
            ("pub fn aspect_ratio", "skeleton.aspect_ratio"),
        )
        return cls.for_page("skeleton", cls.SKELETON_SOURCE, mappings)

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
        source: str,
        mappings: tuple[tuple[str, str], ...],
    ) -> tuple[PublicOptionRequirement, ...]:
        return tuple(
            requirement
            for page in pages
            for requirement in cls.for_page(page, source, mappings)
        )
