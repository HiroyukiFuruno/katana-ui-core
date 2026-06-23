use katana_ui_core::atom::{
    Button, Icon, IconTextButton, ImageSurface, Input, SvgButton, Text, TextArea, TextButton,
};
use katana_ui_core::layout::{Column, Row, ScrollArea, SplitPane};
use katana_ui_core::molecule::{
    Accordion, ChoiceItem, CloseableTab, CloseableTabStrip, ComboBox, ContextMenu, ContextMenuItem,
    MenuButton, ModalOverlay, SearchBox, SelectBox, SelectionList, Toolbar,
};
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{
    UiAdapterCoveragePlan, UiImageSurfaceValidationError, UiNodeKind, UiTree,
};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn adapter_coverage_plan_reports_consumer_shell_surfaces()
-> Result<(), UiImageSurfaceValidationError> {
    let tree = consumer_shell_tree()?;

    let coverage = UiAdapterCoveragePlan::collect_from_tree(&tree);

    assert_eq!(2, coverage.input_count);
    assert_eq!(1, coverage.text_area_count);
    assert_eq!(1, coverage.tab_container_count);
    assert_eq!(4, coverage.selection_count);
    assert_eq!(1, coverage.split_pane_count);
    assert_eq!(1, coverage.scroll_area_count);
    assert_eq!(1, coverage.modal_count);
    assert_eq!(
        UiAdapterCoveragePlan::required_consumer_node_kinds().len(),
        coverage.required_consumer_node_kind_count
    );
    assert!(
        coverage.missing_required_consumer_node_kinds.is_empty(),
        "{:?}",
        coverage.missing_required_consumer_node_kinds
    );
    assert!(coverage.consumer_shell_ready());
    Ok(())
}

#[test]
fn adapter_coverage_plan_blocks_consumer_ready_when_unsupported_nodes_exist()
-> Result<(), UiImageSurfaceValidationError> {
    let tree = consumer_shell_tree()?;

    let coverage = UiAdapterCoveragePlan::collect_from_tree(&tree).with_unsupported_count(1);

    assert!(!coverage.consumer_shell_ready());
    Ok(())
}

#[test]
fn adapter_coverage_plan_blocks_consumer_ready_when_required_kind_is_missing() {
    let coverage = UiAdapterCoveragePlan::collect_from_tree(&UiTree::new(Input::new("Search")));

    assert!(!coverage.missing_required_consumer_node_kinds.is_empty());
    assert!(!coverage.consumer_shell_ready());
}

#[test]
fn adapter_coverage_plan_requires_image_surface_for_native_raster_parity() {
    assert!(
        UiAdapterCoveragePlan::required_consumer_node_kinds().contains(&UiNodeKind::ImageSurface)
    );
}

fn consumer_shell_tree() -> Result<UiTree, UiImageSurfaceValidationError> {
    Ok(UiTree::new(
        Panel::new("Consumer shell", PanelRegion::Root, ThemeSnapshot::dark()).child(
            SplitPane::new()
                .child(
                    ScrollArea::new().child(
                        SelectionList::new("Navigation").item(ChoiceItem::new("docs", "Docs")),
                    ),
                )
                .child(
                    Column::new()
                        .child(
                            CloseableTabStrip::new("Tabs").tab(CloseableTab::new("main", "Main")),
                        )
                        .child(
                            Toolbar::new("Tools")
                                .child(
                                    Icon::new("Doc").svg_source("<svg><path d=\"M2 2h8\"/></svg>"),
                                )
                                .child(Button::new("Run"))
                                .child(
                                    SvgButton::new("Find").svg_source(
                                        "<svg><circle cx=\"4\" cy=\"4\" r=\"3\"/></svg>",
                                    ),
                                )
                                .child(SearchBox::new("Quick search"))
                                .child(
                                    SelectBox::new("Workspace")
                                        .item(ChoiceItem::new("docs", "Docs")),
                                )
                                .child(
                                    ComboBox::new("Symbol").item(ChoiceItem::new("main", "main")),
                                )
                                .child(TextButton::new("Preview"))
                                .child(IconTextButton::new("Open"))
                                .child(
                                    MenuButton::new("More").item(ChoiceItem::new("copy", "Copy")),
                                )
                                .child(Text::new("Ready")),
                        )
                        .child(Input::new("Search"))
                        .child(TextArea::new("Notes"))
                        .child(ImageSurface::from_rgba(
                            "Preview",
                            "preview-fixture",
                            1,
                            1,
                            vec![0, 0, 0, 255],
                        )?)
                        .child(Row::new().child(Text::new("Ready")))
                        .child(Accordion::new("Details").child(Text::new("Body")))
                        .child(
                            ContextMenu::new("Context")
                                .item(ContextMenuItem::action("close", "Close")),
                        )
                        .child(ModalOverlay::new("Command details")),
                ),
        ),
    ))
}
