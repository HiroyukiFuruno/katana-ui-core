mod atom_interactions;
mod atom_layout_story;
mod atom_motion_interactions;
mod atom_text_area_story;
use super::{StoryCatalog, StoryExample};

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
        atom_text_area_story::text_area(),
        atom_interactions::checkbox(),
        atom_interactions::radio(),
        atom_interactions::badge(),
        atom_layout_story::divider(),
        atom_layout_story::spacer(),
        atom_interactions::key_cap(),
        atom_motion_interactions::skeleton(),
        atom_motion_interactions::loading_dots(),
        atom_motion_interactions::spinner(),
        atom_motion_interactions::progress_bar(),
        atom_motion_interactions::color_swatch(),
        atom_motion_interactions::toggle(),
        atom_motion_interactions::slide_control(),
    ]
}
