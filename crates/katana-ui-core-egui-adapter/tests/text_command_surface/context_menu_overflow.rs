use super::super::{assertions, harness};
use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::selection::{
    ContextMenuCloseReason, ContextMenuEvent, ContextMenuItemKind,
};
use katana_ui_core::render_model::{UiRect, UiTextSpan};
use katana_ui_core::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPresentation, ContextMenuPresentationItem, EguiContextMenuFrameRecord,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceChild,
    EguiTextCommandSurfaceOutput,
};

const SCREEN: egui::Vec2 = egui::vec2(480.0, 220.0);
const ROOT_ID: &str = "context.edit";
const CODE_ID: &str = "context.code";
const DIRECT_FINAL_ID: &str = "direct.13";
const CODE_FINAL_ID: &str = "code.text";
const CODE_KEYBOARD_FINAL_ID: &str = "code.sql";
const DISABLED_ID: &str = "opaque.disabled";
const ROOT_LABEL: &str = "編集 ⭐️ 👩‍💻";
const FIXTURE_SOURCE: &str = "日本語の source ⭐️ と 👩‍💻 を含む ContextMenu overflow fixture";
const WHEEL_TO_END: f32 = 10_000.0;

#[derive(Clone, Copy)]
enum ContextMenuOpenRoute {
    Secondary,
    ShiftF10,
    AccessKit,
}

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    assert_unicode_fixture()?;
    for route in routes() {
        select_direct_terminal(route)?;
        select_code_terminal(route)?;
        select_code_terminal_by_keyboard(route)?;
    }
    assert_disabled_leaf_and_dismissal()?;
    Ok(())
}

fn select_direct_terminal(route: ContextMenuOpenRoute) -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = Fixture::new();
    let (_, opened) = fixture.open(route)?;
    let (_, edit) = fixture.open_submenu(&opened, ROOT_ID)?;
    let (scroll_full, scrolled) = fixture.scroll_to_end(&edit)?;
    assert_visible_frame(
        &scroll_full,
        &scrolled,
        DIRECT_FINAL_ID,
        "直接項目 13",
        "直接項目 00",
    )?;
    let (_, selected) = fixture.select_leaf(&scrolled, DIRECT_FINAL_ID)?;
    assert_selected_once(&selected, DIRECT_FINAL_ID)
}

fn select_code_terminal(route: ContextMenuOpenRoute) -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = Fixture::new();
    let (_, opened) = fixture.open(route)?;
    let (_, edit) = fixture.open_submenu(&opened, ROOT_ID)?;
    let (_, scrolled_edit) = fixture.scroll_to_end(&edit)?;
    let (code_full, code) = fixture.open_submenu(&scrolled_edit, CODE_ID)?;
    assert_visible_frame(
        &code_full,
        &code,
        CODE_FINAL_ID,
        "コード種別 00",
        "コード種別 16",
    )?;
    let (_, selected) = fixture.select_leaf(&code, CODE_FINAL_ID)?;
    assert_selected_once(&selected, CODE_FINAL_ID)
}

fn select_code_terminal_by_keyboard(
    route: ContextMenuOpenRoute,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = Fixture::new();
    let (_, opened) = fixture.open(route)?;
    let (_, edit) = fixture.open_submenu(&opened, ROOT_ID)?;
    let (_, scrolled_edit) = fixture.scroll_to_end(&edit)?;
    let (_, _code) = fixture.open_submenu(&scrolled_edit, CODE_ID)?;

    let (mut full, mut focused) = fixture.frame(vec![harness::key(egui::Key::Home, false)])?;
    for _ in 0..16 {
        let (next_full, next) = fixture.frame(vec![harness::key(egui::Key::ArrowDown, false)])?;
        full = next_full;
        focused = next;
    }
    assert_visible_frame(
        &full,
        &focused,
        CODE_KEYBOARD_FINAL_ID,
        "コード種別 16",
        "コード種別 00",
    )?;
    let (_, selected) = fixture.frame(vec![harness::key(egui::Key::Enter, false)])?;
    assert_selected_once(&selected, CODE_KEYBOARD_FINAL_ID)
}

