#![cfg(feature = "egui")]
use katana_ui_core::egui::command_chrome::{
    CommandChromePaintOperationKind, CommandChromePaintStyle, CommandChromeRasterStyle,
    EguiCommandChromeAdapter, EguiCommandChromeDrawLayer, EguiCommandChromeSearchOutput,
    EguiCommandChromeSearchStyle,
};
use katana_ui_core::egui::text_surface::{TextSurfacePaintStyle, TextSurfaceRasterStyle};
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeCapability, CommandChromeDisplayMode,
    CommandChromeSearchEvent, CommandChromeSearchPresentation, CommandChromeSearchStrip,
    CommandChromeText, CommandChromeToolbar, SearchControlCapabilities, SearchControlIconSlot,
    SearchControlIcons, SearchControlStrings, SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::structured::{
    ReplaceMode, SearchControlStrip, SearchControlStripEvent, SearchNavigationDirection,
    SearchOptionKind, SearchReplaceScope,
};
use katana_ui_core::render_model::UiIconProps;
use katana_ui_core::svg_raster::UiSvgRasterConfig;
use katana_ui_core::text_raster::PlatformTextRasterConfig;
use katana_ui_core::theme::{FontFamily, FontToken};

const SCREEN_WIDTH: f32 = 1440.0;
const SCREEN_HEIGHT: f32 = 320.0;

#[test]
fn invalid_first_control_icon_fails_closed_before_later_controls_render() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let icons = SearchControlIcons::default().icon(
        SearchControlIconSlot::MatchCase,
        UiIconProps::new("not-an-svg"),
    );
    let mut strip = strip(SearchControlCapabilities::all_available()).icons(icons);
    let (_, output) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    assert!(output.is_err());
}

#[test]
fn valid_control_icon_reaches_icon_paint_layer() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let icons = SearchControlIcons::default().icon(
        SearchControlIconSlot::MatchCase,
        UiIconProps::new("<svg viewBox=\"0 0 8 8\"><path d=\"M1 1h6v6H1z\"/></svg>"),
    );
    let mut strip = strip(SearchControlCapabilities::all_available()).icons(icons);
    let (_, output) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let output = expect_output(output);
    assert!(
        output
            .artifact
            .paint_plan
            .operations
            .iter()
            .any(|operation| { operation.layer == EguiCommandChromeDrawLayer::IconTexture })
    );
}

#[test]
fn missing_search_font_fails_closed_at_query_surface() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::new(
        PlatformTextRasterConfig {
            proportional_candidates: Vec::new(),
            monospace_candidates: Vec::new(),
            emoji_candidates: Vec::new(),
            emoji_candidate_sha256: Vec::new(),
            cache_capacity: 1,
        },
        UiSvgRasterConfig::default(),
    );
    let mut strip = strip(SearchControlCapabilities::all_available());
    let (_, output) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    assert!(output.is_err());
}

