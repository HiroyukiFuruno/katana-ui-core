use katana_ui_core::atom::{Button, IconTextButton, SvgButton, TextButton};
use katana_ui_core::render_model::{
    UiButtonLayoutDto, UiButtonLayoutPatchDto, UiButtonLayoutPreset, UiButtonLayoutSpec, UiCursor,
    UiNode, UiVariant, UiVisualRole,
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
fn button_atom_variants_default_to_pointer_cursor() {
    let cases = [
        ("button", UiNode::from(Button::new("Save"))),
        ("text-button", UiNode::from(TextButton::new("Save"))),
        ("svg-button", UiNode::from(SvgButton::new("Search"))),
        (
            "icon-text-button",
            UiNode::from(IconTextButton::new("Search")),
        ),
    ];

    for (name, node) in cases {
        assert_eq!(UiCursor::Pointer, node.props().common.cursor, "{name}");
    }
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
fn button_layout_spec_constructors_resolve_custom_default_and_from_contracts() {
    let custom = custom_layout();
    assert_eq!(
        UiButtonLayoutPreset::Classic.to_dto(),
        UiButtonLayoutSpec::preset(UiButtonLayoutPreset::Classic).resolve()
    );
    assert_eq!(custom, UiButtonLayoutSpec::custom(custom.clone()).resolve());
    assert_eq!(
        UiButtonLayoutPreset::Modern.to_dto(),
        UiButtonLayoutSpec::default().resolve()
    );
    assert_eq!(custom, UiButtonLayoutSpec::from(custom.clone()).resolve());
}

#[test]
fn button_layout_label_align_center_is_part_of_core_dto_contract() {
    let default_button = UiNode::from(Button::new("Save"));
    let left_patch = UiNode::from(Button::new("Save").layout_patch(
        UiButtonLayoutPreset::Modern,
        UiButtonLayoutPatchDto::default().with_label_align("left"),
    ));
    let right_custom =
        UiNode::from(Button::new("Save").layout(custom_layout().with_label_align("right")));

    assert_eq!("center", default_button.props().button.layout.label_align);
    assert_eq!("left", left_patch.props().button.layout.label_align);
    assert_eq!("right", right_custom.props().button.layout.label_align);
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

#[test]
fn button_layout_dto_and_patch_cover_every_public_field() {
    let dto = UiButtonLayoutDto::from_preset(UiButtonLayoutPreset::Basic)
        .with_min_size(101, 31)
        .with_width_auto()
        .with_padding(12, 7)
        .with_border(2, 5)
        .with_icon_gap(9)
        .with_label_align("left");
    assert_eq!(101, dto.min_width);
    assert_eq!(31, dto.min_height);
    assert_eq!("auto", dto.width_mode);
    assert_eq!(0, dto.width_value);
    assert_eq!(12, dto.padding_x);
    assert_eq!(7, dto.padding_y);
    assert_eq!(2, dto.border_width);
    assert_eq!(5, dto.radius);
    assert_eq!(9, dto.icon_gap);
    assert_eq!("left", dto.label_align);
    assert_eq!(
        UiButtonLayoutPreset::Modern.to_dto(),
        UiButtonLayoutDto::default()
    );

    let patched = UiButtonLayoutPatchDto::default()
        .with_min_size(110, 32)
        .with_width_percent(80)
        .with_padding(14, 8)
        .with_border(3, 6)
        .with_icon_gap(10)
        .with_label_align("right")
        .apply_to(UiButtonLayoutDto::default());
    assert_eq!(110, patched.min_width);
    assert_eq!(32, patched.min_height);
    assert_eq!("percent", patched.width_mode);
    assert_eq!(80, patched.width_value);
    assert_eq!(14, patched.padding_x);
    assert_eq!(8, patched.padding_y);
    assert_eq!(3, patched.border_width);
    assert_eq!(6, patched.radius);
    assert_eq!(10, patched.icon_gap);
    assert_eq!("right", patched.label_align);

    let auto = UiButtonLayoutPatchDto::default()
        .with_width_auto()
        .apply_to(patched.clone());
    assert_eq!(("auto", 0), (auto.width_mode.as_str(), auto.width_value));
    let fill = UiButtonLayoutPatchDto::default()
        .with_width_fill()
        .apply_to(patched);
    assert_eq!(("fill", 0), (fill.width_mode.as_str(), fill.width_value));
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
