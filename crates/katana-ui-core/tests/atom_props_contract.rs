use katana_ui_core::atom::{
    Badge, Button, Checkbox, ColorSwatch, Divider, Icon, Input, KeyCap, LoadingDots, ProgressBar,
    Radio, SlideControl, Spacer, Spinner, SvgButton, Text, TextArea, Toggle,
};
use katana_ui_core::render_model::{
    UiAlignItems, UiAnimationState, UiBorder, UiCommonProps, UiCursor, UiDimension,
    UiDismissAction, UiDisplay, UiEdgeInsets, UiHostActionSpec, UiIconProps, UiJustifyContent,
    UiNode, UiNodeKind, UiPointerEvents, UiPosition, UiProgressMode, UiSize, UiSlotPlacement,
    UiSvgPaintPolicy, UiTone, UiVariant, UiVisualRole, UiZIndex,
};

const SEARCH_ICON_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" fill=\"none\" stroke=\"#FFFFFF\" stroke-width=\"1.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><circle cx=\"7\" cy=\"7\" r=\"4\"/><line x1=\"10\" y1=\"10\" x2=\"14\" y2=\"14\"/></svg>";
const CLEAR_ICON_SVG: &str = "<svg viewBox=\"0 0 16 16\"><path d=\"M4 4l8 8M12 4l-8 8\"/></svg>";

#[test]
fn input_reserved_leading_and_plain_trailing_slots_keep_typed_placement() {
    let reserved = UiNode::from(Input::new("Search").leading_slot_reserved("Search icon"));
    let trailing = UiNode::from(Input::new("Search").trailing_slot("Result count"));

    assert_eq!(
        Some(UiSlotPlacement::Leading),
        reserved
            .props()
            .text_entry
            .leading_slot
            .as_ref()
            .map(|slot| slot.placement)
    );
    assert_eq!(
        Some(UiSlotPlacement::Trailing),
        trailing
            .props()
            .text_entry
            .trailing_slot
            .as_ref()
            .map(|slot| slot.placement)
    );
}

#[test]
fn text_icon_and_keycap_carry_accessibility_and_visual_roles() {
    let text = UiNode::from(Text::new("Title").visual_role(UiVisualRole::Content));
    let icon = UiNode::from(Icon::new("Search"));
    let key_cap = UiNode::from(
        KeyCap::new("Cmd K")
            .accessibility_label("Command shortcut")
            .visual_role(UiVisualRole::Shortcut),
    );

    assert_eq!(UiVisualRole::Content, text.props().visual_role);
    assert_eq!(UiVisualRole::Icon, icon.props().visual_role);
    assert_eq!("Command shortcut", key_cap.props().accessibility_label);
    assert_eq!(UiVisualRole::Shortcut, key_cap.props().visual_role);
}

#[test]
fn button_props_are_typed_and_action_ready() {
    let node = UiNode::from(
        Button::new("Save")
            .variant(UiVariant::Filled)
            .tone(UiTone::Accent)
            .size(UiSize::Large)
            .loading(true)
            .focusable(true),
    );

    assert_eq!(UiVariant::Filled, node.props().variant);
    assert_eq!(UiTone::Accent, node.props().tone);
    assert_eq!(UiSize::Large, node.props().size);
    assert_eq!("Save", node.props().label);
    assert!(node.props().loading);
    assert!(node.props().focusable);
    assert_eq!(UiCursor::Pointer, node.props().common.cursor);
}

