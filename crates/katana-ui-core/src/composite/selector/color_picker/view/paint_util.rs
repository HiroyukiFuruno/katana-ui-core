use crate::theme::color::Color;
use floem::peniko::Color as PenikoColor;

const CONTRAST_RED_WEIGHT: u16 = 30;
const CONTRAST_GREEN_WEIGHT: u16 = 59;
const CONTRAST_BLUE_WEIGHT: u16 = 11;
const CONTRAST_WEIGHT_TOTAL: u16 = 100;
const CONTRAST_THRESHOLD: u16 = 128;

pub(super) fn contrast_color(color: Color) -> PenikoColor {
    let intensity = (u16::from(color.r) * CONTRAST_RED_WEIGHT
        + u16::from(color.g) * CONTRAST_GREEN_WEIGHT
        + u16::from(color.b) * CONTRAST_BLUE_WEIGHT)
        / CONTRAST_WEIGHT_TOTAL;
    if intensity < CONTRAST_THRESHOLD {
        PenikoColor::WHITE
    } else {
        PenikoColor::BLACK
    }
}
