use super::adapter::EguiStatusBarAdapter;
use super::render::{STATUS_ALIGNMENTS, SegmentSnapshot};
use super::types::{EguiStatusBarError, StatusBarRenderStyle};
use crate::molecule::{StatusBar, StatusBarSegmentAlignment};

impl EguiStatusBarAdapter {
    pub(super) fn alignment_column(
        &mut self,
        root: egui::Rect,
        status: &StatusBar,
        alignment: StatusBarSegmentAlignment,
        style: &StatusBarRenderStyle,
    ) -> Result<egui::Rect, EguiStatusBarError> {
        let widths = STATUS_ALIGNMENTS
            .into_iter()
            .map(|alignment| self.alignment_width(status, alignment, style))
            .collect::<Result<Vec<_>, _>>()?;
        let intervals = STATUS_ALIGNMENTS
            .into_iter()
            .zip(widths)
            .filter(|(_, width)| *width > 0.0)
            .map(|(alignment, width)| Self::alignment_interval(root, width, alignment))
            .collect::<Vec<_>>();
        let intervals_overlap = intervals.iter().enumerate().any(|(index, interval)| {
            intervals[index + 1..]
                .iter()
                .any(|other| interval.0 < other.1 && other.0 < interval.1)
        });
        if !intervals_overlap {
            return Ok(root);
        }

        let column_width = root.width() / STATUS_ALIGNMENTS.len() as f32;
        let column_index = match alignment {
            StatusBarSegmentAlignment::Leading => 0,
            StatusBarSegmentAlignment::Center => 1,
            StatusBarSegmentAlignment::Trailing => 2,
        };
        let left = root.left() + column_width * column_index as f32;
        let right = if column_index + 1 == STATUS_ALIGNMENTS.len() {
            root.right()
        } else {
            left + column_width
        };
        Ok(egui::Rect::from_min_max(
            egui::pos2(left, root.top()),
            egui::pos2(right, root.bottom()),
        ))
    }

    fn alignment_interval(
        root: egui::Rect,
        width: f32,
        alignment: StatusBarSegmentAlignment,
    ) -> (f32, f32) {
        match alignment {
            StatusBarSegmentAlignment::Leading => (root.left(), root.left() + width),
            StatusBarSegmentAlignment::Center => {
                let left = root.center().x - width / 2.0;
                (left, left + width)
            }
            StatusBarSegmentAlignment::Trailing => (root.right() - width, root.right()),
        }
    }

    fn alignment_width(
        &mut self,
        status: &StatusBar,
        alignment: StatusBarSegmentAlignment,
        style: &StatusBarRenderStyle,
    ) -> Result<f32, EguiStatusBarError> {
        let widths = status
            .segments_for(alignment)
            .into_iter()
            .map(|segment| {
                self.raster_width(&SegmentSnapshot::from(segment).display_label(), style)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(widths.iter().sum::<f32>()
            + style.segment_gap_px as f32 * widths.len().saturating_sub(1) as f32)
    }
}
