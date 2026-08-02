const COMPACT_HEADING_LINE_HEIGHT: usize = 40;
const COMPACT_HEADING_2_LINE_HEIGHT: usize = 34;
const COMPACT_HEADING_3_LINE_HEIGHT: usize = 30;
const COMPACT_HTML_BODY_FONT_SIZE: f32 = 13.0;
pub(super) fn compact_heading_line_height(role: &str) -> usize {
    if is_export_heading_2_role(role) {
        return COMPACT_HEADING_2_LINE_HEIGHT;
    }
    if is_export_heading_3_role(role) {
        return COMPACT_HEADING_3_LINE_HEIGHT;
    }
    if is_export_heading_1_role(role) {
        return COMPACT_HEADING_LINE_HEIGHT;
    }
    if is_heading_2_role(role) {
        return COMPACT_HEADING_2_LINE_HEIGHT;
    }
    if is_heading_3_role(role) {
        return COMPACT_HEADING_3_LINE_HEIGHT;
    }
    COMPACT_HEADING_LINE_HEIGHT
}

pub(super) fn compact_heading_font_size(role: &str, body_font_size: f32) -> f32 {
    if role == "heading" {
        return body_font_size;
    }
    body_font_size
}

pub(super) fn is_heading_role(role: &str) -> bool {
    is_heading_1_role(role) || is_heading_2_role(role) || is_heading_3_role(role)
}

pub(super) fn is_html_role(role: &str) -> bool {
    role.starts_with("heading-html-")
        || matches!(
            role,
            "html-centered"
                | "html-right"
                | "html-left"
                | "html-block"
                | "html-accordion"
                | "html-accordion-body"
                | "html-centered-preview"
                | "html-right-preview"
                | "html-left-preview"
                | "html-block-preview"
                | "html-accordion-preview"
                | "html-accordion-body-preview"
        )
}

pub(super) fn html_body_font_size(body_font_size: f32) -> f32 {
    if body_font_size <= super::COMPACT_BODY_FONT_SIZE {
        return COMPACT_HTML_BODY_FONT_SIZE;
    }
    body_font_size
}

pub(super) fn is_preview_html_body_role(role: &str) -> bool {
    matches!(
        role,
        "html-centered-preview"
            | "html-right-preview"
            | "html-left-preview"
            | "html-block-preview"
            | "html-accordion-preview"
            | "html-accordion-body-preview"
    )
}

pub(super) fn is_export_surface_html_body_role(role: &str) -> bool {
    matches!(
        role,
        "html-centered"
            | "html-right"
            | "html-left"
            | "html-block"
            | "html-accordion"
            | "html-accordion-body"
    )
}

pub(super) fn is_document_body_role(role: &str) -> bool {
    matches!(
        role,
        "body" | "paragraph" | "list" | "list-item" | "list-marker" | "blockquote" | "footnote"
    ) || is_preview_html_body_role(role)
        || is_export_surface_html_body_role(role)
}

pub(super) fn is_heading_1_role(role: &str) -> bool {
    role == "heading" || role.starts_with("heading-html-") || is_export_heading_1_role(role)
}

pub(super) fn is_heading_2_role(role: &str) -> bool {
    role == "heading-2"
        || role == "heading-2-long"
        || role.starts_with("heading-2-html-")
        || is_export_heading_2_role(role)
}

pub(super) fn is_heading_3_role(role: &str) -> bool {
    role == "heading-3" || role.starts_with("heading-3-html-") || is_export_heading_3_role(role)
}

pub(super) fn is_export_heading_1_role(role: &str) -> bool {
    role == "heading-export"
}

pub(super) fn is_export_heading_2_role(role: &str) -> bool {
    role == "heading-2-export"
}

pub(super) fn is_export_heading_3_role(role: &str) -> bool {
    role == "heading-3-export"
}

pub(super) fn is_long_heading_2_role(role: &str) -> bool {
    role == "heading-2-long"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_heading_metrics_cover_export_and_html_roles() {
        assert_eq!(
            COMPACT_HEADING_2_LINE_HEIGHT,
            compact_heading_line_height("heading-2-export")
        );
        assert_eq!(
            COMPACT_HEADING_3_LINE_HEIGHT,
            compact_heading_line_height("heading-3-export")
        );
        assert_eq!(
            COMPACT_HEADING_LINE_HEIGHT,
            compact_heading_line_height("heading-export")
        );
        assert!(is_html_role("heading-html-1"));
        assert!(is_html_role("html-accordion-body-preview"));
        assert!(!is_html_role("body"));
    }

    #[test]
    fn html_body_font_size_only_compacts_small_document_text() {
        assert_eq!(
            COMPACT_HTML_BODY_FONT_SIZE,
            html_body_font_size(super::super::COMPACT_BODY_FONT_SIZE)
        );
        assert_eq!(18.0, html_body_font_size(18.0));
    }
}
