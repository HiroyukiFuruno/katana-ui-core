use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAccessibilityActionKind, TextSurfaceAccessibilityLabels,
    TextSurfaceAction, TextSurfaceAnnotation, TextSurfaceAnnotationStyle,
    TextSurfaceClipboardOperation, TextSurfaceEvent, TextSurfaceGutter, TextSurfaceGutterRow,
    TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_egui_adapter::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceDrawLayer, TextSurfaceGutterPaint,
    TextSurfacePaintOperationKind, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
use katana_ui_core_text_raster::PlatformTextRasterConfig;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 360.0;
const GUTTER_WIDTH: u32 = 32;
const FONT_SIZE: f32 = 16.0;
const LINE_HEIGHT: f32 = 24.0;
const FONT_WEIGHT: u16 = 400;
const TEXT_COLOR: [u8; 4] = [235, 235, 235, 255];
const BACKGROUND_COLOR: [u8; 4] = [24, 24, 24, 255];
const GUTTER_COLOR: [u8; 4] = [32, 32, 32, 255];
const SELECTION_COLOR: [u8; 4] = [64, 96, 160, 180];
const PREEDIT_COLOR: [u8; 4] = [255, 196, 64, 255];
const CARET_COLOR: [u8; 4] = [255, 255, 255, 255];
const GUTTER_CLICK_X: f32 = 16.0;
const SURFACE_CLICK_Y: f32 = 8.0;

#[test]
fn missing_font_catalog_fails_closed_before_surface_paint() {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig {
        proportional_candidates: Vec::new(),
        monospace_candidates: Vec::new(),
        emoji_candidates: Vec::new(),
        emoji_candidate_sha256: Vec::new(),
        cache_capacity: 1,
    });
    let mut surface = surface();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
    });
    output.textures_delta.clear();
    assert!(result.is_some_and(|value| value.is_err()));
}

#[test]
fn actual_egui_surface_uses_kuc_raster_texture_and_one_frame_record() {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let mut render_result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            render_result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
        },
    );

    assert!(render_result.is_some());
    let Some(render_result) = render_result else {
        return;
    };
    assert!(render_result.is_ok());
    let Ok(rendered) = render_result else {
        return;
    };
    assert!(!full_output.textures_delta.set.is_empty());
    full_output.textures_delta.clear();
    assert_eq!(
        vec![
            EguiTextSurfaceDrawLayer::Background,
            EguiTextSurfaceDrawLayer::Gutter,
            EguiTextSurfaceDrawLayer::Selection,
            EguiTextSurfaceDrawLayer::Preedit,
            EguiTextSurfaceDrawLayer::Annotation,
            EguiTextSurfaceDrawLayer::TextTexture,
            EguiTextSurfaceDrawLayer::Caret,
        ],
        rendered.record.layers
    );
    assert_eq!(
        rendered.record.frame.layout_identity,
        rendered.record.raster_identity
    );
    assert_eq!(1, rendered.record.frame.gutter.len());
    assert_eq!("active-line", rendered.record.frame.gutter[0].visual_role);
    let gutter_bounds = rendered.record.frame.gutter[0].bounds;
    let active_line_background = egui::Color32::from_rgba_unmultiplied(48, 64, 88, 255);
    assert!(full_output.shapes.iter().any(|clipped| {
        matches!(
            &clipped.shape,
            egui::epaint::Shape::Rect(shape)
                if shape.rect
                    == egui::Rect::from_min_size(
                        egui::pos2(gutter_bounds.x as f32, gutter_bounds.y as f32),
                        egui::vec2(gutter_bounds.width as f32, gutter_bounds.height as f32),
                    )
                    && shape.fill == active_line_background
        )
    }));
    assert!(rendered.record.texture_bounds.width > 0);
    assert!(rendered.record.texture_bounds.height > 0);
    assert!(rendered.record.frame.accessibility.root.selection.is_some());
    assert_eq!(rendered.record, rendered.artifact.record);
    assert_eq!(
        rendered.record.frame.surface_bounds,
        rendered.artifact.paint_plan.surface_bounds
    );
    assert_eq!(
        rendered.record.frame.viewport_bounds,
        rendered.artifact.paint_plan.viewport_bounds
    );
    assert_eq!(64, rendered.artifact.frame_record_hash.len());
    assert_eq!(64, rendered.artifact.paint_plan_hash.len());
    let text_texture = rendered
        .artifact
        .paint_plan
        .operations
        .iter()
        .find_map(|operation| match (&operation.layer, &operation.kind) {
            (
                EguiTextSurfaceDrawLayer::TextTexture,
                TextSurfacePaintOperationKind::Texture { texture, .. },
            ) => Some(texture),
            _ => None,
        })
        .expect("the artifact paint plan did not contain the text raster texture");
    assert_eq!(rendered.record.raster_identity, text_texture.identity);
    assert_eq!(
        usize::try_from(text_texture.width)
            .unwrap_or_default()
            .saturating_mul(usize::try_from(text_texture.height).unwrap_or_default())
            .saturating_mul(4),
        text_texture.rgba_pixels.len()
    );
}

