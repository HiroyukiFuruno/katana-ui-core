use super::{StoryCatalog, StoryExample};
use crate::storybook_svg_fixtures;
use katana_ui_core::atom;
use katana_ui_core::atom::{
    TextAreaAction, TextAreaCompositionPhase, TextAreaNewlineKey, TextAreaSubmitKey,
    TextAreaTabBehavior, TextAreaWrapPolicy,
};
use katana_ui_core::interaction::UiCallbackLog;

const TEXT_AREA_MIN_ROWS: u16 = 2;
const TEXT_AREA_MAX_ROWS: u16 = 4;

pub(super) fn text_area() -> StoryExample {
    let mut text_area = atom::TextArea::new("Text area")
        .value("English\n日本語 🔷")
        .placeholder("message")
        .min_rows(TEXT_AREA_MIN_ROWS)
        .max_rows(TEXT_AREA_MAX_ROWS)
        .auto_grow(true)
        .wrap_policy(TextAreaWrapPolicy::Soft)
        .submit_key(TextAreaSubmitKey::Enter)
        .newline_key(TextAreaNewlineKey::ShiftEnter)
        .tab_behavior(TextAreaTabBehavior::MoveFocus)
        .resize_enabled(false)
        .leading_svg_icon_slot("Search icon", storybook_svg_fixtures::SEARCH_SVG)
        .trailing_svg_icon_button(
            "Clear notes",
            storybook_svg_fixtures::SEARCH_SVG,
            "text_area.clear",
        );
    let target = text_area.state_id().clone();
    let typed = text_area.apply_text_area_action(TextAreaAction::Type("\nemoji 👩‍💻".to_string()));
    let ime = text_area.apply_text_area_action(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "かな",
        "かな".len(),
    ));
    let submit = text_area.apply_text_area_action(TextAreaAction::Submit);
    let newline = text_area.handle_key(katana_ui_core::atom::TextAreaKeyChord::shift_enter());
    let log = UiCallbackLog::new(
        target.clone(),
        "text_area_type",
        "rows=2 wrap=true resize=false",
        format!("typed={} ime={}", typed.events.len(), ime.events.len()),
    );
    let submit_log = UiCallbackLog::new(
        target.clone(),
        "text_area_submit",
        "submit_key=Enter",
        format!("events={}", submit.events.len()),
    );
    let newline_log = UiCallbackLog::new(
        target,
        "text_area_newline",
        "newline_key=ShiftEnter",
        format!(
            "events={}",
            newline.as_ref().map_or(0, |outcome| outcome.events.len())
        ),
    );
    StoryCatalog::interactive_story("text-area", text_area, vec![log, submit_log, newline_log])
}
