use katana_ui_core::egui::text_command_surface::KucNativeUnicodeEvidenceFrame;
use std::num::NonZeroU32;
use winit::window::Window;

const RGBA_CHANNELS: usize = 4;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;

pub(super) struct RootRenderer;
impl RootRenderer {
    pub(super) fn present(
        window: &Window,
        frame: &KucNativeUnicodeEvidenceFrame,
    ) -> Result<(), String> {
        let width = NonZeroU32::new(frame.width).ok_or("root frame width is zero")?;
        let height = NonZeroU32::new(frame.height).ok_or("root frame height is zero")?;
        let context = softbuffer::Context::new(window).map_err(|e| e.to_string())?;
        let mut surface = softbuffer::Surface::new(&context, window).map_err(|e| e.to_string())?;
        surface.resize(width, height).map_err(|e| e.to_string())?;
        let mut buffer = surface.buffer_mut().map_err(|e| e.to_string())?;
        for (pixel, rgba) in buffer
            .iter_mut()
            .zip(frame.rgba_pixels.chunks_exact(RGBA_CHANNELS))
        {
            *pixel = u32::from(rgba[0]) << RED_SHIFT
                | u32::from(rgba[1]) << GREEN_SHIFT
                | u32::from(rgba[2]);
        }
        buffer.present().map_err(|e| e.to_string())
    }
}
