use katana_ui_core::component::{ComponentAction, ComponentTree};
use katana_ui_core::facade::{UiCoreFacade, UiGlobalState};
use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::{
    Column, Length, Row, ScrollArea, ScrollAxis, ScrollbarVisibility, SplitPane, SplitPaneAxis,
    SplitPaneResizeMode, SplitPaneResizeSource,
};
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{
    UiNodeKind, UiScrollAreaAxis, UiScrollbarVisibility, UiSplitPaneAxis, UiSplitPaneResizeMode,
};
use katana_ui_core::theme::{ThemeId, ThemeSnapshot};
use katana_ui_core::widget::atoms::{Button, Text};

#[test]
fn generic_app_can_build_resizable_scrollable_layout_from_public_kuc_api() {
    let tree = ComponentTree::new(
        Panel::new(
            "generic workspace",
            PanelRegion::Root,
            ThemeSnapshot::dark(),
        )
        .child(
            SplitPane::new()
                .axis(SplitPaneAxis::Horizontal)
                .ratio_percent(28)
                .resize_mode(SplitPaneResizeMode::PointerAndKeyboard)
                .child(navigation_column())
                .child(content_column()),
        ),
    )
    .into_tree();
    let split = &tree.root().children()[0];
    let navigation = &split.children()[0];
    let content = &split.children()[1];

    assert_eq!(UiNodeKind::Panel, tree.root().kind());
    assert_eq!(UiNodeKind::SplitPane, split.kind());
    assert_eq!(UiSplitPaneAxis::Horizontal, split.props().split_pane.axis);
    assert_eq!(28, split.props().split_pane.ratio_percent);
    assert_eq!(
        UiSplitPaneResizeMode::PointerAndKeyboard,
        split.props().split_pane.resize_mode
    );
    assert_eq!(UiNodeKind::ScrollArea, navigation.kind());
    assert_eq!(
        UiScrollAreaAxis::Vertical,
        navigation.props().scroll_area.axis
    );
    assert_eq!(
        UiScrollbarVisibility::Auto,
        navigation.props().scroll_area.scrollbar_visibility
    );
    assert_eq!(UiNodeKind::Column, content.kind());
}

#[test]
fn generic_app_scroll_area_uses_typed_public_action_and_state() {
    let mut scroll = ScrollArea::new()
        .axis(ScrollAxis::Both)
        .viewport(120, 60)
        .content_extent(320, 180)
        .scrollbar_visibility(ScrollbarVisibility::Always);
    let action = UiAction::scroll_by(scroll.state_id().clone(), 40, 30);

    let result = scroll.apply_action(&action);

    assert!(result.handled);
    assert_eq!(40, scroll.offset_x());
    assert_eq!(30, scroll.offset_y());
}

#[test]
fn generic_app_split_pane_uses_typed_public_action_and_state() {
    let mut split = SplitPane::new()
        .ratio_percent(40)
        .min_percent(20)
        .max_percent(80)
        .resize_mode(SplitPaneResizeMode::KeyboardOnly);
    let action = UiAction::split_pane_resize_by(
        split.state_id().clone(),
        15,
        SplitPaneResizeSource::Keyboard,
    );

    let result = split.apply_action(&action);

    assert!(result.handled);
    assert_eq!(55, split.ratio_percent_value());
}

#[test]
fn generic_app_redraw_keeps_layout_state_ids_from_caller_contract() {
    let initial_scroll = ScrollArea::new()
        .stable_state_id("generic.nav.scroll")
        .axis(ScrollAxis::Both)
        .viewport(120, 60)
        .content_extent(320, 180);
    let scroll_action = UiAction::scroll_by(initial_scroll.state_id().clone(), 40, 30);
    let mut rebuilt_scroll = ScrollArea::new()
        .stable_state_id("generic.nav.scroll")
        .axis(ScrollAxis::Both)
        .viewport(120, 60)
        .content_extent(320, 180);

    let initial_split = SplitPane::new()
        .stable_state_id("generic.workspace.split")
        .ratio_percent(40)
        .min_percent(20)
        .max_percent(80);
    let split_action = UiAction::split_pane_resize_by(
        initial_split.state_id().clone(),
        10,
        SplitPaneResizeSource::Keyboard,
    );
    let mut rebuilt_split = SplitPane::new()
        .stable_state_id("generic.workspace.split")
        .ratio_percent(40)
        .min_percent(20)
        .max_percent(80);

    let scroll_result = rebuilt_scroll.apply_action(&scroll_action);
    let split_result = rebuilt_split.apply_action(&split_action);

    assert!(scroll_result.handled);
    assert!(split_result.handled);
    assert_eq!("generic.nav.scroll", rebuilt_scroll.state_id().as_str());
    assert_eq!("generic.workspace.split", rebuilt_split.state_id().as_str());
    assert_eq!(40, rebuilt_scroll.offset_x());
    assert_eq!(30, rebuilt_scroll.offset_y());
    assert_eq!(50, rebuilt_split.ratio_percent_value());
}

#[test]
fn generic_app_facade_exposes_theme_state_and_render_context() {
    let mut facade = UiCoreFacade::new(ThemeSnapshot::dark())
        .with_global_state(UiGlobalState::new(ThemeId::new("dark")));

    let change = facade.set_theme(ThemeSnapshot::light());
    let context = facade.render_context(1280.0, 720.0);

    assert_eq!("theme", change.field);
    assert_eq!("dark", change.before);
    assert_eq!("light", change.after);
    assert_eq!("light", context.theme_id.as_str());
    assert_eq!(1280.0, context.viewport_width);
    assert_eq!(720.0, context.viewport_height);
}

fn navigation_column() -> ScrollArea {
    ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(220, 720)
        .content_extent(220, 1280)
        .scrollbar_visibility(ScrollbarVisibility::Auto)
        .child(
            Column::new()
                .gap(Length::px(6.0))
                .child(Text::new("Project"))
                .child(Button::new("Open")),
        )
}

fn content_column() -> Column {
    Column::new()
        .gap(Length::px(8.0))
        .child(
            Row::new()
                .gap(Length::px(4.0))
                .child(Button::new("Run"))
                .child(Button::new("Save")),
        )
        .child(
            ScrollArea::new()
                .axis(ScrollAxis::Vertical)
                .viewport(900, 640)
                .content_extent(900, 1800)
                .child(Text::new("Editor")),
        )
}