#[test]
fn actual_egui_search_strip_uses_shared_text_surface_for_placeholder_ime_and_katana_keys() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut strip = strip(SearchControlCapabilities::all_available());

    let (mut first_frame, first) =
        run_frame_preserving_textures(&context, &mut adapter, &mut strip, Vec::new());
    let first = expect_output(first);
    assert!(first.record.query.placeholder_raster_identity.is_some());
    assert!(first.record.replace.is_some());
    assert_eq!(9, first.record.controls.len());
    assert!(first_frame.textures_delta.set.len() >= 3);
    first_frame.textures_delta.clear();
    let Some(update) = first_frame.platform_output.accesskit_update else {
        panic!("the enabled egui context did not emit an AccessKit tree update");
    };
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput && node.placeholder() == Some("検索語 ⭐️")
    }));
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("次へ")
    }));

    let query_point = center(first.record.query.frame.content_bounds);
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(query_point, true)],
    );
    let (_, focused) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(query_point, false)],
    );
    assert!(expect_output(focused).record.focused_target.is_some());

    let (_, ignored_key) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![key_event(egui::Key::A, egui::Modifiers::default())],
    );
    let ignored_key = expect_output(ignored_key);
    assert!(ignored_key.events.is_empty());
    assert!(ignored_key.text_events.is_empty());
    assert!(strip.query_model().is_empty());

    let (_, typed) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![egui::Event::Text("日本語 ⭐️".to_string())],
    );
    let typed = expect_output(typed);
    assert_eq!(
        1,
        query_changes(&typed.events, "日本語 ⭐️").count(),
        "a text input must issue exactly one typed query event",
    );
    assert!(typed.text_events.iter().any(|event| {
        matches!(
            event,
            katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                katana_ui_core::atom::TextAreaEvent::Change(value)
            ) if value == "日本語 ⭐️"
        )
    }));
    assert_eq!("日本語 ⭐️", strip.query_model());

    let (_, preedit) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    );
    assert!(expect_output(preedit).events.is_empty());
    let (_, committed) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );
    let committed = expect_output(committed);
    assert_eq!(1, committed.events.len());
    assert_eq!("日本語 ⭐️⭐️", strip.query_model());

    assert_navigation(
        run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![key_event(egui::Key::Enter, egui::Modifiers::default())],
        )
        .1,
        SearchNavigationDirection::Next,
    );
    assert_navigation(
        run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![key_event(
                egui::Key::Enter,
                egui::Modifiers {
                    shift: true,
                    ..egui::Modifiers::default()
                },
            )],
        )
        .1,
        SearchNavigationDirection::Previous,
    );
    assert_navigation(
        run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![key_event(egui::Key::ArrowDown, egui::Modifiers::default())],
        )
        .1,
        SearchNavigationDirection::Next,
    );
    assert_navigation(
        run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![key_event(egui::Key::ArrowUp, egui::Modifiers::default())],
        )
        .1,
        SearchNavigationDirection::Previous,
    );

    let (_, escape_closed) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![key_event(egui::Key::Escape, egui::Modifiers::default())],
    );
    assert!(
        expect_output(escape_closed)
            .events
            .contains(&CommandChromeSearchEvent::CloseRequested)
    );

    let controls = first.record.controls;
    let close = controls
        .iter()
        .find(|control| control.control_id.ends_with(":close"))
        .map(|control| center(control.bounds));
    let Some(close) = close else {
        panic!("the search strip did not render a close control");
    };
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(close, true)],
    );
    let (_, closed) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(close, false)],
    );
    assert!(
        expect_output(closed)
            .events
            .contains(&CommandChromeSearchEvent::CloseRequested)
    );
}