#[test]
fn actual_egui_reuses_the_same_immutable_paint_plan_for_an_unchanged_surface() {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();

    let first = run_frame(&context, &mut adapter, &mut surface, Vec::new())
        .expect("the initial text surface frame did not render");
    let second = run_frame(&context, &mut adapter, &mut surface, Vec::new())
        .expect("the repeated text surface frame did not render");

    assert_eq!(first.record, second.record);
    assert_eq!(
        first.artifact.frame_record_hash,
        second.artifact.frame_record_hash
    );
    assert_eq!(
        first.artifact.paint_plan_hash,
        second.artifact.paint_plan_hash
    );
    assert_eq!(first.artifact.paint_plan, second.artifact.paint_plan);
}

#[test]
fn empty_single_line_surface_rasterizes_japanese_emoji_placeholder_and_exposes_it_to_accesskit() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("検索語")
                .placeholder("検索語 ⭐️")
                .min_rows(1)
                .max_rows(1)
                .auto_grow(false),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
        )
        .accessibility_label("検索語"),
    );
    let mut render_result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            render_result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
        },
    );

    let rendered = render_result
        .expect("the single-line placeholder surface did not produce a result")
        .expect("the single-line placeholder surface did not render");
    assert!(
        rendered
            .record
            .placeholder_raster_identity
            .as_deref()
            .is_some_and(|identity| identity.contains("検索語 ⭐️"))
    );
    assert!(rendered.record.placeholder_texture_bounds.is_some());
    assert!(
        rendered
            .record
            .layers
            .contains(&EguiTextSurfaceDrawLayer::PlaceholderTexture)
    );
    assert!(full_output.textures_delta.set.len() >= 2);
    full_output.textures_delta.clear();
    let Some(update) = full_output.platform_output.accesskit_update else {
        panic!("the enabled egui context did not emit an AccessKit tree update");
    };
    let root = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::TextInput);
    let Some(root) = root else {
        panic!("the single-line placeholder surface did not expose a text input");
    };
    assert_eq!(Some("検索語 ⭐️"), root.placeholder());
}

#[test]
fn actual_accesskit_tree_exposes_surface_text_gutter_and_context() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let _ = surface.apply_action(TextSurfaceAction::TextArea(
        katana_ui_core::atom::TextAreaAction::Select(katana_ui_core::atom::TextAreaSelection {
            start: 0,
            end: "日本語 ⭐️".len(),
        }),
    ));
    let mut render_result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            render_result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();

    assert!(render_result.is_some());
    let Some(render_result) = render_result else {
        return;
    };
    assert!(render_result.is_ok());
    let Ok(rendered) = render_result else {
        return;
    };
    let expected_surface_bounds = rendered.record.frame.surface_bounds;
    let Some(update) = full_output.platform_output.accesskit_update else {
        panic!("the enabled egui context did not emit an AccessKit tree update");
    };
    let root = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::MultilineTextInput);
    assert!(root.is_some());
    let Some(root) = root else {
        return;
    };
    assert_eq!(Some("編集領域"), root.label());
    assert_eq!(Some("日本語 ⭐️"), root.value());
    let Some(bounds) = root.bounds() else {
        panic!("the text surface root did not expose surface bounds");
    };
    assert_eq!(f64::from(expected_surface_bounds.x), bounds.x0);
    assert_eq!(f64::from(expected_surface_bounds.y), bounds.y0);
    assert_eq!(
        f64::from(
            expected_surface_bounds
                .x
                .saturating_add_unsigned(expected_surface_bounds.width)
        ),
        bounds.x1
    );
    assert_eq!(
        f64::from(
            expected_surface_bounds
                .y
                .saturating_add_unsigned(expected_surface_bounds.height)
        ),
        bounds.y1
    );
    let Some(selection) = root.text_selection() else {
        panic!("the text surface root did not expose its selection");
    };
    assert_eq!(0, selection.anchor.character_index);
    assert_eq!(6, selection.focus.character_index);
    assert!(root.supports_action(egui::accesskit::Action::SetTextSelection));
    let text_run = update.nodes.iter().find(|(_, node)| {
        node.role() == egui::accesskit::Role::TextRun && node.value() == Some("日本語 ⭐️")
    });
    assert!(text_run.is_some());
    let Some((text_run_id, _)) = text_run else {
        return;
    };
    let accessibility_container = update.nodes.iter().find(|(node_id, node)| {
        root.children().contains(node_id) && node.role() == egui::accesskit::Role::GenericContainer
    });
    assert!(accessibility_container.is_some());
    let Some((_, accessibility_container)) = accessibility_container else {
        return;
    };
    assert!(accessibility_container.children().contains(text_run_id));
    let button_labels = update
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();
    assert!(button_labels.contains(&"1 行目"));
    assert!(button_labels.contains(&"選択範囲のコンテキスト"));
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button
            && node.label() == Some("1 行目")
            && node.description() == Some("現在行")
    }));
}