fn assert_disabled_leaf_and_dismissal() -> Result<(), Box<dyn std::error::Error>> {
    let mut fixture = Fixture::new();
    let (_, opened) = fixture.open(ContextMenuOpenRoute::Secondary)?;
    let (_, edit) = fixture.open_submenu(&opened, ROOT_ID)?;
    let (_, scrolled) = fixture.scroll_to_end(&edit)?;
    require(
        item_disabled(&scrolled, DISABLED_ID)?,
        "disabled leaf was not marked disabled in the public frame record",
    )?;
    let (_, disabled) = fixture.select_leaf(&scrolled, DISABLED_ID)?;
    require(
        context_events(&disabled)?.is_empty(),
        "disabled visible leaf emitted a context-menu event",
    )?;
    require(
        menu_record_optional(&disabled)?.is_some(),
        "disabled visible leaf unexpectedly closed the context menu",
    )?;
    let (_, escaped) = fixture.frame(vec![harness::key(egui::Key::Escape, false)])?;
    assert_closed_with(&escaped, ContextMenuCloseReason::Escape)?;
    let (_, restored) = fixture.frame(Vec::new())?;
    require(
        fixture.surface.text().state().text_area.focused
            && restored.text.record.frame.accessibility.root.focused,
        "Escape did not return focus to the text surface",
    )?;

    let (_, reopened) = fixture.open(ContextMenuOpenRoute::ShiftF10)?;
    let outside = outside_point(&reopened)?;
    let (_, closed) = fixture.frame(vec![harness::button(outside, true)])?;
    assert_closed_with(&closed, ContextMenuCloseReason::OutsideClick)?;
    let (_, restored) = fixture.frame(Vec::new())?;
    require(
        fixture.surface.text().state().text_area.focused
            && restored.text.record.frame.accessibility.root.focused,
        "outside dismissal did not return focus to the text surface",
    )
}

struct Fixture {
    context: egui::Context,
    adapter: EguiTextCommandSurfaceAdapter,
    surface: EguiTextCommandSurface,
}

impl Fixture {
    fn new() -> Self {
        let context = egui::Context::default();
        context.enable_accesskit();
        Self {
            context,
            adapter: EguiTextCommandSurfaceAdapter::with_text_raster_config(
                katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
            )
            .expect("text command adapter"),
            surface: EguiTextCommandSurface::new(overflow_text_surface())
                .with_context_menu(overflow_presentation()),
        }
    }

    fn frame(
        &mut self,
        events: Vec<egui::Event>,
    ) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
        harness::run_frame_sized(
            &self.context,
            &mut self.adapter,
            &mut self.surface,
            &harness::style(),
            SCREEN,
            events,
        )
    }

    fn open(
        &mut self,
        route: ContextMenuOpenRoute,
    ) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
        let (initial_full, initial) = self.frame(Vec::new())?;
        let point = text_point(&initial)?;
        let opened = match route {
            ContextMenuOpenRoute::Secondary => {
                let _ = self.frame(vec![harness::secondary_button(point, true)])?;
                self.frame(vec![harness::secondary_button(point, false)])
            }
            ContextMenuOpenRoute::ShiftF10 => {
                let _ = self.frame(vec![harness::button(point, true)])?;
                let _ = self.frame(vec![harness::button(point, false)])?;
                self.frame(vec![harness::key(egui::Key::F10, true)])
            }
            ContextMenuOpenRoute::AccessKit => {
                self.frame(vec![egui::Event::AccessKitActionRequest(
                    egui::accesskit::ActionRequest {
                        action: egui::accesskit::Action::ShowContextMenu,
                        target_tree: egui::accesskit::TreeId::ROOT,
                        target_node: text_accesskit_id(&initial_full)?,
                        data: None,
                    },
                )])
            }
        }?;
        assert_opening_accesskit(&opened.0, &opened.1)?;
        Ok(opened)
    }

    fn open_submenu(
        &mut self,
        output: &EguiTextCommandSurfaceOutput,
        id: &str,
    ) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
        let bounds = item_bounds(output, id)?;
        let point = center(bounds);
        let _ = self.frame(vec![egui::Event::PointerMoved(point)])?;
        let _ = self.frame(vec![harness::button(point, true)])?;
        let (_, released) = self.frame(vec![harness::button(point, false)])?;
        require(
            context_events(&released)?
                .iter()
                .any(|event| matches!(event, ContextMenuEvent::SubmenuOpened { .. })),
            &format!(
                "physical click did not open submenu {id} at {bounds:?}; events: {:?}",
                context_events(&released)?,
            ),
        )?;
        self.frame(Vec::new())
    }

    fn scroll_to_end(
        &mut self,
        output: &EguiTextCommandSurfaceOutput,
    ) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
        let point = center(menu_record(output)?.bounds);
        let first = self.frame(vec![
            egui::Event::PointerMoved(point),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, WHEEL_TO_END),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            },
        ])?;
        let repeat = self.frame(Vec::new())?;
        require(
            assertions::composite_hash(&first.1)? == assertions::composite_hash(&repeat.1)?,
            "ContextMenu compositor hash changed without new input",
        )?;
        Ok(repeat)
    }

    fn select_leaf(
        &mut self,
        output: &EguiTextCommandSurfaceOutput,
        id: &str,
    ) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
        let point = center(item_bounds(output, id)?);
        let _ = self.frame(vec![egui::Event::PointerMoved(point)])?;
        let _ = self.frame(vec![harness::button(point, true)])?;
        self.frame(vec![harness::button(point, false)])
    }
}

