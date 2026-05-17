use super::{ResolvedComboBox, ops, rows::build_rows};
use crate::composite::input::text::TextInput;
use crate::layout::popover::{AnchorRect, ViewAnchor};
use crate::overlay_lifecycle::{OverlayLifecycle, OverlayLifetime};
use crate::theme::Theme;
use floem::event::{Event, EventListener};
use floem::peniko::kurbo::Point;
use floem::reactive::{SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{Decorators, container, label, v_stack};
use floem::{IntoView, View, ViewId};
use std::cell::Cell;
use std::rc::Rc;

const COMBO_BOX_EMPTY_PADDING_VERT: f32 = 8.0;
const COMBO_BOX_EMPTY_PADDING_HORIZ: f32 = 10.0;
const COMBO_BOX_FALLBACK_SIZE: f32 = 1.0;

impl<K: Clone + PartialEq + 'static> super::ComboBox<K> {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved: ResolvedComboBox<K> = self.resolve();
        let options = self.props.options.clone();
        let strict = resolved.strict;
        let disabled = resolved.disabled;
        let a11y_label = resolved.a11y_label;
        let placement = resolved.placement;
        let on_select = Rc::clone(&resolved.on_select);
        let on_input_change = Rc::clone(&resolved.on_input_change);

        let open = create_rw_signal(resolved.is_open);
        let selected = create_rw_signal(self.props.value);
        let input_value = create_rw_signal(resolved.input_value.clone());
        let anchor = create_rw_signal(default_anchor());
        let overlay_id = create_rw_signal::<Option<ViewId>>(None);
        let overlay_pending = Rc::new(Cell::new(false));
        let overlay_lifetime = OverlayLifetime::new();

        let close_overlay: Rc<dyn Fn()> = {
            let options = options.clone();
            let overlay_lifetime = overlay_lifetime.clone();
            Rc::new(move || {
                let was_open = open.try_update(|state| {
                    if *state {
                        *state = false;
                        true
                    } else {
                        false
                    }
                });

                if was_open.unwrap_or(false) && strict {
                    let query = input_value.try_get().unwrap_or_default();
                    let exact = options.iter().find(|option| option.label == query);
                    if let Some(option) = exact {
                        selected.set(Some(option.value.clone()));
                        input_value.set(option.label.clone());
                    } else if let Some(current) = selected.try_get().unwrap_or(None) {
                        if let Some(previous) =
                            options.iter().find(|option| option.value == current)
                        {
                            input_value.set(previous.label.clone());
                        } else {
                            selected.set(None);
                            input_value.set(String::new());
                        }
                    } else {
                        selected.set(None);
                        input_value.set(String::new());
                    }
                }

                if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                    OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
                }
            })
        };

        create_effect({
            let options = options.clone();
            let close_overlay = Rc::clone(&close_overlay);
            let theme = theme.clone();
            let overlay_pending = Rc::clone(&overlay_pending);
            let overlay_lifetime = overlay_lifetime.clone();
            move |_| {
                if !open.try_get().unwrap_or(false) {
                    overlay_pending.set(false);
                    if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                        OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
                    }
                    return;
                }

                if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                    OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
                }
                if overlay_pending.get() {
                    return;
                }
                overlay_pending.set(true);

                let query = input_value.try_get().unwrap_or_default();
                let overlay_theme = theme.clone();
                let overlay_close = Rc::clone(&close_overlay);
                let selected_value = selected.try_get().unwrap_or(None);
                let items = ops::filtered_options(&query, &options);
                let rows = if items.is_empty() {
                    v_stack((label(|| "候補がありません".to_string()).style(|style| {
                        style
                            .padding_vert(COMBO_BOX_EMPTY_PADDING_VERT)
                            .padding_horiz(COMBO_BOX_EMPTY_PADDING_HORIZ)
                    }),))
                    .into_any()
                } else {
                    build_rows(items, selected_value, disabled, &theme, {
                        let on_select = Rc::clone(&on_select);
                        let close_overlay = Rc::clone(&close_overlay);
                        Rc::new(move |value, label| {
                            selected.set(Some(value.clone()));
                            input_value.set(label);
                            on_select(value);
                            close_overlay();
                        })
                    })
                };

                let current_anchor = anchor.try_get().unwrap_or(default_anchor());
                let overlay_lifetime_for_added = overlay_lifetime.clone();
                OverlayLifecycle::add_overlay_next_tick(
                    &overlay_lifetime,
                    Point::new(0.0, 0.0),
                    move |_| {
                        super::overlay::build_overlay(
                            rows,
                            current_anchor,
                            placement,
                            overlay_theme.clone(),
                            Rc::clone(&overlay_close),
                        )
                    },
                    {
                        let overlay_pending = Rc::clone(&overlay_pending);
                        move |next_overlay_id| {
                            overlay_pending.set(false);
                            if open.try_get().unwrap_or(false) {
                                overlay_id.set(Some(next_overlay_id));
                            } else {
                                OverlayLifecycle::remove_overlay_next_tick(
                                    &overlay_lifetime_for_added,
                                    next_overlay_id,
                                );
                            }
                        }
                    },
                );
            }
        });

        let text_input = TextInput::new(a11y_label)
            .value(resolved.input_value)
            .placeholder(resolved.placeholder)
            .on_change({
                let on_input_change = on_input_change;
                move |next| {
                    input_value.set(next.clone());
                    selected.set(None);
                    open.set(true);
                    on_input_change(next);
                }
            })
            .view(theme.clone());

        let input_container = container(text_input);
        let input_id = input_container.id();
        input_container
            .on_event_stop(EventListener::PointerDown, {
                move |event| {
                    if disabled {
                        return;
                    }
                    let Event::PointerDown(_) = event else {
                        return;
                    };
                    let next = ops::toggle_open(open.try_get().unwrap_or(false), disabled);
                    open.set(next);
                    if next {
                        anchor.set(ViewAnchor::rect_for_view(input_id, default_anchor()));
                    }
                }
            })
            .on_cleanup(move || {
                overlay_lifetime.dispose();
                if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                    OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
                }
            })
    }
}

fn default_anchor() -> AnchorRect {
    AnchorRect::new(0.0, 0.0, COMBO_BOX_FALLBACK_SIZE, COMBO_BOX_FALLBACK_SIZE)
}
