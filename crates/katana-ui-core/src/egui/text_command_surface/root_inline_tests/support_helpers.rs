struct EguiTextSurfaceForTest;

impl EguiTextSurfaceForTest {
    fn surface() -> crate::text_surface::TextSurface {
        let mut props = TextSurfaceProps::new(
            TextArea::new("collision-text").value("本文 ⭐️"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, ROOT_FRAME_WIDTH_PX, ROOT_FRAME_HEIGHT_PX),
        );
        props.accessibility_label = "collision text".to_owned();
        TextSurface::new(props)
    }
}

fn render(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
    render_with_input(context, root, egui::RawInput::default())
}

fn render_with_input(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    input: egui::RawInput,
) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
    render_with_input_at_size(
        context,
        root,
        input,
        egui::vec2(ROOT_FRAME_WIDTH, ROOT_FRAME_HEIGHT),
    )
}

fn render_with_input_at_size(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    input: egui::RawInput,
    size: egui::Vec2,
) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
    let style = TextCommandSurfaceStyle::standard()?;
    let mut output = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..input
        },
        |ui| output = Some(root.show(ui, &style)),
    );
    platform_output.textures_delta.clear();
    output.ok_or_else(|| {
        EguiTextCommandSurfaceRootError::Serialization("root frame missing".to_owned())
    })?
}

fn render_with_platform_output(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    input: egui::RawInput,
) -> Result<(egui::FullOutput, EguiTextCommandSurfaceRootOutput), EguiTextCommandSurfaceRootError> {
    let style = TextCommandSurfaceStyle::standard()?;
    let mut root_output = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(ROOT_FRAME_WIDTH, ROOT_FRAME_HEIGHT),
            )),
            ..input
        },
        |ui| root_output = Some(root.show(ui, &style)),
    );
    platform_output.textures_delta.clear();
    Ok((
        platform_output,
        root_output.ok_or_else(|| {
            EguiTextCommandSurfaceRootError::Serialization("root frame missing".to_owned())
        })??,
    ))
}

fn selected_surface() -> crate::text_surface::TextSurface {
    let value = "選択範囲 ⭐️";
    let mut props = TextSurfaceProps::new(
        TextArea::new("selected-text").value(value),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, ROOT_FRAME_WIDTH_PX, ROOT_FRAME_HEIGHT_PX),
    );
    props.accessibility_label = "selected text".to_owned();
    let mut presentation =
        crate::text_surface::TextSurfacePresentation::from_props(&props);
    presentation.selection_start = 0;
    presentation.selection_end = value.len();
    let mut surface = TextSurface::new(props);
    assert!(surface.synchronize_presentation(presentation));
    surface
}

fn context_for_test() -> egui::Context {
    egui::Context::default()
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn secondary_pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn key_press(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

fn accesskit_click(node: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::Click,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: node,
        data: None,
    })
}

fn accesskit_button(
    output: &egui::FullOutput,
    label: &str,
) -> Result<(egui::accesskit::NodeId, egui::Rect, bool), String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                    .then(|| {
                        node.bounds().map(|bounds| {
                            (
                                *node_id,
                                egui::Rect::from_min_max(
                                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                ),
                                node.is_disabled(),
                            )
                        })
                    })
                    .flatten()
            })
        })
        .ok_or_else(|| format!("current frame lacks AccessKit button: {label}"))
}

fn accesskit_text_input(output: &egui::FullOutput, label: &str) -> Result<egui::Rect, String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::TextInput && node.label() == Some(label))
                    .then(|| {
                        node.bounds().map(|bounds| {
                            egui::Rect::from_min_max(
                                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                            )
                        })
                    })
                    .flatten()
            })
        })
        .ok_or_else(|| format!("current frame lacks AccessKit text input: {label}"))
}

fn search_strip() -> CommandChromeSearchStrip {
    let text = |label: &str| CommandChromeText::new(label, label, label);
    CommandChromeSearchStrip::new(
        SearchControlStrip::new("検索")
            .query("検索語")
            .replace_mode(crate::molecule::structured::ReplaceMode::Visible)
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