fn overflow_text_surface() -> TextSurface {
    TextSurface::new(TextSurfaceProps::new(
        TextArea::new("context-menu-overflow").value(FIXTURE_SOURCE),
        UiTextSpan::emoji_marked_spans(FIXTURE_SOURCE, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN.x as u32, SCREEN.y as u32),
    ))
}

fn overflow_presentation() -> ContextMenuPresentation {
    ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("context.save", "保存 ⭐️"),
            ContextMenuPresentationItem::action("context.format", "整形"),
            ContextMenuPresentationItem {
                kind: ContextMenuItemKind::Submenu,
                ..ContextMenuPresentationItem::action(ROOT_ID, ROOT_LABEL)
                    .child(direct_item(0))
                    .child(direct_item(1))
                    .child(direct_item(2))
                    .child(direct_item(3))
                    .child(direct_item(4))
                    .child(direct_item(5))
                    .child(direct_item(6))
                    .child(direct_item(7))
                    .child(direct_item(8))
                    .child(direct_item(9))
                    .child(code_submenu())
                    .child(direct_item(10))
                    .child(direct_item(11))
                    .child(direct_item(12))
                    .child(direct_item(13))
                    .child(disabled_item())
            },
            ContextMenuPresentationItem {
                kind: ContextMenuItemKind::Submenu,
                ..ContextMenuPresentationItem::action("context.ingest", "画像 👩‍💻")
                    .child(ContextMenuPresentationItem::action(
                        "image.file",
                        "ファイル",
                    ))
                    .child(ContextMenuPresentationItem::action(
                        "image.clipboard-image",
                        "クリップボード画像",
                    ))
            },
        ],
    }
}

fn direct_item(index: usize) -> ContextMenuPresentationItem {
    ContextMenuPresentationItem::action(
        format!("direct.{index:02}"),
        format!("直接項目 {index:02} ⭐️"),
    )
}

fn code_submenu() -> ContextMenuPresentationItem {
    let mut item = ContextMenuPresentationItem {
        kind: ContextMenuItemKind::Submenu,
        ..ContextMenuPresentationItem::action(CODE_ID, "入れ子 code 👩‍💻")
    };
    for (index, id) in [
        "text",
        "markdown",
        "bash",
        "zsh",
        "mermaid",
        "drawio",
        "plantuml",
        "json",
        "yaml",
        "toml",
        "rust",
        "typescript",
        "javascript",
        "python",
        "html",
        "css",
        "sql",
    ]
    .iter()
    .enumerate()
    {
        item = item.child(ContextMenuPresentationItem::action(
            format!("code.{id}"),
            format!("コード種別 {index:02} ⭐️"),
        ));
    }
    item
}

fn disabled_item() -> ContextMenuPresentationItem {
    ContextMenuPresentationItem {
        enabled: false,
        ..ContextMenuPresentationItem::action(DISABLED_ID, "無効項目 ⭐️")
    }
}

