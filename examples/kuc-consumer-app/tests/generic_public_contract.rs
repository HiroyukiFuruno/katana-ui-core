use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{
    UiAction, UiGestureSurface, UiSurfaceGestureCapabilities, UiSurfaceGestureCommand,
    UiSurfaceGestureController, UiSurfaceGestureInput, UiSurfaceGestureOverride,
    UiSurfaceHostEvent, UiSurfacePoint,
};
use katana_ui_core::render_model::{UiNodeKind, UiRect, UiStateId};
use katana_ui_core::widget::molecules::{
    ChoiceItem, ComboBox, ContextMenu, ContextMenuAction, ContextMenuAnchor, ContextMenuEvent,
    ContextMenuItem, GenericGrid, GridCellContent, GridCoordinate, GridTrackSizeProvider,
    GridViewport, ModalOverlay, SearchBox, SelectBox,
};
use kuc_consumer_app::ConsumerApp;

#[test]
fn consumer_app_public_shell_contains_real_toolbar_controls() {
    let app = ConsumerApp::new();
    let tree = app.render();
    let split = &tree.root().children()[0];
    let content = &split.children()[1];
    let toolbar = &content.children()[1];
    let toolbar_kinds = toolbar
        .children()
        .iter()
        .map(|node| node.kind())
        .collect::<Vec<_>>();

    assert_eq!(UiNodeKind::Toolbar, toolbar.kind());
    assert_eq!(
        vec![
            UiNodeKind::Icon,
            UiNodeKind::Input,
            UiNodeKind::SearchBox,
            UiNodeKind::SelectBox,
            UiNodeKind::ComboBox,
            UiNodeKind::SvgButton,
            UiNodeKind::TextButton,
            UiNodeKind::IconTextButton,
            UiNodeKind::MenuButton,
            UiNodeKind::Button,
            UiNodeKind::Button,
        ],
        toolbar_kinds
    );
    assert_eq!("Search files", toolbar.children()[1].props().placeholder);
    assert_eq!("main", toolbar.children()[2].props().interaction.value);
    assert_eq!(3, toolbar.children()[3].props().interaction.item_count);
    assert_eq!(3, toolbar.children()[4].props().interaction.item_count);
}

#[test]
fn public_selection_controls_emit_ordered_state_action_contract() {
    let mut quick_search = SearchBox::new("Quick search")
        .value("main")
        .submit_on_enter(true);
    let mut workspace = SelectBox::new("Workspace")
        .item(ChoiceItem::new("source", "Source"))
        .item(ChoiceItem::new("tests", "Tests"))
        .item(ChoiceItem::new("docs", "Docs"))
        .selected_index(0)
        .open(true);
    let mut symbol = ComboBox::new("Symbol")
        .item(ChoiceItem::new("main", "main"))
        .item(ChoiceItem::new("render", "render"))
        .item(ChoiceItem::new("tests", "tests"))
        .input_value("ma")
        .free_input(true)
        .open(true);

    let quick_input = quick_search.apply_action(&UiAction::input_value(
        quick_search.state_id().clone(),
        "render",
    ));
    let quick_submit =
        quick_search.apply_action(&UiAction::search_submitted(quick_search.state_id().clone()));
    let workspace_select = workspace.apply_action(&UiAction::select_box_selected(
        workspace.state_id().clone(),
        2,
    ));
    let symbol_input =
        symbol.apply_action(&UiAction::input_value(symbol.state_id().clone(), "ren"));
    let symbol_select =
        symbol.apply_action(&UiAction::select_box_selected(symbol.state_id().clone(), 1));
    let action_names = [
        &quick_input,
        &quick_submit,
        &workspace_select,
        &symbol_input,
        &symbol_select,
    ]
    .into_iter()
    .map(|result| result.callback_log[0].action.as_str())
    .collect::<Vec<_>>();

    assert_eq!(
        vec![
            "input_value",
            "search_submitted",
            "select_box_selected",
            "input_value",
            "select_box_selected",
        ],
        action_names
    );
    assert_eq!("render", quick_submit.after.value);
    assert_eq!("docs", workspace_select.after.value);
    assert_eq!("render", symbol_select.after.value);
    assert!(!workspace_select.after.open);
    assert!(!symbol_select.after.open);
}

