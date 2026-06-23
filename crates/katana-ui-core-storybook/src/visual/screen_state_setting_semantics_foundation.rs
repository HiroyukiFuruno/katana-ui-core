pub(in crate::visual) fn icon_state(setting: &'static str) -> &'static str {
    match setting {
        "content.value" => "icon.content.value=custom",
        "visual.role" => "icon.visual.role=icon",
        "a11y.label" => "icon.a11y.label=changed",
        "theme.color" => "icon.theme.color=accent",
        "icon.svg_source" => "icon.svg_source=custom-svg",
        "icon.svg_icon" => "icon.svg_icon=props-object",
        "icon.view_box" => "icon.view_box=0 0 24 24",
        "icon.path_summary" => "icon.path_summary=search-outline",
        "icon.paint_policy" => "icon.paint_policy=currentColor",
        "icon.role" => "icon.role=action",
        "icon.color_token" => "icon.color_token=accent",
        "icon.theme_token" => "icon.theme_token=muted",
        _ => setting,
    }
}

pub(in crate::visual) fn text_state(setting: &'static str) -> &'static str {
    match setting {
        "text.role" => "text.role=heading",
        "text.content" => "text.content=empty",
        "text.script" => "text.script=jp+emoji",
        "text.color" => "text.color=accent",
        "text.color_token" => "text.color_token=accent",
        "text.line_metrics" => "text.line_metrics=compact",
        "text.vertical_centered" => "text.vertical_centered=true",
        "text.spans" => "text.spans=rich",
        "text.wrap" => "text.wrap=multi",
        _ => setting,
    }
}

pub(in crate::visual) fn skeleton_state(setting: &'static str) -> &'static str {
    match setting {
        "skeleton.shape" => "skeleton.shape=Line",
        "skeleton.text_lines" => "skeleton.text_lines=2",
        "skeleton.last_line_ratio" => "skeleton.last_line_ratio=0.62",
        "skeleton.line_thickness" => "skeleton.line_thickness=12",
        "size" => "skeleton.size=Fill",
        "skeleton.animation" => "skeleton.animation=Wave",
        "tone" => "skeleton.tone=Accent",
        "skeleton.radius_px" => "skeleton.radius_px=14",
        "skeleton.reduced_motion" => "skeleton.reduced_motion=true",
        "a11y.label" => "skeleton.a11y.label=Loading profile",
        "skeleton.aspect_ratio" => "skeleton.aspect_ratio=16:9",
        _ => setting,
    }
}

pub(in crate::visual) fn loading_indicator_state(
    page_state_prefix: &'static str,
    setting: &'static str,
) -> &'static str {
    match (page_state_prefix, setting) {
        ("loading_dots", "variant") => "loading_dots.variant=alternate",
        ("loading_dots", "loading.animation_state") => "loading_dots.animation_state=Paused",
        ("loading_dots", "loading.reduced_motion") => "loading_dots.reduced_motion=true",
        ("loading_dots", "loading.label") => "loading_dots.label=Saving",
        ("loading_dots", "loading.speed_ms") => "loading_dots.speed_ms=96",
        ("loading_dots", "loading.dot_count") => "loading_dots.dot_count=5",
        ("loading_dots", "tone") => "loading_dots.tone=accent",
        ("loading_dots", "size") => "loading_dots.size=large",
        ("spinner", "variant") => "spinner.variant=alternate",
        ("spinner", "loading.animation_state") => "spinner.animation_state=Paused",
        ("spinner", "loading.reduced_motion") => "spinner.reduced_motion=true",
        ("spinner", "loading.label") => "spinner.label=Saving",
        ("spinner", "loading.speed_ms") => "spinner.speed_ms=96",
        ("spinner", "loading.dot_count") => "spinner.dot_count=5",
        ("spinner", "tone") => "spinner.tone=accent",
        ("spinner", "size") => "spinner.size=large",
        _ => setting,
    }
}

pub(in crate::visual) fn progress_bar_state(setting: &'static str) -> &'static str {
    match setting {
        "variant" => "progress_bar.variant=alternate",
        "progress.percent" => "progress_bar.percent=82",
        "loading.animation_state" => "progress_bar.animation_state=Paused",
        "loading.label" => "progress_bar.label=Syncing",
        "loading.speed_ms" => "progress_bar.speed_ms=96",
        "loading.dot_count" => "progress_bar.dot_count=5",
        "loading.reduced_motion" => "progress_bar.reduced_motion=true",
        "tone" => "progress_bar.tone=accent",
        "size" => "progress_bar.size=large",
        _ => setting,
    }
}

pub(in crate::visual) fn split_pane_state(setting: &'static str) -> &'static str {
    match setting {
        "axis" => "split_pane.axis=Vertical",
        "gap" => "split_pane.gap=12",
        "alignment" => "split_pane.alignment=Center",
        "overflow" => "split_pane.overflow=Scroll",
        "split_pane.ratio_percent" => "split_pane.ratio_percent=64",
        "split_pane.min_percent" => "split_pane.min_percent=24",
        "split_pane.max_percent" => "split_pane.max_percent=76",
        "split_pane.reset_percent" => "split_pane.reset_percent=55",
        "split_pane.handle_width_px" => "split_pane.handle_width_px=10",
        "split_pane.resize_mode" => "split_pane.resize_mode=KeyboardOnly",
        _ => setting,
    }
}

pub(in crate::visual) fn layout_state(
    page_state_prefix: &'static str,
    setting: &'static str,
) -> &'static str {
    match (page_state_prefix, setting) {
        ("scroll_area", "axis") => "scroll_area.axis=y",
        ("scroll_area", "gap") => "scroll_area.gap=large",
        ("scroll_area", "alignment") => "scroll_area.alignment=center",
        ("scroll_area", "overflow") => "scroll_area.overflow=scroll",
        ("align_center", "axis") => "align_center.axis=y",
        ("align_center", "gap") => "align_center.gap=large",
        ("align_center", "alignment") => "align_center.alignment=center",
        ("align_center", "overflow") => "align_center.overflow=scroll",
        _ => setting,
    }
}

pub(in crate::visual) fn primitive_state(
    page_state_prefix: &'static str,
    setting: &'static str,
) -> &'static str {
    match (page_state_prefix, setting) {
        ("divider", "variant") => "divider.variant=alternate",
        ("divider", "tone") => "divider.tone=accent",
        ("divider", "size") => "divider.size=large",
        ("divider", "theme.slot") => "divider.theme.slot=custom",
        ("spacer", "variant") => "spacer.variant=alternate",
        ("spacer", "tone") => "spacer.tone=accent",
        ("spacer", "size") => "spacer.size=large",
        ("spacer", "theme.slot") => "spacer.theme.slot=custom",
        ("color_swatch", "variant") => "color_swatch.variant=alternate",
        ("color_swatch", "tone") => "color_swatch.tone=accent",
        ("color_swatch", "size") => "color_swatch.size=large",
        ("color_swatch", "theme.slot") => "color_swatch.theme.slot=custom",
        ("slide_control", "variant") => "slide_control.variant=alternate",
        ("slide_control", "tone") => "slide_control.tone=accent",
        ("slide_control", "size") => "slide_control.size=large",
        ("slide_control", "theme.slot") => "slide_control.theme.slot=custom",
        _ => setting,
    }
}

pub(in crate::visual) fn binary_choice_state(
    page_state_prefix: &'static str,
    setting: &'static str,
) -> &'static str {
    match (page_state_prefix, setting) {
        ("checkbox", "selected") => "checkbox.selected=true",
        ("checkbox", "disabled") => "checkbox.disabled=true",
        ("checkbox", "focus") => "checkbox.focus=visible",
        ("checkbox", "checked") => "checkbox.checked=true",
        ("radio", "selected") => "radio.selected=true",
        ("radio", "disabled") => "radio.disabled=true",
        ("radio", "focus") => "radio.focus=visible",
        ("radio", "checked") => "radio.checked=true",
        ("toggle", "selected") => "toggle.selected=true",
        ("toggle", "disabled") => "toggle.disabled=true",
        ("toggle", "focus") => "toggle.focus=visible",
        ("toggle", "checked") => "toggle.checked=true",
        ("segmented_toggle", "selected") => "segmented_toggle.selected=true",
        ("segmented_toggle", "disabled") => "segmented_toggle.disabled=true",
        ("segmented_toggle", "focus") => "segmented_toggle.focus=visible",
        ("segmented_toggle", "checked") => "segmented_toggle.checked=true",
        _ => setting,
    }
}
