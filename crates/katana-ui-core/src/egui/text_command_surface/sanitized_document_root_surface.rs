use super::super::super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceFloatingPresentation,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceSearchPresentation,
};
use super::super::sanitized_command_projection_adapter::command_chrome_toolbar_presentation;
use super::super::sanitized_context_projection_adapter::context_menu_presentation;
use super::super::sanitized_document_root_input::SanitizedDocumentRootInput;
use super::super::sanitized_search_projection_adapter::SanitizedSearchPresentation;
use crate::atom::TextArea;
use crate::molecule::command_chrome::{CommandChromeFamilyId, FloatingCommandToolbarVisibility};
use crate::render_model::UiStateId;
use crate::text_surface::{
    TextSurface, TextSurfaceAccessibilityLabels, TextSurfaceAutomaticGutterPresentation,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

pub(super) fn from_input(
    input: &SanitizedDocumentRootInput,
) -> (EguiTextCommandSurface, EguiTextCommandSurfacePresentation) {
    let identity = format!(
        "kuc.sanitized-document/{}",
        input.identity.stable_fingerprint()
    );
    let state_id = UiStateId::new(identity.clone());
    let presentation = presentation_from_input(input);
    let text_area = TextArea::new(input.snapshot.clone())
        .stable_state_id(state_id.clone())
        .value(input.snapshot.clone())
        .readonly(input.readonly)
        .ime_enabled(true);
    let mut props =
        TextSurfaceProps::new(text_area, Vec::new(), TextSurfaceViewport::new(0, 0, 1, 1))
            .adapter_measured_viewport();
    props.accessibility_label = input.snapshot.clone();
    let text = TextSurface::new(props);
    let mut surface = EguiTextCommandSurface::new(text);
    let _ = surface.synchronize_presentation(presentation.clone());
    apply_internal_command_families(&mut surface, &identity);
    (surface, presentation)
}

fn apply_internal_command_families(surface: &mut EguiTextCommandSurface, identity: &str) {
    let primary = surface.toolbar.take().map(|toolbar| {
        toolbar.command_family(CommandChromeFamilyId::new(format!(
            "{identity}/primary-command-family"
        )))
    });
    let floating = surface.deferred_floating_toolbar.take().map(|toolbar| {
        toolbar.command_family(CommandChromeFamilyId::new(format!(
            "{identity}/floating-command-family"
        )))
    });
    surface.toolbar = primary;
    surface.deferred_floating_toolbar = floating;
    surface.synchronize_command_families(
        surface
            .toolbar
            .as_ref()
            .map(|toolbar| toolbar.command_family_id().clone()),
        surface
            .deferred_floating_toolbar
            .as_ref()
            .map(|toolbar| toolbar.command_family_id().clone()),
    );
}

pub(super) fn presentation_from_input(
    input: &SanitizedDocumentRootInput,
) -> EguiTextCommandSurfacePresentation {
    let mut presentation = base_presentation_from_input(input);
    presentation.search = input.search_projection.as_ref().map(|projection| {
        let search = SanitizedSearchPresentation::from(projection);
        EguiTextCommandSurfaceSearchPresentation {
            state_id: UiStateId::new(format!(
                "kuc.sanitized-document/{}/search",
                input.identity.stable_fingerprint()
            )),
            label: search.label,
            value: search.value,
        }
    });
    presentation
}

pub(super) fn base_presentation_from_input(
    input: &SanitizedDocumentRootInput,
) -> EguiTextCommandSurfacePresentation {
    let state_id = UiStateId::new(format!(
        "kuc.sanitized-document/{}",
        input.identity.stable_fingerprint()
    ));
    EguiTextCommandSurfacePresentation {
        text_state_id: Some(state_id),
        text: text_presentation(input),
        toolbar: input
            .command_projection
            .as_ref()
            .map(command_chrome_toolbar_presentation),
        floating: input
            .floating_command_projection
            .as_ref()
            .map(|projection| EguiTextCommandSurfaceFloatingPresentation {
                toolbar: command_chrome_toolbar_presentation(projection),
                visibility: FloatingCommandToolbarVisibility::Visible,
            }),
        search: None,
        context_menu: input
            .context_projection
            .as_ref()
            .map(context_menu_presentation),
    }
}

fn text_presentation(input: &SanitizedDocumentRootInput) -> TextSurfacePresentation {
    TextSurfacePresentation {
        value: input.snapshot.clone(),
        selection_start: 0,
        selection_end: 0,
        spans: Vec::new(),
        annotations: Vec::new(),
        automatic_gutter: Some(TextSurfaceAutomaticGutterPresentation::new()),
        accessibility_label: input.snapshot.clone(),
        accessibility_actions: TextSurfaceAccessibilityLabels::new(),
        context_target_label: None,
        disabled_reason: None,
        readonly: input.readonly,
        disabled: false,
        ime_enabled: true,
        scroll_request: None,
        focus_request: None,
    }
}

#[cfg(test)]
mod tests {
    use super::presentation_from_input;
    use crate::egui::text_command_surface::sanitized_document_root::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget, SanitizedDocumentRootIdentity, SanitizedDocumentRootInput,
        SanitizedDocumentRootStyleKey, SanitizedSearchControlPresentation,
        SanitizedSearchLocalizedPresentation, SanitizedSearchOperationPresentation,
        SanitizedSearchProjectionBuilder, SanitizedSearchResultSummaryPresentation,
        SanitizedSearchTarget, SanitizedSearchTextPresentation,
        SanitizedSearchUnavailablePresentation,
    };

    fn text(value: &str) -> SanitizedSearchTextPresentation {
        SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
    }

    fn localized() -> SanitizedSearchLocalizedPresentation {
        SanitizedSearchLocalizedPresentation::new(
            SanitizedSearchControlPresentation::new(
                text("検索"),
                text("検索語"),
                text("置換"),
                text("大文字小文字"),
                text("単語"),
                text("正規表現"),
            ),
            SanitizedSearchOperationPresentation::new(
                text("前へ"),
                text("次へ"),
                text("置換"),
                text("すべて置換"),
                text("閉じる"),
            ),
            SanitizedSearchResultSummaryPresentation::new(
                "検索待機 ⭐️",
                "一致なし",
                "1 / 1",
                "{active} / {count}",
                "{count} 件",
            ),
            SanitizedSearchUnavailablePresentation::new(
                "正規表現は利用不可 ⭐️",
                "置換は利用不可 ⭐️",
                "移動は利用不可 ⭐️",
                "閉じる操作は利用不可 ⭐️",
            ),
        )
    }

    fn command_projection(label: &str, target: u8) -> SanitizedCommandProjection {
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "command").item(
            SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes([target]),
                1,
                label,
            ),
        )])
    }

    #[test]
    fn search_root_label_comes_from_localized_projection() {
        let input = SanitizedDocumentRootInput::new(
            1,
            SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
            "日本語 ⭐️",
            SanitizedDocumentRootStyleKey::default(),
        )
        .with_search_projection(
            SanitizedSearchProjectionBuilder::new()
                .localized_presentation(localized())
                .next_enabled(true)
                .next_target(SanitizedSearchTarget::from_opaque_bytes([1]))
                .build()
                .expect("valid localized projection"),
        );

        let presentation = presentation_from_input(&input);
        let search = presentation.search.expect("search projection is present");

        assert_eq!(search.label, "検索");
        assert_eq!(search.value.strings.next.visible, "次へ");
        assert_eq!(search.value.strings.next.tooltip, "次へ ⭐️");
    }

    #[test]
    fn floating_projection_is_optional_and_defaults_to_visible() {
        let input = SanitizedDocumentRootInput::new(
            1,
            SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
            "日本語 ⭐️",
            SanitizedDocumentRootStyleKey::default(),
        )
        .with_floating_command_projection(command_projection("floating", 2));

        let presentation = presentation_from_input(&input);
        assert!(presentation.toolbar.is_none());
        let floating = presentation
            .floating
            .expect("floating projection is present");
        assert_eq!(
            floating.visibility,
            crate::molecule::command_chrome::FloatingCommandToolbarVisibility::Visible
        );
        assert_eq!(floating.toolbar.actions[0].label_model(), "floating");
    }

    #[test]
    fn top_and_floating_command_projections_are_mapped_independently() {
        let input = SanitizedDocumentRootInput::new(
            1,
            SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
            "日本語 ⭐️",
            SanitizedDocumentRootStyleKey::default(),
        )
        .with_command_projection(command_projection("top", 1))
        .with_floating_command_projection(command_projection("floating", 2));

        let presentation = presentation_from_input(&input);
        let toolbar = presentation.toolbar.expect("top projection is present");
        let floating = presentation
            .floating
            .expect("floating projection is present");

        assert_eq!(toolbar.actions[0].label_model(), "top");
        assert_eq!(floating.toolbar.actions[0].label_model(), "floating");
    }

    #[test]
    fn readonly_is_mapped_to_the_generic_text_surface_presentation() {
        let input = SanitizedDocumentRootInput::new(
            1,
            SanitizedDocumentRootIdentity::from_opaque_bytes([9]),
            "日本語 ⭐️",
            SanitizedDocumentRootStyleKey::default(),
        )
        .with_readonly(true);

        let presentation = presentation_from_input(&input);
        assert!(presentation.text.readonly);
        assert!(!presentation.text.disabled);
        assert!(presentation.text.ime_enabled);
    }
}
