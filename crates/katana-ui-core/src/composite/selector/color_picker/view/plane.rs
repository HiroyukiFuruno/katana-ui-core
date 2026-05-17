use crate::composite::selector::color_picker::ops;
use crate::composite::selector::color_picker::types::ColorPickerValue;
use crate::composite::selector::color_picker::view::apply_state;
use crate::composite::selector::color_picker::view::paint_util::contrast_color;
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::context::{ComputeLayoutCx, EventCx, PaintCx, UpdateCx};
use floem::event::{Event, EventPropagation};
use floem::kurbo::{Circle, Point, Rect, Stroke};
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, create_effect};
use floem::views::Decorators;
use floem::{IntoView, View, ViewId};
use floem_renderer::Renderer;
use std::rc::Rc;

const PLANE_BASE_WIDTH: f32 = 520.0;
const PLANE_BASE_HEIGHT: f32 = 320.0;
const GRID_COLUMNS: usize = 128;
const GRID_ROWS: usize = 96;
const CELL_OVERLAP: f64 = 0.6;
const BORDER_WIDTH: f64 = 1.0;
const HANDLE_RADIUS: f64 = 26.0;
const SAMPLE_CENTER_OFFSET: f64 = 0.5;

enum PlaneUpdate {
    Value(ColorPickerValue),
}

struct EguiColorPlane {
    id: ViewId,
    state: RwSignal<ColorPickerValue>,
    on_change: Rc<dyn Fn(Color)>,
    current: ColorPickerValue,
    locked: bool,
    border_color: PenikoColor,
    held: bool,
    size: floem::taffy::prelude::Size<f32>,
}

pub(super) fn color_plane(
    state: RwSignal<ColorPickerValue>,
    on_change: Rc<dyn Fn(Color)>,
    locked: bool,
    panel_scale: f32,
    theme: Theme,
    _allows_alpha: bool,
) -> impl IntoView {
    EguiColorPlane::new(state, on_change, locked, theme).style(move |style| {
        style
            .width(PLANE_BASE_WIDTH * panel_scale)
            .height(PLANE_BASE_HEIGHT * panel_scale)
    })
}

impl EguiColorPlane {
    fn new(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        theme: Theme,
    ) -> Self {
        let id = ViewId::new();
        create_effect(move |_| id.update_state(PlaneUpdate::Value(state.get())));
        Self {
            id,
            state,
            on_change,
            current: state.get_untracked(),
            locked,
            border_color: FloemColor::from_token(theme.color.border),
            held: false,
            size: floem::taffy::prelude::Size::default(),
        }
    }

    fn apply_pointer(&self, position_x: f64, position_y: f64) {
        let width = f64::from(self.size.width).max(1.0);
        let height = f64::from(self.size.height).max(1.0);
        let saturation = (position_x / width).clamp(0.0, 1.0);
        let value = 1.0 - (position_y / height).clamp(0.0, 1.0);
        let next = ops::ColorPickerOps::set_hue_saturation_value(
            self.state.get_untracked(),
            self.current.hsva.hue,
            saturation,
            value,
        );
        apply_state(&self.state, Rc::clone(&self.on_change), next);
    }

    fn paint_cell(&self, cx: &mut PaintCx, x_index: usize, y_index: usize) {
        let width = f64::from(self.size.width);
        let height = f64::from(self.size.height);
        let cell_width = width / GRID_COLUMNS as f64;
        let cell_height = height / GRID_ROWS as f64;
        let saturation = (x_index as f64 + SAMPLE_CENTER_OFFSET) / GRID_COLUMNS as f64;
        let value = 1.0 - (y_index as f64 + SAMPLE_CENTER_OFFSET) / GRID_ROWS as f64;
        let color = ops::ColorPickerOps::color_grid_color(
            self.current.hsva.hue,
            saturation,
            value,
            u8::MAX,
        );
        let rect = Rect::new(
            x_index as f64 * cell_width,
            y_index as f64 * cell_height,
            ((x_index + 1) as f64 * cell_width + CELL_OVERLAP).min(width),
            ((y_index + 1) as f64 * cell_height + CELL_OVERLAP).min(height),
        );
        cx.fill(&rect, FloemColor::from_token(color), 0.0);
    }

    fn paint_handle(&self, cx: &mut PaintCx) {
        let width = f64::from(self.size.width);
        let height = f64::from(self.size.height);
        let x = self.current.hsva.saturation * width;
        let y = (1.0 - self.current.hsva.value) * height;
        let color = ops::ColorPickerOps::color_grid_color(
            self.current.hsva.hue,
            self.current.hsva.saturation,
            self.current.hsva.value,
            u8::MAX,
        );
        let circle = Circle::new(Point::new(x, y), HANDLE_RADIUS);
        cx.fill(&circle, FloemColor::from_token(color), 0.0);
        cx.stroke(&circle, contrast_color(color), &Stroke::new(BORDER_WIDTH));
    }
}

impl View for EguiColorPlane {
    fn id(&self) -> ViewId {
        self.id
    }

    fn update(&mut self, _cx: &mut UpdateCx, state: Box<dyn std::any::Any>) {
        let Ok(PlaneUpdate::Value(value)) = state.downcast::<PlaneUpdate>().map(|it| *it) else {
            return;
        };
        self.current = value;
        self.id.request_paint();
    }

    fn event_before_children(&mut self, cx: &mut EventCx, event: &Event) -> EventPropagation {
        match event {
            Event::PointerDown(pointer) if pointer.button.is_primary() && !self.locked => {
                cx.update_active(self.id);
                self.held = true;
                self.apply_pointer(pointer.pos.x, pointer.pos.y);
                EventPropagation::Stop
            }
            Event::PointerMove(pointer) if self.held && !self.locked => {
                self.apply_pointer(pointer.pos.x, pointer.pos.y);
                EventPropagation::Stop
            }
            Event::PointerUp(_) => {
                self.held = false;
                EventPropagation::Stop
            }
            _ => EventPropagation::Continue,
        }
    }

    fn compute_layout(&mut self, _cx: &mut ComputeLayoutCx) -> Option<Rect> {
        self.size = self.id.get_layout().unwrap_or_default().size;
        None
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        for y_index in 0..GRID_ROWS {
            for x_index in 0..GRID_COLUMNS {
                self.paint_cell(cx, x_index, y_index);
            }
        }
        let rect = Rect::new(
            0.0,
            0.0,
            f64::from(self.size.width),
            f64::from(self.size.height),
        );
        cx.stroke(&rect, self.border_color, &Stroke::new(BORDER_WIDTH));
        self.paint_handle(cx);
    }
}
