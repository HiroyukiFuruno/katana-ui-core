use super::{StatusBar, StatusSeverity};
use crate::floem_view::EMPTY_SIZE;
use floem::IntoView;
use floem::views::{Decorators, button, container, empty, h_stack, label};
use std::rc::Rc;

const ACTION_PADDING_H: f32 = crate::floem_view::GAP_XS;
const ACTION_PADDING_V: f32 = 3.0;
const STATUS_ICON_SIZE: f32 = 16.0;

fn empty_slot() -> Box<dyn floem::View> {
    container(empty())
        .style(|style| style.width(EMPTY_SIZE).height(EMPTY_SIZE))
        .into_any()
}

fn action_button(
    severity: StatusSeverity,
    text: String,
    on_action: Rc<dyn Fn()>,
    theme: &crate::theme::Theme,
) -> Box<dyn floem::View> {
    let color = crate::layout::status_bar::icon_color(severity, theme);
    let border = crate::floem_view::FloemColor::from_token(color);
    let text_color = crate::floem_view::FloemColor::from_token(theme.color.text);
    let font_size = theme.typography.body.font_size;
    let label_text = text;
    button(
        label(move || label_text.clone())
            .style(move |style| style.font_size(font_size).color(text_color)),
    )
    .action(move || {
        (on_action)();
    })
    .style(move |style| {
        style
            .padding_horiz(ACTION_PADDING_H)
            .padding_vert(ACTION_PADDING_V)
            .border(1.0)
            .border_color(border)
    })
    .into_any()
}

impl StatusBar {
    #[must_use]
    pub fn view(self, theme: crate::theme::Theme) -> impl IntoView {
        let icon_size = theme.typography.body.font_size;
        let resolved = self.resolve(&theme);
        let has_trailing = self.props.trailing.is_some();
        let trailing = self.props.trailing.unwrap_or_else(empty_slot);
        let on_action = Rc::clone(&self.props.on_action);
        let action_label = self.props.action_label;
        let has_action = action_label.is_some();

        let leading = {
            let icon_color = crate::floem_view::FloemColor::from_token(resolved.icon_color);
            let text_color = crate::floem_view::FloemColor::from_token(resolved.text_color);
            let resolved_gap = resolved.gap;
            h_stack((
                label(move || resolved.icon.to_string())
                    .style(move |style| style.font_size(STATUS_ICON_SIZE).color(icon_color)),
                label(move || resolved.message.clone())
                    .style(move |style| style.font_size(icon_size).color(text_color)),
            ))
            .style(move |style| style.gap(resolved_gap).items_center())
        };

        let action = if has_action {
            action_button(
                resolved.severity,
                action_label.unwrap_or_default(),
                Rc::clone(&on_action),
                &theme,
            )
        } else {
            empty_slot()
        };

        let right = match (has_trailing, has_action) {
            (false, false) => empty_slot(),
            (true, false) => trailing,
            (false, true) => action,
            (true, true) => h_stack((trailing, action))
                .style(move |s| s.gap(resolved.gap).items_center())
                .into_any(),
        };

        let bar_bg = crate::floem_view::FloemColor::from_token(resolved.bar_color);
        h_stack((leading, right)).style(move |style| {
            style
                .width_full()
                .height(resolved.height)
                .padding(resolved.padding)
                .justify_between()
                .items_center()
                .background(bar_bg)
                .gap(resolved.gap)
        })
    }
}
