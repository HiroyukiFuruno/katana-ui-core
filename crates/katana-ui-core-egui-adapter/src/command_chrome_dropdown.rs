use super::command_chrome_interaction::CommandChromeInteraction;
use super::command_chrome_paint::{ActionPaintSource, DropdownPaintSource};
use super::command_chrome_types::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeDropdownFrame,
    EguiCommandChromeDropdownItemFrame, EguiCommandChromeError, RenderedAction,
};
use super::{EguiCommandChromeAdapter, ui_rect};
use katana_ui_core::interaction::placement::{Rect, Size};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDropdown, CommandChromeDropdownLayout, CommandChromeToolbar,
    CommandChromeToolbarAction, CommandChromeToolbarEvent,
};
use katana_ui_core::molecule::toolbar::ToolbarActionId;
use katana_ui_core::render_model::UiRect;

const MENU_ROW_PADDING_PX: u32 = 8;

struct RenderedDropdownItem {
    item: katana_ui_core::molecule::command_chrome::CommandChromeDropdownItem,
    rendered: RenderedAction,
}

pub(super) struct DropdownPresentation {
    pub(super) record: EguiCommandChromeDropdownFrame,
    pub(super) paint: DropdownPaintSource,
}

pub(super) fn dropdown_layout(
    adapter: &mut EguiCommandChromeAdapter,
    ui: &egui::Ui,
    trigger_bounds: UiRect,
    dropdown: &CommandChromeDropdown,
    style: &CommandChromeRasterStyle,
) -> Result<CommandChromeDropdownLayout, EguiCommandChromeError> {
    let rendered = render_dropdown_items(adapter, ui, dropdown, style)?;
    let size = dropdown_size(&rendered);
    Ok(CommandChromeDropdownLayout::new(
        placement_rect(trigger_bounds),
        placement_rect(ui_rect(ui.ctx().content_rect())),
        size,
    ))
}

pub(super) fn show_dropdown(
    adapter: &mut EguiCommandChromeAdapter,
    ui: &mut egui::Ui,
    toolbar: &mut CommandChromeToolbar,
    toolbar_bounds: UiRect,
    raster_style: &CommandChromeRasterStyle,
    paint_style: &CommandChromePaintStyle,
    events: &mut Vec<CommandChromeToolbarEvent>,
) -> Result<Option<DropdownPresentation>, EguiCommandChromeError> {
    let Some(open) = toolbar.open_dropdown_model().cloned() else {
        return Ok(None);
    };
    let actions = toolbar.actions();
    let action_id = open.action_id();
    let Some(dropdown) = dropdown_for_action(actions, action_id) else {
        return Ok(None);
    };
    let rendered = render_dropdown_items(adapter, ui, &dropdown, raster_style)?;
    let bounds = ui_rect_from_placement(open.bounds());
    let menu_rect = egui_rect(bounds);
    let menu_id = ui
        .id()
        .with(("kuc-command-chrome-dropdown", open.action_id().as_str()));
    let mut menu_ui = ui.new_child(egui::UiBuilder::new().id(menu_id).max_rect(menu_rect));
    let mut items = Vec::with_capacity(rendered.len());
    let mut paint_sources = Vec::with_capacity(rendered.len());
    for (index, item) in rendered.iter().enumerate() {
        let row_size = egui::vec2(bounds.width as f32, item.rendered.bounds.height as f32);
        let (row_rect, response) = menu_ui.allocate_exact_size(row_size, egui::Sense::click());
        let focused = open.focused_item_index() == Some(index);
        let row_bounds = ui_rect(row_rect);
        let accessibility_label = item
            .item
            .accessibility_label_model()
            .or_else(|| item.item.tooltip_model())
            .map_or_else(|| item.item.label_model().to_string(), Clone::clone);
        CommandChromeInteraction::publish_labeled_button_accesskit(
            &menu_ui,
            response.id,
            &accessibility_label,
            item.item.disabled_model(),
            row_bounds,
            item.item.id().as_str(),
            crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DropdownItem,
        );
        if response.clicked() && !item.item.disabled_model() {
            events.extend(
                toolbar.apply_action(CommandChromeToolbarAction::select_dropdown_item(
                    open.action_id().clone(),
                    item.item.id().clone(),
                )),
            );
        }
        items.push(EguiCommandChromeDropdownItemFrame {
            item_id: item.item.id().as_str().to_string(),
            bounds: row_bounds,
            icon_raster_identity: item.rendered.icon_identity.clone(),
            label_raster_identity: item.rendered.label_identity.clone().unwrap_or_default(),
            disabled: item.item.disabled_model(),
            selected: item.item.selected_model(),
            focused,
        });
        paint_sources.push(ActionPaintSource::new(
            row_bounds,
            None,
            response.hovered() || item.item.selected_model() || focused,
            false,
            item.item.disabled_model(),
            item.rendered.clone(),
        ));
    }
    if pointer_pressed_outside(ui, bounds, toolbar_bounds, &items) {
        events.extend(toolbar.apply_action(CommandChromeToolbarAction::DismissDropdown {
            reason: katana_ui_core::molecule::command_chrome::CommandChromeDropdownCloseReason::OutsideClick,
        }));
    }
    Ok(Some(DropdownPresentation {
        record: EguiCommandChromeDropdownFrame {
            action_id: open.action_id().as_str().to_string(),
            trigger_bounds: ui_rect_from_placement(open.trigger_bounds()),
            bounds,
            items,
        },
        paint: DropdownPaintSource::new(bounds, paint_style.action_rgba, paint_sources),
    }))
}

