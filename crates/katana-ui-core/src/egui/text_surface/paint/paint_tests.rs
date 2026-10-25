use super::gutter_label_bounds;
use crate::egui::raster_extent::LogicalRasterExtent;
use crate::egui::text_surface::TextSurfaceRasterStyle;
use crate::egui::text_surface::model::SharedTextMetrics;
use crate::egui::text_surface::raster::rasterize_gutter_label;
use crate::render_model::UiRect;
use crate::text_raster::{
    PlatformTextMetricsFrame, PlatformTextRasterConfig, PlatformTextRasterizer,
};
use crate::theme::{FontFamily, FontToken};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn texture_tiles_preserve_pixels_and_partition_scaled_bounds() {
    let pixels = (0_u8..15)
        .flat_map(|value| [value, value + 20, value + 40, 255])
        .collect::<Vec<_>>();
    let texture = super::TextSurfacePaintTexture {
        identity: "pixel-grid".to_owned(),
        width: 5,
        height: 3,
        rgba_pixels: pixels.clone(),
    };
    for limit in [1, 2, 4, 8] {
        let mut restored = vec![0; pixels.len()];
        for (bounds, tile) in super::texture_tiles(UiRect::new(0, 0, 5, 3), &texture, limit) {
            assert!(tile.width as usize <= limit && tile.height as usize <= limit);
            for row in 0..tile.height as usize {
                let start = ((bounds.y as usize + row) * 5 + bounds.x as usize) * 4;
                let tile_start = row * tile.width as usize * 4;
                let count = tile.width as usize * 4;
                restored[start..start + count]
                    .copy_from_slice(&tile.rgba_pixels[tile_start..tile_start + count]);
            }
        }
        assert_eq!(restored, pixels);
        let mut coverage = [0_u8; 13 * 7];
        for (bounds, tile) in super::texture_tiles(UiRect::new(-4, 7, 13, 7), &texture, limit) {
            assert!(tile.width as usize <= limit && tile.height as usize <= limit);
            let left = usize::try_from(bounds.x + 4).expect("left");
            let top = usize::try_from(bounds.y - 7).expect("top");
            assert!(left + bounds.width as usize <= 13);
            assert!(top + bounds.height as usize <= 7);
            for y in top..top + bounds.height as usize {
                for x in left..left + bounds.width as usize {
                    coverage[y * 13 + x] += 1;
                }
            }
        }
        assert!(coverage.iter().all(|count| *count == 1));
    }
}

#[test]
fn empty_texture_dimensions_allocate_neither_meshes_nor_gpu_textures() {
    let context = egui::Context::default();
    let mut warmup = context.run_ui(egui::RawInput::default(), |_| {});
    warmup.textures_delta.clear();
    let mut cache = super::RgbaTextureCache::new(1);
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        for (width, height) in [(0, 2), (2, 0)] {
            super::paint_texture_tiles(
                ui.painter(),
                &mut cache,
                &context,
                UiRect::new(0, 0, width, height),
                &super::TextSurfacePaintTexture {
                    identity: "empty-texture".to_owned(),
                    width,
                    height,
                    rgba_pixels: Vec::new(),
                },
                2048,
            );
        }
        super::paint_texture_tiles(
            ui.painter(),
            &mut cache,
            &context,
            UiRect::new(0, 0, 2, 2),
            &super::TextSurfacePaintTexture {
                identity: "zero-limit-texture".to_owned(),
                width: 2,
                height: 2,
                rgba_pixels: vec![255; 16],
            },
            0,
        );
    });
    let texture_count = output.textures_delta.set.len();
    output.textures_delta.clear();
    assert!(output.shapes.is_empty());
    assert_eq!(texture_count, 0);
}

#[test]
fn gutter_raster_uses_the_same_logical_extent_for_reservation_and_paint() {
    let style = TextSurfaceRasterStyle::new(
        FontToken {
            name: "system-ui".to_string(),
            family: FontFamily::Monospace,
            size: 14.0,
            weight: 400,
        },
        [222, 222, 222, 255],
        20.0,
    );
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    for scale in [1.0, 1.5, 2.0] {
        let metrics: SharedTextMetrics = Rc::new(RefCell::new(PlatformTextMetricsFrame::default()));
        let raster =
            rasterize_gutter_label(&mut rasterizer, "128 日本語 ⭐️", &style, scale, &metrics)
                .expect("gutter label should rasterize");
        let (width, height) = LogicalRasterExtent::size(&raster, scale);
        let reservation = UiRect::new(0, 0, width.saturating_add(12), height);
        let bounds = gutter_label_bounds(reservation, &raster, scale);
        let text_viewport = UiRect::new(reservation.width as i32, 0, 240, reservation.height);
        assert_eq!((bounds.width, bounds.height), (width, height));
        assert!(bounds.width <= reservation.width);
        assert!(bounds.height <= reservation.height);
        assert!(
            bounds.x.saturating_add_unsigned(bounds.width) <= text_viewport.x,
            "gutter label must not overlap the text viewport at scale {scale}"
        );
    }
}
