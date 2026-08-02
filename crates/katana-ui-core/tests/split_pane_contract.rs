use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::event::UiEvent;
use katana_ui_core::interaction::{UiAction, UiActionSource};
use katana_ui_core::layout::{
    Alignment, Length, OverflowBehavior, SplitPane, SplitPaneAction, SplitPaneAxis, SplitPaneEvent,
    SplitPaneOptions, SplitPaneResizeMode, SplitPaneResizeSource,
};
use katana_ui_core::render_model::{
    UiAlignItems, UiDimension, UiDisplay, UiJustifyContent, UiLayoutAxis, UiNodeKind, UiOverflow,
    UiSplitPaneAxis, UiSplitPaneResizeMode, UiStateId, UiTree,
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
fn split_pane_second_slot_materializes_a_stable_empty_first_slot() {
    let tree = UiTree::new(SplitPane::new().second(Text::new("Second")));

    assert_eq!(2, tree.root().children().len());
    assert_eq!(UiNodeKind::Spacer, tree.root().children()[0].kind());
    assert_eq!("Second", tree.root().children()[1].props().label);
}

#[test]
fn split_pane_default_and_value_accessors_round_trip() {
    let split = SplitPane::default()
        .stable_state_id("stable-split")
        .axis(SplitPaneAxis::Vertical)
        .ratio_percent(44)
        .min_percent(12)
        .max_percent(88)
        .handle_width_px(9)
        .reset_percent(50)
        .resize_mode(SplitPaneResizeMode::KeyboardOnly)
        .overflow(OverflowBehavior::Hidden)
        .child(Text::new("First"));

    assert_eq!("stable-split", split.state_id().as_str());
    assert_eq!(SplitPaneAxis::Vertical, split.axis_value());
    assert_eq!(44, split.ratio_percent_value());
    assert_eq!(12, split.min_percent_value());
    assert_eq!(88, split.max_percent_value());
    assert_eq!(9, split.handle_width_px_value());
    assert_eq!(50, split.reset_percent_value());
    assert_eq!(SplitPaneResizeMode::KeyboardOnly, split.resize_mode_value());
    assert_eq!(OverflowBehavior::Hidden, split.overflow_value());
    assert_eq!(1, split.children().len());
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
fn split_pane_typed_action_builders_preserve_payloads_and_lifecycle() {
    let target = katana_ui_core::render_model::UiStateId::new("split");

    assert!(matches!(
        UiAction::split_pane_set_ratio(target.clone(), 45),
        UiAction::SplitPaneSetRatio {
            target: actual,
            ratio_percent: 45,
        } if actual == target
    ));
    assert!(matches!(
        UiAction::split_pane_reset_ratio(target.clone()),
        UiAction::SplitPaneResetRatio { target: actual } if actual == target
    ));
    assert!(matches!(
        UiAction::split_pane_start_resize(target.clone()),
        UiAction::SplitPaneStartResize { target: actual } if actual == target
    ));
    assert!(matches!(
        UiAction::split_pane_end_resize(target.clone()),
        UiAction::SplitPaneEndResize { target: actual } if actual == target
    ));
    assert!(matches!(
        UiAction::scroll_into_view(
            target.clone(),
            katana_ui_core::render_model::UiRect::new(1, 2, 3, 4),
        ),
        UiAction::ScrollIntoView {
            target: actual,
            target_rect,
        } if actual == target
            && target_rect == katana_ui_core::render_model::UiRect::new(1, 2, 3, 4)
    ));
}

#[test]
fn split_pane_resize_modes_emit_typed_source_and_disabled_rejections() {
    let mut pointer_only = SplitPane::new().resize_mode(SplitPaneResizeMode::PointerOnly);
    let keyboard_rejected = pointer_only.apply_split_action(SplitPaneAction::ResizeBy {
        delta_percent: 5,
        source: SplitPaneResizeSource::Keyboard,
    });
    assert!(matches!(
        keyboard_rejected.as_slice(),
        [SplitPaneEvent::ResizeRejected {
            reason: katana_ui_core::layout::SplitPaneRejectionReason::SourceNotAllowed,
            ..
        }]
    ));
    assert!(matches!(
        pointer_only
            .apply_split_action(SplitPaneAction::StartResize)
            .as_slice(),
        [SplitPaneEvent::ResizeStarted { .. }]
    ));
    assert!(matches!(
        pointer_only
            .apply_split_action(SplitPaneAction::EndResize)
            .as_slice(),
        [SplitPaneEvent::ResizeEnded { .. }]
    ));

    let mut keyboard_only = SplitPane::new().resize_mode(SplitPaneResizeMode::KeyboardOnly);
    assert!(matches!(
        keyboard_only
            .apply_split_action(SplitPaneAction::StartResize)
            .as_slice(),
        [SplitPaneEvent::ResizeRejected {
            reason: katana_ui_core::layout::SplitPaneRejectionReason::SourceNotAllowed,
            ..
        }]
    ));
    assert!(matches!(
        keyboard_only
            .apply_split_action(SplitPaneAction::ResizeBy {
                delta_percent: -5,
                source: SplitPaneResizeSource::Keyboard,
            })
            .as_slice(),
        [SplitPaneEvent::RatioChanged {
            source: SplitPaneResizeSource::Keyboard,
            ..
        }]
    ));

    let mut disabled = SplitPane::new().resize_mode(SplitPaneResizeMode::Disabled);
    for action in [
        SplitPaneAction::StartResize,
        SplitPaneAction::ResetRatio,
        SplitPaneAction::SetRatio(80),
    ] {
        assert!(matches!(
            disabled.apply_split_action(action).as_slice(),
            [SplitPaneEvent::ResizeRejected {
                reason: katana_ui_core::layout::SplitPaneRejectionReason::ResizeDisabled,
                ..
            }]
        ));
    }
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
fn split_pane_second_slot_inserts_stable_empty_first_pane() {
    let tree = UiTree::new(SplitPane::new().second(Text::new("Preview")));

    assert_eq!(2, tree.root().children().len());
    assert_eq!(UiNodeKind::Spacer, tree.root().children()[0].kind());
    assert_eq!("Preview", tree.root().children()[1].props().label);
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
fn split_pane_component_action_covers_typed_lifecycle_hover_and_rejections() {
    let mut split = SplitPane::new().resize_mode(SplitPaneResizeMode::PointerAndKeyboard);
    let target = split.state_id().clone();
    assert!(!split.apply_action(&UiAction::focus(target.clone())).handled);

    for action in [
        UiAction::split_pane_set_ratio(target.clone(), 40),
        UiAction::split_pane_resize_by(target.clone(), 5, SplitPaneResizeSource::Keyboard),
        UiAction::split_pane_reset_ratio(target.clone()),
        UiAction::split_pane_start_resize(target.clone()),
        UiAction::split_pane_end_resize(target.clone()),
        UiAction::hover(target.clone(), true),
    ] {
        assert!(split.apply_action(&action).handled, "{}", action.name());
    }

    let wrong_target =
        split.apply_action(&UiAction::split_pane_set_ratio(UiStateId::new("other"), 60));
    let invalid_ratio = split.apply_action(&UiAction::SetValue {
        target: target.clone(),
        value: "invalid".to_string(),
        source: UiActionSource::SplitPane,
        progress: None,
        color_drag: None,
    });
    assert!(!wrong_target.handled);
    assert!(!invalid_ratio.handled);

    let mut disabled = SplitPane::new().resize_mode(SplitPaneResizeMode::Disabled);
    let dragging = disabled.apply_action(&UiAction::dragging(disabled.state_id().clone(), true));
    assert!(!dragging.handled);
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

#[test]
fn split_pane_options_value_and_legacy_value_cover_all_render_modes() {
    let options = SplitPaneOptions {
        axis: SplitPaneAxis::Vertical,
        ratio_percent: 90,
        min_percent: 20,
        max_percent: 80,
        reset_percent: 5,
        handle_width_px: 3,
        resize_mode: SplitPaneResizeMode::PointerOnly,
        overflow: OverflowBehavior::Fit,
    };
    let split = SplitPane::new().options(options);
    let resolved = split.options_value();

    assert_eq!(SplitPaneAxis::Vertical, resolved.axis);
    assert_eq!(80, resolved.ratio_percent);
    assert_eq!(20, resolved.min_percent);
    assert_eq!(80, resolved.max_percent);
    assert_eq!(20, resolved.reset_percent);
    assert_eq!(3, resolved.handle_width_px);
    assert_eq!(SplitPaneResizeMode::PointerOnly, resolved.resize_mode);
    assert_eq!(OverflowBehavior::Fit, resolved.overflow);

    let rendered = UiTree::new(split);
    assert_eq!(
        UiSplitPaneResizeMode::PointerOnly,
        rendered.root().props().split_pane.resize_mode
    );
    assert_eq!(
        UiLayoutAxis::Vertical,
        rendered.root().props().common.layout_axis
    );

    let valid = SplitPane::new().min_percent(20).max_percent(80).value("75");
    assert_eq!(75, valid.ratio_percent_value());
    let invalid = valid.value("not-a-ratio");
    assert_eq!(75, invalid.ratio_percent_value());
    let invalid_node = UiTree::new(invalid);
    assert_eq!("not-a-ratio", invalid_node.root().props().interaction.value);

    let keyboard_only =
        UiTree::new(SplitPane::new().resize_mode(SplitPaneResizeMode::KeyboardOnly));
    let disabled = UiTree::new(SplitPane::new().resize_mode(SplitPaneResizeMode::Disabled));
    assert_eq!(
        UiSplitPaneResizeMode::KeyboardOnly,
        keyboard_only.root().props().split_pane.resize_mode
    );
    assert_eq!(
        UiSplitPaneResizeMode::Disabled,
        disabled.root().props().split_pane.resize_mode
    );
}
