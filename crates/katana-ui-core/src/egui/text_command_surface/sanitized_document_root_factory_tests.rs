const SEARCH_IME_KEYBOARD_TARGET: [u8; 2] = [9, 1];
const SEARCH_IME_COMMAND_TARGET: [u8; 2] = [9, 2];
const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const FLOATING_SURFACE_HORIZONTAL_OFFSET: f32 = 8.0;

use super::{SanitizedDocumentRoot, SanitizedDocumentRootFrame};

include!("sanitized_document_root_factory_tests/sanitized_document_root_factory_coverage_tests.rs");