#[test]
fn actual_accesskit_tree_exposes_disabled_root_and_gutter_reasons() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一\n二";
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("disabled-editor").value(text).disabled(true),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        )
        .accessibility_label("編集領域")
        .disabled_reason("読み込み中")
        .gutter(
            TextSurfaceGutter::new(GUTTER_WIDTH)
                .row(
                    TextSurfaceGutterRow::new(0, "1")
                        .accessibility_label("1 行目")
                        .accessibility_description("明示的な説明"),
                )
                .row(TextSurfaceGutterRow::new(1, "2").accessibility_label("2 行目")),
        ),
    );
    let mut render_result = None;
    let mut full_output = context.run_ui(egui::RawInput::default(), |ui| {
        render_result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
    });
    assert!(render_result.is_some_and(|result| result.is_ok()));
    full_output.textures_delta.clear();
    let update = full_output
        .platform_output
        .accesskit_update
        .expect("the disabled surface did not emit an AccessKit update");

    let root = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::MultilineTextInput)
        .expect("the disabled surface did not expose its root node");
    assert!(root.is_disabled());
    assert_eq!(Some("読み込み中"), root.description());

    let first_row = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some("1 行目"))
        .expect("the first gutter row was not exposed");
    assert!(first_row.is_disabled());
    assert_eq!(Some("明示的な説明"), first_row.description());

    let second_row = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some("2 行目"))
        .expect("the second gutter row was not exposed");
    assert!(second_row.is_disabled());
    assert_eq!(Some("読み込み中"), second_row.description());
}

#[test]
fn actual_accesskit_root_reports_focus_and_readonly_state() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = readonly_surface();
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let mut render_result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            render_result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();

    assert!(render_result.is_some());
    let Some(render_result) = render_result else {
        return;
    };
    assert!(render_result.is_ok());
    let Some(update) = full_output.platform_output.accesskit_update else {
        panic!("the enabled egui context did not emit an AccessKit tree update");
    };
    let root = update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == egui::accesskit::Role::MultilineTextInput);
    assert!(root.is_some());
    let Some((root_id, root)) = root else {
        return;
    };
    assert!(root.is_read_only());
    assert_eq!(*root_id, update.focus);
}

#[test]
fn actual_accesskit_text_run_bounds_follow_the_scrolled_surface_layout() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("accesskit-scroll").value(text),
        UiTextSpan::emoji_marked_spans(text, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = run_frame(&context, &mut adapter, &mut surface, Vec::new())
        .expect("the initial accessible text surface did not render");

    let mut render_result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events: vec![egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, 4_000.0),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            render_result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    let rendered = render_result
        .expect("the scrolled accessible text surface did not produce a result")
        .expect("the scrolled accessible text surface did not render");
    assert!(surface.state().scroll_y > 0);
    let Some(update) = full_output.platform_output.accesskit_update else {
        panic!("the scrolled text surface did not emit an AccessKit tree update");
    };
    let third_line = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() == egui::accesskit::Role::TextRun && node.value() == Some("三行目")
        })
        .expect("the visible scrolled line did not expose a TextRun node");
    let bounds = third_line
        .bounds()
        .expect("the visible scrolled TextRun did not expose bounds");
    let viewport = rendered.record.frame.viewport_bounds;
    assert_eq!(f64::from(viewport.x), bounds.x0);
    assert_eq!(f64::from(viewport.y), bounds.y0);
    assert!(bounds.x1 > bounds.x0);
    assert!(bounds.x1 <= f64::from(viewport.x.saturating_add_unsigned(viewport.width)));
    assert!(bounds.y1 <= f64::from(viewport.y.saturating_add_unsigned(viewport.height)));
}

