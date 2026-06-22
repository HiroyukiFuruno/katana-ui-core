use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::event::UiEvent;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::{
    Alignment, Length, OverflowBehavior, SplitPane, SplitPaneAction, SplitPaneAxis, SplitPaneEvent,
    SplitPaneResizeMode, SplitPaneResizeSource,
};
use katana_ui_core::render_model::{
    UiAlignItems, UiDimension, UiDisplay, UiJustifyContent, UiLayoutAxis, UiNodeKind, UiOverflow,
    UiSplitPaneAxis, UiSplitPaneResizeMode, UiTree,
};

#[test]
fn split_pane_render_props_expose_axis_ratio_bounds_and_handle() {
    let tree = UiTree::new(
        SplitPane::new()
            .axis(SplitPaneAxis::Vertical)
            .ratio_percent(42)
            .min_percent(20)
            .max_percent(80)
            .reset_percent(50)
            .handle_width_px(10)
            .resize_mode(SplitPaneResizeMode::PointerAndKeyboard)
            .gap(Length::px(8.0))
            .align(Alignment::Center)
            .overflow(OverflowBehavior::Scroll)
            .child(Text::new("Top"))
            .child(Text::new("Bottom")),
    );
    let props = tree.root().props();

    assert_eq!(UiNodeKind::SplitPane, tree.root().kind());
    assert_eq!(UiSplitPaneAxis::Vertical, props.split_pane.axis);
    assert_eq!(42, props.split_pane.ratio_percent);
    assert_eq!(20, props.split_pane.min_percent);
    assert_eq!(80, props.split_pane.max_percent);
    assert_eq!(50, props.split_pane.reset_percent);
    assert_eq!(10, props.split_pane.handle_width_px);
    assert_eq!(
        UiSplitPaneResizeMode::PointerAndKeyboard,
        props.split_pane.resize_mode
    );
    assert_eq!(2, tree.root().children().len());
    assert_eq!(UiDisplay::Flex, props.common.display);
    assert_eq!(UiLayoutAxis::Vertical, props.common.layout_axis);
    assert_eq!(UiDimension::Px(8), props.common.gap);
    assert_eq!(UiAlignItems::Center, props.common.align_items);
    assert_eq!(UiJustifyContent::Center, props.common.justify_content);
    assert_eq!(UiOverflow::Scroll, props.common.overflow);
}

#[test]
fn split_pane_rejects_extra_primary_panes_in_render_contract() {
    let tree = UiTree::new(
        SplitPane::new()
            .child(Text::new("One"))
            .child(Text::new("Two"))
            .child(Text::new("Three")),
    );

    assert_eq!(2, tree.root().children().len());
    assert_eq!(
        "ignored_extra_children=1",
        tree.root().props().interaction.dismiss_reason
    );
}

#[test]
fn split_pane_first_second_slots_are_the_only_primary_panes() {
    let tree = UiTree::new(
        SplitPane::new()
            .first(Text::new("Editor"))
            .second(Text::new("Preview"))
            .child(Text::new("Ignored")),
    );

    let child_labels: Vec<&str> = tree
        .root()
        .children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect();
    assert_eq!(["Editor", "Preview"], child_labels.as_slice());
    assert_eq!(
        "ignored_extra_children=1",
        tree.root().props().interaction.dismiss_reason
    );
}

#[test]
fn split_pane_resize_modes_gate_pointer_keyboard_and_reset_actions() {
    let mut pointer_only = SplitPane::new().resize_mode(SplitPaneResizeMode::PointerOnly);
    let keyboard = pointer_only.apply_action(&UiAction::split_pane_keyboard_resize(
        pointer_only.state_id().clone(),
        60,
    ));
    let pointer = pointer_only.apply_action(&UiAction::split_pane_resized(
        pointer_only.state_id().clone(),
        60,
    ));

    assert!(!keyboard.handled);
    assert!(pointer.handled);
    assert_eq!("60", pointer.after.value);

    let mut keyboard_only = SplitPane::new().resize_mode(SplitPaneResizeMode::KeyboardOnly);
    let pointer = keyboard_only.apply_action(&UiAction::split_pane_resized(
        keyboard_only.state_id().clone(),
        65,
    ));
    let keyboard = keyboard_only.apply_action(&UiAction::split_pane_keyboard_resize(
        keyboard_only.state_id().clone(),
        65,
    ));

    assert!(!pointer.handled);
    assert!(keyboard.handled);
    assert_eq!("65", keyboard.after.value);

    let mut locked = SplitPane::new().resize_mode(SplitPaneResizeMode::Disabled);
    let reset = locked.apply_action(&UiAction::split_pane_reset(locked.state_id().clone()));
    assert!(!reset.handled);
}

