use katana_ui_core::atom::{LoadingDots, Text};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{UiAnimationState, UiNode, UiVisualRole};

const HEADING_LINE_HEIGHT: u16 = 24;
const HEADING_BASELINE_OFFSET: i16 = 2;
const LOADING_SPEED_MS: u16 = 420;
const LOADING_DOT_COUNT: u8 = 4;
const ANIMATION_PHASE: u16 = 7;

#[test]
fn text_atom_keeps_role_color_and_line_metrics() {
    let node = UiNode::from(
        Text::new("日本語 Text 🔷")
            .visual_role(UiVisualRole::Content)
            .text_role("heading")
            .text_color_token("accent")
            .line_metrics(HEADING_LINE_HEIGHT, HEADING_BASELINE_OFFSET)
            .vertical_centered(true),
    );

    assert_eq!("heading", node.props().text.role);
    assert_eq!("accent", node.props().text.color_token);
    assert_eq!(HEADING_LINE_HEIGHT, node.props().text.line_height_px);
    assert_eq!(
        HEADING_BASELINE_OFFSET,
        node.props().text.baseline_offset_px
    );
    assert!(node.props().text.vertical_centered);
}

#[test]
fn loading_atom_keeps_motion_options_and_animation_state() {
    let mut loading = LoadingDots::new("Loading")
        .animation_state(UiAnimationState::Paused)
        .speed_ms(LOADING_SPEED_MS)
        .dot_count(LOADING_DOT_COUNT)
        .reduced_motion(true);
    let result = loading.apply_action(&UiAction::animation_tick(
        loading.state_id().clone(),
        ANIMATION_PHASE,
    ));
    let node = UiNode::from(loading);

    assert!(result.handled);
    assert_eq!(
        UiAnimationState::Paused,
        node.props().loading_indicator.animation_state
    );
    assert_eq!(LOADING_SPEED_MS, node.props().loading_indicator.speed_ms);
    assert_eq!(LOADING_DOT_COUNT, node.props().loading_indicator.dot_count);
    assert!(node.props().loading_indicator.reduced_motion);
    assert_eq!(ANIMATION_PHASE, node.props().interaction.animation_phase);
}
