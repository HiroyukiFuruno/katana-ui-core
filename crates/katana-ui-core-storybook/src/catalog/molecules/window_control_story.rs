use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::molecule::{
    self, WindowControlButtonGroupAction, WindowControlButtonGroupOptions, WindowControlKind,
    WindowControlSize, WindowControlVisibility, WindowControlsPosition,
};

const WINDOW_CONTROL_TARGET: &str = "state:WindowControlButtonGroup:storybook";

pub(super) fn example() -> StoryExample {
    let presets = presets();
    let root = presets.iter().fold(
        molecule::List::new("Window control button group presets"),
        |list, preset| list.child(preset.preview()),
    );
    let logs = presets
        .iter()
        .map(WindowControlPreset::callback_log)
        .collect();

    StoryCatalog::interactive_story("window-control-button-group", root, logs)
}

fn presets() -> [WindowControlPreset; 5] {
    [
        preset(
            "macOS",
            "window_control_press",
            WindowControlsPosition::Leading,
            WindowControlVisibility::Always,
            WindowControlSize::Compact,
            desktop_controls(),
            WindowControlButtonGroupAction::Press(WindowControlKind::Close),
        ),
        preset(
            "Windows",
            "window_controls_trailing_press",
            WindowControlsPosition::Trailing,
            WindowControlVisibility::Always,
            WindowControlSize::Default,
            desktop_controls(),
            WindowControlButtonGroupAction::Press(WindowControlKind::Maximize),
        ),
        preset(
            "Linux",
            "window_controls_hover",
            WindowControlsPosition::Auto,
            WindowControlVisibility::Hover,
            WindowControlSize::Tall,
            linux_controls(),
            WindowControlButtonGroupAction::SetHover(true),
        ),
        preset(
            "fullscreen hover",
            "window_controls_fullscreen",
            WindowControlsPosition::Leading,
            WindowControlVisibility::FullscreenHover,
            WindowControlSize::Compact,
            desktop_controls(),
            WindowControlButtonGroupAction::SetFullscreen(true),
        ),
        preset(
            "close only",
            "window_controls_close_only",
            WindowControlsPosition::Trailing,
            WindowControlVisibility::Always,
            WindowControlSize::Default,
            vec![WindowControlKind::Close],
            WindowControlButtonGroupAction::Press(WindowControlKind::Close),
        ),
    ]
}

fn preset(
    name: &'static str,
    action: &'static str,
    position: WindowControlsPosition,
    visibility: WindowControlVisibility,
    size: WindowControlSize,
    controls: Vec<WindowControlKind>,
    action_kind: WindowControlButtonGroupAction,
) -> WindowControlPreset {
    WindowControlPreset {
        name,
        action,
        options: WindowControlButtonGroupOptions {
            controls,
            position,
            visibility,
            size,
        },
        action_kind,
    }
}

fn desktop_controls() -> Vec<WindowControlKind> {
    vec![
        WindowControlKind::Close,
        WindowControlKind::Minimize,
        WindowControlKind::Maximize,
    ]
}

fn linux_controls() -> Vec<WindowControlKind> {
    vec![
        WindowControlKind::Minimize,
        WindowControlKind::Maximize,
        WindowControlKind::Restore,
        WindowControlKind::Close,
    ]
}

struct WindowControlPreset {
    name: &'static str,
    action: &'static str,
    options: WindowControlButtonGroupOptions,
    action_kind: WindowControlButtonGroupAction,
}

impl WindowControlPreset {
    fn preview(&self) -> molecule::WindowControlButtonGroup {
        molecule::WindowControlButtonGroup::new(self.preview_label()).options(self.options.clone())
    }

    fn preview_label(&self) -> String {
        format!(
            "preset={} position={:?} size={:?} controls={} visibility={:?}",
            self.name,
            self.options.position,
            self.options.size,
            self.control_names(),
            self.options.visibility
        )
    }

    fn callback_log(&self) -> UiCallbackLog {
        let target = katana_ui_core::render_model::UiStateId::new(WINDOW_CONTROL_TARGET);
        let mut group =
            molecule::WindowControlButtonGroup::new(self.name).options(self.options.clone());
        let events = group.apply_action(self.action_kind);
        let after = if matches!(
            self.options.visibility,
            WindowControlVisibility::FullscreenHover
        ) {
            let hover_events = group.apply_action(WindowControlButtonGroupAction::SetHover(true));
            format!("events={events:?}+{hover_events:?}")
        } else {
            format!("events={events:?}")
        };

        UiCallbackLog::new(target, self.action, self.before_state(), after)
    }

    fn before_state(&self) -> String {
        format!(
            "preset={} position={:?} size={:?} controls={} visibility={:?} state=visible",
            self.name,
            self.options.position,
            self.options.size,
            self.control_names(),
            self.options.visibility
        )
    }

    fn control_names(&self) -> String {
        self.options
            .controls
            .iter()
            .map(|it| format!("{it:?}"))
            .collect::<Vec<_>>()
            .join("+")
    }
}