#[test]
fn split_pane_clamp_reset_and_drag_lifecycle_are_deterministic() {
    let mut split = SplitPane::new()
        .min_percent(30)
        .max_percent(70)
        .reset_percent(55);

    let clamped = split.apply_action(&UiAction::split_pane_resized(split.state_id().clone(), 95));
    let drag_start = split.apply_action(&UiAction::dragging(split.state_id().clone(), true));
    let drag_end = split.apply_action(&UiAction::dragging(split.state_id().clone(), false));
    let reset = split.apply_action(&UiAction::split_pane_reset(split.state_id().clone()));

    assert!(clamped.handled);
    assert_eq!("70", clamped.after.value);
    assert_eq!("clamped:95->70", clamped.after.dismiss_reason);
    assert!(drag_start.after.dragging);
    assert!(!drag_end.after.dragging);
    assert_eq!("55", reset.after.value);
}

#[test]
fn split_pane_typed_drag_sequence_emits_ordered_events() {
    let mut split = SplitPane::new().min_percent(20).max_percent(80);
    let target = split.state_id().clone();

    let events = split.apply_split_action_sequence([
        SplitPaneAction::StartResize,
        SplitPaneAction::SetRatio(96),
        SplitPaneAction::EndResize,
    ]);

    assert_eq!(
        &[
            SplitPaneEvent::ResizeStarted {
                target: target.clone()
            },
            SplitPaneEvent::RatioChanged {
                target: target.clone(),
                ratio_percent: 80,
                clamped: true,
                source: SplitPaneResizeSource::Pointer,
            },
            SplitPaneEvent::ResizeEnded { target },
        ],
        events.as_slice()
    );
    assert!(matches!(
        UiEvent::SplitPane(events[1].clone()),
        UiEvent::SplitPane(SplitPaneEvent::RatioChanged {
            ratio_percent: 80,
            clamped: true,
            ..
        })
    ));
}

#[test]
fn split_pane_keyboard_resize_uses_axis_step_and_reset_ratio() {
    let mut split = SplitPane::new()
        .axis(SplitPaneAxis::Vertical)
        .ratio_percent(40)
        .reset_percent(55);
    let target = split.state_id().clone();

    let resized = split.apply_split_action(SplitPaneAction::ResizeBy {
        delta_percent: 6,
        source: SplitPaneResizeSource::Keyboard,
    });
    let reset = split.apply_split_action(SplitPaneAction::ResetRatio);

    assert_eq!(
        vec![SplitPaneEvent::RatioChanged {
            target: target.clone(),
            ratio_percent: 46,
            clamped: false,
            source: SplitPaneResizeSource::Keyboard,
        }],
        resized
    );
    assert_eq!(
        vec![SplitPaneEvent::RatioChanged {
            target,
            ratio_percent: 55,
            clamped: false,
            source: SplitPaneResizeSource::Pointer,
        }],
        reset
    );
    assert_eq!(SplitPaneAxis::Vertical, split.axis_value());
    assert_eq!(55, split.ratio_percent_value());
}

#[test]
fn split_pane_public_contract_does_not_absorb_shell_or_sidebar_contracts() {
    let split_pane_sources = [
        include_str!("../src/layout/split_pane.rs"),
        include_str!("../src/layout/split_pane_actions.rs"),
        include_str!("../src/layout/split_pane_contract.rs"),
        include_str!("../src/render_model/typed_split_pane.rs"),
    ]
    .join("\n");

    for forbidden in [
        "AppShell",
        "CollapsiblePanel",
        "CollapsibleSidebar",
        "sidebar",
        "collapse",
        "viewer-editor",
        "storage",
        "persist",
    ] {
        assert!(
            !split_pane_sources.contains(forbidden),
            "SplitPane contract must not absorb {forbidden}"
        );
    }
}
