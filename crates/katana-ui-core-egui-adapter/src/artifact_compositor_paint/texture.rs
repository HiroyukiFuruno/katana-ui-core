use super::super::artifact_compositor_blend::TextureRef;
use crate::command_chrome::CommandChromePaintTexture;
use crate::context_menu::ContextMenuPaintTexture;
use crate::diagnostics_list::DiagnosticsListPaintTexture;
use crate::source_address_strip::SourceAddressPaintTexture;
use crate::status_bar::StatusBarPaintTexture;
use crate::tab_strip_paint::TabStripPaintTexture;
use crate::text_surface::TextSurfacePaintTexture;

macro_rules! impl_texture_ref {
    ($type:ty) => {
        impl TextureRef for $type {
            fn identity(&self) -> &str {
                &self.identity
            }

            fn width(&self) -> u32 {
                self.width
            }

            fn height(&self) -> u32 {
                self.height
            }

            fn rgba_pixels(&self) -> &[u8] {
                &self.rgba_pixels
            }
        }
    };
}

impl_texture_ref!(TextSurfacePaintTexture);
impl_texture_ref!(SourceAddressPaintTexture);
impl_texture_ref!(StatusBarPaintTexture);
impl_texture_ref!(DiagnosticsListPaintTexture);
impl_texture_ref!(TabStripPaintTexture);
impl_texture_ref!(CommandChromePaintTexture);
impl_texture_ref!(ContextMenuPaintTexture);
