#[test]
fn retained_root_shares_one_catalog_across_all_text_children()
-> Result<(), Box<dyn std::error::Error>> {
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
    )?;
    let context = egui::Context::default();
    let _ = render(&context, &mut root)?;

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
    assert!(std::sync::Arc::ptr_eq(&text_catalog, &source_address_catalog));
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

    Ok(())
}

#[test]
fn actual_root_shares_one_metrics_frame_across_text_slots_and_scales()
-> Result<(), Box<dyn std::error::Error>> {
    let value = "本文 日本語 ⭐️\n二行目";
    let mut surface = selected_surface();
    let mut presentation =
        katana_ui_core::text_surface::TextSurfacePresentation::from_props(surface.props());
    presentation.value = value.to_owned();
    presentation.selection_start = 0;
    presentation.selection_end = value.len();
    presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    assert!(surface.synchronize_presentation(presentation));
    let surface = EguiTextCommandSurface::new(surface)
        .with_toolbar(
            CommandChromeToolbar::new()
                .command_family(CommandChromeFamilyId::new("metrics-primary"))
                .action(CommandChromeAction::new("base", "本文 ⭐️")),
        )
        .with_floating_toolbar(
            CommandChromeToolbar::new()
                .command_family(CommandChromeFamilyId::new("metrics-floating"))
                .action(CommandChromeAction::new("float", "選択")),
            FloatingCommandToolbarVisibility::Visible,
        )
        .with_search_strip(search_strip())
        .with_context_menu(ContextMenuPresentation {
            visible: true,
            items: vec![ContextMenuPresentationItem::action(
                "context-format",
                "整形 ⭐️",
            )],
        });
    let mut root = EguiTextCommandSurfaceRoot::with_identity("metrics-frame", surface)?;
    let context = context_for_test();

    let initial = render(&context, &mut root)?;
    let mut context_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .request_context_open()
        .expect("context menu opener is exposed by the actual root frame")
        .apply_to_raw_input_once(&mut context_input)
        .expect("context menu opener request is one-shot");
    let opened = render_with_input(&context, &mut root, context_input)?;
    assert!(opened.context_menu_record.is_some());
    let first = root.adapter.metrics.borrow().clone();
    assert!(first.records().iter().any(|metric| metric.text == value));
    assert!(first.records().iter().any(|metric| metric.text == "1"));
    assert!(first.records().iter().any(|metric| metric.text == "検索語"));
    assert!(
        first
            .records()
            .iter()
            .any(|metric| metric.text == "本文 ⭐️")
    );
    assert!(first.records().iter().any(|metric| metric.text == "選択"));
    assert!(
        first
            .records()
            .iter()
            .any(|metric| metric.text == "整形 ⭐️")
    );
    assert!(
        first
            .records()
            .iter()
            .any(|metric| metric.text.contains("⭐️") && !metric.text.contains('☆'))
    );

    let _ = render(&context, &mut root)?;
    assert!(
        root.adapter
            .metrics
            .borrow()
            .records()
            .iter()
            .all(|metric| metric.scale_factor == 1.0)
    );

    context.set_pixels_per_point(2.0);
    let _ = render(&context, &mut root)?;
    let scaled = root.adapter.metrics.borrow().clone();
    assert!(
        scaled
            .records()
            .iter()
            .all(|metric| metric.scale_factor == 2.0)
    );
    assert_ne!(scaled, first);

    Ok(())
}