#[test]
fn search_artifact_plan_owns_input_and_control_paint_with_raw_input_contracts() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut strip = strip(SearchControlCapabilities::all_available());

    let (first_frame, first) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let first = expect_output(first);
    assert_eq!(first.record, first.artifact.record);
    assert_eq!(first.events, first.artifact.events);
    assert_eq!(first.text_events, first.artifact.text_events);
    assert_eq!(64, first.artifact.frame_record_hash.len());
    assert_eq!(64, first.artifact.paint_plan_hash.len());
    assert!(plan_has_fill(
        &first,
        EguiCommandChromeDrawLayer::PanelFill,
        first.record.query.frame.surface_bounds,
        [24, 24, 24, 255],
    ));
    for control in &first.record.controls {
        assert!(
            first
                .artifact
                .paint_plan
                .operations
                .iter()
                .any(|operation| {
                    matches!(
                        &operation.kind,
                        CommandChromePaintOperationKind::Texture { texture, .. }
                            if texture.identity == control.raster_identity
                    )
                })
        );
    }
    let Some(update) = first_frame.platform_output.accesskit_update else {
        panic!("the enabled egui context did not emit an AccessKit tree update");
    };
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput && node.label() == Some("検索語 ⭐️")
    }));

    let match_case = control_point(&first, ":match-case");
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(match_case, true)],
    );
    let (_, match_case_released) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(match_case, false)],
    );
    let match_case_released = expect_output(match_case_released);
    assert!(
        match_case_released
            .events
            .contains(&CommandChromeSearchEvent::Strip {
                event: SearchControlStripEvent::SearchOptionChanged {
                    option: SearchOptionKind::MatchCase,
                    enabled: true,
                },
            })
    );
    let (_, active) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let active = expect_output(active);
    let active_control = active
        .record
        .controls
        .iter()
        .find(|control| control.control_id.ends_with(":match-case"))
        .expect("the match-case control did not render");
    assert!(active_control.active);
    assert!(plan_has_fill(
        &active,
        EguiCommandChromeDrawLayer::ActionFill,
        active_control.bounds,
        [64, 96, 160, 255],
    ));

    let replace_point = center(
        active
            .record
            .replace
            .as_ref()
            .expect("the replacement input did not render")
            .frame
            .content_bounds,
    );
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(replace_point, true)],
    );
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(replace_point, false)],
    );
    let (_, replacement_typed) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![egui::Event::Text("置換語 ⭐️".to_string())],
    );
    let replacement_typed = expect_output(replacement_typed);
    assert!(
        replacement_typed
            .events
            .contains(&CommandChromeSearchEvent::Strip {
                event: SearchControlStripEvent::ReplaceValueChanged("置換語 ⭐️".to_string()),
            })
    );
    assert!(replacement_typed.text_events.iter().any(|event| {
        matches!(
            event,
            katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                katana_ui_core::atom::TextAreaEvent::Change(value)
            ) if value == "置換語 ⭐️"
        )
    }));

    for (suffix, scope) in [
        (":replace-one", SearchReplaceScope::One),
        (":replace-all", SearchReplaceScope::All),
    ] {
        let point = control_point(&replacement_typed, suffix);
        let _ = run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![pointer_button(point, true)],
        );
        let (_, replaced) = run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![pointer_button(point, false)],
        );
        assert!(
            expect_output(replaced)
                .events
                .contains(&CommandChromeSearchEvent::Strip {
                    event: SearchControlStripEvent::ReplaceRequested {
                        scope,
                        value: "置換語 ⭐️".to_string(),
                    },
                })
        );
    }

    let close = control_point(&replacement_typed, ":close");
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(close, true)],
    );
    let (_, closed) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(close, false)],
    );
    assert!(
        expect_output(closed)
            .events
            .contains(&CommandChromeSearchEvent::CloseRequested)
    );
}

#[test]
fn equivalent_raw_input_frames_produce_equal_search_artifacts() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut strip = strip(SearchControlCapabilities::all_available());

    let (_, first) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let (_, second) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let first = expect_output(first);
    let second = expect_output(second);

    assert_eq!(first.record, second.record);
    assert_eq!(first.events, second.events);
    assert_eq!(first.text_events, second.text_events);
    assert_eq!(
        first.artifact.frame_record_hash,
        second.artifact.frame_record_hash
    );
    assert_eq!(
        first.artifact.paint_plan_hash,
        second.artifact.paint_plan_hash
    );
}

#[test]
fn unavailable_search_controls_reject_actual_egui_clicks() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut strip = strip(SearchControlCapabilities {
        regex: CommandChromeCapability::unavailable("正規表現は利用できません"),
        navigation: CommandChromeCapability::unavailable("検索結果がありません"),
        ..SearchControlCapabilities::all_available()
    });
    let (_, first) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let first = expect_output(first);
    for suffix in [":use-regex", ":next"] {
        let point = first
            .record
            .controls
            .iter()
            .find(|control| control.control_id.ends_with(suffix))
            .map(|control| center(control.bounds));
        let Some(point) = point else {
            panic!("the search strip did not render {suffix}");
        };
        let _ = run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![pointer_button(point, true)],
        );
        let (_, released) = run_frame(
            &context,
            &mut adapter,
            &mut strip,
            vec![pointer_button(point, false)],
        );
        assert!(expect_output(released).events.is_empty());
    }
}

