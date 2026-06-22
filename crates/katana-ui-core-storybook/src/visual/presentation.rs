use super::canvas::Canvas;
use super::presentation_frame;

#[must_use]
pub struct StorybookPresentation;

impl StorybookPresentation {
    #[must_use]
    pub fn present_frame_for_window(
        source: &Canvas,
        width: usize,
        height: usize,
        fill: u32,
    ) -> Canvas {
        presentation_frame::present_frame_for_window(source, width, height, fill)
    }

    pub fn present_frame_for_window_into(
        source: &Canvas,
        target: &mut Canvas,
        width: usize,
        height: usize,
        fill: u32,
    ) {
        presentation_frame::present_frame_for_window_into(source, target, width, height, fill);
    }

    pub fn present_frame_region_for_window_into(
        source: &Canvas,
        target: &mut Canvas,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> bool {
        presentation_frame::present_frame_region_for_window_into(
            source, target, x, y, width, height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::StorybookPresentation;
    use crate::visual::Canvas;

    #[test]
    fn frame_presentation_uses_public_associated_function() {
        let lib_source = include_str!("../lib.rs");
        let visual_mod_source = include_str!("mod.rs");

        assert!(!lib_source.contains("present_frame,"));
        assert!(!visual_mod_source.contains("present_frame,"));
        assert!(lib_source.contains("StorybookPresentation"));
        assert!(visual_mod_source.contains("StorybookPresentation"));
    }

    #[test]
    fn associated_function_delegates_to_frame_presenter() {
        let source = Canvas::new(4, 2, 0xffffff);
        let presented = StorybookPresentation::present_frame_for_window(&source, 12, 8, 0x111111);

        assert_eq!(12, presented.width());
        assert_eq!(8, presented.height());
    }

    #[test]
    fn associated_function_presents_hidpi_frame_at_window_buffer_size() {
        let source = Canvas::new_scaled(4, 2, 2.0, 0xffffff);
        let mut target = Canvas::new(4, 2, 0);

        StorybookPresentation::present_frame_for_window_into(&source, &mut target, 4, 2, 0x111111);

        assert_eq!(4, target.width());
        assert_eq!(2, target.height());
        assert_eq!(4, target.logical_width());
        assert_eq!(2, target.logical_height());
        assert_eq!(1.0, target.scale_factor());
        assert_eq!(0xffffff, target.pixels()[0]);
    }

    #[test]
    fn associated_function_updates_presented_region() {
        let mut source = Canvas::new_scaled(4, 4, 2.0, 0x000000);
        source.fill_rect(1, 1, 2, 2, 0xffffff);
        let mut target = Canvas::new(4, 4, 0x111111);

        assert!(StorybookPresentation::present_frame_region_for_window_into(
            &source,
            &mut target,
            1,
            1,
            2,
            2
        ));

        assert_eq!(0xffffff, target.pixels()[target.width() + 1]);
        assert_eq!(0x111111, target.pixels()[0]);
    }
}