#[test]
fn actual_egui_scroll_uses_one_coordinate_system_for_gutter_annotation_selection_caret_and_accesskit()
 {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目";
    let third_line_start = "一行目\n二行目\n".len();
    let third_line_end = third_line_start + "三行目".len();
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("scroll-coordinate-system").value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
        )
        .annotation(TextSurfaceAnnotation::new(
            "third-line-annotation",
            katana_ui_core::text_selection::UiTextSelectionRange::new(8, 11),
            "review-marker",
            TextSurfaceAnnotationStyle::Underline,
        ))
        .annotation(TextSurfaceAnnotation::new(
            "third-line-outline",
            katana_ui_core::text_selection::UiTextSelectionRange::new(8, 11),
            "review-outline",
            TextSurfaceAnnotationStyle::Outline,
        ))
        .annotation(TextSurfaceAnnotation::new(
            "third-line-fill",
            katana_ui_core::text_selection::UiTextSelectionRange::new(8, 11),
            "review-fill",
            TextSurfaceAnnotationStyle::Fill,
        ))
        .gutter(
            TextSurfaceGutter::new(GUTTER_WIDTH)
                .row(TextSurfaceGutterRow::new(0, "1").visual_role("first-line"))
                .row(TextSurfaceGutterRow::new(1, "2").visual_role("second-line"))
                .row(TextSurfaceGutterRow::new(2, "3").visual_role("third-line")),
        ),
    );
    let _ = surface.apply_action(TextSurfaceAction::TextArea(
        katana_ui_core::atom::TextAreaAction::Select(katana_ui_core::atom::TextAreaSelection {
            start: third_line_start,
            end: third_line_end,
        }),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = run_frame(&context, &mut adapter, &mut surface, Vec::new())
        .expect("the initial scroll-coordinate surface did not render");

    let mut render_result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events: vec![egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, 4_000.0),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            render_result = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    let rendered = render_result
        .expect("the scrolled coordinate-system surface did not produce a result")
        .expect("the scrolled coordinate-system surface did not render");
    let frame = &rendered.record.frame;
    let viewport = frame.viewport_bounds;
    assert!(surface.state().scroll_y > 0);

    let gutter = frame
        .gutter
        .iter()
        .find(|row| row.logical_row == 2)
        .expect("the visible third line did not expose its gutter row");
    assert_eq!(viewport.y, gutter.bounds.y);
    assert_eq!("third-line", gutter.visual_role);
    let annotation = frame
        .annotations
        .iter()
        .find(|annotation| annotation.id == "third-line-annotation")
        .expect("the visible third line did not expose its annotation");
    assert!(!annotation.rects.is_empty());
    assert!(annotation.rects.iter().all(|bounds| bounds.y == viewport.y));
    assert!(!frame.selection.rects.is_empty());
    assert!(
        frame
            .selection
            .rects
            .iter()
            .all(|bounds| bounds.y == viewport.y)
    );
    assert_eq!(viewport.y, frame.selection.caret.y);

    let Some(update) = full_output.platform_output.accesskit_update else {
        panic!("the scrolled coordinate-system surface did not emit an AccessKit update");
    };
    let text_run = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() == egui::accesskit::Role::TextRun && node.value() == Some("三行目")
        })
        .expect("the visible third line did not expose an AccessKit TextRun");
    let bounds = text_run
        .bounds()
        .expect("the visible third-line AccessKit TextRun did not expose bounds");
    assert_eq!(f64::from(viewport.y), bounds.y0);
    assert!(bounds.y1 <= f64::from(viewport.y.saturating_add_unsigned(viewport.height)));
}

#[test]
fn actual_egui_text_ime_clipboard_and_history_events_stay_typed() {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));

    let typed = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Text("⭐️".to_string())],
    );
    assert!(typed.is_ok());
    let Some(typed) = typed.ok() else {
        return;
    };
    assert_eq!("日本語 ⭐️⭐️", surface.state().text_area.value);
    assert!(typed.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::TextArea(katana_ui_core::atom::TextAreaEvent::EmojiInput {
            grapheme_count: 1
        })
    )));

    let preedit = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    );
    assert!(preedit.is_ok());
    let Some(preedit) = preedit.ok() else {
        return;
    };
    assert!(preedit.record.frame.preedit.is_some());
    assert_ne!(typed.record.raster_identity, preedit.record.raster_identity);
    let committed = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );
    assert!(committed.is_ok());
    assert!(surface.state().text_area.composition.is_none());
    assert!(surface.state().text_area.value.ends_with("⭐️"));

    let _ = surface.apply_action(TextSurfaceAction::TextArea(
        katana_ui_core::atom::TextAreaAction::Select(katana_ui_core::atom::TextAreaSelection {
            start: 0,
            end: "日".len(),
        }),
    ));
    let copied = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Copy],
    );
    assert!(copied.is_ok());
    let Some(copied) = copied.ok() else {
        return;
    };
    assert!(
        copied
            .events
            .contains(&TextSurfaceEvent::ClipboardRequested {
                operation: TextSurfaceClipboardOperation::Copy,
                selection_start: 0,
                selection_end: "日".len(),
            })
    );
    let history = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Key {
            key: egui::Key::Z,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        }],
    );
    assert!(history.is_ok());
    let Some(history) = history.ok() else {
        return;
    };
    assert!(history.events.contains(&TextSurfaceEvent::HistoryRequested(
        katana_ui_core::text_surface::TextSurfaceHistoryOperation::Undo,
    )));
}

