use super::ops::{SelectionListItemPath, SelectionListOps};
use super::row::item_row;
use super::{SelectionList, SelectionListSection, SelectionListShowMore};
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::style::Display;
use floem::views::{Decorators, button, label, v_stack, v_stack_from_iter};
use std::rc::Rc;

const EXTRA_LINK_SIZE: f32 = 12.0;
const ROW_GAP: f32 = 6.0;
const SECTION_GAP: f32 = 14.0;
const SECTION_TITLE_SIZE: f32 = 13.0;
const TITLE_PADDING: f32 = 2.0;
const SHOW_MORE_PADDING_LEFT: f32 = 10.0;
const SHOW_MORE_PADDING_TOP: f32 = 4.0;

fn section_title(title: String) -> impl IntoView {
    label(move || title.clone()).style(move |style| {
        style
            .font_size(SECTION_TITLE_SIZE)
            .padding_left(TITLE_PADDING)
            .padding_bottom(TITLE_PADDING)
            .padding_top(TITLE_PADDING)
    })
}

fn section_view(
    section: SelectionListSection,
    section_index: usize,
    theme: Theme,
    selected_path: RwSignal<Option<SelectionListItemPath>>,
    hidden_revealed: RwSignal<bool>,
    initial_selected_path: Option<SelectionListItemPath>,
) -> Box<dyn View> {
    let section_title = section_title(section.label.clone());
    let rows = v_stack_from_iter(section.items.into_iter().enumerate().map(
        move |(item_index, item)| {
            item_row(
                item,
                theme.clone(),
                SelectionListItemPath::new(section_index, item_index),
                selected_path,
                hidden_revealed,
                initial_selected_path,
            )
        },
    ))
    .style(move |style| style.gap(ROW_GAP));

    v_stack((section_title, rows))
        .style(move |style| style.gap(ROW_GAP))
        .into_any()
}

fn show_more_view(
    show_more: SelectionListShowMore,
    theme: Theme,
    hidden_revealed: RwSignal<bool>,
    has_hidden_items: bool,
) -> Box<dyn View> {
    let on_select = Rc::clone(&show_more.on_select);
    let color = theme.color.accent;
    button(label(move || show_more.label.clone()).style(move |style| {
        style
            .font_size(EXTRA_LINK_SIZE)
            .color(crate::floem_view::FloemColor::from_token(color))
    }))
    .action(move || {
        if has_hidden_items {
            hidden_revealed.set(true);
        }
        on_select();
    })
    .style(move |style| {
        let display = if SelectionListOps::show_more_visible(
            has_hidden_items,
            hidden_revealed.try_get().unwrap_or(false),
        ) {
            Display::Flex
        } else {
            Display::None
        };

        style
            .display(display)
            .padding_left(SHOW_MORE_PADDING_LEFT)
            .padding_top(SHOW_MORE_PADDING_TOP)
    })
    .into_any()
}

fn section_views(
    sections: Vec<SelectionListSection>,
    theme: &Theme,
    selected_path: RwSignal<Option<SelectionListItemPath>>,
    hidden_revealed: RwSignal<bool>,
    initial_selected_path: Option<SelectionListItemPath>,
) -> Vec<Box<dyn View>> {
    sections
        .into_iter()
        .enumerate()
        .map(|(section_index, section)| {
            section_view(
                section,
                section_index,
                theme.clone(),
                selected_path,
                hidden_revealed,
                initial_selected_path,
            )
        })
        .collect()
}

impl SelectionList {
    pub(crate) fn build_view(self, theme: Theme) -> impl IntoView {
        let initial_selected_path = SelectionListOps::initial_selected_path(&self.props.sections);
        let has_hidden_items = SelectionListOps::has_hidden_items(&self.props.sections);
        let selected_path = create_rw_signal(initial_selected_path);
        let hidden_revealed = create_rw_signal(false);
        let sections = section_views(
            self.props.sections,
            &theme,
            selected_path,
            hidden_revealed,
            initial_selected_path,
        );
        let mut rows: Vec<Box<dyn View>> = sections;

        if let Some(show_more) = self.props.show_more {
            rows.push(show_more_view(
                show_more,
                theme.clone(),
                hidden_revealed,
                has_hidden_items,
            ));
        }

        v_stack_from_iter(rows)
            .style(move |style| style.gap(SECTION_GAP).width_full().items_start())
    }
}