#[test]
fn public_overlay_and_context_menu_emit_lifecycle_event_contract() {
    let mut overlay = ModalOverlay::new("Confirm")
        .open(true)
        .focus_return("editor")
        .dismiss_policy("escape")
        .escape_dismiss(true)
        .outside_click_dismiss(true);
    let dismissed = overlay.apply_action(&UiAction::modal_escape(overlay.state_id().clone()));

    assert!(dismissed.handled);
    assert!(!dismissed.after.open);
    assert_eq!("modal_escape", dismissed.callback_log[0].action);
    assert_eq!("focus_return=editor", dismissed.after.value);

    let mut menu = ContextMenu::new("Actions")
        .item(ContextMenuItem::action("close", "Close"))
        .item(ContextMenuItem::action("copy", "Copy path"));
    menu.apply_context_action(&ContextMenuAction::Open {
        anchor: ContextMenuAnchor::NodeId("toolbar.more".to_string()),
    });
    menu.apply_context_action(&ContextMenuAction::Highlight { path: vec![1] });
    let selected = menu.apply_context_action(&ContextMenuAction::Activate { path: vec![1] });
    let event_names = menu
        .callback_log()
        .iter()
        .map(ContextMenuEvent::name)
        .collect::<Vec<_>>();

    assert_eq!(
        ContextMenuEvent::ItemSelected {
            path: vec![1],
            command: "copy".to_string(),
        },
        selected
    );
    assert_eq!(
        vec![
            "context_menu_opened",
            "context_menu_item_highlighted",
            "context_menu_item_selected",
            "context_menu_closed",
        ],
        event_names
    );
}

#[test]
fn public_grid_contract_bounds_one_hundred_thousand_cell_model()
-> Result<(), Box<dyn std::error::Error>> {
    let grid = GenericGrid::new("Data", 1_000, 100)
        .row_tracks(GridTrackSizeProvider::fixed(24))
        .column_tracks(GridTrackSizeProvider::fixed(96))
        .viewport(GridViewport::new(480, 240).scroll(288, 2_400))
        .overscan(2, 1)
        .frozen(1, 1)
        .active_cell(GridCoordinate::new(100, 3));
    let coordinates = grid.visible_coordinates();
    let content = coordinates
        .iter()
        .copied()
        .map(|coordinate| {
            GridCellContent::new(
                coordinate,
                format!("r{}c{}", coordinate.row, coordinate.column),
            )
        })
        .collect::<Vec<_>>();
    let rendered = katana_ui_core::render_model::UiNode::from(grid.with_visible_cells(content)?);

    assert_eq!(UiNodeKind::Grid, rendered.kind());
    assert_eq!(1_000, rendered.props().grid.row_count);
    assert_eq!(100, rendered.props().grid.column_count);
    assert_eq!(288, rendered.props().grid.viewport.scroll_x);
    assert_eq!(2_400, rendered.props().grid.viewport.scroll_y);
    assert!(coordinates.len() < 150);
    assert_eq!(coordinates.len(), rendered.props().grid.cells.len());
    assert!(rendered.props().grid.validate().is_ok());
    Ok(())
}

#[test]
fn public_typed_gesture_contract_needs_no_consumer_string_or_geometry_parsing() {
    let target = UiStateId::new("opaque-surface");
    let capabilities = UiSurfaceGestureCapabilities::default()
        .pointer_pan(true)
        .smooth_scroll_pan(true)
        .zoom(true)
        .fullscreen(true);
    let mut controller = UiSurfaceGestureController::new([UiGestureSurface::new(
        target.clone(),
        UiRect::new(10, 20, 100, 80),
    )
    .capabilities(capabilities)]);

    let down = controller.apply(UiSurfaceGestureInput::PointerDown {
        pointer_id: 7,
        position: UiSurfacePoint::new(30, 40),
    });
    assert!(down.captured);
    assert_eq!(down.target.as_ref(), Some(&target));

    let moved = controller.apply(UiSurfaceGestureInput::PointerMove {
        pointer_id: 7,
        position: UiSurfacePoint::new(34, 47),
    });
    assert_eq!(
        moved.command,
        Some(UiSurfaceGestureCommand::PanBy {
            delta_x: 4.0,
            delta_y: 7.0,
        })
    );

    let zoomed = controller.apply_with_override(
        UiSurfaceGestureInput::Zoom {
            multiplier: 1.25,
            center: UiSurfacePoint::new(34, 47),
        },
        |event| match event.input {
            UiSurfaceGestureInput::Zoom { center, .. } => {
                UiSurfaceGestureOverride::Command(UiSurfaceGestureCommand::ZoomBy {
                    multiplier: 2.0,
                    center,
                })
            }
            _ => UiSurfaceGestureOverride::UseDefault,
        },
    );
    assert!(zoomed.captured);
    assert_eq!(
        zoomed.command,
        Some(UiSurfaceGestureCommand::ZoomBy {
            multiplier: 2.0,
            center: UiSurfacePoint::new(34, 47),
        })
    );
    assert_eq!(
        controller.set_fullscreen(&target, true),
        Some(UiSurfaceHostEvent::FullscreenChanged {
            target,
            fullscreen: true,
        })
    );
}
