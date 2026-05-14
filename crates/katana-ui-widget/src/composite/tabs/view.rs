mod parts;

use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::reactive::create_rw_signal;
use floem::views::{Decorators, v_stack_from_iter};
use std::rc::Rc;

const TAB_CONTENT_GAP: f32 = 10.0;

/// Build Tabs view with overflow, close, external callback and selected content support.
pub(super) fn build_view(tabs: crate::composite::tabs::Tabs, theme: Theme) -> impl IntoView {
    let items = tabs.props.items;
    let initial_selected = items
        .iter()
        .position(|item| item.selected && !item.disabled)
        .unwrap_or(0);
    let selected_index = create_rw_signal(initial_selected);
    let closed_states = Rc::new(
        items
            .iter()
            .map(|_| create_rw_signal(false))
            .collect::<Vec<_>>(),
    );
    let disabled_flags = Rc::new(items.iter().map(|item| item.disabled).collect::<Vec<_>>());
    let content_builders = Rc::new(
        items
            .iter()
            .map(|item| item.content.clone())
            .collect::<Vec<_>>(),
    );
    let has_content = content_builders.iter().any(Option::is_some);

    let tab_items = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            parts::tab_node(
                item,
                index,
                selected_index,
                closed_states[index],
                Rc::clone(&closed_states),
                Rc::clone(&disabled_flags),
                &theme,
            )
        })
        .collect::<Vec<_>>();

    let mut sections: Vec<Box<dyn View>> = Vec::new();
    sections.push(parts::tabs_container(tab_items, tabs.props.overflow, theme));

    if has_content {
        sections.push(parts::selected_content(
            selected_index,
            closed_states,
            content_builders,
        ));
    }

    v_stack_from_iter(sections).style(|style| style.width_full().gap(TAB_CONTENT_GAP))
}
