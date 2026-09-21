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
