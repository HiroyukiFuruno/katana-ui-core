use crate::theme::color::Color;

/// Raw props passed to `ProgressBar`.
#[derive(Debug, Clone)]
pub struct ProgressBarProps {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub indeterminate: bool,
    pub size: f32,
    pub radius: f32,
    pub track_width: f32,
    pub track_color: Option<Color>,
    pub fill_color: Option<Color>,
    pub label: Option<String>,
    pub show_label: bool,
    pub animation_speed_ms: u64,
}

impl Default for ProgressBarProps {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            indeterminate: false,
            size: 0.0,
            radius: -1.0,
            track_width: 0.0,
            track_color: None,
            fill_color: None,
            label: None,
            show_label: false,
            animation_speed_ms: 0,
        }
    }
}
