use super::RichTextStyle;

impl RichTextStyle {
    pub(crate) const fn new(size: f32, color: u32) -> Self {
        Self {
            size,
            color,
            bold: false,
            italic: false,
            emoji: false,
            raster_vertical_scale: 1.0,
        }
    }

    pub(crate) const fn bold(mut self, value: bool) -> Self {
        self.bold = value;
        self
    }

    pub(crate) const fn italic(mut self, value: bool) -> Self {
        self.italic = value;
        self
    }

    pub(crate) const fn emoji(mut self, value: bool) -> Self {
        self.emoji = value;
        self
    }

    pub(crate) const fn raster_vertical_scale(mut self, value: f32) -> Self {
        self.raster_vertical_scale = value;
        self
    }
}
