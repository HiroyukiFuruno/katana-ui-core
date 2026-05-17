use crate::composite::dynamic_array_editor::DynamicArrayEditor;
use crate::composite::dynamic_array_editor::layout::DynamicArrayEditorLayout;
use crate::composite::dynamic_array_editor::types::DynamicArrayEditorItem;
use crate::theme::Theme;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, container, dyn_container, empty, label, v_stack, v_stack_from_iter,
};
use std::rc::Rc;

const LIST_GAP: f32 = 8.0;
const HINT_TEXT_SIZE: f32 = 11.0;

type ItemsSignal<T> = floem::reactive::RwSignal<Vec<DynamicArrayEditorItem<T>>>;
type VersionSignal = floem::reactive::RwSignal<u32>;

fn with_updated_items<T: Clone + 'static>(
    items: &ItemsSignal<T>,
    version: &VersionSignal,
    mut mutation: impl FnMut(&mut Vec<DynamicArrayEditorItem<T>>) -> bool,
) {
    let changed = items
        .try_update(|values| mutation(values).then_some(()))
        .is_some();
    if changed {
        version.update(|value| *value += 1)
    }
}

fn can_move_up<T>(index: usize, items: &[DynamicArrayEditorItem<T>], disabled: bool) -> bool {
    !disabled && items[index].reorderable && index > 0 && items[index - 1].reorderable
}

fn can_move_down<T>(index: usize, items: &[DynamicArrayEditorItem<T>], disabled: bool) -> bool {
    let next = index + 1;
    !disabled && next < items.len() && items[index].reorderable && items[next].reorderable
}

fn add_hint(
    theme: Theme,
    disabled: bool,
    has_space: bool,
    max_items: Option<usize>,
) -> impl IntoView {
    if has_space {
        container(empty())
    } else if disabled {
        container(
            label(|| "編集不能: 追加・並び替え・削除は無効です".to_string()).style(move |style| {
                style
                    .color(crate::floem_view::FloemColor::from_token(
                        theme.color.text_muted,
                    ))
                    .font_size(HINT_TEXT_SIZE)
            }),
        )
    } else if let Some(limit) = max_items {
        container(
            label(move || format!("上限 {limit} 件に達したため追加できません")).style(
                move |style| {
                    style
                        .color(crate::floem_view::FloemColor::from_token(
                            theme.color.text_muted,
                        ))
                        .font_size(HINT_TEXT_SIZE)
                },
            ),
        )
    } else {
        container(empty())
    }
}

impl<T: Clone + 'static> DynamicArrayEditor<T> {
    pub(crate) fn build_view(self, theme: Theme) -> impl IntoView {
        let items_signal = create_rw_signal(self.props.items.clone());
        let version = create_rw_signal(0_u32);
        let max_items = self.props.max_items;
        let disabled = self.props.disabled;
        let empty_state = self.props.empty_state;
        let renderer = Rc::clone(&self.props.item_renderer);
        let create_item = Rc::clone(&self.props.create_item);
        let on_change = Rc::clone(&self.props.on_change);
        let on_add = Rc::clone(&self.props.on_add);
        let on_edit = Rc::clone(&self.props.on_edit);
        let on_delete = Rc::clone(&self.props.on_delete);
        let on_move = Rc::clone(&self.props.on_move);

        dyn_container(
            move || version.get(),
            move |_| {
                let current_items = items_signal.get();
                let items_len = current_items.len();
                let has_space = !disabled && max_items.is_none_or(|limit| items_len < limit);

                let rows = if current_items.is_empty() {
                    DynamicArrayEditorLayout::empty_view(empty_state.clone(), theme.clone())
                        .into_any()
                } else {
                    let row_views = current_items
                    .iter()
                    .enumerate()
                    .map(|(index, item)| {
                            let can_move_up_current = can_move_up(index, &current_items, disabled);
                            let can_move_down_current =
                                can_move_down(index, &current_items, disabled);
                            let can_delete = !disabled && item.deletable;
                            let can_edit = !disabled;
                            let move_up = {
                                let on_change = Rc::clone(&on_change);
                                let on_move = Rc::clone(&on_move);
                                move || {
                                    with_updated_items::<T>(&items_signal, &version, |values| {
                                        if !can_move_up(index, values, disabled) {
                                            return false;
                                        }
                                        values.swap(index, index - 1);
                                        on_move(index, index - 1);
                                        on_change(values.clone());
                                        true
                                    });
                                }
                            };
                            let move_down = {
                                let on_change = Rc::clone(&on_change);
                                let on_move = Rc::clone(&on_move);
                                move || {
                                    with_updated_items::<T>(&items_signal, &version, |values| {
                                        if !can_move_down(index, values, disabled) {
                                            return false;
                                        }
                                        values.swap(index, index + 1);
                                        on_move(index, index + 1);
                                        on_change(values.clone());
                                        true
                                    });
                                }
                            };
                            let edit = {
                                let on_edit = Rc::clone(&on_edit);
                                move || on_edit(index)
                            };
                            let remove = {
                                let on_change = Rc::clone(&on_change);
                                let on_delete = Rc::clone(&on_delete);
                                move || {
                                    with_updated_items::<T>(&items_signal, &version, |values| {
                                        if index >= values.len() || !values[index].deletable {
                                            return false;
                                        }
                                        values.remove(index);
                                        on_delete(index);
                                        on_change(values.clone());
                                        true
                                    });
                                }
                            };

                            DynamicArrayEditorLayout::item_row(crate::composite::dynamic_array_editor::layout::ItemRowConfig {
                                theme: theme.clone(),
                                item,
                                index,
                                can_move_up: can_move_up_current,
                                can_move_down: can_move_down_current,
                                can_delete,
                                can_edit,
                                render: renderer.as_ref(),
                                callbacks: crate::composite::dynamic_array_editor::layout::ItemRowCallbacks {
                                    move_up: Box::new(move_up),
                                    move_down: Box::new(move_down),
                                    edit: Box::new(edit),
                                    delete: Box::new(remove),
                                },
                            })
                        })
                        .collect::<Vec<_>>();

                    v_stack_from_iter(row_views)
                        .style(|style| style.gap(LIST_GAP))
                        .into_any()
                };

                let add_item = {
                    let on_add = Rc::clone(&on_add);
                    let on_change = Rc::clone(&on_change);
                    let create_item = Rc::clone(&create_item);
                    move || {
                        with_updated_items::<T>(&items_signal, &version, |values| {
                            if disabled || max_items.is_some_and(|limit| values.len() >= limit) {
                                return false;
                            }
                            let next_index = values.len();
                            values.push(create_item());
                            on_add(next_index);
                            on_change(values.clone());
                            true
                        });
                    }
                };

                let add_button = DynamicArrayEditorLayout::action_button(
                    theme.clone(),
                    "＋追加",
                    !has_space,
                    add_item,
                );
                let add_hint = add_hint(theme.clone(), disabled, has_space, max_items);

                v_stack((rows, add_button, add_hint))
                    .style(|style| style.gap(LIST_GAP).width_full().items_start())
            },
        )
    }
}
