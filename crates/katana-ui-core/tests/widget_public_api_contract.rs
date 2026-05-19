use katana_ui_core::render_model::{UiNode, UiNodeKind, UiTree};
use katana_ui_core::widget::atoms::{
    Badge, Button, Checkbox, ColorSwatch, Divider, Icon, IconTextButton, Input, KeyCap,
    LoadingDots, ProgressBar, Radio, SlideControl, Spacer, Spinner, SvgButton, Text, TextButton,
    TextInput, Toggle,
};
use katana_ui_core::widget::molecules::{
    Accordion, Breadcrumb, Card, CodeDiff, ColorPicker, ComboBox, CommandPalette, ContextMenu,
    DynamicArrayEditor, FormField, List, Menu, MenuButton, Modal, ModalOverlay, NotificationToast,
    Popover, SearchBox, SegmentedToggle, SelectBox, SelectionList, SideMenu,
    SlideControl as MoleculeSlideControl, SplitPane, StatusBar, Tabs, Toolbar, Tooltip, TreeView,
};

#[test]
fn widget_atoms_expose_legacy_02_to_12_and_15_to_16() {
    let nodes = [
        UiNode::from(Text::new("text")),
        UiNode::from(Icon::new("icon")),
        UiNode::from(Button::new("button")),
        UiNode::from(LoadingDots::new("dots")),
        UiNode::from(Spinner::new("spinner")),
        UiNode::from(ProgressBar::new("progress")),
        UiNode::from(SvgButton::new("svg")),
        UiNode::from(TextButton::new("text button")),
        UiNode::from(IconTextButton::new("icon text")),
        UiNode::from(Checkbox::new("checkbox")),
        UiNode::from(Radio::new("radio")),
        UiNode::from(Toggle::new("toggle")),
        UiNode::from(SlideControl::new("slide")),
        UiNode::from(ColorSwatch::new("swatch")),
        UiNode::from(Input::new("input")),
        UiNode::from(TextInput::new("text input")),
        UiNode::from(Badge::new("badge")),
        UiNode::from(Divider::new("divider")),
        UiNode::from(Spacer::new("spacer")),
        UiNode::from(KeyCap::new("shortcut")),
    ];

    assert_kinds(
        &nodes,
        &[
            UiNodeKind::Text,
            UiNodeKind::Icon,
            UiNodeKind::Button,
            UiNodeKind::LoadingDots,
            UiNodeKind::Spinner,
            UiNodeKind::ProgressBar,
            UiNodeKind::SvgButton,
            UiNodeKind::TextButton,
            UiNodeKind::IconTextButton,
            UiNodeKind::Checkbox,
            UiNodeKind::Radio,
            UiNodeKind::Toggle,
            UiNodeKind::SlideControl,
            UiNodeKind::ColorSwatch,
            UiNodeKind::Input,
            UiNodeKind::Input,
            UiNodeKind::Badge,
            UiNodeKind::Divider,
            UiNodeKind::Spacer,
            UiNodeKind::KeyCap,
        ],
    );
}

#[test]
fn widget_molecules_expose_legacy_10_and_13_to_24() {
    let nodes = [
        UiNode::from(SegmentedToggle::new("segments")),
        UiNode::from(SelectBox::new("select")),
        UiNode::from(ComboBox::new("combo")),
        UiNode::from(MenuButton::new("menu button")),
        UiNode::from(CommandPalette::new("commands")),
        UiNode::from(DynamicArrayEditor::new("array")),
        UiNode::from(SearchBox::new("search")),
        UiNode::from(Tooltip::new("tooltip")),
        UiNode::from(Card::new("card")),
        UiNode::from(Accordion::new("accordion")),
        UiNode::from(SplitPane::new()),
        UiNode::from(Tabs::new("tabs")),
        UiNode::from(Breadcrumb::new("breadcrumb")),
        UiNode::from(Toolbar::new("toolbar")),
        UiNode::from(StatusBar::new("status")),
        UiNode::from(SideMenu::new("side")),
        UiNode::from(MoleculeSlideControl::new("slide molecule")),
        UiNode::from(Modal::new("modal")),
        UiNode::from(ModalOverlay::new("overlay")),
        UiNode::from(NotificationToast::new("toast")),
        UiNode::from(Popover::new("popover")),
        UiNode::from(List::new("list")),
        UiNode::from(Menu::new("menu")),
        UiNode::from(ContextMenu::new("context menu")),
        UiNode::from(FormField::new("field")),
        UiNode::from(SelectionList::new("selection")),
        UiNode::from(TreeView::new("tree")),
        UiNode::from(ColorPicker::new("picker")),
        UiNode::from(CodeDiff::new("diff")),
    ];

    assert_kinds(
        &nodes,
        &[
            UiNodeKind::SegmentedToggle,
            UiNodeKind::SelectBox,
            UiNodeKind::ComboBox,
            UiNodeKind::MenuButton,
            UiNodeKind::CommandPalette,
            UiNodeKind::DynamicArrayEditor,
            UiNodeKind::SearchBox,
            UiNodeKind::Tooltip,
            UiNodeKind::Card,
            UiNodeKind::Accordion,
            UiNodeKind::SplitPane,
            UiNodeKind::Tabs,
            UiNodeKind::Breadcrumb,
            UiNodeKind::Toolbar,
            UiNodeKind::StatusBar,
            UiNodeKind::SideMenu,
            UiNodeKind::SlideControl,
            UiNodeKind::Modal,
            UiNodeKind::ModalOverlay,
            UiNodeKind::NotificationToast,
            UiNodeKind::Popover,
            UiNodeKind::List,
            UiNodeKind::Menu,
            UiNodeKind::ContextMenu,
            UiNodeKind::FormField,
            UiNodeKind::SelectionList,
            UiNodeKind::TreeView,
            UiNodeKind::ColorPicker,
            UiNodeKind::CodeDiff,
        ],
    );
}

#[test]
fn widget_consumer_can_compose_atoms_and_molecules_without_pages() {
    let tree = UiTree::new(
        Card::new("settings")
            .child(Text::new("Theme"))
            .child(Toggle::new("Dark")),
    );

    assert_eq!(UiNodeKind::Card, tree.root().kind());
    assert_eq!(2, tree.root().children().len());
}

fn assert_kinds(nodes: &[UiNode], expected: &[UiNodeKind]) {
    let actual: Vec<UiNodeKind> = nodes.iter().map(UiNode::kind).collect();

    assert_eq!(expected, actual.as_slice());
}
