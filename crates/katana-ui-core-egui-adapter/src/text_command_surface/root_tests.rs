pub(super) use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact_compositor::ArtifactCompositeError;
    use crate::context_menu::{ContextMenuPresentation, ContextMenuPresentationItem};
    use crate::text_command_surface::EguiTextCommandSurfaceArtifactError;
    use katana_ui_core::atom::TextArea;
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeAction, CommandChromeFamilyId, CommandChromeToolbar,
        FloatingCommandToolbarVisibility,
    };
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeSearchStrip, CommandChromeText, SearchControlStrings,
        SearchResultSummaryTemplate,
    };
    use katana_ui_core::molecule::structured::SearchControlStrip;
    use katana_ui_core::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

    fn base_root() -> EguiTextCommandSurfaceRoot {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()).with_toolbar(
            CommandChromeToolbar::new().action(CommandChromeAction::new("base", "基準")),
        );
        EguiTextCommandSurfaceRoot::with_identity("base-root", surface).expect("base fixture root")
    }

    #[test]
    fn derived_identity_and_root_error_conversions_are_stable() {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface());
        let root = EguiTextCommandSurfaceRoot::new(surface).expect("derived root identity");
        assert!(root.identity.starts_with("kuc.text-command-root/"));

        let errors = [
            EguiTextCommandSurfaceRootError::from(
                EguiTextCommandSurfaceError::DuplicateCommandFamilyMount {
                    family: CommandChromeFamilyId::new("duplicate"),
                },
            ),
            EguiTextCommandSurfaceRootError::from(
                EguiTextCommandSurfaceArtifactError::MissingToolbar,
            ),
            EguiTextCommandSurfaceRootError::from(ArtifactCompositeError::ZeroCanvas),
            EguiTextCommandSurfaceRootError::Serialization("invalid payload".into()),
        ];
        let messages = errors.iter().map(ToString::to_string).collect::<Vec<_>>();
        assert!(messages[0].starts_with("text-command root surface failed:"));
        assert!(messages[1].starts_with("text-command root artifact failed:"));
        assert!(messages[2].starts_with("text-command root composition failed:"));
        assert_eq!(
            messages[3],
            "text-command root serialization failed: invalid payload"
        );
    }

    #[test]
    fn every_root_artifact_error_has_a_stable_child_specific_message() {
        let cases = [
            (
                EguiTextCommandSurfaceArtifactError::MissingToolbar,
                "toolbar child requires a toolbar plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingSearch,
                "search child requires a search plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingSourceAddress,
                "source-address child requires a source-address plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingTabStrip,
                "tab-strip child requires a tab-strip plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingTabStripOverlay,
                "tab-strip overlay requires an overlay plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingFloating,
                "floating child requires a floating output",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingFloatingPaintPlan,
                "floating child requires a floating paint plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingContextMenu,
                "context-menu child requires a context-menu output",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingContextMenuPaintPlan,
                "context-menu child requires a context-menu paint plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingStatusBar,
                "status-bar child requires a status-bar paint plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingDiagnosticsList,
                "diagnostics-list child requires a diagnostics-list paint plan",
            ),
            (
                EguiTextCommandSurfaceArtifactError::MissingPreview,
                "preview child requires a preview paint plan",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn duplicate_command_family_is_rejected_before_render() {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
            .with_toolbar(CommandChromeToolbar::new().action(CommandChromeAction::new("p", "P")))
            .with_floating_toolbar(
                CommandChromeToolbar::new().action(CommandChromeAction::new("f", "F")),
                FloatingCommandToolbarVisibility::Visible,
            );
        let mut root = EguiTextCommandSurfaceRoot::with_identity("duplicate-family", surface)
            .expect("duplicate family is rejected at render time");
        let style = TextCommandSurfaceStyle::standard().expect("standard root style");
        let context = egui::Context::default();
        let mut result = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| result = Some(root.show(ui, &style)),
        );
        let error = result
            .expect("root invocation")
            .expect_err("duplicate family must fail");
        assert!(matches!(
            error,
            EguiTextCommandSurfaceRootError::Surface(
                EguiTextCommandSurfaceError::DuplicateCommandFamilyMount { .. }
            )
        ));
    }

    #[test]
    fn distinct_command_families_render_once_in_their_slots() {
        let surface = EguiTextCommandSurface::new(selected_surface())
            .with_toolbar(
                CommandChromeToolbar::new()
                    .command_family(CommandChromeFamilyId::new("primary"))
                    .action(CommandChromeAction::new("p", "P")),
            )
            .with_floating_toolbar(
                CommandChromeToolbar::new()
                    .command_family(CommandChromeFamilyId::new("floating"))
                    .action(CommandChromeAction::new("f", "F")),
                FloatingCommandToolbarVisibility::Visible,
            );
        let mut root = EguiTextCommandSurfaceRoot::with_identity("distinct-families", surface)
            .expect("distinct family root");
        let output = render(&context_for_test(), &mut root);
        assert_eq!(
            output.toolbar_record.as_ref().map(|record| {
                record
                    .actions
                    .iter()
                    .filter(|action| action.action_id == "p")
                    .count()
            }),
            Some(1)
        );
        assert_eq!(
            output
                .floating
                .as_ref()
                .and_then(|value| value.record.as_ref())
                .map(|record| {
                    record
                        .toolbar
                        .actions
                        .iter()
                        .filter(|action| action.action_id == "f")
                        .count()
                }),
            Some(1)
        );
    }

    #[test]
    fn floating_only_surface_remains_supported() {
        let surface = EguiTextCommandSurface::new(selected_surface()).with_floating_toolbar(
            CommandChromeToolbar::new().action(CommandChromeAction::new("f", "F")),
            FloatingCommandToolbarVisibility::Visible,
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity("floating-only", surface)
            .expect("floating-only root");
        let output = render(&context_for_test(), &mut root);
        assert!(output.toolbar_record.is_none());
        assert!(output.floating.and_then(|value| value.record).is_some());
    }

    #[test]
    fn status_diagnostics_lease_mounts_both_children_in_an_actual_root_frame() {
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "status-diagnostics-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )
        .expect("status diagnostics root");
        root.attach_status_diagnostics(
            StatusDiagnosticsProjectionLease::new()
                .with_status_bar(katana_ui_core::molecule::StatusBar::new("status").segment(
                    katana_ui_core::molecule::StatusBarSegment::new("status-segment", "Ready"),
                ))
                .with_diagnostics_list(katana_ui_core::molecule::DiagnosticsList::new(
                    "diagnostics",
                )),
        );

        let output = render(&context_for_test(), &mut root);

        assert!(
            output
                .artifact_order()
                .contains(&crate::text_command_surface::EguiTextCommandSurfaceChild::StatusBar)
        );
        assert!(
            output.artifact_order().contains(
                &crate::text_command_surface::EguiTextCommandSurfaceChild::DiagnosticsList,
            )
        );
    }

    struct EguiTextSurfaceForTest;

    impl EguiTextSurfaceForTest {
        fn surface() -> katana_ui_core::text_surface::TextSurface {
            let mut props = TextSurfaceProps::new(
                TextArea::new("collision-text").value("本文 ⭐️"),
                Vec::new(),
                TextSurfaceViewport::new(0, 0, 640, 360),
            );
            props.accessibility_label = "collision text".to_owned();
            TextSurface::new(props)
        }
    }

    fn render(
        context: &egui::Context,
        root: &mut EguiTextCommandSurfaceRoot,
    ) -> EguiTextCommandSurfaceRootOutput {
        let mut output = None;
        let style = TextCommandSurfaceStyle::standard().expect("standard root style");
        crate::run_ui_discard(
            context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| output = Some(root.show(ui, &style)),
        );
        output.expect("root frame").expect("root render")
    }

    fn selected_surface() -> katana_ui_core::text_surface::TextSurface {
        let value = "選択範囲 ⭐️";
        let mut props = TextSurfaceProps::new(
            TextArea::new("selected-text").value(value),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 640, 360),
        );
        props.accessibility_label = "selected text".to_owned();
        let mut presentation =
            katana_ui_core::text_surface::TextSurfacePresentation::from_props(&props);
        presentation.selection_start = 0;
        presentation.selection_end = value.len();
        let mut surface = TextSurface::new(props);
        assert!(surface.synchronize_presentation(presentation));
        surface
    }

    fn context_for_test() -> egui::Context {
        egui::Context::default()
    }

    fn search_strip() -> CommandChromeSearchStrip {
        let text = |label: &str| CommandChromeText::new(label, label, label);
        CommandChromeSearchStrip::new(
            SearchControlStrip::new("検索")
                .query("検索語")
                .replace_mode(katana_ui_core::molecule::structured::ReplaceMode::Visible)
                .replace_value("置換語")
                .result_position(2, Some(0)),
            SearchControlStrings {
                strip: text("検索"),
                query: text("検索語"),
                replace: text("置換"),
                match_case: text("大文字小文字"),
                whole_word: text("単語"),
                use_regex: text("正規表現"),
                previous: text("前へ"),
                next: text("次へ"),
                replace_one: text("置換"),
                replace_all: text("すべて置換"),
                close: text("閉じる"),
                result_summary: SearchResultSummaryTemplate {
                    empty: "検索結果なし".into(),
                    zero_results: "0".into(),
                    single_result: "1".into(),
                    indexed_result: "{active} / {count}".into(),
                    count_results: "{count}".into(),
                },
            },
        )
    }

    #[test]
    fn retained_root_shares_one_catalog_across_all_text_children() {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
            .with_toolbar(
                CommandChromeToolbar::new().action(CommandChromeAction::new("base", "基準 ⭐️")),
            )
            .with_search_strip(search_strip())
            .with_context_menu(ContextMenuPresentation {
                visible: false,
                items: vec![ContextMenuPresentationItem::action("copy", "コピー")],
            });
        let mut root = EguiTextCommandSurfaceRoot::with_text_raster_config(
            "shared-catalog-root",
            surface,
            katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
        )
        .expect("shared catalog root");
        let context = egui::Context::default();
        let _ = render(&context, &mut root);

        let root_catalog = root.evidence_catalog();
        assert_eq!(root_catalog.stats().font_database_discoveries, 1);
        let text_catalog = root.adapter.text.catalog();
        let chrome_catalog = root.adapter.chrome.catalog();
        let source_address_catalog = root.adapter.source_address.catalog();
        let menu_catalog = root
            .adapter
            .context_menu
            .as_ref()
            .expect("context-menu child is instantiated by the real frame")
            .catalog();
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &chrome_catalog));
        assert!(std::sync::Arc::ptr_eq(
            &text_catalog,
            &source_address_catalog
        ));
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &menu_catalog));
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &root.adapter.catalog));
        assert!(std::sync::Arc::ptr_eq(
            &text_catalog,
            &root.adapter.text_raster_resources.catalog(),
        ));
        assert_eq!(text_catalog.stats().font_database_discoveries, 1);
        assert_eq!(chrome_catalog.stats().font_database_discoveries, 1);
        assert_eq!(source_address_catalog.stats().font_database_discoveries, 1);
        assert_eq!(menu_catalog.stats().font_database_discoveries, 1);
        assert_eq!(text_catalog.fingerprint(), root_catalog.fingerprint());
        assert_eq!(chrome_catalog.fingerprint(), root_catalog.fingerprint());
        assert_eq!(menu_catalog.fingerprint(), root_catalog.fingerprint());
    }

    #[test]
    fn narrow_actual_root_fails_closed_before_issuing_a_selection_continuation() {
        let context = egui::Context::default();
        let mut root = base_root();
        let style = TextCommandSurfaceStyle::standard().expect("standard root style");
        let mut output = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(20.0, 80.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| output = Some(root.show(ui, &style)),
        );
        let output = output
            .expect("narrow root frame ran")
            .expect("narrow root frame renders");

        assert!(matches!(
            output.interaction_locator().begin_text_selection(),
            Err(KucTextSelectionContinuationError::Unavailable)
        ));
    }
}