#[test]
fn actual_egui_gutter_and_context_requests_use_the_current_frame_targets() {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let gutter_point = egui::pos2(GUTTER_CLICK_X, SURFACE_CLICK_Y);

    let _ = run_frame(&context, &mut adapter, &mut surface, Vec::new());

    let gutter = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(
            gutter_point,
            egui::PointerButton::Primary,
            true,
        )],
    );
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(
            gutter_point,
            egui::PointerButton::Primary,
            false,
        )],
    );
    assert!(gutter.is_ok());
    let Some(gutter) = gutter.ok() else {
        return;
    };
    assert!(
        gutter
            .events
            .contains(&TextSurfaceEvent::GutterRowActivated { logical_row: 0 })
    );

    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(
            gutter_point,
            egui::PointerButton::Secondary,
            true,
        )],
    );
    let context_target = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(
            gutter_point,
            egui::PointerButton::Secondary,
            false,
        )],
    );
    assert!(context_target.is_ok());
    let Some(context_target) = context_target.ok() else {
        return;
    };
    assert!(
        context_target
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::ContextTargetRequested { .. }))
    );
}

fn surface() -> TextSurface {
    let text = "日本語 ⭐️";
    surface_with_text_area(TextArea::new("editor").value(text), text)
}

fn readonly_surface() -> TextSurface {
    let text = "読み取り専用 ⭐️";
    surface_with_text_area(
        TextArea::new("readonly-editor").value(text).readonly(true),
        text,
    )
}

fn surface_with_text_area(text_area: TextArea, text: &str) -> TextSurface {
    let labels = TextSurfaceAccessibilityLabels::new()
        .with_label(TextSurfaceAccessibilityActionKind::Copy, "コピー");
    TextSurface::new(
        TextSurfaceProps::new(
            text_area,
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        )
        .accessibility_label("編集領域")
        .context_target_label("選択範囲のコンテキスト")
        .accessibility_actions(labels)
        .gutter(
            TextSurfaceGutter::new(GUTTER_WIDTH).row(
                TextSurfaceGutterRow::new(0, "1")
                    .accessibility_label("1 行目")
                    .accessibility_description("現在行")
                    .visual_role("active-line"),
            ),
        ),
    )
}

fn raster_style() -> TextSurfaceRasterStyle {
    TextSurfaceRasterStyle::new(
        FontToken {
            name: "editor".to_string(),
            family: FontFamily::Monospace,
            size: FONT_SIZE,
            weight: FONT_WEIGHT,
        },
        TEXT_COLOR,
        LINE_HEIGHT,
    )
}

fn paint_style() -> TextSurfacePaintStyle {
    TextSurfacePaintStyle {
        background_rgba: BACKGROUND_COLOR,
        gutter_background_rgba: GUTTER_COLOR,
        gutter_paints: vec![
            TextSurfaceGutterPaint::new("active-line", TEXT_COLOR).background([48, 64, 88, 255]),
        ],
        selection_rgba: SELECTION_COLOR,
        preedit_rgba: PREEDIT_COLOR,
        caret_rgba: CARET_COLOR,
        annotation_paints: Vec::new(),
    }
}

fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
) -> Result<
    katana_ui_core_egui_adapter::text_surface::EguiTextSurfaceOutput,
    katana_ui_core_egui_adapter::text_surface::EguiTextSurfaceError,
> {
    let mut result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            result = Some(adapter.show(ui, surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    result
        .ok_or(katana_ui_core_egui_adapter::text_surface::EguiTextSurfaceError::FrameNotProduced)?
}

fn pointer_button(pos: egui::Pos2, button: egui::PointerButton, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}
