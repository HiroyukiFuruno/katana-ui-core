use super::{preview_contract, render};
use std::collections::BTreeMap;

const PREVIEW_SIGNATURE_SEED: u64 = 17;
const PREVIEW_SIGNATURE_PRIME: u64 = 1_099_511_628_211;
const LEGACY_DOD_PREVIEW_PAGES: &[&str] = &[
    "theme-tokens",
    "text",
    "icon",
    "chip",
    "loading-dots",
    "spinner",
    "button",
    "text-button",
    "svg-button",
    "icon-text-button",
    "toggle",
    "segmented-toggle",
    "select-box",
    "color-swatch",
    "text-input",
    "search-box",
    "tooltip",
    "badge",
    "key-cap",
    "card",
    "accordion",
    "split-pane",
    "modal",
    "popover",
    "color-picker-rgba",
    "code-diff",
    "attachment-chip",
    "chip-group",
];

pub(super) struct LegacyPreviewSignatureStats {
    pub(super) signatures: usize,
    pub(super) collisions: usize,
}

pub(super) fn legacy_preview_signature_stats() -> LegacyPreviewSignatureStats {
    let mut signatures = BTreeMap::new();
    let mut collisions = 0;
    for page in LEGACY_DOD_PREVIEW_PAGES {
        let canvas = render::render_storybook_canvas_for("dark", page, false);
        let signature = hero_preview_signature(&canvas);
        collisions += usize::from(signatures.insert(signature, *page).is_some());
    }
    LegacyPreviewSignatureStats {
        signatures: signatures.len(),
        collisions,
    }
}

fn hero_preview_signature(canvas: &super::Canvas) -> u64 {
    let (x, y, width, height) = preview_contract::selected_detail_rect();
    let mut signature = PREVIEW_SIGNATURE_SEED;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let index = current_y * canvas.width() + current_x;
            let pixel = u64::from(canvas.pixels()[index]);
            signature ^= pixel.wrapping_add(index as u64);
            signature = signature.wrapping_mul(PREVIEW_SIGNATURE_PRIME);
        }
    }
    signature
}
