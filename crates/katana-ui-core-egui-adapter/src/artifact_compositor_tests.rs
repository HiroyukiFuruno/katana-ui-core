use super::*;
use crate::command_chrome::{
    CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
    EguiCommandChromeDrawLayer,
};
use crate::context_menu::{
    ContextMenuPaintOperation, ContextMenuPaintOperationKind, ContextMenuPaintPlan,
    ContextMenuPaintTexture,
};
use crate::diagnostics_list::{
    DiagnosticsListPaintOperation, DiagnosticsListPaintOperationKind, DiagnosticsListPaintPlan,
    DiagnosticsListPaintTexture,
};
use crate::source_address_strip::{
    SourceAddressPaintOperation, SourceAddressPaintOperationKind, SourceAddressPaintPlan,
    SourceAddressPaintTexture,
};
use crate::status_bar::{
    StatusBarPaintOperation, StatusBarPaintOperationKind, StatusBarPaintPlan, StatusBarPaintTexture,
};
use crate::tab_strip_paint::{
    TabStripPaintOperation, TabStripPaintOperationKind, TabStripPaintPlan, TabStripPaintTexture,
};
use crate::text_surface::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
    TextSurfacePaintPlan, TextSurfacePaintTexture,
};
use katana_ui_core::render_model::UiRect;

const CANVAS_X: i32 = 10;
const CANVAS_Y: i32 = 10;
const OVERLAY_X: i32 = 11;
const DRAW_START_X: i32 = 9;
const SURFACE_WIDTH: u32 = 2;
const SURFACE_HEIGHT: u32 = 2;
const DRAW_WIDTH: u32 = 3;
const ONE_PIXEL: u32 = 1;

fn text_plan(kind: TextSurfacePaintOperationKind) -> TextSurfacePaintPlan {
    TextSurfacePaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        viewport_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![TextSurfacePaintOperation {
            layer: EguiTextSurfaceDrawLayer::Background,
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn chrome_plan(kind: CommandChromePaintOperationKind) -> CommandChromePaintPlan {
    CommandChromePaintPlan {
        surface_bounds: UiRect::new(OVERLAY_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        operations: vec![CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::PanelFill,
            clip_bounds: UiRect::new(OVERLAY_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            kind,
        }],
    }
}

fn source_address_plan(kind: SourceAddressPaintOperationKind) -> SourceAddressPaintPlan {
    SourceAddressPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![SourceAddressPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn status_bar_plan(kind: StatusBarPaintOperationKind) -> StatusBarPaintPlan {
    StatusBarPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![StatusBarPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn diagnostics_list_plan(kind: DiagnosticsListPaintOperationKind) -> DiagnosticsListPaintPlan {
    DiagnosticsListPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![DiagnosticsListPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn context_menu_plan(kind: ContextMenuPaintOperationKind) -> ContextMenuPaintPlan {
    ContextMenuPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![ContextMenuPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn tab_strip_plan(kind: TabStripPaintOperationKind) -> TabStripPaintPlan {
    TabStripPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![TabStripPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn require_ok<T: std::fmt::Debug, E: std::fmt::Debug>(
    result: Result<T, E>,
    context: &str,
) -> Option<T> {
    assert!(result.is_ok(), "{context}: {result:?}");
    result.ok()
}

fn require_err<T: std::fmt::Debug, E: std::fmt::Debug>(
    result: Result<T, E>,
    context: &str,
) -> Option<E> {
    assert!(result.is_err(), "{context}: {result:?}");
    result.err()
}

#[path = "artifact_compositor_tests/basic.rs"]
mod basic;
#[path = "artifact_compositor_tests/components.rs"]
mod components;
#[path = "artifact_compositor_tests/failures.rs"]
mod failures;
#[path = "artifact_compositor_tests/tab_strip_bounds.rs"]
mod tab_strip_bounds;
