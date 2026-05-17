use super::{ResolvedToolbar, Toolbar, ToolbarAlignment};
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use floem::views::{Decorators, container, empty, h_stack};
use floem::{IntoView, View, style::Style};

const EMPTY_SIZE: f32 = crate::floem_view::EMPTY_SIZE;

fn apply_alignment(alignment: ToolbarAlignment, style: Style) -> Style {
    match alignment {
        ToolbarAlignment::Top => style.items_start(),
        ToolbarAlignment::Center => style.items_center(),
        ToolbarAlignment::Bottom => style.items_end(),
    }
}

fn empty_slot() -> Box<dyn View> {
    container(empty())
        .style(|style| style.width(EMPTY_SIZE).height(EMPTY_SIZE))
        .into_any()
}

impl Toolbar {
    fn render_root(self, theme: Theme, resolved: ResolvedToolbar) -> impl IntoView {
        let background = resolved
            .background
            .map(FloemColor::from_token)
            .unwrap_or_else(|| FloemColor::from_token(theme.color.bg));
        let border_color = FloemColor::from_token(resolved.border_color);
        let show_background = resolved.background.is_some();
        let leading = self.props.leading.unwrap_or_else(empty_slot);
        let trailing = self.props.trailing.unwrap_or_else(empty_slot);

        h_stack((leading, trailing)).style(move |style| {
            let mut style = style
                .width_full()
                .padding(resolved.padding)
                .justify_between()
                .gap(resolved.gap);
            if let Some(height) = resolved.height {
                style = style.height(height);
            }
            if show_background {
                style = style.background(background);
            }
            if resolved.show_border {
                style = style.border(1.0).border_color(border_color);
            }
            apply_alignment(resolved.alignment, style)
        })
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        self.render_root(theme, resolved)
    }
}
