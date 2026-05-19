use katana_ui_core::atom::{Button, IconTextButton, SvgButton, TextButton};
use katana_ui_core::render_model::{
    UiButtonLayoutDto, UiButtonLayoutPatchDto, UiButtonLayoutPreset, UiButtonLayoutSpec, UiNode,
    UiVariant, UiVisualRole,
};

const CUSTOM_WIDTH: u16 = 144;
const CUSTOM_HEIGHT: u16 = 40;
const CUSTOM_PERCENT_WIDTH: u16 = 72;
const CUSTOM_PADDING_X: u16 = 22;
const CUSTOM_PADDING_Y: u16 = 10;
const CUSTOM_BORDER_WIDTH: u16 = 3;
const CUSTOM_RADIUS: u16 = 9;
const CUSTOM_ICON_GAP: u16 = 11;

#[test]
fn specialized_button_atoms_render_as_controls_by_default() {
    let svg_button = UiNode::from(SvgButton::new("Open"));
    let text_button = UiNode::from(TextButton::new("Cancel"));
    let icon_text_button = UiNode::from(IconTextButton::new("Open folder"));

    assert_eq!(UiVisualRole::Control, svg_button.props().visual_role);
    assert_eq!(UiVisualRole::Control, text_button.props().visual_role);
    assert_eq!(UiVisualRole::Control, icon_text_button.props().visual_role);
    assert_eq!(UiVariant::Icon, svg_button.props().variant);
    assert_eq!(UiVariant::Text, text_button.props().variant);
    assert_eq!(UiVariant::IconText, icon_text_button.props().variant);
}

#[test]
fn button_layout_uses_dto_as_the_customization_contract() {
    let default_button = UiNode::from(Button::new("Save"));
    let classic_button =
        UiNode::from(Button::new("Save").layout_preset(UiButtonLayoutPreset::Classic));
    let partial_override = UiNode::from(
        Button::new("Save").layout_from_preset(UiButtonLayoutPreset::Basic, |layout| {
            layout.with_padding(CUSTOM_PADDING_X, CUSTOM_PADDING_Y)
        }),
    );
    let full_override = UiNode::from(Button::new("Save").layout(custom_layout()));

    assert_eq!(
        UiButtonLayoutPreset::Modern.to_dto(),
        default_button.props().button.layout
    );
    assert_eq!("auto", default_button.props().button.layout.width_mode);
    assert_eq!(0, default_button.props().button.layout.width_value);
    assert_eq!(
        UiButtonLayoutPreset::Classic.to_dto(),
        classic_button.props().button.layout
    );
    assert_eq!(
        CUSTOM_PADDING_X,
        partial_override.props().button.layout.padding_x
    );
    assert_eq!(
        CUSTOM_PADDING_Y,
        partial_override.props().button.layout.padding_y
    );
    assert_eq!(CUSTOM_WIDTH, full_override.props().button.layout.min_width);
    assert_eq!(CUSTOM_RADIUS, full_override.props().button.layout.radius);
}

#[test]
fn button_layout_accepts_flexible_spec_entrypoints() {
    let patch = UiButtonLayoutPatchDto::default()
        .with_min_size(CUSTOM_WIDTH, CUSTOM_HEIGHT)
        .with_border(CUSTOM_BORDER_WIDTH, CUSTOM_RADIUS);
    let preset_spec = UiNode::from(Button::new("Save").layout_spec(UiButtonLayoutPreset::Dense));
    let patch_spec = UiNode::from(Button::new("Save").layout_spec(
        UiButtonLayoutSpec::preset_patch(UiButtonLayoutPreset::Dense, patch),
    ));
    let patch_shortcut = UiNode::from(Button::new("Save").layout_patch(
        UiButtonLayoutPreset::Classic,
        UiButtonLayoutPatchDto::default().with_icon_gap(CUSTOM_ICON_GAP),
    ));

    assert_eq!(
        UiButtonLayoutPreset::Dense.to_dto(),
        preset_spec.props().button.layout
    );
    assert_eq!(CUSTOM_WIDTH, patch_spec.props().button.layout.min_width);
    assert_eq!(
        UiButtonLayoutPreset::Dense.to_dto().padding_x,
        patch_spec.props().button.layout.padding_x
    );
    assert_eq!(
        CUSTOM_ICON_GAP,
        patch_shortcut.props().button.layout.icon_gap
    );
    assert_eq!(
        UiButtonLayoutPreset::Classic.to_dto().min_width,
        patch_shortcut.props().button.layout.min_width
    );
}

#[test]
fn button_layout_width_mode_is_part_of_the_core_dto_contract() {
    let px = UiNode::from(Button::new("Save").layout(custom_layout().with_width_px(CUSTOM_WIDTH)));
    let percent = UiNode::from(
        Button::new("Save").layout(custom_layout().with_width_percent(CUSTOM_PERCENT_WIDTH)),
    );
    let fill = UiNode::from(Button::new("Save").layout(custom_layout().with_width_fill()));
    let patch = UiNode::from(Button::new("Save").layout_patch(
        UiButtonLayoutPreset::Modern,
        UiButtonLayoutPatchDto::default().with_width_px(CUSTOM_WIDTH),
    ));

    assert_eq!("px", px.props().button.layout.width_mode);
    assert_eq!(CUSTOM_WIDTH, px.props().button.layout.width_value);
    assert_eq!("percent", percent.props().button.layout.width_mode);
    assert_eq!(
        CUSTOM_PERCENT_WIDTH,
        percent.props().button.layout.width_value
    );
    assert_eq!("fill", fill.props().button.layout.width_mode);
    assert_eq!(0, fill.props().button.layout.width_value);
    assert_eq!("px", patch.props().button.layout.width_mode);
    assert_eq!(CUSTOM_WIDTH, patch.props().button.layout.width_value);
}

fn custom_layout() -> UiButtonLayoutDto {
    UiButtonLayoutDto::new(
        CUSTOM_WIDTH,
        CUSTOM_HEIGHT,
        CUSTOM_PADDING_X,
        CUSTOM_PADDING_Y,
        CUSTOM_BORDER_WIDTH,
        CUSTOM_RADIUS,
        CUSTOM_ICON_GAP,
    )
}
