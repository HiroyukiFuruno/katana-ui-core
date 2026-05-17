use super::ops;
use super::render::{execute_selected, rows_view};
use super::types::{CommandPalette, CommandPaletteItem, CommandPaletteProps};
use crate::composite::input::text::TextInput;
use crate::theme::Theme;
use floem::IntoView;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::reactive::{SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{Decorators, v_stack};
use std::rc::Rc;

const GAP: f32 = 6.0;

impl<P: Clone + 'static> CommandPalette<P> {
    pub(crate) fn build_view(self, theme: Theme) -> impl IntoView {
        let props: CommandPaletteProps<P> = self.props;
        let provider = props.provider;
        let on_execute = props.on_execute;
        let on_selection_change = props.on_selection_change;
        let on_query = props.on_query;
        let on_close = props.on_close;
        let placeholder = props.placeholder;
        let disabled = props.disabled;

        let query_signal = create_rw_signal(String::new());
        let results_signal = create_rw_signal(Vec::<CommandPaletteItem<P>>::new());
        let selected_signal = create_rw_signal(0_usize);
        let version_signal = create_rw_signal(0_u32);

        create_effect({
            let on_query = Rc::clone(&on_query);
            let on_selection_change = Rc::clone(&on_selection_change);
            move |_| {
                let query = query_signal.get();
                let mut rows = provider.query(&query);
                ops::sort_by_score(&mut rows);
                selected_signal.set(0);
                results_signal.set(rows);
                on_query(query.clone());
                on_selection_change(query, 0);
                version_signal.update(|value| *value += 1);
            }
        });

        let execute_at: Rc<dyn Fn(usize)> = {
            let on_selection_change = Rc::clone(&on_selection_change);
            let on_execute = Rc::clone(&on_execute);
            Rc::new(move |index: usize| {
                execute_selected(
                    index,
                    &query_signal,
                    &results_signal,
                    &selected_signal,
                    &on_selection_change,
                    &on_execute,
                );
            })
        };

        let list = floem::views::dyn_container(
            move || format!("{}-{}", version_signal.get(), selected_signal.get()),
            {
                let theme = theme.clone();
                let on_execute = Rc::clone(&execute_at);
                move |_| {
                    let items = results_signal.get();
                    let selected_index = selected_signal.get();
                    rows_view(
                        theme.clone(),
                        items,
                        selected_index,
                        disabled,
                        Rc::clone(&on_execute),
                    )
                }
            },
        );

        let input = TextInput::new("コマンド検索")
            .placeholder(placeholder)
            .disabled(disabled)
            .on_change({
                move |query| {
                    query_signal.set(query);
                }
            })
            .view(theme.clone())
            .keyboard_navigable()
            .on_event_stop(EventListener::KeyDown, {
                let on_close = Rc::clone(&on_close);
                let on_selection_change = Rc::clone(&on_selection_change);
                let execute_at = Rc::clone(&execute_at);
                move |event| {
                    if disabled {
                        return;
                    }
                    let Event::KeyDown(key_event) = event else {
                        return;
                    };
                    let query = query_signal.get();
                    let len = results_signal.get().len();
                    let selected = selected_signal.get();
                    match key_event.key.logical_key {
                        Key::Named(NamedKey::ArrowDown) => {
                            let next = ops::move_next(len, selected);
                            selected_signal.set(next);
                            on_selection_change(query, next);
                        }
                        Key::Named(NamedKey::ArrowUp) => {
                            let previous = ops::move_previous(len, selected);
                            selected_signal.set(previous);
                            on_selection_change(query, previous);
                        }
                        Key::Named(NamedKey::Enter) if len > 0 => {
                            execute_at(selected);
                        }
                        Key::Named(NamedKey::Escape) => {
                            on_close();
                        }
                        _ => {}
                    }
                }
            });

        v_stack((input, list)).style(|style| style.gap(GAP).width_full())
    }
}
