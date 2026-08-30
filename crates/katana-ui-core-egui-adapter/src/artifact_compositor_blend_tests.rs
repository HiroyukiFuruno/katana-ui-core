use super::*;
use crate::text_surface::TextSurfacePaintTexture;
use std::cell::Cell;

const OPAQUE_WHITE_RGBA: [u8; RGBA_CHANNELS] = [u8::MAX; RGBA_CHANNELS];

fn paint_texture(width: u32, height: u32, pixels: Vec<u8>) -> TextSurfacePaintTexture {
    TextSurfacePaintTexture {
        identity: "test-texture".to_owned(),
        width,
        height,
        rgba_pixels: pixels,
    }
}

fn require_ok<T: std::fmt::Debug, E: std::fmt::Debug>(
    result: Result<T, E>,
    context: &str,
) -> Option<T> {
    assert!(result.is_ok(), "{context}: {result:?}");
    result.ok()
}

fn require_err<T: std::fmt::Debug, E: std::fmt::Debug>(
    result: Result<T, E>,
    context: &str,
) -> Option<E> {
    assert!(result.is_err(), "{context}: {result:?}");
    result.err()
}

struct ChangingTexture {
    pixel_reads: Cell<usize>,
    invalidate_after_validation: bool,
}

impl ChangingTexture {
    fn invalidating() -> Self {
        Self {
            pixel_reads: Cell::new(0),
            invalidate_after_validation: true,
        }
    }

    fn stable() -> Self {
        Self {
            pixel_reads: Cell::new(0),
            invalidate_after_validation: false,
        }
    }
}

impl TextureRef for ChangingTexture {
    fn identity(&self) -> &str {
        "changing-texture"
    }

    fn width(&self) -> u32 {
        1
    }

    fn height(&self) -> u32 {
        1
    }

    fn rgba_pixels(&self) -> &[u8] {
        let current = self.pixel_reads.get();
        self.pixel_reads.set(current.saturating_add(1));
        if current == 0 || !self.invalidate_after_validation {
            &OPAQUE_WHITE_RGBA
        } else {
            &[]
        }
    }
}

#[path = "artifact_compositor_blend_tests/basic.rs"]
mod basic;
#[path = "artifact_compositor_blend_tests/failure.rs"]
mod failure;
