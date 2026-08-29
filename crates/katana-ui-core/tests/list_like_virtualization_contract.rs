use katana_ui_core::atom::Text;
use katana_ui_core::interaction::{RowHeightProvider, VirtualRange, VirtualizationConfig};
use katana_ui_core::molecule::{
    ChoiceItem, CommandPalette, CommandResultRow, DiagnosticId, DiagnosticItem, DiagnosticLocation,
    DiagnosticSeverity, DiagnosticsList, DiagnosticsListAction, List, SelectionList, TreeNode,
    TreeView,
};
use katana_ui_core::render_model::{UiCommonProps, UiTree};

const TOTAL_ROWS: usize = 100;
const ROW_HEIGHT: u32 = 10;
const VIEWPORT_OFFSET: u32 = 40;
const VIEWPORT_HEIGHT: u32 = 40;
const OVERSCAN: usize = 1;
const FOCUSED_ROW: usize = 80;
const LARGE_TOTAL_ROWS: usize = 10_000;

#[test]
fn shared_config_produces_same_range_without_shared_state() -> Result<(), String> {
    let config = virtual_config(false);
    let list = list_rows(TOTAL_ROWS).virtualization(config.clone());
    let selection = selection_rows(TOTAL_ROWS).virtualization(config.clone());
    let tree = tree_rows(TOTAL_ROWS).virtualization(config.clone());
    let command = command_rows(TOTAL_ROWS).virtualization(config.clone());
    let diagnostics = diagnostics_rows(TOTAL_ROWS).virtualization(config);

    let ranges = [
        list.virtual_range_model(),
        selection.virtual_range_model(),
        tree.virtual_range_model(),
        command.virtual_range_model(),
        diagnostics.virtual_range_model(),
    ];

    for range in ranges {
        let range = range
            .as_ref()
            .ok_or_else(|| "virtual range is missing".to_string())?;
        assert_range(range);
    }
    assert_ne!(
        UiTree::new(list).root().props().state_id,
        UiTree::new(selection).root().props().state_id
    );
    Ok(())
}

#[test]
fn keep_focused_row_announces_global_position() -> Result<(), String> {
    let range = tree_rows(TOTAL_ROWS)
        .virtualization(virtual_config(true))
        .virtual_range_model()
        .ok_or_else(|| "virtual range is missing".to_string())?;

    assert_eq!(Some(FOCUSED_ROW), range.focused_row.map(|it| it.index));
    assert_eq!(
        "row-80, 81 of 100",
        range.announce_row("row-80", FOCUSED_ROW)
    );
    Ok(())
}

#[test]
fn disabled_virtualization_preserves_full_list_rendering() {
    let tree = UiTree::new(list_rows(12).virtualization(VirtualizationConfig {
        enabled: false,
        ..virtual_config(false)
    }));

    assert_eq!(12, tree.root().children().len());
    assert_eq!(12, tree.root().props().interaction.item_count);
}

#[test]
fn list_public_props_expose_selection_and_virtual_range_state() {
    let tree = UiTree::new(
        list_rows(TOTAL_ROWS)
            .selected_index(1)
            .row_theme_slot("accent-row")
            .virtualization(virtual_config(true)),
    );
    let interaction = &tree.root().props().interaction;

    assert_eq!("accent-row", tree.root().props().common.theme_slot);
    assert!(interaction.has_selection);
    assert_eq!(1, interaction.selected_index);
    assert_eq!("3..9/100", interaction.value);
    assert_eq!(3, interaction.selection_start);
    assert_eq!(9, interaction.selection_end);
    assert_eq!("aria-setsize=100", interaction.dismiss_reason);
}

#[test]
fn list_common_and_children_accessors_preserve_consumer_rows() {
    let list = List::new("Rows")
        .common(UiCommonProps::default().accessibility_label("Result rows"))
        .child(Text::new("first"))
        .child(Text::new("second"));

    assert_eq!(2, list.children().len());
    let tree = UiTree::new(list);
    assert_eq!("Result rows", tree.root().props().accessibility_label);
}

