#[test]
fn real_egui_root_record_changes_when_command_projection_is_updated() -> Result<(), String> {
    let context = egui::Context::default();
    let mut process = SanitizedDocumentRootProcess::new(input_with_projection(
        1,
        b"doc",
        "日本語 ⭐️",
        projection("first"),
    ))?;
    let first = render_record(&mut process, &context)?;

    process
        .synchronize(input_with_projection(
            2,
            b"doc",
            "日本語 ⭐️",
            projection("second"),
        ))
        .map_err(|error| format!("{error:?}"))?;

    let second = render_record(&mut process, &context)?;

    assert_ne!(first.record_hash(), second.record_hash());
    Ok(())
}

#[test]
fn real_egui_root_record_changes_when_tab_projection_is_added() -> Result<(), String> {
    let context = egui::Context::default();
    let mut process = SanitizedDocumentRootProcess::new(input(1, b"doc", "本文"))?;
    let first = render_record(&mut process, &context)?;

    process
        .synchronize(input_with_tab_projection(
            2,
            b"doc",
            "本文",
            tab_projection("次の文書"),
        ))
        .map_err(|error| format!("{error:?}"))?;
    let second = render_record(&mut process, &context)?;

    assert!(process.tab_rendered);
    assert_ne!(first.record_hash(), second.record_hash());
    Ok(())
}

#[test]
fn physical_pointer_click_selects_tab_at_sanitized_root_boundary() -> Result<(), String> {
    let context = egui::Context::default();
    let mut process = SanitizedDocumentRootProcess::new(input_with_tab_projection(
        1,
        b"doc",
        "本文 ⭐️",
        tab_projection("次の文書"),
    ))?;
    let _ = render_record(&mut process, &context)?;
    let target = process
        .tab_frame
        .as_ref()
        .ok_or_else(|| "tab frame is not retained".to_owned())?
        .boundary_facts()
        .tab_rects
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .ok_or_else(|| "second tab widget rect is zero or absent".to_owned())?;

    let _ = render_record_with_events(
        &mut process,
        &context,
        vec![egui::Event::PointerButton {
            pos: target,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    let _ = render_record_with_events(
        &mut process,
        &context,
        vec![egui::Event::PointerButton {
            pos: target,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    )?;

    assert_eq!(
        process.tab_adapter.active_tab_id(),
        Some("sanitized-tab-0-1")
    );
    let frame = process
        .tab_frame
        .as_ref()
        .ok_or_else(|| "release frame is not retained".to_owned())?;
    let facts = frame.boundary_facts();
    assert!(facts.widget_rect.width() > 0.0);
    assert!(facts.events.iter().any(|event| matches!(
        event,
        CloseableTabStripEvent::TabSelected { tab_id }
            if tab_id.as_str() == "sanitized-tab-0-1"
    )));
    Ok(())
}

#[test]
fn real_egui_root_record_changes_when_search_projection_is_updated() -> Result<(), String> {
    let context = egui::Context::default();
    let mut process = SanitizedDocumentRootProcess::new(
        input_with_search_projection(1, b"doc", "日本語 ⭐️", search_projection("次へ", 1)?)
            .with_command_projection(projection("stable")),
    )?;
    let first = render_record(&mut process, &context)?;

    process
        .synchronize(
            input_with_search_projection(2, b"doc", "日本語 ⭐️", search_projection("次の一致", 2)?)
                .with_command_projection(projection("stable")),
        )
        .map_err(|error| format!("{error:?}"))?;

    let second = render_record(&mut process, &context)?;

    assert_ne!(first.record_hash(), second.record_hash());
    Ok(())
}

#[test]
fn real_egui_root_record_changes_when_context_projection_is_updated() -> Result<(), String> {
    let context = egui::Context::default();
    let mut process = SanitizedDocumentRootProcess::new(input_with_context_projection(
        1,
        b"doc",
        "日本語 ⭐️",
        context_projection("表示", 1),
    ))?;

    let _ = render_record_with_events(
        &mut process,
        &context,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(48.0, 8.0),
            button: egui::PointerButton::Secondary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    let first = render_record_with_events(
        &mut process,
        &context,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(48.0, 8.0),
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    )?;

    process
        .synchronize(input_with_context_projection(
            2,
            b"doc",
            "日本語 ⭐️",
            context_projection("別の表示", 2),
        ))
        .map_err(|error| format!("{error:?}"))?;

    let second = render_record_with_events(&mut process, &context, Vec::new())?;

    assert_ne!(first.record_hash(), second.record_hash());
    Ok(())
}

fn render_record_with_events(
    process: &mut SanitizedDocumentRootProcess,
    context: &egui::Context,
    events: Vec<egui::Event>,
) -> Result<SanitizedDocumentRootRecord, String> {
    let mut output = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                ROOT_VIEWPORT_SIZE,
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                output = Some(process.show(ui).map_err(|error| error.to_string()));
            });
        },
    );
    platform_output.textures_delta.clear();
    let output = output.ok_or_else(|| "frame output was not produced".to_owned())??;
    Ok(SanitizedDocumentRootRecord::from_output(
        process.input.revision,
        &output,
    ))
}

#[test]
fn zero_sized_real_root_frame_propagates_the_render_error() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input(1, b"zero-canvas", "本文 ⭐️"))?;
    let context = egui::Context::default();
    let mut result = None;
    crate::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::Vec2::ZERO,
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            result = Some(process.show(ui));
        },
    );

    assert!(result.expect("real root route executes").is_err());
    Ok(())
}
