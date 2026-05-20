use super::{
    UiButtonProps, UiColorSwatchProps, UiCommandResultProps, UiContextMenuProps, UiDisclosureProps,
    UiIconProps, UiInteractionState, UiLoadingProps, UiNode, UiPanelProps, UiSearchControlProps,
    UiShortcutProps, UiSize, UiSkeletonProps, UiStatusProps, UiTextAreaProps, UiTextEntryProps,
    UiTextProps, UiTone, UiTreeProps, UiVariant, UiVisualRole,
};
use crate::theme::ThemeSnapshot;

impl UiNode {
    #[must_use]
    pub fn interaction(mut self, value: UiInteractionState) -> Self {
        self.props.interaction = value;
        self
    }

    #[must_use]
    pub fn theme(mut self, value: &ThemeSnapshot) -> Self {
        self.props.theme_id = value.id.as_str().to_string();
        self
    }

    #[must_use]
    pub fn theme_id(mut self, value: impl Into<String>) -> Self {
        self.props.theme_id = value.into();
        self
    }

    #[must_use]
    pub fn font_role(mut self, value: impl Into<String>) -> Self {
        self.props.font_role = value.into();
        self
    }

    #[must_use]
    pub fn visual_role(mut self, value: UiVisualRole) -> Self {
        self.props.visual_role = value;
        self
    }

    #[must_use]
    pub fn variant(mut self, value: UiVariant) -> Self {
        self.props.variant = value;
        self
    }

    #[must_use]
    pub fn tone(mut self, value: UiTone) -> Self {
        self.props.tone = value;
        self
    }

    #[must_use]
    pub fn size(mut self, value: UiSize) -> Self {
        self.props.size = value;
        self
    }

    #[must_use]
    pub fn loading(mut self, value: bool) -> Self {
        self.props.loading = value;
        self
    }

    #[must_use]
    pub fn readonly(mut self, value: bool) -> Self {
        self.props.readonly = value;
        self
    }

    #[must_use]
    pub fn invalid(mut self, value: bool) -> Self {
        self.props.invalid = value;
        self
    }

    #[must_use]
    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.props.placeholder = value.into();
        self
    }

    #[must_use]
    pub fn checked(mut self, value: bool) -> Self {
        self.props.checked = value;
        self
    }

    #[must_use]
    pub fn progress(mut self, determinate: bool, percent: u8) -> Self {
        self.props.determinate = determinate;
        self.props.progress_percent = percent;
        self
    }

    #[must_use]
    pub fn severity(mut self, value: UiTone) -> Self {
        self.props.severity = value;
        self
    }

    #[must_use]
    pub fn text(mut self, value: UiTextProps) -> Self {
        self.props.text = value;
        self
    }

    #[must_use]
    pub fn button(mut self, value: UiButtonProps) -> Self {
        self.props.button = value;
        self
    }

    #[must_use]
    pub fn color_swatch(mut self, value: UiColorSwatchProps) -> Self {
        self.props.color_swatch = value;
        self
    }

    #[must_use]
    pub fn command_result(mut self, value: UiCommandResultProps) -> Self {
        self.props.command_result = value;
        self
    }

    #[must_use]
    pub fn shortcut(mut self, value: UiShortcutProps) -> Self {
        self.props.shortcut = value;
        self
    }

    #[must_use]
    pub fn search_control(mut self, value: UiSearchControlProps) -> Self {
        self.props.search_control = value;
        self
    }

    #[must_use]
    pub fn text_entry(mut self, value: UiTextEntryProps) -> Self {
        self.props.text_entry = value;
        self
    }

    #[must_use]
    pub fn text_area(mut self, value: UiTextAreaProps) -> Self {
        self.props.text_area = value;
        self
    }

    #[must_use]
    pub fn status(mut self, value: UiStatusProps) -> Self {
        self.props.status = value;
        self
    }

    #[must_use]
    pub fn loading_indicator(mut self, value: UiLoadingProps) -> Self {
        self.props.loading_indicator = value;
        self
    }

    #[must_use]
    pub fn skeleton(mut self, value: UiSkeletonProps) -> Self {
        self.props.skeleton = value;
        self
    }

    #[must_use]
    pub fn disclosure(mut self, value: UiDisclosureProps) -> Self {
        self.props.disclosure = value;
        self
    }

    #[must_use]
    pub fn icon(mut self, value: UiIconProps) -> Self {
        self.props.icon = value;
        self
    }

    #[must_use]
    pub fn panel(mut self, value: UiPanelProps) -> Self {
        self.props.panel = value;
        self
    }

    #[must_use]
    pub fn tree(mut self, value: UiTreeProps) -> Self {
        self.props.tree = value;
        self
    }

    #[must_use]
    pub fn context_menu(mut self, value: UiContextMenuProps) -> Self {
        self.props.context_menu = value;
        self
    }

    #[must_use]
    pub fn style_class(mut self, value: impl Into<String>) -> Self {
        self.props.style_classes.push(value.into());
        self
    }

    #[must_use]
    pub fn style_classes(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.props.style_classes.extend(values);
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }
}
