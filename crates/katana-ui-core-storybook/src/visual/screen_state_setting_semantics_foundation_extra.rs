pub(in crate::visual) fn theme_state(setting: &'static str) -> &'static str {
    match setting {
        "theme.id" => "theme.id=light",
        "color.background" => "theme.color.background=light",
        "color.surface" => "theme.color.surface=contrast",
        "color.accent" => "theme.color.accent=green",
        _ => setting,
    }
}

pub(in crate::visual) fn key_cap_state(setting: &'static str) -> &'static str {
    match setting {
        "content.value" => "key_cap.content.value=custom",
        "visual.role" => "key_cap.visual.role=icon",
        "a11y.label" => "key_cap.a11y.label=changed",
        "theme.color" => "key_cap.theme.color=accent",
        _ => setting,
    }
}

pub(in crate::visual) fn motion_state(setting: &'static str) -> &'static str {
    match setting {
        "motion.primitive" => "motion.primitive=Shimmer",
        "motion.duration" => "motion.duration=Fast",
        "motion.distance" => "motion.distance=Compact",
        "motion.reduced_policy" => "motion.reduced_policy=ForceReduced",
        _ => setting,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundation_semantics_preserve_unknown_settings() {
        assert_eq!("unknown.theme", theme_state("unknown.theme"));
        assert_eq!("unknown.key_cap", key_cap_state("unknown.key_cap"));
        assert_eq!("unknown.motion", motion_state("unknown.motion"));
    }
}
