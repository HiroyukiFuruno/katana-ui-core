pub(super) use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_menu::{ContextMenuPresentation, ContextMenuPresentationItem};
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

    fn raw_input_snapshot(input: &egui::RawInput) -> String {
        format!("{input:#?}")
    }

    fn collision_root() -> EguiTextCommandSurfaceRoot {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()).with_toolbar(
            CommandChromeToolbar::new().action(CommandChromeAction::new("base", "基準")),
        );
        EguiTextCommandSurfaceRoot::with_identity("collision-root", surface)
    }

    #[test]
    fn derived_identity_and_root_error_conversions_are_stable() {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface());
        let root = EguiTextCommandSurfaceRoot::new(surface);
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
    fn duplicate_command_family_is_rejected_before_render() {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
            .with_toolbar(CommandChromeToolbar::new().action(CommandChromeAction::new("p", "P")))
            .with_floating_toolbar(
                CommandChromeToolbar::new().action(CommandChromeAction::new("f", "F")),
                FloatingCommandToolbarVisibility::Visible,
            );
        let mut root = EguiTextCommandSurfaceRoot::with_identity("duplicate-family", surface);
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
            |ui| result = Some(root.show(ui, &TextCommandSurfaceStyle::standard())),
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
        let mut root = EguiTextCommandSurfaceRoot::with_identity("distinct-families", surface);
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
        let mut root = EguiTextCommandSurfaceRoot::with_identity("floating-only", surface);
        let output = render(&context_for_test(), &mut root);
        assert!(output.toolbar_record.is_none());
        assert!(output.floating.and_then(|value| value.record).is_some());
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
        crate::run_ui_discard(
            context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| output = Some(root.show(ui, &TextCommandSurfaceStyle::standard())),
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
        );
        let context = egui::Context::default();
        let _ = render(&context, &mut root);

        let root_catalog = root.evidence_catalog();
        assert_eq!(root_catalog.stats().font_database_discoveries, 1);
        let text_catalog = root.adapter.text.catalog();
        let chrome_catalog = root.adapter.chrome.catalog();
        let menu_catalog = root
            .adapter
            .context_menu
            .as_ref()
            .expect("context-menu child is instantiated by the real frame")
            .catalog();
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &chrome_catalog));
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &menu_catalog));
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &root.adapter.catalog));
        assert_eq!(text_catalog.stats().font_database_discoveries, 1);
        assert_eq!(chrome_catalog.stats().font_database_discoveries, 1);
        assert_eq!(menu_catalog.stats().font_database_discoveries, 1);
        assert_eq!(text_catalog.fingerprint(), root_catalog.fingerprint());
        assert_eq!(chrome_catalog.fingerprint(), root_catalog.fingerprint());
        assert_eq!(menu_catalog.fingerprint(), root_catalog.fingerprint());
    }

    #[test]
    fn actual_root_same_bounds_is_fail_closed_without_input_or_effect_dispatch() {
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut root = collision_root();
        let output = render(&context, &mut root);
        let before_context = output.events().current_context();
        let input = egui::RawInput::default();
        let before_input = raw_input_snapshot(&input);
        for identity in ["collision-left", "collision-right"] {
            assert!(matches!(
                output
                    .interaction_locator()
                    .request(KucInteractionSelector::new(
                        identity,
                        KucInteractionActionClass::Toolbar,
                    )),
                Err(KucInteractionLocatorError::Ambiguous)
            ));
            assert_eq!(raw_input_snapshot(&input), before_input);
        }
        let unknown = output
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "collision-unknown",
                KucInteractionActionClass::Toolbar,
            ));
        assert!(matches!(unknown, Err(KucInteractionLocatorError::Missing)));
        assert_eq!(raw_input_snapshot(&input), before_input);
        assert_eq!(output.events().current_context(), before_context);

        let effect_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let effect_count_for_handler = effect_count.clone();
        let effect = KucOpaqueHostEffectBatch::from_handler(move || {
            effect_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        });
        output
            .events()
            .attach_opaque_host_effect_batch(effect)
            .expect("effect attached to untouched batch");
        assert_eq!(effect_count.load(std::sync::atomic::Ordering::SeqCst), 0);
    }
}
