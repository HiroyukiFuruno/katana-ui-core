use crate::composite::selector::color_picker::types::ColorPickerValue;
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::context::{ComputeLayoutCx, PaintCx, UpdateCx};
use floem::kurbo::Rect;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, create_effect};
use floem::views::{Decorators, h_stack, label, v_stack};
use floem::{IntoView, View, ViewId};
use floem_renderer::Renderer;

const PREVIEW_BASE_WIDTH: f32 = 520.0;
const PREVIEW_HEIGHT: f32 = 24.0;
const TEXT_PREVIEW_GAP: f32 = 6.0;
const LABEL_U8_SIZE: f32 = 11.0;
const LABEL_VALUE_SIZE: f32 = 12.0;
const PREVIEW_PADDING: f32 = 6.0;
const PREVIEW_BORDER_WIDTH: f32 = 1.0;
const CHECKER_DARK: PenikoColor = PenikoColor::rgb8(32, 32, 32);
const CHECKER_BRIGHT: PenikoColor = PenikoColor::rgb8(128, 128, 128);

enum PreviewUpdate {
    Value(ColorPickerValue),
}

struct SelectedColorPreview {
    id: ViewId,
    current: ColorPickerValue,
    size: floem::taffy::prelude::Size<f32>,
}

pub(super) fn color_preview(
    state: RwSignal<ColorPickerValue>,
    theme: Theme,
    panel_scale: f32,
) -> impl IntoView {
    let text_color = FloemColor::from_token(theme.color.text);
    let muted_color = FloemColor::from_token(theme.color.text_muted);
    let border_color = FloemColor::from_token(theme.color.border);

    v_stack((
        SelectedColorPreview::new(state).style(move |style| {
            style
                .width(PREVIEW_BASE_WIDTH * panel_scale)
                .height(PREVIEW_HEIGHT * panel_scale)
        }),
        h_stack((
            label(|| "U8").style(move |style| style.font_size(LABEL_U8_SIZE).color(muted_color)),
            label(move || {
                let color = state.get().color;
                format!("R:{} G:{} B:{} A:{}", color.r, color.g, color.b, color.a)
            })
            .style(move |style| style.font_size(LABEL_VALUE_SIZE).color(text_color)),
        ))
        .style(|style| style.gap(TEXT_PREVIEW_GAP).items_center()),
    ))
    .style(move |style| {
        style
            .gap(TEXT_PREVIEW_GAP)
            .padding(PREVIEW_PADDING)
            .border(PREVIEW_BORDER_WIDTH)
            .border_color(border_color)
    })
}

impl SelectedColorPreview {
    fn new(state: RwSignal<ColorPickerValue>) -> Self {
        let id = ViewId::new();
        create_effect(move |_| id.update_state(PreviewUpdate::Value(state.get())));
        Self {
            id,
            current: state.get_untracked(),
            size: floem::taffy::prelude::Size::default(),
        }
    }

    fn paint_checkers(&self, cx: &mut PaintCx) {
        let width = f64::from(self.size.width);
        let height = f64::from(self.size.height);
        let checker = height / 2.0;
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
}

impl View for SelectedColorPreview {
    fn id(&self) -> ViewId {
        self.id
    }

    fn update(&mut self, _cx: &mut UpdateCx, state: Box<dyn std::any::Any>) {
        let Ok(PreviewUpdate::Value(value)) = state.downcast::<PreviewUpdate>().map(|it| *it)
        else {
            return;
        };
        self.current = value;
        self.id.request_paint();
    }

    fn compute_layout(&mut self, _cx: &mut ComputeLayoutCx) -> Option<Rect> {
        self.size = self.id.get_layout().unwrap_or_default().size;
        None
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        let color = self.current.color;
        let width = f64::from(self.size.width);
        let height = f64::from(self.size.height);
        let full = Rect::new(0.0, 0.0, width, height);
        if color.a == u8::MAX {
            cx.fill(&full, FloemColor::from_token(color), 0.0);
            return;
        }

        self.paint_checkers(cx);
        if color.a > 0 {
            let left = Rect::new(0.0, 0.0, width / 2.0, height);
            let right = Rect::new(width / 2.0, 0.0, width, height);
            cx.fill(&left, FloemColor::from_token(color), 0.0);
            cx.fill(&right, FloemColor::from_token(opaque(color)), 0.0);
        }
    }
}

fn opaque(color: Color) -> Color {
    Color {
        a: u8::MAX,
        ..color
    }
}
