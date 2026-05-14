use crate::composite::menu_button::types::MenuButtonTrigger;
use crate::primitive::icon::{Icon, IconSize};
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::views::{Decorators, label};

const MENU_TRIGGER_ICON_SIZE: f32 = 14.0;

pub(super) fn build_trigger(trigger: &MenuButtonTrigger, theme: &Theme) -> Box<dyn View> {
    match trigger {
        MenuButtonTrigger::Label(label_text) => {
            let font_size = theme.typography.body.font_size;
            let text = label_text.clone();

            label(move || text.clone())
                .style(move |style| style.font_size(font_size))
                .into_any()
        }
        MenuButtonTrigger::Icon(source) => Icon::new(source.clone())
            .size(IconSize::Pt(MENU_TRIGGER_ICON_SIZE))
            .color_override(theme.color.text)
            .view(theme.clone())
            .into_any(),
        MenuButtonTrigger::Node(builder) => builder(),
    }
}