#[test]
fn controlled_search_presentation_preserves_actual_focus_and_rejects_disabled_commands() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut strip = strip(SearchControlCapabilities::all_available());
    let (_, initial) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let initial = expect_output(initial);
    let query_point = center(initial.record.query.frame.content_bounds);
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(query_point, true)],
    );
    let (_, focused) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(query_point, false)],
    );
    let focused = expect_output(focused);
    let focused_id = focused
        .record
        .focused_target
        .clone()
        .expect("query pointer input must establish a KUC focus identity");

    assert!(
        strip.synchronize_presentation(CommandChromeSearchPresentation {
            query: "同期検索 ⭐️".to_string(),
            options: *strip.options_model(),
            result_count: Some(4),
            active_index: Some(1),
            replace_mode: ReplaceMode::Visible,
            replace_value: "同期置換 ⭐️".to_string(),
            strings: strings(),
            capabilities: SearchControlCapabilities::all_available(),
            icons: SearchControlIcons::default(),
        })
    );
    let (_, synchronized) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let synchronized = expect_output(synchronized);
    assert_eq!(
        synchronized.record.focused_target.as_ref(),
        Some(&focused_id)
    );
    assert_eq!(strip.query_model(), "同期検索 ⭐️");
    assert_eq!(strip.replace_value_model(), "同期置換 ⭐️");

    let (_, typed_query) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![egui::Event::Text("追加入力 ⭐️".to_string())],
    );
    let typed_query = expect_output(typed_query);
    assert!(typed_query.events.iter().any(|event| matches!(
        event,
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged(value)
        } if value.contains("同期検索 ⭐️") && value.contains("追加入力 ⭐️")
    )));

    let replace_point = center(
        typed_query
            .record
            .replace
            .as_ref()
            .expect("controlled replace input must render")
            .frame
            .content_bounds,
    );
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(replace_point, true)],
    );
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(replace_point, false)],
    );
    let (_, typed_replace) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![egui::Event::Text("実入力 ⭐️".to_string())],
    );
    let typed_replace = expect_output(typed_replace);
    assert!(typed_replace.events.iter().any(|event| matches!(
        event,
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::ReplaceValueChanged(value)
        } if value.contains("実入力 ⭐️")
    )));

    assert!(
        strip.synchronize_presentation(CommandChromeSearchPresentation {
            query: strip.query_model().to_string(),
            options: *strip.options_model(),
            result_count: strip.result_count_model(),
            active_index: strip.active_index_model(),
            replace_mode: strip.replace_mode_model(),
            replace_value: strip.replace_value_model().to_string(),
            strings: strings(),
            capabilities: SearchControlCapabilities {
                regex: CommandChromeCapability::unavailable("unsupported"),
                ..SearchControlCapabilities::all_available()
            },
            icons: SearchControlIcons::default(),
        })
    );
    let (_, disabled) = run_frame(&context, &mut adapter, &mut strip, Vec::new());
    let disabled = expect_output(disabled);
    let regex = control_point(&disabled, ":use-regex");
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(regex, true)],
    );
    let (_, released) = run_frame(
        &context,
        &mut adapter,
        &mut strip,
        vec![pointer_button(regex, false)],
    );
    assert!(expect_output(released).events.is_empty());
}

