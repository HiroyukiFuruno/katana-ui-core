use katana_ui_core::interaction::{
    RowHeightOverride, RowHeightProvider, UiCallbackLog, VirtualRange, VirtualizationConfig,
};
use katana_ui_core::render_model::UiStateId;

pub(super) const LIST_TOTAL_COUNT: usize = 128;
pub(super) const SELECTION_TOTAL_COUNT: usize = 96;
pub(super) const TREE_TOTAL_COUNT: usize = 240;
pub(super) const COMMAND_TOTAL_COUNT: usize = 5;
pub(super) const DIAGNOSTIC_TOTAL_COUNT: usize = 64;

pub(super) fn fixed_config(
    total_count: usize,
    focused_index: Option<usize>,
) -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count,
        viewport_offset: 56,
        viewport_height: 112,
        overscan: 2,
        row_height_provider: RowHeightProvider::Fixed { height: 28 },
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
        viewport_offset: 60,
        viewport_height: 120,
        overscan: 3,
        row_height_provider: RowHeightProvider::Variable {
            row_heights: vec![24, 28, 32, 36, 40, 44],
            fallback_height: 30,
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
        viewport_offset: 84,
        viewport_height: 140,
        overscan: 4,
        row_height_provider: RowHeightProvider::Estimated {
            estimated_height: 28,
            measured_overrides: vec![
                RowHeightOverride {
                    index: 3,
                    height: 44,
                },
                RowHeightOverride {
                    index: 9,
                    height: 36,
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
        overscan: 0,
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