#[test]
fn atom_builder_exposes_the_complete_neutral_host_contract() {
    let node = UiNode::from(
        Button::new("Run")
            .theme_slot("primary")
            .visible(false)
            .width(UiDimension::Fill)
            .height(UiDimension::px(36))
            .display(UiDisplay::Inline)
            .position(UiPosition::Sticky)
            .tab_index(2)
            .z_index(UiZIndex::value(9))
            .cursor(UiCursor::Move)
            .pointer_events(UiPointerEvents::None)
            .host_action(UiHostActionSpec::command("run", "Run"))
            .command_action("stop", "Stop")
            .surface_control_action("zoom", "Zoom", "preview")
            .task_control_action("Retry", "task-7", 7)
            .selectable(true)
            .selected(true)
            .value("running")
            .font_role("button")
            .visual_role(UiVisualRole::Control)
            .variant(UiVariant::Outline)
            .tone(UiTone::Warning)
            .size(UiSize::Small)
            .loading(true)
            .readonly(true),
    );

    assert!(!node.props().common.visible);
    assert_eq!(UiDimension::Fill, node.props().common.width);
    assert_eq!(UiDisplay::Inline, node.props().common.display);
    assert_eq!(UiPosition::Sticky, node.props().common.position);
    assert_eq!(Some(2), node.props().common.tab_index);
    assert_eq!(UiZIndex::value(9), node.props().common.z_index);
    assert_eq!(UiCursor::Move, node.props().common.cursor);
    assert_eq!(UiPointerEvents::None, node.props().common.pointer_events);
    assert_eq!(4, node.props().common.host_actions.len());
    assert!(node.props().common.selectable);
    assert!(node.props().checked);
    assert_eq!("running", node.props().interaction.value);
    assert_eq!("button", node.props().font_role);
    assert_eq!(UiVisualRole::Control, node.props().visual_role);
    assert_eq!(UiVariant::Outline, node.props().variant);
    assert_eq!(UiTone::Warning, node.props().tone);
    assert!(node.props().loading);
    assert!(node.props().readonly);
}

#[test]
fn ui_node_builder_projects_surface_control_target_and_multiple_style_classes() {
    let node = UiNode::new(UiNodeKind::Button, "Run")
        .surface_control_target_id("preview")
        .style_classes(["compact".to_string(), "selected".to_string()]);

    assert_eq!(
        "preview",
        node.props().interaction.surface_control_target_id
    );
    assert_eq!(
        vec!["compact".to_string(), "selected".to_string()],
        node.props().style_classes
    );
}

#[test]
fn atoms_share_common_layout_and_interaction_props() {
    let common = UiCommonProps::default()
        .visible(false)
        .width(UiDimension::percent(100))
        .height(UiDimension::px(40))
        .border(UiBorder::solid(1, 4, "control.border"))
        .padding(UiEdgeInsets::axis(UiDimension::px(12), UiDimension::px(8)))
        .display(UiDisplay::Flex)
        .position(UiPosition::Relative)
        .align_items(UiAlignItems::Center)
        .justify_content(UiJustifyContent::Start)
        .tab_index(0)
        .z_index(UiZIndex::token("popover"))
        .cursor(UiCursor::Pointer)
        .pointer_events(UiPointerEvents::Auto)
        .selectable(true)
        .accessibility_label("Save command");
    let node = UiNode::from(
        Button::new("Save")
            .common(common)
            .disabled(true)
            .focusable(true),
    );

    assert!(!node.props().common.visible);
    assert!(node.props().common.disabled);
    assert!(node.props().common.focusable);
    assert!(node.props().common.selectable);
    assert_eq!(UiDimension::percent(100), node.props().common.width);
    assert_eq!(UiDimension::px(40), node.props().common.height);
    assert_eq!(
        UiBorder::solid(1, 4, "control.border"),
        node.props().common.border
    );
    assert_eq!(UiDisplay::Flex, node.props().common.display);
    assert_eq!(UiPosition::Relative, node.props().common.position);
    assert_eq!(Some(0), node.props().common.tab_index);
    assert_eq!(UiZIndex::token("popover"), node.props().common.z_index);
    assert_eq!(UiCursor::Pointer, node.props().common.cursor);
    assert_eq!(UiPointerEvents::Auto, node.props().common.pointer_events);
    assert_eq!("Save command", node.props().accessibility_label);
    assert!(node.props().disabled);
    assert!(node.props().focusable);
}