#[test]
fn query_focus_prevents_toolbar_shortcuts_from_consuming_katana_search_keys() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut strip = strip(SearchControlCapabilities::all_available());
    let mut toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::LabelOnly)
        .action(CommandChromeAction::new("bold", "太字"));
    let (_, first_toolbar, first_search) =
        run_frame_with_toolbar(&context, &mut adapter, &mut toolbar, &mut strip, Vec::new());
    assert!(
        first_toolbar
            .expect("the toolbar did not render")
            .events
            .is_empty()
    );
    let query_point = center(
        expect_output(first_search)
            .record
            .query
            .frame
            .content_bounds,
    );
    let _ = run_frame_with_toolbar(
        &context,
        &mut adapter,
        &mut toolbar,
        &mut strip,
        vec![pointer_button(query_point, true)],
    );
    let (_, _, focused_search) = run_frame_with_toolbar(
        &context,
        &mut adapter,
        &mut toolbar,
        &mut strip,
        vec![pointer_button(query_point, false)],
    );
    let focused_search = expect_output(focused_search);
    assert!(
        focused_search.record.focused_target.is_some(),
        "query point {query_point:?} did not focus search bounds {:?}",
        focused_search.record.query.frame.content_bounds,
    );
    let (_, toolbar_output, search_output) = run_frame_with_toolbar(
        &context,
        &mut adapter,
        &mut toolbar,
        &mut strip,
        vec![key_event(egui::Key::Enter, egui::Modifiers::default())],
    );
    assert!(
        toolbar_output
            .expect("the toolbar did not render")
            .events
            .is_empty()
    );
    assert_navigation(search_output, SearchNavigationDirection::Next);
}

fn strip(capabilities: SearchControlCapabilities) -> CommandChromeSearchStrip {
    CommandChromeSearchStrip::new(
        SearchControlStrip::new("検索")
            .result_position(2, Some(0))
            .replace_mode(ReplaceMode::Visible),
        strings(),
    )
    .capabilities(capabilities)
}

fn strings() -> SearchControlStrings {
    SearchControlStrings {
        strip: text("検索"),
        query: text("検索語 ⭐️"),
        replace: text("置換後の文字列"),
        match_case: text("大文字小文字を区別"),
        whole_word: text("単語単位"),
        use_regex: text("正規表現"),
        previous: text("前へ"),
        next: text("次へ"),
        replace_one: text("置換"),
        replace_all: text("すべて置換"),
        close: text("閉じる"),
        result_summary: SearchResultSummaryTemplate {
            empty: String::new(),
            zero_results: "0 件".to_string(),
            single_result: "1 / 1".to_string(),
            indexed_result: "{active} 件目 / {count} 件".to_string(),
            count_results: "{count} 件".to_string(),
        },
    }
}

fn text(value: &str) -> CommandChromeText {
    CommandChromeText::new(value, value, value)
}

fn raster_style() -> CommandChromeRasterStyle {
    CommandChromeRasterStyle {
        font: FontToken {
            name: "command".to_string(),
            family: FontFamily::Monospace,
            size: 16.0,
            weight: 400,
        },
        text_color_rgba: [235, 235, 235, 255],
        icon_color: RgbaColor::new(235, 235, 235, 255),
        line_height_px: 24.0,
        icon_size_px: 16,
    }
}

fn paint_style() -> CommandChromePaintStyle {
    CommandChromePaintStyle {
        action_rgba: [32, 32, 32, 255],
        hovered_action_rgba: [56, 72, 96, 255],
        disabled_action_rgba: [24, 24, 24, 255],
    }
}

fn search_style() -> EguiCommandChromeSearchStyle {
    EguiCommandChromeSearchStyle {
        input_raster: TextSurfaceRasterStyle::new(
            FontToken {
                name: "input".to_string(),
                family: FontFamily::Monospace,
                size: 16.0,
                weight: 400,
            },
            [235, 235, 235, 255],
            24.0,
        ),
        input_paint: TextSurfacePaintStyle {
            background_rgba: [24, 24, 24, 255],
            gutter_background_rgba: [24, 24, 24, 255],
            gutter_paints: Vec::new(),
            selection_rgba: [64, 96, 160, 180],
            preedit_rgba: [255, 196, 64, 255],
            caret_rgba: [255, 255, 255, 255],
            annotation_paints: Vec::new(),
        },
        input_width_px: 192,
        input_height_px: 28,
        gap_px: 8,
        control_padding_px: 6,
        active_control_rgba: [64, 96, 160, 255],
    }
}

fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    strip: &mut CommandChromeSearchStrip,
    events: Vec<egui::Event>,
) -> (
    egui::FullOutput,
    Result<
        EguiCommandChromeSearchOutput,
        katana_ui_core::egui::command_chrome::EguiCommandChromeError,
    >,
) {
    let (mut frame, output) = run_frame_preserving_textures(context, adapter, strip, events);
    frame.textures_delta.clear();
    (frame, output)
}

