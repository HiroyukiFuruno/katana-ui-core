use katana_ui_core::interaction::{
    RowHeightOverride, RowHeightProvider, UiCallbackLog, VirtualRange, VirtualizationConfig,
};
use katana_ui_core::render_model::UiStateId;

pub(super) const LIST_TOTAL_COUNT: usize = 128;
pub(super) const SELECTION_TOTAL_COUNT: usize = 96;
pub(super) const TREE_TOTAL_COUNT: usize = 240;
pub(super) const COMMAND_TOTAL_COUNT: usize = 5;
pub(super) const DIAGNOSTIC_TOTAL_COUNT: usize = 64;
const FIXED_VIEWPORT_OFFSET_PX: u32 = 56;
const FIXED_VIEWPORT_HEIGHT_PX: u32 = 112;
const FIXED_OVERSCAN_ROWS: usize = 2;
const FIXED_ROW_HEIGHT_PX: u32 = 28;
const VARIABLE_VIEWPORT_OFFSET_PX: u32 = 60;
const VARIABLE_VIEWPORT_HEIGHT_PX: u32 = 120;
const VARIABLE_OVERSCAN_ROWS: usize = 3;
const VARIABLE_ROW_HEIGHTS_PX: [u32; 6] = [24, 28, 32, 36, 40, 44];
const VARIABLE_FALLBACK_HEIGHT_PX: u32 = 30;
const ESTIMATED_VIEWPORT_OFFSET_PX: u32 = 84;
const ESTIMATED_VIEWPORT_HEIGHT_PX: u32 = 140;
const ESTIMATED_OVERSCAN_ROWS: usize = 4;
const ESTIMATED_ROW_HEIGHT_PX: u32 = 28;
const FIRST_MEASURED_ROW_INDEX: usize = 3;
const FIRST_MEASURED_ROW_HEIGHT_PX: u32 = 44;
const SECOND_MEASURED_ROW_INDEX: usize = 9;
const SECOND_MEASURED_ROW_HEIGHT_PX: u32 = 36;
const DISABLED_OVERSCAN_ROWS: usize = 0;

pub(super) fn fixed_config(
    total_count: usize,
    focused_index: Option<usize>,
) -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count,
        viewport_offset: FIXED_VIEWPORT_OFFSET_PX,
        viewport_height: FIXED_VIEWPORT_HEIGHT_PX,
        overscan: FIXED_OVERSCAN_ROWS,
        row_height_provider: RowHeightProvider::Fixed {
            height: FIXED_ROW_HEIGHT_PX,
        },
        keep_focused_in_window: true,
        focused_index,
    }
}

pub(super) fn variable_config(
    total_count: usize,
    focused_index: Option<usize>,
) -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count,
        viewport_offset: VARIABLE_VIEWPORT_OFFSET_PX,
        viewport_height: VARIABLE_VIEWPORT_HEIGHT_PX,
        overscan: VARIABLE_OVERSCAN_ROWS,
        row_height_provider: RowHeightProvider::Variable {
            row_heights: VARIABLE_ROW_HEIGHTS_PX.to_vec(),
            fallback_height: VARIABLE_FALLBACK_HEIGHT_PX,
        },
        keep_focused_in_window: true,
        focused_index,
    }
}

pub(super) fn estimated_config(
    total_count: usize,
    focused_index: Option<usize>,
) -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count,
        viewport_offset: ESTIMATED_VIEWPORT_OFFSET_PX,
        viewport_height: ESTIMATED_VIEWPORT_HEIGHT_PX,
        overscan: ESTIMATED_OVERSCAN_ROWS,
        row_height_provider: RowHeightProvider::Estimated {
            estimated_height: ESTIMATED_ROW_HEIGHT_PX,
            measured_overrides: vec![
                RowHeightOverride {
                    index: FIRST_MEASURED_ROW_INDEX,
                    height: FIRST_MEASURED_ROW_HEIGHT_PX,
                },
                RowHeightOverride {
                    index: SECOND_MEASURED_ROW_INDEX,
                    height: SECOND_MEASURED_ROW_HEIGHT_PX,
                },
            ],
        },
        keep_focused_in_window: true,
        focused_index,
    }
}

pub(super) fn log(
    target: UiStateId,
    action: &'static str,
    config: &VirtualizationConfig,
) -> UiCallbackLog {
    let disabled = disabled_config(config);
    UiCallbackLog::new(
        target,
        action,
        summary(config),
        format!(
            "{} provider_switch={}",
            summary(&disabled),
            provider_name(&config.row_height_provider)
        ),
    )
}

pub(super) fn summary(config: &VirtualizationConfig) -> String {
    let range = config.visible_range();
    format!(
        "virtualization enabled={} overscan={} row_height_provider={} visible_range={} total_count={}",
        config.enabled,
        config.overscan,
        provider_name(&config.row_height_provider),
        range_summary(&range),
        range.total_count
    )
}

pub(super) fn compact_label(config: &VirtualizationConfig) -> String {
    let range = config.visible_range();
    format!(
        "Virtualization: {} / total {} / {}",
        range_summary(&range),
        range.total_count,
        provider_name(&config.row_height_provider)
    )
}

fn disabled_config(config: &VirtualizationConfig) -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: false,
        overscan: DISABLED_OVERSCAN_ROWS,
        ..config.clone()
    }
}

fn range_summary(range: &VirtualRange) -> String {
    format!("{}..{}", range.start, range.end)
}

fn provider_name(provider: &RowHeightProvider) -> String {
    match provider {
        RowHeightProvider::Fixed { height } => format!("Fixed({height})"),
        RowHeightProvider::Variable {
            fallback_height, ..
        } => format!("Variable(fallback={fallback_height})"),
        RowHeightProvider::Estimated {
            estimated_height,
            measured_overrides,
        } => format!(
            "Estimated({estimated_height},measured={})",
            measured_overrides.len()
        ),
    }
}