fn dropdown_for_action(
    actions: &[CommandChromeAction],
    action_id: &ToolbarActionId,
) -> Option<CommandChromeDropdown> {
    actions
        .iter()
        .find(|action| action.id() == action_id)
        .and_then(CommandChromeAction::dropdown_model)
        .cloned()
}

#[cfg(test)]
mod missing_dropdown_tests {
    use super::*;
    use katana_ui_core::molecule::RgbaColor;
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeDropdownItem, CommandChromeDropdownTrigger, CommandChromeIcon,
    };
    use katana_ui_core::theme::{FontFamily, FontToken};

    #[test]
    fn missing_action_or_dropdown_fails_closed() {
        let missing = ToolbarActionId::new("missing");
        assert!(dropdown_for_action(&[], &missing).is_none());
        let action = CommandChromeAction::new("plain", "Plain");
        assert!(dropdown_for_action(&[action], &ToolbarActionId::new("plain")).is_none());
    }

    #[test]
    fn stale_open_dropdown_without_an_action_returns_no_presentation() {
        let action_id = ToolbarActionId::new("menu");
        let mut toolbar = CommandChromeToolbar::new().action(
            CommandChromeAction::new("menu", "Menu").dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
                    .item(CommandChromeDropdownItem::new("item", "Item")),
            ),
        );
        let layout = CommandChromeDropdownLayout::new(
            Rect::new(0, 0, 10, 10),
            Rect::new(0, 0, 100, 100),
            Size::new(20, 10),
        );
        let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
            action_id.clone(),
            layout,
        ));
        let _ = toolbar.apply_action(CommandChromeToolbarAction::activate(action_id));
        assert!(toolbar.open_dropdown_model().is_some());
        let mut value = serde_json::to_value(&toolbar).expect("toolbar serialization");
        value["actions"] = serde_json::json!([]);
        let mut stale: CommandChromeToolbar =
            serde_json::from_value(value).expect("stale toolbar fixture");
        let context = egui::Context::default();
        let mut adapter = EguiCommandChromeAdapter::default();
        let raster = CommandChromeRasterStyle {
            font: FontToken {
                name: "test".into(),
                family: FontFamily::Proportional,
                size: 12.0,
                weight: 400,
            },
            text_color_rgba: [255; 4],
            icon_color: RgbaColor::new(255, 255, 255, 255),
            line_height_px: 16.0,
            icon_size_px: 12,
        };
        let paint = CommandChromePaintStyle {
            action_rgba: [0; 4],
            hovered_action_rgba: [0; 4],
            disabled_action_rgba: [0; 4],
        };
        let mut result = None;
        crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
            let mut events = Vec::new();
            result = Some(show_dropdown(
                &mut adapter,
                ui,
                &mut stale,
                UiRect::new(0, 0, 100, 20),
                &raster,
                &paint,
                &mut events,
            ));
        });
        assert!(result.is_some_and(|value| matches!(value, Ok(None))));
    }

    #[test]
    fn dropdown_size_and_containment_helpers_cover_bounds_and_defaults() {
        let empty = dropdown_size(&[]);
        assert_eq!(empty, Size::new(1, 1));

        let rendered = vec![
            RenderedDropdownItem {
                item: CommandChromeDropdownItem::new("first", "first"),
                rendered: RenderedAction {
                    bounds: UiRect::new(0, 0, 10, 4),
                    icon: None,
                    label: None,
                    icon_identity: Some("icon".into()),
                    label_identity: None,
                },
            },
            RenderedDropdownItem {
                item: CommandChromeDropdownItem::new("second", "second"),
                rendered: RenderedAction {
                    bounds: UiRect::new(0, 0, 30, 6),
                    icon: None,
                    label: None,
                    icon_identity: None,
                    label_identity: Some("label".into()),
                },
            },
        ];
        assert_eq!(dropdown_size(&rendered), Size::new(30, 10));

        let layout = placement_rect(UiRect::new(1, 2, 3, 4));
        assert_eq!(layout, Rect::new(1, 2, 3, 4));
        assert_eq!(
            ui_rect_from_placement(Rect::new(2, 3, 4, 5)),
            UiRect::new(2, 3, 4, 5)
        );

        assert!(contains(UiRect::new(0, 0, 10, 10), egui::pos2(0.0, 0.0)));
        assert!(!contains(UiRect::new(0, 0, 10, 10), egui::pos2(10.0, 10.0)));
    }

    #[test]
    fn dropdown_item_icon_identity_is_preserved() {
        let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary).item(
            CommandChromeDropdownItem::new("item", "Item")
                .icon(CommandChromeIcon::EmphasisStrong.icon_props()),
        );
        let context = egui::Context::default();
        let mut adapter = EguiCommandChromeAdapter::default();
        let style = CommandChromeRasterStyle {
            font: FontToken {
                name: "test".into(),
                family: FontFamily::Proportional,
                size: 12.0,
                weight: 400,
            },
            text_color_rgba: [255; 4],
            icon_color: RgbaColor::new(255, 255, 255, 255),
            line_height_px: 16.0,
            icon_size_px: 12,
        };
        let mut identity = None;
        crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
            let rendered = render_dropdown_items(&mut adapter, ui, &dropdown, &style)
                .expect("dropdown rasterization");
            identity = rendered[0].rendered.icon_identity.clone();
        });
        assert!(identity.is_some());
    }

    #[test]
    fn show_dropdown_renders_generic_label_and_icon_inputs() {
        let action_id = ToolbarActionId::new("format");
        let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary).item(
            CommandChromeDropdownItem::new("strong", "⭐️ 日本語")
                .icon(CommandChromeIcon::EmphasisStrong.icon_props()),
        );
        let mut toolbar = CommandChromeToolbar::new()
            .action(CommandChromeAction::new("format", "Format").dropdown(dropdown));
        let layout = CommandChromeDropdownLayout::new(
            Rect::new(20, 20, 80, 24),
            Rect::new(0, 0, 640, 480),
            Size::new(240, 48),
        );
        let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
            action_id.clone(),
            layout,
        ));
        let _ = toolbar.apply_action(CommandChromeToolbarAction::activate(action_id));

        let context = egui::Context::default();
        let mut adapter = EguiCommandChromeAdapter::default();
        let raster = CommandChromeRasterStyle {
            font: FontToken {
                name: "test".into(),
                family: FontFamily::Proportional,
                size: 14.0,
                weight: 400,
            },
            text_color_rgba: [255; 4],
            icon_color: RgbaColor::new(255, 255, 255, 255),
            line_height_px: 18.0,
            icon_size_px: 16,
        };
        let paint = CommandChromePaintStyle {
            action_rgba: [0; 4],
            hovered_action_rgba: [0; 4],
            disabled_action_rgba: [0; 4],
        };
        let mut presentation = None;
        crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
            let mut events = Vec::new();
            presentation = show_dropdown(
                &mut adapter,
                ui,
                &mut toolbar,
                UiRect::new(20, 20, 80, 24),
                &raster,
                &paint,
                &mut events,
            )
            .expect("dropdown presentation");
            assert!(events.is_empty());
        });

        let presentation = presentation.expect("open dropdown presentation");
        let item = presentation.record.items.first().expect("dropdown item");
        assert_eq!(presentation.record.action_id, "format");
        assert!(!item.label_raster_identity.is_empty());
        assert!(item.icon_raster_identity.is_some());
        assert!(item.bounds.width > 0);
        assert!(item.bounds.height > 0);
    }

    #[test]
    fn pointer_pressed_outside_respects_menu_toolbar_items() {
        let ui_bounds = UiRect::new(0, 0, 30, 30);
        let toolbar_bounds = UiRect::new(100, 100, 10, 10);
        let items = vec![EguiCommandChromeDropdownItemFrame {
            item_id: "first".into(),
            bounds: UiRect::new(0, 0, 8, 8),
            icon_raster_identity: None,
            label_raster_identity: String::new(),
            disabled: false,
            selected: false,
            focused: false,
        }];
        let mut outside = false;
        let context = egui::Context::default();
        {
            let raw = egui::RawInput {
                events: vec![egui::Event::PointerButton {
                    pos: egui::pos2(50.0, 50.0),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..egui::RawInput::default()
            };
            let mut output = context.run_ui(raw, |ui| {
                outside = pointer_pressed_outside(ui, ui_bounds, toolbar_bounds, &items);
            });
            output.textures_delta.clear();
        }
        assert!(outside);

        let mut inside = false;
        {
            let raw = egui::RawInput {
                events: vec![egui::Event::PointerButton {
                    pos: egui::pos2(2.0, 2.0),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..egui::RawInput::default()
            };
            let mut output = context.run_ui(raw, |ui| {
                inside = pointer_pressed_outside(ui, ui_bounds, toolbar_bounds, &items);
            });
            output.textures_delta.clear();
        }
        assert!(!inside);
    }
}

fn render_dropdown_items(
    adapter: &mut EguiCommandChromeAdapter,
    ui: &egui::Ui,
    dropdown: &CommandChromeDropdown,
    style: &CommandChromeRasterStyle,
) -> Result<Vec<RenderedDropdownItem>, EguiCommandChromeError> {
    dropdown
        .items()
        .iter()
        .cloned()
        .map(|item| {
            let icon = item
                .icon_model()
                .map(|icon| adapter.raster_icon(icon, style, ui.ctx().pixels_per_point()))
                .transpose()?;
            let label =
                adapter.raster_label(item.label_model(), style, ui.ctx().pixels_per_point())?;
            let width = icon
                .as_ref()
                .map_or(0, |raster| raster.width)
                .saturating_add(label.width)
                .saturating_add(MENU_ROW_PADDING_PX.saturating_mul(2));
            let height = icon
                .as_ref()
                .map_or(0, |raster| raster.height)
                .max(label.height)
                .saturating_add(MENU_ROW_PADDING_PX.saturating_mul(2));
            Ok(RenderedDropdownItem {
                rendered: RenderedAction {
                    bounds: UiRect::new(0, 0, width.max(1), height.max(1)),
                    icon_identity: icon.as_ref().map(|raster| raster.identity.clone()),
                    label_identity: Some(label.identity.clone()),
                    icon,
                    label: Some(label),
                },
                item,
            })
        })
        .collect()
}

fn dropdown_size(items: &[RenderedDropdownItem]) -> Size {
    let width = items
        .iter()
        .map(|item| item.rendered.bounds.width)
        .max()
        .unwrap_or(1);
    let height = items.iter().fold(0_u32, |total, item| {
        total.saturating_add(item.rendered.bounds.height)
    });
    Size::new(width, height.max(1))
}

fn placement_rect(bounds: UiRect) -> Rect {
    Rect::new(bounds.x, bounds.y, bounds.width, bounds.height)
}

fn ui_rect_from_placement(bounds: Rect) -> UiRect {
    UiRect::new(bounds.x, bounds.y, bounds.width, bounds.height)
}

fn egui_rect(bounds: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    )
}

fn pointer_pressed_outside(
    ui: &egui::Ui,
    dropdown: UiRect,
    toolbar: UiRect,
    items: &[EguiCommandChromeDropdownItemFrame],
) -> bool {
    ui.input(|input| {
        input.events.iter().any(|event| {
            let egui::Event::PointerButton {
                pos, pressed: true, ..
            } = event
            else {
                return false;
            };
            !contains(dropdown, *pos)
                && !contains(toolbar, *pos)
                && !items
                    .iter()
                    .any(|item| !item.disabled && contains(item.bounds, *pos))
        })
    })
}

fn contains(bounds: UiRect, point: egui::Pos2) -> bool {
    let x = point.x.round() as i32;
    let y = point.y.round() as i32;
    x >= bounds.x
        && x < bounds.x.saturating_add_unsigned(bounds.width)
        && y >= bounds.y
        && y < bounds.y.saturating_add_unsigned(bounds.height)
}
