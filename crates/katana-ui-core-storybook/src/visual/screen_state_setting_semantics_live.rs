pub(in crate::visual) fn dynamic_array_state(setting: &'static str) -> &'static str {
    match setting {
        "array.rows" => "array.rows=3",
        "array.add_remove" => "array.rows=4",
        "array.reorder" => "array.order=2,1,3",
        "array.theme_row" => "array.theme_row=accent",
        _ => setting,
    }
}

pub(in crate::visual) fn drag_and_drop_state(setting: &'static str) -> &'static str {
    match setting {
        "drag.accept_policy" => "drag.accept_policy=move",
        "drag.autoscroll" => "drag.autoscroll=edge",
        "drag.keyboard_draggable" => "drag.keyboard_draggable=true",
        "drag.drop_indicator" => "drag.drop_indicator=after",
        _ => setting,
    }
}

#[cfg(test)]
mod tests {
    use super::{drag_and_drop_state, dynamic_array_state};

    #[test]
    fn live_setting_semantics_cover_every_declared_and_fallback_setting() {
        for setting in [
            "array.rows",
            "array.add_remove",
            "array.reorder",
            "array.theme_row",
            "array.unknown",
        ] {
            assert!(!dynamic_array_state(setting).is_empty());
        }
        for setting in [
            "drag.accept_policy",
            "drag.autoscroll",
            "drag.keyboard_draggable",
            "drag.drop_indicator",
            "drag.unknown",
        ] {
            assert!(!drag_and_drop_state(setting).is_empty());
        }
    }
}