#[test]
fn input_props_are_typed() {
    let node = UiNode::from(
        Input::new("Search")
            .placeholder("Search command")
            .readonly(true)
            .invalid(true)
            .value("query")
            .leading_svg_icon_slot("Search icon", SEARCH_ICON_SVG)
            .trailing_svg_icon_button("Clear", CLEAR_ICON_SVG, "input.clear")
            .trailing_svg_icon_button("Match word", CLEAR_ICON_SVG, "input.match_word")
            .trailing_svg_icon_button("Match case", CLEAR_ICON_SVG, "input.match_case")
            .clear_action("Clear search")
            .visual_role(UiVisualRole::Input),
    );

    assert_eq!("Search command", node.props().placeholder);
    assert!(node.props().readonly);
    assert!(node.props().invalid);
    assert_eq!("query", node.props().interaction.value);
    assert_eq!(UiVisualRole::Input, node.props().visual_role);
    assert_eq!(
        Some(UiSlotPlacement::Leading),
        node.props()
            .text_entry
            .leading_slot
            .as_ref()
            .map(|slot| slot.placement)
    );
    assert_eq!(
        Some(true),
        node.props()
            .text_entry
            .leading_slot
            .as_ref()
            .map(|slot| slot.reserve_space)
    );
    assert_eq!(
        Some(SEARCH_ICON_SVG),
        node.props()
            .text_entry
            .leading_slot
            .as_ref()
            .and_then(|slot| slot.icon.as_ref())
            .map(|icon| icon.svg_source.as_str())
    );
    assert_eq!(
        Some("Clear"),
        node.props()
            .text_entry
            .trailing_slot
            .as_ref()
            .map(|slot| slot.label.as_str())
    );
    assert_eq!(3, node.props().text_entry.trailing_icon_buttons.len());
    assert_eq!(
        Some("input.clear"),
        node.props()
            .text_entry
            .trailing_icon_buttons
            .first()
            .and_then(|slot| slot.action.as_ref())
            .map(|action| action.callback.as_str())
    );
    assert_eq!(
        Some("input.match_case"),
        node.props()
            .text_entry
            .trailing_icon_buttons
            .get(2)
            .and_then(|slot| slot.action.as_ref())
            .map(|action| action.callback.as_str())
    );
    assert_eq!(
        Some("Clear search"),
        node.props()
            .text_entry
            .clear_action
            .as_ref()
            .map(|action| action.label.as_str())
    );
}

#[test]
fn input_text_entry_defaults_do_not_reserve_icon_space() {
    let node = UiNode::from(Input::new("Plain").value("query"));

    assert!(node.props().text_entry.leading_slot.is_none());
    assert!(node.props().text_entry.trailing_slot.is_none());
    assert!(node.props().text_entry.trailing_icon_buttons.is_empty());
}

#[test]
fn text_area_text_entry_slots_accept_external_svg_and_callbacks() {
    let node = UiNode::from(
        TextArea::new("Notes")
            .value("line 1\nline 2")
            .leading_svg_icon_slot("Search icon", SEARCH_ICON_SVG)
            .trailing_svg_icon_button("Clear", CLEAR_ICON_SVG, "notes.clear")
            .trailing_svg_icon_button("Format", CLEAR_ICON_SVG, "notes.format")
            .clear_action("Clear notes"),
    );
    let text_entry = &node.props().text_entry;

    assert_eq!(
        Some(SEARCH_ICON_SVG),
        text_entry
            .leading_slot
            .as_ref()
            .and_then(|slot| slot.icon.as_ref())
            .map(|icon| icon.svg_source.as_str())
    );
    assert_eq!(2, text_entry.trailing_icon_buttons.len());
    assert_eq!(
        Some("notes.clear"),
        text_entry
            .trailing_icon_buttons
            .first()
            .and_then(|slot| slot.action.as_ref())
            .map(|action| action.callback.as_str())
    );
    assert_eq!(
        Some("notes.format"),
        text_entry
            .trailing_icon_buttons
            .get(1)
            .and_then(|slot| slot.action.as_ref())
            .map(|action| action.callback.as_str())
    );
    assert_eq!(
        Some("Clear notes"),
        text_entry
            .clear_action
            .as_ref()
            .map(|action| action.label.as_str())
    );
}

#[test]
fn selection_and_status_props_are_typed() {
    let checkbox = UiNode::from(Checkbox::new("Enabled").checked(true));
    let radio = UiNode::from(Radio::new("Mode").selected(true));
    let toggle = UiNode::from(Toggle::new("Live").checked(true));
    let badge = UiNode::from(
        Badge::new("Ready")
            .severity(UiTone::Success)
            .variant(UiVariant::Outline),
    );
    let color = UiNode::from(ColorSwatch::new("Accent").value("#4080ff"));

    assert!(checkbox.props().checked);
    assert!(checkbox.props().interaction.has_selection);
    assert_eq!(UiCursor::Pointer, checkbox.props().common.cursor);
    assert!(checkbox.props().common.hover_border.visible);
    assert!(radio.props().checked);
    assert_eq!(UiCursor::Pointer, radio.props().common.cursor);
    assert!(radio.props().common.hover_border.visible);
    assert!(toggle.props().checked);
    assert_eq!(UiCursor::Pointer, toggle.props().common.cursor);
    assert!(toggle.props().common.hover_border.visible);
    assert_eq!(UiCursor::Pointer, color.props().common.cursor);
    assert!(color.props().common.hover_border.visible);
    assert_eq!(UiTone::Success, badge.props().severity);
    assert_eq!(UiTone::Success, badge.props().status.severity);
    assert_eq!(UiVariant::Outline, badge.props().status.variant);
    assert_eq!(UiDismissAction::None, badge.props().status.dismiss_action);
    assert_eq!("#4080ff", color.props().interaction.value);
}

