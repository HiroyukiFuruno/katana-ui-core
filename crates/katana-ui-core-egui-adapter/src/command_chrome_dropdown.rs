use super::command_chrome_interaction::publish_labeled_button_accesskit;
use super::command_chrome_paint::{ActionPaintSource, DropdownPaintSource};
use super::command_chrome_types::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeDropdownFrame,
    EguiCommandChromeDropdownItemFrame, EguiCommandChromeError, RenderedAction,
};
use super::{EguiCommandChromeAdapter, ui_rect};
use katana_ui_core::interaction::placement::{Rect, Size};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdown, CommandChromeDropdownLayout, CommandChromeToolbar,
    CommandChromeToolbarAction, CommandChromeToolbarEvent,
};
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
    let Some(dropdown) = toolbar
        .actions()
        .iter()
        .find(|action| action.id() == open.action_id())
        .and_then(|action| action.dropdown_model())
        .cloned()
    else {
        return Ok(None);
    };
    let rendered = render_dropdown_items(adapter, ui, &dropdown, raster_style)?;
    let bounds = ui_rect_from_placement(open.bounds());
    let menu_rect = egui_rect(bounds);
    let menu_id = ui
        .id()
        .with(("kuc-command-chrome-dropdown", open.action_id().as_str()));
    /* WHY: The toolbar is commonly rendered in a height-constrained child UI. A dropdown must
    use a KUC-owned foreground layer so both its pixels and hit targets escape that child clip. */
    let (items, paint_sources) = egui::Area::new(menu_id)
        .order(egui::Order::Foreground)
        .fixed_pos(menu_rect.min)
        .show(ui.ctx(), |menu_ui| {
            menu_ui.set_min_size(menu_rect.size());
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
                publish_labeled_button_accesskit(
                    menu_ui,
                    response.id,
                    &accessibility_label,
                    item.item.disabled_model(),
                    row_bounds,
                    item.item.id().as_str(),
                    crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DropdownItem,
                );
                if response.clicked()
                    && !keyboard_dropdown_activation(menu_ui)
                    && !item.item.disabled_model()
                {
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
            (items, paint_sources)
        })
        .inner;
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

pub(super) fn keyboard_dropdown_activation(ui: &egui::Ui) -> bool {
    ui.input(|input| {
        input.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key {
                    key: egui::Key::Enter | egui::Key::Space,
                    pressed: true,
                    ..
                }
            )
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