fn assert_visible_frame(
    full: &egui::FullOutput,
    output: &EguiTextCommandSurfaceOutput,
    terminal_id: &str,
    terminal_label: &str,
    hidden_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let record = menu_record(output)?;
    require(
        record
            .items
            .iter()
            .all(|item| contains_rect(record.bounds, item.bounds)),
        "context-menu frame exposed an item outside its bounds",
    )?;
    let terminal = record.items.iter().any(|item| item.id == terminal_id);
    require(
        terminal,
        &format!(
            "physical scroll did not publish {terminal_id}; visible ids: {:?}",
            record
                .items
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>()
        ),
    )?;
    require(
        output.artifact_order()
            == [
                EguiTextCommandSurfaceChild::Text,
                EguiTextCommandSurfaceChild::ContextMenu,
            ],
        "context-menu artifact order changed",
    )?;
    let artifact = context_output(output)?
        .artifact
        .as_ref()
        .ok_or_else(|| error("context-menu artifact was absent"))?;
    require(
        artifact
            .paint_plan
            .operations
            .iter()
            .all(|operation| operation.clip_bounds == record.bounds),
        "context-menu paint plan escaped the clipped record bounds",
    )?;
    assert_accesskit(full, terminal_label, output.root_bounds)?;
    assert_accesskit_omits(full, hidden_label)
}

fn assert_opening_accesskit(
    full: &egui::FullOutput,
    output: &EguiTextCommandSurfaceOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    require(
        menu_record(output)?
            .items
            .iter()
            .any(|item| item.id == ROOT_ID),
        "opening route did not publish the root submenu item",
    )?;
    assert_accesskit(full, ROOT_LABEL, output.root_bounds)
}

fn assert_unicode_fixture() -> Result<(), Box<dyn std::error::Error>> {
    require(
        FIXTURE_SOURCE.contains("日本語")
            && FIXTURE_SOURCE.contains("⭐️")
            && FIXTURE_SOURCE.contains("👩‍💻")
            && ROOT_LABEL.contains("⭐️")
            && ROOT_LABEL.contains("👩‍💻"),
        "overflow fixture lost Japanese, VS16, or ZWJ coverage",
    )
}

fn assert_selected_once(
    output: &EguiTextCommandSurfaceOutput,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let events = context_events(output)?;
    let count = events
        .iter()
        .filter(|event| matches!(event, ContextMenuEvent::ItemSelected { command: actual, .. } if actual == command))
        .count();
    require(count == 1, "terminal leaf did not activate exactly once")?;
    require(
        !events.iter().any(|event| {
            matches!(
                event,
                ContextMenuEvent::Closed {
                    reason: ContextMenuCloseReason::OutsideClick
                }
            )
        }),
        "visible terminal leaf was dismissed as an outside click",
    )
}

fn assert_closed_with(
    output: &EguiTextCommandSurfaceOutput,
    reason: ContextMenuCloseReason,
) -> Result<(), Box<dyn std::error::Error>> {
    require(
        context_events(output)?.iter().any(|event| matches!(event, ContextMenuEvent::Closed { reason: actual } if *actual == reason)),
        "context menu did not close with the expected KUC-owned reason",
    )?;
    require(
        menu_record_optional(output)?.is_none(),
        "closed context menu retained a frame record",
    )
}

fn assert_accesskit(
    full: &egui::FullOutput,
    label: &str,
    root: UiRect,
) -> Result<(), Box<dyn std::error::Error>> {
    let update = full
        .platform_output
        .accesskit_update
        .as_ref()
        .ok_or_else(|| error("AccessKit update was absent"))?;
    let has_text = update
        .nodes
        .iter()
        .any(|(_, node)| node.role() == egui::accesskit::Role::MultilineTextInput);
    let has_menu = update
        .nodes
        .iter()
        .any(|(_, node)| node.role() == egui::accesskit::Role::Menu);
    let has_item = update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::MenuItem
            && node.label().is_some_and(|value| value.contains(label))
    });
    require(
        has_text && has_menu && has_item,
        "AccessKit did not expose text, menu, and visible item roles",
    )?;
    let escaped = update.nodes.iter().find_map(|(_, node)| {
        node.bounds()
            .filter(|bounds| {
                bounds.x0 < f64::from(root.x)
                    || bounds.y0 < f64::from(root.y)
                    || bounds.x1 > f64::from(root.x.saturating_add_unsigned(root.width))
                    || bounds.y1 > f64::from(root.y.saturating_add_unsigned(root.height))
            })
            .map(|bounds| (node.role(), node.label().unwrap_or_default(), bounds))
    });
    let Some((role, label, bounds)) = escaped else {
        return Ok(());
    };
    Err(error(&format!(
        "AccessKit node escaped root: role={role:?} label={label:?} bounds={bounds:?}"
    )))
}

