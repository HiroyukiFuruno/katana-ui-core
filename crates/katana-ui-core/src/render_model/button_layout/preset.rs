use serde::{Deserialize, Serialize};

use super::dto::UiButtonLayoutDto;

const MODERN_MIN_WIDTH: u16 = 120;
const MODERN_MIN_HEIGHT: u16 = 36;
const MODERN_PADDING_X: u16 = 18;
const MODERN_PADDING_Y: u16 = 9;
const MODERN_BORDER_WIDTH: u16 = 1;
const MODERN_RADIUS: u16 = 6;
const MODERN_ICON_GAP: u16 = 8;
const CLASSIC_MIN_WIDTH: u16 = 112;
const CLASSIC_MIN_HEIGHT: u16 = 34;
const CLASSIC_PADDING_X: u16 = 14;
const CLASSIC_PADDING_Y: u16 = 8;
const CLASSIC_BORDER_WIDTH: u16 = 2;
const CLASSIC_RADIUS: u16 = 2;
const CLASSIC_ICON_GAP: u16 = 6;
const BASIC_MIN_WIDTH: u16 = 96;
const BASIC_MIN_HEIGHT: u16 = 32;
const BASIC_PADDING_X: u16 = 12;
const BASIC_PADDING_Y: u16 = 7;
const BASIC_BORDER_WIDTH: u16 = 1;
const BASIC_RADIUS: u16 = 0;
const BASIC_ICON_GAP: u16 = 6;
const DENSE_MIN_WIDTH: u16 = 76;
const DENSE_MIN_HEIGHT: u16 = 26;
const DENSE_PADDING_X: u16 = 8;
const DENSE_PADDING_Y: u16 = 5;
const DENSE_BORDER_WIDTH: u16 = 1;
const DENSE_RADIUS: u16 = 4;
const DENSE_ICON_GAP: u16 = 4;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiButtonLayoutPreset {
    #[default]
    Modern,
    Classic,
    Basic,
    Dense,
}

impl UiButtonLayoutPreset {
    #[must_use]
    pub fn to_dto(self) -> UiButtonLayoutDto {
        match self {
            Self::Modern => UiButtonLayoutDto::new(
                MODERN_MIN_WIDTH,
                MODERN_MIN_HEIGHT,
                MODERN_PADDING_X,
                MODERN_PADDING_Y,
                MODERN_BORDER_WIDTH,
                MODERN_RADIUS,
                MODERN_ICON_GAP,
            ),
            Self::Classic => UiButtonLayoutDto::new(
                CLASSIC_MIN_WIDTH,
                CLASSIC_MIN_HEIGHT,
                CLASSIC_PADDING_X,
                CLASSIC_PADDING_Y,
                CLASSIC_BORDER_WIDTH,
                CLASSIC_RADIUS,
                CLASSIC_ICON_GAP,
            ),
            Self::Basic => UiButtonLayoutDto::new(
                BASIC_MIN_WIDTH,
                BASIC_MIN_HEIGHT,
                BASIC_PADDING_X,
                BASIC_PADDING_Y,
                BASIC_BORDER_WIDTH,
                BASIC_RADIUS,
                BASIC_ICON_GAP,
            ),
            Self::Dense => UiButtonLayoutDto::new(
                DENSE_MIN_WIDTH,
                DENSE_MIN_HEIGHT,
                DENSE_PADDING_X,
                DENSE_PADDING_Y,
                DENSE_BORDER_WIDTH,
                DENSE_RADIUS,
                DENSE_ICON_GAP,
            ),
        }
    }
}