#[test]
fn progress_and_loading_props_are_typed() {
    let progress = UiNode::from(
        ProgressBar::new("Sync")
            .progress(true, 42)
            .loading_label("Sync progress"),
    );
    let loading_dots =
        UiNode::from(LoadingDots::new("Loading").animation_state(UiAnimationState::Paused));
    let spinner = UiNode::from(
        Spinner::new("Loading")
            .visual_role(UiVisualRole::Loading)
            .loading(true)
            .loading_label("Loading workspace"),
    );

    assert!(progress.props().determinate);
    assert_eq!(42, progress.props().progress_percent);
    assert_eq!(
        UiProgressMode::Determinate,
        progress.props().loading_indicator.mode
    );
    assert_eq!("Sync progress", progress.props().loading_indicator.label);
    assert_eq!(UiVisualRole::Loading, loading_dots.props().visual_role);
    assert_eq!(
        UiAnimationState::Paused,
        loading_dots.props().loading_indicator.animation_state
    );
    assert_eq!(UiVisualRole::Loading, spinner.props().visual_role);
    assert_eq!("Loading workspace", spinner.props().loading_indicator.label);
    assert!(spinner.props().loading);
}

#[test]
fn structural_atom_props_do_not_need_style_classes() {
    let divider = UiNode::from(Divider::new("Section").theme_slot("separator.emphasis"));
    let slide = UiNode::from(
        SlideControl::new("Opacity")
            .value("0.8")
            .theme_slot("control.track"),
    );
    let spacer = UiNode::from(Spacer::new("Gap").theme_slot("layout.gap"));

    assert_eq!(UiNodeKind::Divider, divider.kind());
    assert_eq!(UiVisualRole::Separator, divider.props().visual_role);
    assert_eq!("separator.emphasis", divider.props().common.theme_slot);
    assert!(divider.props().style_classes.is_empty());
    assert_eq!(UiVisualRole::Control, slide.props().visual_role);
    assert_eq!("0.8", slide.props().interaction.value);
    assert_eq!("control.track", slide.props().common.theme_slot);
    assert_eq!("layout.gap", spacer.props().common.theme_slot);
}

#[test]
fn svg_icon_atoms_keep_typed_svg_props() {
    let icon = UiNode::from(
        Icon::new("Folder")
            .svg_source("<svg data-icon=\"folder\"/>")
            .icon_view_box("0 0 16 16")
            .icon_path_summary("folder outline")
            .icon_paint_policy(UiSvgPaintPolicy::StrokeOnly)
            .icon_role("directory")
            .icon_color_token("accent")
            .icon_theme_token("accent")
            .accessibility_label("Directory"),
    );
    let button = UiNode::from(
        SvgButton::new("Expand")
            .svg_icon(
                UiIconProps::new("<svg data-icon=\"chevron\"/>")
                    .view_box("0 0 12 12")
                    .path_summary("chevron right")
                    .paint_policy(UiSvgPaintPolicy::CurrentColor)
                    .role("toggle")
                    .color_token("text")
                    .theme_token("text"),
            )
            .accessibility_label("Expand directory"),
    );

    assert_eq!("<svg data-icon=\"folder\"/>", icon.props().icon.svg_source);
    assert_eq!("0 0 16 16", icon.props().icon.view_box);
    assert_eq!("folder outline", icon.props().icon.path_summary);
    assert_eq!(UiSvgPaintPolicy::StrokeOnly, icon.props().icon.paint_policy);
    assert_eq!("directory", icon.props().icon.role);
    assert_eq!("accent", icon.props().icon.color_token);
    assert_eq!("accent", icon.props().icon.theme_token);
    assert_eq!("Directory", icon.props().accessibility_label);
    assert_eq!(
        "<svg data-icon=\"chevron\"/>",
        button.props().icon.svg_source
    );
    assert_eq!("0 0 12 12", button.props().icon.view_box);
    assert_eq!("chevron right", button.props().icon.path_summary);
    assert_eq!("toggle", button.props().icon.role);
    assert_eq!("text", button.props().icon.theme_token);
    assert_eq!("Expand directory", button.props().accessibility_label);
}
