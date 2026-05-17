use crate::composite::dynamic_array_editor::types::{
    DynamicArrayEditorItem, DynamicArrayItemRenderer,
};
use crate::theme::Theme;
use floem::views::{Decorators, button, container, h_stack, label};
use floem::{IntoView, View};

const ACTION_SIZE: f32 = 10.0;
const ACTION_GAP: f32 = 8.0;
const BORDER_RADIUS: f32 = 6.0;
const BORDER_WIDTH: f32 = 1.0;
const ACTION_PADDING_H: f32 = 6.0;
const ACTION_PADDING_V: f32 = 3.0;
const EMPTY_PADDING: f32 = 16.0;
const EMPTY_FONT: f32 = 13.0;
const H_PAD: f32 = 10.0;
const V_PAD: f32 = 10.0;
const ROW_GAP: f32 = 8.0;
const BUTTON_BORDER_RADIUS: f32 = 4.0;

pub(crate) struct ItemRowCallbacks {
    pub(crate) move_up: Box<dyn Fn() + 'static>,
    pub(crate) move_down: Box<dyn Fn() + 'static>,
    pub(crate) edit: Box<dyn Fn() + 'static>,
    pub(crate) delete: Box<dyn Fn() + 'static>,
}

pub(crate) struct ItemRowConfig<'a, T: Clone + 'static> {
    pub(crate) theme: Theme,
    pub(crate) item: &'a DynamicArrayEditorItem<T>,
    pub(crate) index: usize,
    pub(crate) can_move_up: bool,
    pub(crate) can_move_down: bool,
    pub(crate) can_delete: bool,
    pub(crate) can_edit: bool,
    pub(crate) render: &'a DynamicArrayItemRenderer<T>,
    pub(crate) callbacks: ItemRowCallbacks,
}

pub(crate) struct DynamicArrayEditorLayout;

impl DynamicArrayEditorLayout {
    pub(crate) fn action_button(
        theme: Theme,
        label_text: &'static str,
        disabled: bool,
        on_press: impl Fn() + 'static,
    ) -> impl IntoView {
        let text = if disabled {
            theme.color.text_disabled
        } else {
            theme.color.text
        };
        let token = crate::floem_view::FloemColor::from_token(text);

        button(
            label(move || label_text.to_string())
                .style(move |style| style.font_size(ACTION_SIZE).color(token)),
        )
        .action(move || {
            if !disabled {
                on_press();
            }
        })
        .style(move |style| {
            style
                .padding_horiz(ACTION_PADDING_H)
                .padding_vert(ACTION_PADDING_V)
                .border(BORDER_WIDTH)
                .border_color(crate::floem_view::FloemColor::from_token(
                    theme.color.border,
                ))
                .border_radius(BUTTON_BORDER_RADIUS)
        })
    }

    pub(crate) fn empty_view(message: String, theme: Theme) -> impl IntoView {
        container(label(move || message.clone()).style(move |style| {
            style
                .font_size(EMPTY_FONT)
                .color(crate::floem_view::FloemColor::from_token(
                    theme.color.text_muted,
                ))
        }))
        .style(move |style| {
            style
                .padding(EMPTY_PADDING)
                .width_full()
                .border(BORDER_WIDTH)
                .border_radius(BORDER_RADIUS)
                .border_color(crate::floem_view::FloemColor::from_token(
                    theme.color.border,
                ))
                .background(crate::floem_view::FloemColor::from_token(
                    theme.color.surface,
                ))
        })
    }

    pub(crate) fn item_row<T: Clone + 'static>(config: ItemRowConfig<'_, T>) -> Box<dyn View> {
        let ItemRowConfig {
            theme,
            item,
            index,
            can_move_up,
            can_move_down,
            can_delete,
            can_edit,
            render,
            callbacks:
                ItemRowCallbacks {
                    move_up,
                    move_down,
                    edit,
                    delete,
                },
        } = config;

        let content = (render)(item, index);
        let actions = h_stack((
            Self::action_button(theme.clone(), "↑", !can_move_up, move_up),
            Self::action_button(theme.clone(), "↓", !can_move_down, move_down),
            Self::action_button(theme.clone(), "編集", !can_edit, edit),
            Self::action_button(theme.clone(), "削除", !can_delete, delete),
        ))
        .style(move |style| style.gap(ACTION_GAP).items_center());

        h_stack((content, actions))
            .style(move |style| {
                style
                    .items_center()
                    .justify_between()
                    .gap(ROW_GAP)
                    .padding_horiz(H_PAD)
                    .padding_vert(V_PAD)
                    .border(BORDER_WIDTH)
                    .border_color(crate::floem_view::FloemColor::from_token(
                        theme.color.border,
                    ))
                    .border_radius(BORDER_RADIUS)
                    .background(crate::floem_view::FloemColor::from_token(theme.color.bg))
            })
            .into_any()
    }
}
