const XXS: f32 = 2.0;
const XS: f32 = 4.0;
const SM: f32 = 8.0;
const MD: f32 = 12.0;
const LG: f32 = 16.0;
const XL: f32 = 24.0;
const XXL: f32 = 32.0;

/// Fixed spacing scale in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpacingTokens {
    pub xxs: f32,
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            xxs: XXS,
            xs: XS,
            sm: SM,
            md: MD,
            lg: LG,
            xl: XL,
            xxl: XXL,
        }
    }
}
