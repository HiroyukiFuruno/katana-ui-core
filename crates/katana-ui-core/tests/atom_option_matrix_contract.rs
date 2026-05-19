use katana_ui_core::atom::{Badge, ColorSwatch, IconTextButton, Input, KeyCap};
use katana_ui_core::render_model::UiNode;

const PALETTE_COLOR_COUNT: usize = 2;

#[test]
fn button_input_color_badge_and_keycap_keep_required_options() {
    let button = UiNode::from(
        IconTextButton::new("Open")
            .command("open-file")
            .keyboard_activation(true)
            .icon_position("leading"),
    );
    let input = UiNode::from(
        Input::new("Search")
            .submit_on_enter(true)
            .ime_enabled(true)
            .emoji_enabled(true),
    );
    let color = UiNode::from(
        ColorSwatch::new("Accent")
            .palette_color("accent")
            .palette_color("danger")
            .selected_color("accent"),
    );
    let badge = UiNode::from(Badge::new("Ready").leading_icon("<svg data-icon=\"check\"/>"));
    let key_cap = UiNode::from(KeyCap::new("Cmd K").platform("macos").combo("cmd+k"));

    assert_eq!("open-file", button.props().button.command);
    assert!(button.props().button.keyboard_activation);
    assert_eq!("leading", button.props().button.icon_position);
    assert!(input.props().text_entry.submit_on_enter);
    assert!(input.props().text_entry.ime_enabled);
    assert!(input.props().text_entry.emoji_enabled);
    assert_eq!(
        PALETTE_COLOR_COUNT,
        color.props().color_swatch.palette.len()
    );
    assert_eq!("accent", color.props().color_swatch.selected_color);
    assert_eq!("accent", color.props().interaction.value);
    assert_eq!(
        "<svg data-icon=\"check\"/>",
        badge.props().status.leading_icon
    );
    assert_eq!("macos", key_cap.props().shortcut.platform);
    assert_eq!("cmd+k", key_cap.props().shortcut.combo);
}
