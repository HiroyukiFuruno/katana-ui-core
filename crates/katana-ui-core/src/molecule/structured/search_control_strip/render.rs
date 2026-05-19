use super::{ReplaceMode, SearchControlStrip, SearchOptionKind};
use crate::atom::{Button, Text};
use crate::molecule::SearchBox;
use crate::render_model::{
    UiNode, UiNodeKind, UiSearchControlProps, UiTextEntryProps, UiVisualRole,
};

pub(super) fn render(value: SearchControlStrip) -> UiNode {
    let result_summary = value.result_summary_model();
    let props = UiSearchControlProps {
        query: value.query.clone(),
        match_case: value.options.match_case,
        whole_word: value.options.whole_word,
        use_regex: value.options.use_regex,
        result_count: value.result_count,
        active_index: value.active_index,
        result_summary: result_summary.clone(),
        replace_mode: value.replace_mode.into(),
        replace_value: value.replace_value.clone(),
    };
    let mut node = UiNode::from_state(
        UiNodeKind::SearchControlStrip,
        value.label.clone(),
        value.state_id.clone(),
    )
    .search_control(props)
    .visual_role(UiVisualRole::Control)
    .accessibility_label("Search controls")
    .child(SearchBox::new("Search query").value(value.query.clone()))
    .child(option_button(
        SearchOptionKind::MatchCase,
        value.options.match_case,
    ))
    .child(option_button(
        SearchOptionKind::WholeWord,
        value.options.whole_word,
    ))
    .child(option_button(
        SearchOptionKind::UseRegex,
        value.options.use_regex,
    ))
    .child(Button::new("Previous result").accessibility_label("Previous search result"))
    .child(Button::new("Next result").accessibility_label("Next search result"))
    .child(Text::new(result_summary));

    if value.replace_mode != ReplaceMode::Hidden {
        node = node
            .child(replace_input(&value))
            .child(replace_button("Replace", value.replace_mode))
            .child(replace_button("Replace all", value.replace_mode));
    }
    node
}

fn option_button(kind: SearchOptionKind, enabled: bool) -> UiNode {
    let label = option_label(kind);
    UiNode::from(Button::new(label))
        .checked(enabled)
        .accessibility_label(format!("{label} search option"))
        .child(UiNode::new(UiNodeKind::Tooltip, format!("{label} option")))
}

fn replace_input(value: &SearchControlStrip) -> UiNode {
    UiNode::new(UiNodeKind::Input, "Replace value")
        .text_entry(UiTextEntryProps::default())
        .disabled(value.replace_mode == ReplaceMode::Disabled)
        .accessibility_label("Replace value")
}

fn replace_button(label: &'static str, mode: ReplaceMode) -> UiNode {
    UiNode::from(Button::new(label))
        .disabled(mode == ReplaceMode::Disabled)
        .accessibility_label(label)
}

fn option_label(kind: SearchOptionKind) -> &'static str {
    match kind {
        SearchOptionKind::MatchCase => "Match case",
        SearchOptionKind::WholeWord => "Whole word",
        SearchOptionKind::UseRegex => "Use regex",
    }
}
