use crate::theme::color::Color;

pub(crate) const EMPTY_SIZE: f32 = 0.0;
pub(crate) const GAP_XS: f32 = 4.0;
pub(crate) const GAP_SM: f32 = 8.0;
pub(crate) const CORNER_RADIUS_SM: f32 = 4.0;

pub(crate) struct FloemColor;

impl FloemColor {
    pub(crate) fn from_token(value: Color) -> floem::peniko::Color {
        floem::peniko::Color::rgba8(value.r, value.g, value.b, value.a)
    }
}
