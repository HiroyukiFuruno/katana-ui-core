use super::helpers::{ActivePop, empty_slot};
use super::types::{DEFAULT_EXPANDED_PANEL_WIDTH, SideMenuItem, SideMenuPopMode};
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use floem::reactive::{RwSignal, SignalGet};
use floem::views::{Decorators, container, dyn_container};
use floem::{IntoView, View};
use std::rc::Rc;

const EXPAND_PANEL_PADDING: f32 = 6.0;
const EXPAND_BORDER_WIDTH: f32 = 1.0;

pub(super) fn expand_panel(
    items: Rc<Vec<SideMenuItem>>,
    active: RwSignal<Option<ActivePop>>,
    theme: Theme,
    width: impl Fn() -> f32 + 'static,
) -> Box<dyn View> {
    let expand_surface = FloemColor::from_token(theme.color.surface);
    let expand_border = FloemColor::from_token(theme.color.border);

    container(
        dyn_container(
            move || active.get(),
            move |state| -> Box<dyn View> {
                let pop = match state {
                    Some(ActivePop {
                        index,
                        mode: SideMenuPopMode::Expand,
                        ..
                    }) => items
                        .get(index)
                        .and_then(|it| it.pop.as_ref().map(|entry| Rc::clone(&entry.content))),
                    _ => None,
                };
                let Some(pop) = pop else {
                    return empty_slot();
                };
                container((pop)())
                    .style(move |style| {
                        style
                            .width(DEFAULT_EXPANDED_PANEL_WIDTH)
                            .padding(EXPAND_PANEL_PADDING)
                            .background(expand_surface)
                            .border(EXPAND_BORDER_WIDTH)
                            .border_color(expand_border)
                    })
                    .into_any()
            },
        )
        .style(move |style| style.width(width()).min_width(width()).height_full()),
    )
    .into_any()
}
