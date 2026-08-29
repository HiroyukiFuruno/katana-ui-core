use katana_ui_core::molecule::selection::ContextMenuItemKind;
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPaintStyle, ContextMenuPresentation, ContextMenuPresentationItem,
    ContextMenuRasterStyle,
};

const FONT_SIZE_PX: f32 = 15.0;
const FONT_WEIGHT: u16 = 400;
const LINE_HEIGHT_PX: f32 = 22.0;
const TEXT_RGBA: [u8; 4] = [235, 235, 235, 255];
const MENU_BACKGROUND_RGBA: [u8; 4] = [30, 30, 32, 255];
const MENU_HIGHLIGHTED_RGBA: [u8; 4] = [60, 80, 112, 255];
const MENU_DISABLED_RGBA: [u8; 4] = [45, 45, 48, 255];

pub(super) fn context_menu_presentation() -> ContextMenuPresentation {
    ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("format", "整形 ⭐️"),
            ContextMenuPresentationItem {
                id: "code-kind".to_string(),
                label: "コード種別".to_string(),
                accessibility_label: "コード種別".to_string(),
                icon: None,
                enabled: true,
                checked: false,
                kind: ContextMenuItemKind::Submenu,
                children: vec![ContextMenuPresentationItem::action(
                    "opaque-code-kind",
                    "opaque code kind",
                )],
            },
            ContextMenuPresentationItem {
                id: "disabled".to_string(),
                label: "利用不可".to_string(),
                accessibility_label: "利用不可".to_string(),
                icon: None,
                enabled: false,
                checked: false,
                kind: ContextMenuItemKind::Action,
                children: Vec::new(),
            },
        ],
    }
}

pub(super) fn context_menu_raster_style() -> ContextMenuRasterStyle {
    ContextMenuRasterStyle {
        font: FontToken {
            name: "storybook-context-menu".to_string(),
            family: FontFamily::Proportional,
            size: FONT_SIZE_PX,
            weight: FONT_WEIGHT,
        },
        text_color_rgba: TEXT_RGBA,
        icon_color_rgba: TEXT_RGBA,
        line_height_px: LINE_HEIGHT_PX,
    }
}

pub(super) const fn context_menu_paint_style() -> ContextMenuPaintStyle {
    ContextMenuPaintStyle {
        background_rgba: MENU_BACKGROUND_RGBA,
        highlighted_rgba: MENU_HIGHLIGHTED_RGBA,
        disabled_rgba: MENU_DISABLED_RGBA,
    }
}

#[cfg(test)]
#[path = "context_menu_surface_integration_support.rs"]
mod context_menu_surface_integration_support;
#[cfg(test)]
#[path = "context_menu_surface_integration_tests.rs"]
mod context_menu_surface_integration_tests;
