use minifb::{ScaleMode, WindowOptions};

pub(super) fn main_window_options() -> WindowOptions {
    WindowOptions {
        resize: true,
        scale_mode: ScaleMode::AspectRatioStretch,
        ..WindowOptions::default()
    }
}

pub(super) fn modal_window_options() -> WindowOptions {
    WindowOptions::default()
}

#[cfg(test)]
mod tests {
    use super::main_window_options;
    use minifb::ScaleMode;

    #[test]
    fn main_window_allows_macos_zoom_and_resize() {
        let options = main_window_options();

        assert!(options.resize);
        assert!(options.title);
        assert!(!options.borderless);
        assert_eq!(ScaleMode::AspectRatioStretch, options.scale_mode);
    }
}
