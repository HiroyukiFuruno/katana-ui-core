use super::{StoryCatalog, StoryExample, atom_interactions, atom_motion_interactions};
use katana_ui_core::atom;
use katana_ui_core::render_model::{UiSize, UiVisualRole};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        atom_interactions::text(),
        atom_interactions::icon(),
        atom_interactions::chip(),
        atom_interactions::button(),
        atom_interactions::text_button(),
        atom_interactions::svg_button(),
        atom_interactions::icon_text_button(),
        atom_interactions::input(),
        atom_interactions::text_area(),
        atom_interactions::checkbox(),
        atom_interactions::radio(),
        atom_interactions::badge(),
        StoryCatalog::story(
            "divider",
            atom::Divider::new("Divider").visual_role(UiVisualRole::Separator),
        ),
        StoryCatalog::story(
            "spacer",
            atom::Spacer::new("Spacer")
                .visual_role(UiVisualRole::Separator)
                .size(UiSize::Large),
        ),
        atom_interactions::key_cap(),
        atom_motion_interactions::loading_dots(),
        atom_motion_interactions::spinner(),
        atom_motion_interactions::progress_bar(),
        atom_motion_interactions::color_swatch(),
        atom_motion_interactions::toggle(),
        atom_motion_interactions::slide_control(),
    ]
}
