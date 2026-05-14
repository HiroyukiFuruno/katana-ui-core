use super::{AlignCenterWrapper, ResolvedAlignCenterWrapper};
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use floem::IntoView;
use floem::style::Style;
use floem::views::{Decorators, container};

fn apply_axis_style(resolved: &ResolvedAlignCenterWrapper, style: Style) -> Style {
    let mut style = style;

    if let Some(width) = resolved.width {
        style = style.width(width);
    } else if resolved.horizontal {
        style = style.width_full();
    }

    if let Some(height) = resolved.height {
        style = style.height(height);
    } else if resolved.vertical {
        style = style.height_full();
    }

    if resolved.horizontal {
        style = style.justify_center();
    }

    if resolved.vertical {
        style = style.items_center();
    }

    style.gap(resolved.gap)
}

impl AlignCenterWrapper {
    /// Builds a centered container that applies configured size/padding/background.
    #[must_use]
    pub fn view(self, theme: Theme, child: impl IntoView + 'static) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let background = if resolved.disabled {
            Some(FloemColor::from_token(theme.color.border))
        } else {
            resolved.background.map(FloemColor::from_token)
        };
        let disabled_text = FloemColor::from_token(theme.color.text_disabled);

        container(child).style(move |style| {
            let mut style = apply_axis_style(&resolved, style.padding(resolved.padding));
            if let Some(bg) = background {
                style = style.background(bg);
            }
            if resolved.disabled {
                style = style.color(disabled_text);
            }
            style
        })
    }
}
