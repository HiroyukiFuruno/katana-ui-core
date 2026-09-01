use super::EguiTextCommandSurfaceHostRootFrame;

impl EguiTextCommandSurfaceHostRootFrame {
    #[cfg(feature = "storybook-artifacts")]
    pub(crate) fn artifact_rgba(&self) -> (&[u8], u32, u32, &str) {
        let dimensions = self.record.dimensions();
        (
            self.output.rgba_pixels(),
            dimensions.width(),
            dimensions.height(),
            self.record.rgba_hash(),
        )
    }
}