#[test]
fn diagnostics_virtualized_selection_stays_item_id_based() {
    let target = DiagnosticId::new("diagnostic-80");
    let mut diagnostics = diagnostics_rows(TOTAL_ROWS).virtualization(virtual_config(true));
    let events = diagnostics.apply_action(DiagnosticsListAction::Select(target.clone()));

    assert_eq!(
        [katana_ui_core::molecule::DiagnosticsListEvent::DiagnosticSelected { id: target }],
        events.as_slice()
    );
}

#[test]
fn ten_thousand_item_molecules_keep_rendered_rows_bounded() -> Result<(), String> {
    let config = VirtualizationConfig {
        total_count: LARGE_TOTAL_ROWS,
        viewport_offset: 1_000,
        viewport_height: 200,
        overscan: 3,
        ..virtual_config(false)
    };
    let allowed_rows = (config.viewport_height / ROW_HEIGHT) as usize + config.overscan * 2;
    let ranges = [
        list_rows(LARGE_TOTAL_ROWS)
            .virtualization(config.clone())
            .virtual_range_model(),
        selection_rows(LARGE_TOTAL_ROWS)
            .virtualization(config.clone())
            .virtual_range_model(),
        tree_rows(LARGE_TOTAL_ROWS)
            .virtualization(config.clone())
            .virtual_range_model(),
        command_rows(LARGE_TOTAL_ROWS)
            .virtualization(config.clone())
            .virtual_range_model(),
        diagnostics_rows(LARGE_TOTAL_ROWS)
            .virtualization(config)
            .virtual_range_model(),
    ];

    for range in ranges {
        let range = range.ok_or_else(|| "virtual range is missing".to_string())?;
        assert_eq!(LARGE_TOTAL_ROWS, range.total_count);
        assert!(range.rows.len() <= allowed_rows);
    }
    Ok(())
}

fn assert_range(range: &VirtualRange) {
    assert_eq!(3, range.start);
    assert_eq!(9, range.end);
    assert_eq!(TOTAL_ROWS, range.total_count);
    assert_eq!(TOTAL_ROWS, range.aria_set_size);
    assert_eq!(6, range.rows.len());
}

fn virtual_config(focused: bool) -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count: TOTAL_ROWS,
        viewport_offset: VIEWPORT_OFFSET,
        viewport_height: VIEWPORT_HEIGHT,
        overscan: OVERSCAN,
        row_height_provider: RowHeightProvider::Fixed { height: ROW_HEIGHT },
        keep_focused_in_window: focused,
        focused_index: focused.then_some(FOCUSED_ROW),
    }
}

fn list_rows(count: usize) -> List {
    (0..count).fold(List::new("Rows"), |list, index| {
        list.child(Text::new(format!("row-{index}")))
    })
}

fn selection_rows(count: usize) -> SelectionList {
    (0..count).fold(SelectionList::new("Selection"), |list, index| {
        list.item(ChoiceItem::new(
            format!("choice-{index}"),
            format!("row-{index}"),
        ))
    })
}

fn tree_rows(count: usize) -> TreeView {
    (0..count).fold(TreeView::new("Tree"), |tree, index| {
        tree.item(TreeNode::new(
            format!("node-{index}"),
            format!("row-{index}"),
            0,
        ))
    })
}

fn command_rows(count: usize) -> CommandPalette {
    (0..count).fold(CommandPalette::new("Commands"), |palette, index| {
        palette.result_row(CommandResultRow::new(
            format!("command-{index}"),
            format!("row-{index}"),
        ))
    })
}

fn diagnostics_rows(count: usize) -> DiagnosticsList {
    (0..count).fold(DiagnosticsList::new("Diagnostics"), |list, index| {
        list.item(DiagnosticItem::new(
            format!("diagnostic-{index}"),
            DiagnosticSeverity::Warning,
            format!("row-{index}"),
            DiagnosticLocation::new("src/lib.rs", index as u32 + 1, 1),
        ))
    })
}
