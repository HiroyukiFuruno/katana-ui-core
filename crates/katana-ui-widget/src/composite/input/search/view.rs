use super::types::{
    SearchBox, SearchBoxControlMode, SearchBoxIconConfig, SearchBoxIconMode, SearchBoxOptions,
};
use crate::primitive::icon::Icon;
use crate::theme::Theme;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, button, container, empty, h_stack, label, text_input as floem_text_input,
};
use floem::{IntoView, View};
use std::rc::Rc;

const SEARCH_GAP: f32 = crate::floem_view::GAP_XS;
const ICON_RESERVED_SIZE: f32 = 16.0;
const CONTROL_RESERVED_WIDTH: f32 = 28.0;

pub(super) fn build_view(search: SearchBox, theme: Theme) -> impl IntoView {
    let value = create_rw_signal(search.props.value.clone());
    let disabled = search.props.disabled;
    let on_submit = Rc::clone(&search.props.on_submit);
    let placeholder = search.props.placeholder.clone().unwrap_or_default();
    let resolved = search.resolve(&theme);
    let leading_icon = search.props.leading_icon.clone();
    let clear_icon = search.props.clear_icon.clone();
    let submit_icon = search.props.submit_icon.clone();
    let options = create_rw_signal(search.props.options);
    let on_options_change = Rc::clone(&search.props.on_options_change);
    let regex_control = search.props.regex_control;
    let whole_word_control = search.props.whole_word_control;
    let case_sensitive_control = search.props.case_sensitive_control;
    let bg = crate::floem_view::FloemColor::from_token(resolved.bg_color);
    let text = crate::floem_view::FloemColor::from_token(resolved.text_color);
    let border = crate::floem_view::FloemColor::from_token(resolved.border_color);
    let accent = crate::floem_view::FloemColor::from_token(theme.color.accent);
    let muted = crate::floem_view::FloemColor::from_token(theme.color.text_muted);

    h_stack((
        icon_slot(leading_icon, &theme),
        floem_text_input(value)
            .placeholder(placeholder)
            .disabled(move || disabled)
            .style(move |style| style.font_size(resolved.font_size).color(text)),
        action_icon_slot(clear_icon, &theme, move || {
            if !disabled {
                value.set(String::new());
            }
        }),
        control_slot(
            regex_control,
            ".*",
            options,
            on_options_change.clone(),
            accent,
            muted,
            |it| {
                it.regex = !it.regex;
            },
        ),
        control_slot(
            whole_word_control,
            "W",
            options,
            on_options_change.clone(),
            accent,
            muted,
            |it| {
                it.whole_word = !it.whole_word;
            },
        ),
        control_slot(
            case_sensitive_control,
            "Aa",
            options,
            on_options_change,
            accent,
            muted,
            |it| {
                it.case_sensitive = !it.case_sensitive;
            },
        ),
        action_icon_slot(submit_icon, &theme, move || {
            if !disabled {
                on_submit(value.try_get_untracked().unwrap_or_default());
            }
        }),
    ))
    .style(move |style| {
        style
            .gap(SEARCH_GAP)
            .items_center()
            .background(bg)
            .border(1.0)
            .border_color(border)
            .padding_vert(resolved.pad_v)
            .padding_horiz(resolved.pad_h)
    })
}

fn icon_slot(config: SearchBoxIconConfig, theme: &Theme) -> Box<dyn View> {
    match config.mode {
        SearchBoxIconMode::Hidden => empty_icon_space(0.0),
        SearchBoxIconMode::Reserved => empty_icon_space(ICON_RESERVED_SIZE),
        SearchBoxIconMode::Visible => Icon::new(config.source).view(theme.clone()).into_any(),
    }
}

fn action_icon_slot(
    config: SearchBoxIconConfig,
    theme: &Theme,
    action: impl Fn() + 'static,
) -> Box<dyn View> {
    match config.mode {
        SearchBoxIconMode::Hidden => empty_icon_space(0.0),
        SearchBoxIconMode::Reserved => empty_icon_space(ICON_RESERVED_SIZE),
        SearchBoxIconMode::Visible => button(Icon::new(config.source).view(theme.clone()))
            .action(action)
            .into_any(),
    }
}

fn empty_icon_space(width: f32) -> Box<dyn View> {
    container(empty())
        .style(move |style| style.width(width).height(ICON_RESERVED_SIZE))
        .into_any()
}

fn control_slot(
    mode: SearchBoxControlMode,
    text: &'static str,
    options: RwSignal<SearchBoxOptions>,
    on_change: Rc<dyn Fn(SearchBoxOptions)>,
    active_color: floem::peniko::Color,
    inactive_color: floem::peniko::Color,
    toggle: impl Fn(&mut SearchBoxOptions) + 'static,
) -> Box<dyn View> {
    match mode {
        SearchBoxControlMode::Hidden => empty_control_space(0.0),
        SearchBoxControlMode::Reserved => empty_control_space(CONTROL_RESERVED_WIDTH),
        SearchBoxControlMode::Visible => button(label(move || text))
            .action(move || {
                if let Some(next) = options.try_update(|value| {
                    toggle(value);
                    *value
                }) {
                    on_change(next);
                }
            })
            .style(move |style| {
                let current = options.try_get().unwrap_or_default();
                let active = match text {
                    ".*" => current.regex,
                    "W" => current.whole_word,
                    _ => current.case_sensitive,
                };
                style.width(CONTROL_RESERVED_WIDTH).color(if active {
                    active_color
                } else {
                    inactive_color
                })
            })
            .into_any(),
    }
}

fn empty_control_space(width: f32) -> Box<dyn View> {
    container(empty())
        .style(move |style| style.width(width).height(ICON_RESERVED_SIZE))
        .into_any()
}

/// Returns the value after Esc key: always clears the input.
pub(super) fn on_esc(_current: &str) -> String {
    String::new()
}

/// Returns whether clear button should be shown (value non-empty and not disabled).
pub(super) fn show_clear(value: &str, disabled: bool) -> bool {
    !value.is_empty() && !disabled
}
