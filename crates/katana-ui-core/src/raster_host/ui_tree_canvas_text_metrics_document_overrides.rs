use super::{
    UiTreeTextMetrics,
    metric_roles::{
        is_document_body_role, is_heading_1_role, is_heading_2_role, is_heading_3_role,
    },
    metric_scaling::{strikethrough_offset, underline_offset},
};
use crate::raster_host::document_typography::UiTreeDocumentTypography;

pub(super) fn with_document_typography(
    mut metrics: UiTreeTextMetrics,
    role: &str,
    document_typography: UiTreeDocumentTypography,
) -> UiTreeTextMetrics {
    let role_typography = if is_heading_1_role(role) {
        document_typography.heading_1()
    } else if is_heading_2_role(role) {
        document_typography.heading_2()
    } else if is_heading_3_role(role) {
        document_typography.heading_3()
    } else if is_document_body_role(role) {
        document_typography.body()
    } else {
        None
    };
    let Some(role_typography) = role_typography.filter(|typography| typography.is_valid()) else {
        return metrics;
    };
    metrics.font_size = role_typography.font_size;
    metrics.line_height = role_typography.line_height;
    metrics.top_margin = role_typography.baseline_offset;
    metrics.background_height = role_typography.line_height;
    metrics.highlight_height = role_typography
        .line_height
        .saturating_sub(role_typography.baseline_offset);
    metrics.underline_offset = underline_offset(role_typography.font_size);
    metrics.strikethrough_offset = strikethrough_offset(role_typography.font_size);
    metrics
}