fn assert_accesskit_omits(
    full: &egui::FullOutput,
    hidden_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let update = full
        .platform_output
        .accesskit_update
        .as_ref()
        .ok_or_else(|| error("AccessKit update was absent"))?;
    let hidden = update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::MenuItem
            && node
                .label()
                .is_some_and(|label| label.contains(hidden_label))
    });
    require(
        !hidden,
        "AccessKit exposed a menu item outside the public frame record",
    )
}

fn context_output(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<
    &katana_ui_core_egui_adapter::context_menu::EguiContextMenuOutput,
    Box<dyn std::error::Error>,
> {
    output
        .context_menu
        .as_ref()
        .ok_or_else(|| error("root context-menu output was absent"))
}

fn menu_record(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<&EguiContextMenuFrameRecord, Box<dyn std::error::Error>> {
    menu_record_optional(output)?.ok_or_else(|| error("context-menu frame record was absent"))
}

fn menu_record_optional(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<Option<&EguiContextMenuFrameRecord>, Box<dyn std::error::Error>> {
    Ok(context_output(output)?.record.as_ref())
}

fn context_events(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<&[ContextMenuEvent], Box<dyn std::error::Error>> {
    Ok(context_output(output)?.events.as_slice())
}

fn item_bounds(
    output: &EguiTextCommandSurfaceOutput,
    id: &str,
) -> Result<UiRect, Box<dyn std::error::Error>> {
    menu_record(output)?
        .items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.bounds)
        .ok_or_else(|| error("requested item was absent from the public frame record"))
}

fn item_disabled(
    output: &EguiTextCommandSurfaceOutput,
    id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    menu_record(output)?
        .items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.disabled)
        .ok_or_else(|| error("requested item was absent from the public frame record"))
}

fn text_point(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<egui::Pos2, Box<dyn std::error::Error>> {
    let bounds = output.text.record.frame.content_bounds;
    require(
        bounds.width > 0 && bounds.height > 0,
        "text surface content bounds were empty",
    )?;
    Ok(egui::pos2(bounds.x as f32 + 1.0, bounds.y as f32 + 1.0))
}

fn text_accesskit_id(
    full: &egui::FullOutput,
) -> Result<egui::accesskit::NodeId, Box<dyn std::error::Error>> {
    let update = full
        .platform_output
        .accesskit_update
        .as_ref()
        .ok_or_else(|| error("TextSurface AccessKit update was absent"))?;
    update
        .nodes
        .iter()
        .find_map(|(id, node)| {
            (node.role() == egui::accesskit::Role::MultilineTextInput).then_some(*id)
        })
        .ok_or_else(|| error("TextSurface AccessKit input node was absent"))
}

fn outside_point(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<egui::Pos2, Box<dyn std::error::Error>> {
    let root = output.root_bounds;
    let menu = menu_record(output)?.bounds;
    [
        egui::pos2(root.x as f32 + 1.0, root.y as f32 + 1.0),
        egui::pos2(
            root.x.saturating_add_unsigned(root.width).saturating_sub(1) as f32,
            root.y as f32 + 1.0,
        ),
    ]
    .into_iter()
    .find(|point| !contains(menu, *point))
    .ok_or_else(|| error("KUC root did not expose an outside point"))
}

fn routes() -> [ContextMenuOpenRoute; 3] {
    [
        ContextMenuOpenRoute::Secondary,
        ContextMenuOpenRoute::ShiftF10,
        ContextMenuOpenRoute::AccessKit,
    ]
}

fn center(bounds: UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

fn contains(bounds: UiRect, point: egui::Pos2) -> bool {
    point.x >= bounds.x as f32
        && point.x < bounds.x.saturating_add_unsigned(bounds.width) as f32
        && point.y >= bounds.y as f32
        && point.y < bounds.y.saturating_add_unsigned(bounds.height) as f32
}

fn contains_rect(bounds: UiRect, item: UiRect) -> bool {
    item.x >= bounds.x
        && item.y >= bounds.y
        && item.x.saturating_add_unsigned(item.width)
            <= bounds.x.saturating_add_unsigned(bounds.width)
        && item.y.saturating_add_unsigned(item.height)
            <= bounds.y.saturating_add_unsigned(bounds.height)
}

fn require(value: bool, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    value.then_some(()).ok_or_else(|| error(message))
}

fn error(message: &str) -> Box<dyn std::error::Error> {
    std::io::Error::other(message).into()
}
