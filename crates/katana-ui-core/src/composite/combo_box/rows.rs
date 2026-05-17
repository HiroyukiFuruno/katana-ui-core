use super::ComboBoxOption;
use crate::theme::Theme;
use floem::views::Decorators;
use floem::views::{button, label, v_stack_from_iter};
use floem::{IntoView, View};
use std::rc::Rc;

const ROW_PADDING_VERTICAL: f32 = 8.0;
const ROW_PADDING_HORIZ: f32 = 10.0;
const ROW_BORDER_WIDTH: f32 = 1.0;
const ROW_BORDER_RADIUS: f32 = 8.0;
const ROW_GAP: f32 = 4.0;
const ROW_MAX_HEIGHT: f32 = 260.0;

pub(super) type PickAction<K> = Rc<dyn Fn(K, String)>;

pub(super) fn build_rows<K: Clone + PartialEq + 'static>(
    items: Vec<ComboBoxOption<K>>,
    current: Option<K>,
    disabled: bool,
    theme: &Theme,
    on_pick: PickAction<K>,
) -> Box<dyn View> {
    let row_views = v_stack_from_iter(items.into_iter().map({
        let theme = theme.clone();
        move |option| {
            let is_selected = current.as_ref() == Some(&option.value);
            let value = option.value.clone();
            let label_text = option.label.clone();
            let row_label = label_text.clone();
            let pick = Rc::clone(&on_pick);
            button(label(move || row_label.clone()).style({
                let fg = if is_selected {
                    theme.color.accent
                } else {
                    theme.color.text
                };
                move |style| style.color(crate::floem_view::FloemColor::from_token(fg))
            }))
            .on_event_stop(floem::event::EventListener::PointerDown, |_| {})
            .action(move || {
                if disabled {
                    return;
                }
                pick(value.clone(), label_text.clone());
            })
            .style(move |style| {
                let bg = if is_selected {
                    theme.color.accent_muted
                } else {
                    theme.color.bg
                };
                style
                    .background(crate::floem_view::FloemColor::from_token(bg))
                    .padding_vert(ROW_PADDING_VERTICAL)
                    .padding_horiz(ROW_PADDING_HORIZ)
                    .border(ROW_BORDER_WIDTH)
                    .border_color(crate::floem_view::FloemColor::from_token(
                        theme.color.border,
                    ))
                    .border_radius(ROW_BORDER_RADIUS)
            })
            .into_any()
        }
    }));

    row_views
        .style(|style| style.gap(ROW_GAP).max_height(ROW_MAX_HEIGHT))
        .into_any()
}
