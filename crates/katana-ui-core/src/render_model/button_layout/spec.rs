use serde::{Deserialize, Serialize};

use super::dto::UiButtonLayoutDto;
use super::patch::UiButtonLayoutPatchDto;
use super::preset::UiButtonLayoutPreset;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiButtonLayoutSpec {
    Preset(UiButtonLayoutPreset),
    Custom(UiButtonLayoutDto),
    PresetPatch {
        preset: UiButtonLayoutPreset,
        patch: UiButtonLayoutPatchDto,
    },
}

impl UiButtonLayoutSpec {
    #[must_use]
    pub fn preset(preset: UiButtonLayoutPreset) -> Self {
        Self::Preset(preset)
    }

    #[must_use]
    pub fn custom(layout: UiButtonLayoutDto) -> Self {
        Self::Custom(layout)
    }

    #[must_use]
    pub fn preset_patch(preset: UiButtonLayoutPreset, patch: UiButtonLayoutPatchDto) -> Self {
        Self::PresetPatch { preset, patch }
    }

    #[must_use]
    pub fn resolve(self) -> UiButtonLayoutDto {
        match self {
            Self::Preset(preset) => preset.to_dto(),
            Self::Custom(layout) => layout,
            Self::PresetPatch { preset, patch } => patch.apply_to(preset.to_dto()),
        }
    }
}

impl Default for UiButtonLayoutSpec {
    fn default() -> Self {
        Self::Preset(UiButtonLayoutPreset::default())
    }
}

impl From<UiButtonLayoutPreset> for UiButtonLayoutSpec {
    fn from(value: UiButtonLayoutPreset) -> Self {
        Self::Preset(value)
    }
}

impl From<UiButtonLayoutDto> for UiButtonLayoutSpec {
    fn from(value: UiButtonLayoutDto) -> Self {
        Self::Custom(value)
    }
}
