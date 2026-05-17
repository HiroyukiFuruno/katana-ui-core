use crate::theme::color::Color;

const CHANNEL_MAX_FLOAT: f64 = 255.0;
const HSV_SECTOR_COUNT: f64 = 6.0;
const HSV_SECTOR_COUNT_INT: i32 = 6;
const HSV_GREEN_OFFSET: f64 = 1.0 / 3.0;
const HSV_BLUE_OFFSET: f64 = 2.0 / 3.0;
const SECTOR_CYAN: i32 = 3;
const SECTOR_BLUE: i32 = 4;

/// HSV state used by the egui-style editor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorPickerHsva {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
    pub alpha: f64,
}

impl ColorPickerHsva {
    #[must_use]
    pub fn from_color(color: Color) -> Self {
        let red = f64::from(color.r) / CHANNEL_MAX_FLOAT;
        let green = f64::from(color.g) / CHANNEL_MAX_FLOAT;
        let blue = f64::from(color.b) / CHANNEL_MAX_FLOAT;
        let max_channel = red.max(green.max(blue));
        let min_channel = red.min(green.min(blue));
        let channel_range = max_channel - min_channel;
        let saturation = if max_channel == 0.0 {
            0.0
        } else {
            1.0 - min_channel / max_channel
        };

        Self {
            hue: hue_from_rgb(red, green, blue, max_channel, channel_range),
            saturation,
            value: max_channel,
            alpha: f64::from(color.a) / CHANNEL_MAX_FLOAT,
        }
    }

    #[must_use]
    pub fn to_color(self) -> Color {
        let hue = (self.hue.fract() + 1.0).fract();
        let saturation = self.saturation.clamp(0.0, 1.0);
        let value = self.value.clamp(0.0, 1.0);
        let sector = (hue * HSV_SECTOR_COUNT).floor() as i32;
        let fraction = hue * HSV_SECTOR_COUNT - f64::from(sector);
        let low = value * (1.0 - saturation);
        let falling = value * (1.0 - fraction * saturation);
        let rising = value * (1.0 - (1.0 - fraction) * saturation);

        let (red, green, blue) = match sector.rem_euclid(HSV_SECTOR_COUNT_INT) {
            0 => (value, rising, low),
            1 => (falling, value, low),
            2 => (low, value, rising),
            SECTOR_CYAN => (low, falling, value),
            SECTOR_BLUE => (rising, low, value),
            _ => (value, low, falling),
        };

        Color {
            r: unit_to_channel(red),
            g: unit_to_channel(green),
            b: unit_to_channel(blue),
            a: unit_to_channel(self.alpha.abs()),
        }
    }
}

fn hue_from_rgb(red: f64, green: f64, blue: f64, max_channel: f64, channel_range: f64) -> f64 {
    if channel_range == 0.0 {
        return 0.0;
    }

    let hue = if max_channel == red {
        (green - blue) / (HSV_SECTOR_COUNT * channel_range)
    } else if max_channel == green {
        (blue - red) / (HSV_SECTOR_COUNT * channel_range) + HSV_GREEN_OFFSET
    } else {
        (red - green) / (HSV_SECTOR_COUNT * channel_range) + HSV_BLUE_OFFSET
    };
    (hue + 1.0).fract()
}

fn unit_to_channel(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * CHANNEL_MAX_FLOAT).round() as u8
}
