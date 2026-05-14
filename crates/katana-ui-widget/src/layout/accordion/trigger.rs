use std::rc::Rc;

use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::style::CursorStyle;
use floem::views::{Decorators, container, empty, h_stack, label};
use floem::{IntoView, View};

use super::view_helpers::HEADER_SLOT_WIDTH;
use super::{AccordionHeaderView, AccordionTriggerArea, IndicatorPosition};

const CHEVRON_FONT_SIZE: f32 = 13.0;
const HEADER_ROW_GAP: f32 = 6.0;

pub(super) struct TriggerTargetConfig {
    pub(super) header: AccordionHeaderView,
    pub(super) header_font_size: f32,
    pub(super) text_color: floem::peniko::Color,
    pub(super) icon: Option<&'static str>,
    pub(super) indicator: IndicatorPosition,
    pub(super) trigger_area: AccordionTriggerArea,
    pub(super) disabled: bool,
    pub(super) on_toggle: Rc<dyn Fn()>,
}

fn icon_view(symbol: Option<&'static str>, text_color: floem::peniko::Color) -> Box<dyn View> {
    symbol
        .map(|value| {
            label(move || value.to_string())
                .style(move |style| {
                    style
                        .width(HEADER_SLOT_WIDTH)
                        .font_size(CHEVRON_FONT_SIZE)
                        .color(text_color)
                })
                .into_any()
        })
        .unwrap_or_else(|| {
            container(empty())
                .style(move |style| style.width(HEADER_SLOT_WIDTH))
                .into_any()
        })
}

fn header_view(
    header: &AccordionHeaderView,
    font_size: f32,
    text_color: floem::peniko::Color,
) -> Box<dyn View> {
    container(header())
        .style(move |style| {
            style
                .font_size(font_size)
                .color(text_color)
                .flex_grow(1.0)
                .min_width(0.0)
        })
        .into_any()
}

fn is_primary_pointer(event: &Event) -> bool {
    matches!(event, Event::PointerDown(pointer_event) if pointer_event.button.is_primary())
}

fn is_toggle_key(event: &Event) -> bool {
    matches!(
        event,
        Event::KeyDown(key_event)
            if matches!(
                key_event.key.logical_key,
                Key::Named(NamedKey::Enter | NamedKey::Space)
            )
    )
}

fn interactive(content: Box<dyn View>, disabled: bool, on_toggle: Rc<dyn Fn()>) -> Box<dyn View> {
    let target = container(content).style(move |style| {
        let style = style.items_center().width_full();
        if disabled {
            style
        } else {
            style.cursor(CursorStyle::Pointer)
        }
    });

    if disabled {
        return target.into_any();
    }

    target
        .keyboard_navigable()
        .on_event_stop(EventListener::PointerDown, {
            let action = Rc::clone(&on_toggle);
            move |event| {
                if is_primary_pointer(event) {
                    action();
                }
            }
        })
        .on_event_stop(EventListener::KeyDown, move |event| {
            if is_toggle_key(event) {
                on_toggle();
            }
        })
        .into_any()
}

fn row_view(
    header: &AccordionHeaderView,
    font_size: f32,
    text_color: floem::peniko::Color,
    icon: Option<&'static str>,
    indicator: IndicatorPosition,
) -> Box<dyn View> {
    let header = header_view(header, font_size, text_color);
    let icon = icon_view(icon, text_color);
    match indicator {
        IndicatorPosition::Leading => h_stack((icon, header)),
        IndicatorPosition::Trailing | IndicatorPosition::None => h_stack((header, icon)),
    }
    .style(|style| style.items_center().gap(HEADER_ROW_GAP).width_full())
    .into_any()
}

pub(super) fn make_trigger_target(config: TriggerTargetConfig) -> Box<dyn View> {
    let TriggerTargetConfig {
        header,
        header_font_size,
        text_color,
        icon,
        indicator,
        trigger_area,
        disabled,
        on_toggle,
    } = config;
    let row = row_view(&header, header_font_size, text_color, icon, indicator);

    match trigger_area {
        AccordionTriggerArea::FullRow | AccordionTriggerArea::IconAndLabel => {
            interactive(row, disabled, on_toggle)
        }
        AccordionTriggerArea::IconOnly => {
            interactive(icon_view(icon, text_color), disabled, on_toggle)
        }
        AccordionTriggerArea::LabelOnly => interactive(
            header_view(&header, header_font_size, text_color),
            disabled,
            on_toggle,
        ),
    }
}