fn run_frame_preserving_textures(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    strip: &mut CommandChromeSearchStrip,
    events: Vec<egui::Event>,
) -> (
    egui::FullOutput,
    Result<
        EguiCommandChromeSearchOutput,
        katana_ui_core::egui::command_chrome::EguiCommandChromeError,
    >,
) {
    let mut output = None;
    let mut frame = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            output = Some(adapter.show_search_strip(
                ui,
                strip,
                &raster_style(),
                &paint_style(),
                &search_style(),
            ))
        },
    );
    let Some(output) = output else {
        frame.textures_delta.clear();
        panic!("the search strip did not produce an egui frame");
    };
    (frame, output)
}

fn run_frame_with_toolbar(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    strip: &mut CommandChromeSearchStrip,
    events: Vec<egui::Event>,
) -> (
    egui::FullOutput,
    Result<
        katana_ui_core::egui::command_chrome::EguiCommandChromeOutput,
        katana_ui_core::egui::command_chrome::EguiCommandChromeError,
    >,
    Result<
        EguiCommandChromeSearchOutput,
        katana_ui_core::egui::command_chrome::EguiCommandChromeError,
    >,
) {
    let mut toolbar_output = None;
    let mut search_output = None;
    let mut frame = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            toolbar_output =
                Some(adapter.show_toolbar(ui, toolbar, &raster_style(), &paint_style()));
            search_output = Some(adapter.show_search_strip(
                ui,
                strip,
                &raster_style(),
                &paint_style(),
                &search_style(),
            ));
        },
    );
    let Some(toolbar_output) = toolbar_output else {
        frame.textures_delta.clear();
        panic!("the toolbar did not produce an egui frame");
    };
    let Some(search_output) = search_output else {
        frame.textures_delta.clear();
        panic!("the search strip did not produce an egui frame");
    };
    frame.textures_delta.clear();
    (frame, toolbar_output, search_output)
}

fn expect_output(
    value: Result<
        EguiCommandChromeSearchOutput,
        katana_ui_core::egui::command_chrome::EguiCommandChromeError,
    >,
) -> EguiCommandChromeSearchOutput {
    value.expect("the search strip did not render")
}

fn query_changes<'a>(
    events: &'a [CommandChromeSearchEvent],
    value: &'a str,
) -> impl Iterator<Item = &'a CommandChromeSearchEvent> {
    events.iter().filter(move |event| {
        matches!(event, CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged(current)
        } if current == value)
    })
}

fn assert_navigation(
    output: Result<
        EguiCommandChromeSearchOutput,
        katana_ui_core::egui::command_chrome::EguiCommandChromeError,
    >,
    direction: SearchNavigationDirection,
) {
    assert!(
        expect_output(output)
            .events
            .contains(&CommandChromeSearchEvent::Strip {
                event: SearchControlStripEvent::SearchNavigationRequested { direction },
            })
    );
}

fn control_point(output: &EguiCommandChromeSearchOutput, suffix: &str) -> egui::Pos2 {
    output
        .record
        .controls
        .iter()
        .find(|control| control.control_id.ends_with(suffix))
        .map(|control| center(control.bounds))
        .unwrap_or_else(|| panic!("the search strip did not render {suffix}"))
}

fn plan_has_fill(
    output: &EguiCommandChromeSearchOutput,
    layer: EguiCommandChromeDrawLayer,
    bounds: katana_ui_core::render_model::UiRect,
    color_rgba: [u8; 4],
) -> bool {
    output
        .artifact
        .paint_plan
        .operations
        .iter()
        .any(|operation| {
            operation.layer == layer
                && matches!(
                    operation.kind,
                    CommandChromePaintOperationKind::Fill {
                        bounds: operation_bounds,
                        color_rgba: operation_color,
                    } if operation_bounds == bounds && operation_color == color_rgba
                )
        })
}

fn center(bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn key_event(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}
