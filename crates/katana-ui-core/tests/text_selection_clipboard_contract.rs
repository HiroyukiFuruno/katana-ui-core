use katana_ui_core::atom::{Input, Text, TextArea};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{UiRect, UiStateId};
use katana_ui_core::text_selection::{
    UiTextGlyphBox, UiTextLineBox, UiTextSelectionModel, UiTextSelectionRange,
};

fn rect(x: i32, y: i32, width: u32, height: u32) -> UiRect {
    UiRect::new(x, y, width, height)
}

fn glyph(
    text: &str,
    grapheme_index: usize,
    byte_start: usize,
    byte_end: usize,
    bounds: UiRect,
) -> UiTextGlyphBox {
    UiTextGlyphBox::new(
        grapheme_index,
        byte_start..byte_end,
        bounds,
        bounds.y + bounds.height as i32,
    )
    .with_text(text)
}

fn mixed_model() -> UiTextSelectionModel {
    UiTextSelectionModel::new(
        "A日🔷b",
        vec![UiTextLineBox::new(
            0..9,
            vec![
                glyph("A", 0, 0, 1, rect(10, 20, 7, 16)),
                glyph("日", 1, 1, 4, rect(17, 20, 18, 16)),
                glyph("🔷", 2, 4, 8, rect(35, 20, 20, 16)),
                glyph("b", 3, 8, 9, rect(55, 20, 8, 16)),
            ],
        )],
    )
}

#[test]
fn point_to_caret_uses_exact_grapheme_boundaries_for_variable_width_text() {
    let model = mixed_model();

    assert_eq!(0, model.point_to_caret(10, 28));
    assert_eq!(1, model.point_to_caret(16, 28));
    assert_eq!(2, model.point_to_caret(29, 28));
    assert_eq!(3, model.point_to_caret(48, 28));
    assert_eq!(4, model.point_to_caret(63, 28));
}

#[test]
fn drag_selection_copy_preserves_multibyte_boundaries_and_selected_glyph_rects() {
    let model = mixed_model();
    let selection = model.drag_range((18, 22), (54, 34));

    assert_eq!(UiTextSelectionRange::new(1, 3), selection);
    assert_eq!("日🔷", model.selected_text(selection));
    assert_eq!(
        vec![rect(17, 20, 18, 16), rect(35, 20, 20, 16)],
        model.highlight_rects(selection)
    );
}

#[test]
fn collapsed_selection_returns_caret_rect_on_exact_boundary() {
    let model = mixed_model();

    assert_eq!(
        rect(35, 20, 1, 16),
        model.caret_rect(UiTextSelectionRange::caret(2))
    );
}

#[test]
fn paste_replaces_selection_and_places_caret_after_inserted_text() {
    let model = mixed_model();
    let replaced = model.replace_selection(UiTextSelectionRange::new(1, 3), "文");

    assert_eq!("A文b", replaced.text);
    assert_eq!(UiTextSelectionRange::caret(2), replaced.selection);
}

#[test]
fn text_copy_action_is_allowed_but_text_paste_action_is_ignored() {
    let state_id = UiStateId::from("text.copyable");
    let mut text = Text::new("copyable")
        .stable_state_id(state_id.clone())
        .selectable(true);

    let copy = text.apply_action(&UiAction::copy_selection(state_id.clone()));
    assert!(copy.handled);
    assert_eq!("copy_selection", copy.callback_log[0].action);

    let paste = text.apply_action(&UiAction::paste_text(state_id, "ignored"));
    assert!(!paste.handled);
}

#[test]
fn input_and_text_area_paste_replace_selection_but_readonly_and_disabled_block_mutation() {
    let input_id = UiStateId::from("input.pasteable");
    let mut input = Input::new("Input")
        .stable_state_id(input_id.clone())
        .value("abcdef");
    input.apply_action(&UiAction::cursor_selection(input_id.clone(), 4, 1, 4));

    let pasted = input.apply_action(&UiAction::paste_text(input_id.clone(), "ZZ"));
    assert!(pasted.handled);
    assert_eq!("aZZef", pasted.after.value);
    assert_eq!(3, pasted.after.cursor);
    assert_eq!(3, pasted.after.selection_start);
    assert_eq!(3, pasted.after.selection_end);

    let readonly = Input::new("Readonly")
        .stable_state_id(UiStateId::from("input.readonly"))
        .value("abcdef")
        .readonly(true);
    let mut readonly = readonly;
    let readonly_paste = readonly.apply_action(&UiAction::paste_text(
        UiStateId::from("input.readonly"),
        "ZZ",
    ));
    assert!(!readonly_paste.handled);

    let disabled_id = UiStateId::from("textarea.disabled");
    let mut text_area = TextArea::new("Disabled")
        .stable_state_id(disabled_id)
        .value("abcdef")
        .disabled(true);
    let disabled_paste = text_area.apply_action(&UiAction::paste_text(
        UiStateId::from("textarea.disabled"),
        "ZZ",
    ));
    assert!(!disabled_paste.handled);
}
