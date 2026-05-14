use floem::IntoView;
use floem::View;
use floem::views::{Decorators, container, empty, h_stack, v_stack};

use super::NotificationToastPosition;

pub(super) fn position_toast_stack(
    list: Box<dyn View>,
    position: NotificationToastPosition,
) -> Box<dyn View> {
    match position {
        NotificationToastPosition::TopRight => h_stack((list,))
            .style(|style| style.width_full().justify_end())
            .into_any(),
        NotificationToastPosition::TopLeft => list,
        NotificationToastPosition::BottomRight => v_stack((
            container(empty()).style(|style| style.flex_grow(1.0)),
            h_stack((list,)).style(|style| style.width_full().justify_end()),
        ))
        .style(|style| style.height_full())
        .into_any(),
        NotificationToastPosition::BottomLeft => v_stack((
            container(empty()).style(|style| style.flex_grow(1.0)),
            h_stack((list, container(empty()).style(|style| style.flex_grow(1.0))))
                .style(|style| style.width_full()),
        ))
        .style(|style| style.height_full())
        .into_any(),
    }
}
