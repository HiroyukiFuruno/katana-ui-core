use crate::composite::selector::color_picker::ops;
use crate::composite::selector::color_picker::types::ColorPickerValue;
use crate::composite::selector::color_picker::view::apply_state;
use crate::composite::selector::color_picker::view::paint_util::contrast_color;
use crate::floem_view::FloemColor;
use crate::theme::color::Color;
use floem::context::{ComputeLayoutCx, EventCx, PaintCx, UpdateCx};
use floem::event::{Event, EventPropagation};
use floem::kurbo::{BezPath, Point, Rect, Stroke};
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, create_effect};
use floem::{View, ViewId};
use floem_renderer::Renderer;
use std::rc::Rc;

const GRADIENT_MIN_STEPS: usize = 256;
const GRADIENT_OVERLAP: f64 = 1.2;
const BORDER_WIDTH: f64 = 1.0;
const HANDLE_RADIUS_RATE: f64 = 4.0;
const CHANNEL_MAX_FLOAT: f64 = 255.0;
const SAMPLE_CENTER_OFFSET: f64 = 0.5;
const CHECKER_DIVISOR: f64 = 2.0;
const CHECKER_DARK: PenikoColor = PenikoColor::rgb8(174, 174, 174);
const CHECKER_BRIGHT: PenikoColor = PenikoColor::rgb8(202, 202, 202);

#[derive(Clone, Copy)]
pub(crate) enum EguiColorSliderKind {
    Hue,
    Alpha,
}

enum SliderUpdate {
    Value(ColorPickerValue),
}

pub(crate) struct EguiColorSlider {
    id: ViewId,
    state: RwSignal<ColorPickerValue>,
    on_change: Rc<dyn Fn(Color)>,
    kind: EguiColorSliderKind,
    current: ColorPickerValue,
    locked: bool,
    held: bool,
    border_color: PenikoColor,
    size: floem::taffy::prelude::Size<f32>,
}

impl EguiColorSlider {
    pub(crate) fn new(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        kind: EguiColorSliderKind,
        locked: bool,
        border_color: Color,
    ) -> Self {
        let id = ViewId::new();
        create_effect(move |_| id.update_state(SliderUpdate::Value(state.get())));
        Self {
            id,
            state,
            on_change,
            kind,
            current: state.get_untracked(),
            locked,
            held: false,
            border_color: FloemColor::from_token(border_color),
            size: floem::taffy::prelude::Size::default(),
        }
    }

    fn current_value(&self) -> f64 {
        match self.kind {
            EguiColorSliderKind::Hue => self.current.hsva.hue,
            EguiColorSliderKind::Alpha => f64::from(self.current.color.a) / CHANNEL_MAX_FLOAT,
        }
    }

    fn color_at(&self, value: f64) -> Color {
        match self.kind {
            EguiColorSliderKind::Hue => {
                ops::ColorPickerOps::color_grid_color(value, 1.0, 1.0, u8::MAX)
            }
            EguiColorSliderKind::Alpha => Color {
                a: (value.clamp(0.0, 1.0) * CHANNEL_MAX_FLOAT).round() as u8,
                ..self.current.color
            },
        }
    }

    fn apply_pointer(&self, position_x: f64) {
        let width = f64::from(self.size.width).max(1.0);
        let value = (position_x / width).clamp(0.0, 1.0);
        let next = match self.kind {
            EguiColorSliderKind::Hue => {
                ops::ColorPickerOps::set_hue(self.state.get_untracked(), value)
            }
            EguiColorSliderKind::Alpha => ops::ColorPickerOps::set_alpha(
                self.state.get_untracked(),
                (value * CHANNEL_MAX_FLOAT).round() as u8,
            ),
        };
        apply_state(&self.state, Rc::clone(&self.on_change), next);
    }

    fn paint_checkers(&self, cx: &mut PaintCx) {
        let height = f64::from(self.size.height);
        let width = f64::from(self.size.width);
        let checker = height / CHECKER_DIVISOR;
        cx.fill(&Rect::new(0.0, 0.0, width, height), CHECKER_DARK, 0.0);
        let count = (width / checker).ceil() as usize;
        for index in 0..count {
            let y = if index % 2 == 0 { 0.0 } else { checker };
            let rect = Rect::new(
                index as f64 * checker,
                y,
                ((index + 1) as f64 * checker).min(width),
                (y + checker).min(height),
            );
            cx.fill(&rect, CHECKER_BRIGHT, 0.0);
        }
    }

    fn paint_gradient(&self, cx: &mut PaintCx) {
        let width = f64::from(self.size.width);
        let height = f64::from(self.size.height);
        let steps = (width.ceil() as usize).max(GRADIENT_MIN_STEPS);
        let step = width / steps as f64;
        for index in 0..steps {
            let value = (index as f64 + SAMPLE_CENTER_OFFSET) / steps as f64;
            let rect = Rect::new(
                index as f64 * step,
                0.0,
                ((index + 1) as f64 * step + GRADIENT_OVERLAP).min(width),
                height,
            );
            cx.fill(&rect, FloemColor::from_token(self.color_at(value)), 0.0);
        }
    }

    fn paint_handle(&self, cx: &mut PaintCx) {
        let height = f64::from(self.size.height);
        let center = self.current_value() * f64::from(self.size.width);
        let radius = height / HANDLE_RADIUS_RATE;
        let mut marker = BezPath::new();
        marker.move_to(Point::new(center, height / CHECKER_DIVISOR));
        marker.line_to(Point::new(center + radius, height));
        marker.line_to(Point::new(center - radius, height));
        marker.close_path();
        let color = self.color_at(self.current_value());
        cx.fill(&marker, FloemColor::from_token(color), 0.0);
        cx.stroke(&marker, contrast_color(color), &Stroke::new(BORDER_WIDTH));
    }
}

impl View for EguiColorSlider {
    fn id(&self) -> ViewId {
        self.id
    }

    fn update(&mut self, _cx: &mut UpdateCx, state: Box<dyn std::any::Any>) {
        let Ok(SliderUpdate::Value(value)) = state.downcast::<SliderUpdate>().map(|it| *it) else {
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
                self.apply_pointer(pointer.pos.x);
                EventPropagation::Stop
            }
            Event::PointerMove(pointer) if self.held && !self.locked => {
                self.apply_pointer(pointer.pos.x);
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
        if matches!(self.kind, EguiColorSliderKind::Alpha) {
            self.paint_checkers(cx);
        }
        self.paint_gradient(cx);
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
