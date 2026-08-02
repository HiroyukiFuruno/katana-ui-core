use katana_ui_core_storybook::{
    DEFAULT_STORYBOOK_PAGE, StoryCatalog, StorybookPanel, StorybookSummary, StorybookVisual,
};
use std::fs;
use std::path::Path;

use crate::snapshot_command::SnapshotCommand;

const HEADLESS_SCENARIO_STEP_COUNT: usize = 6;

pub(super) fn run_headless_scenario() -> Result<(), String> {
    let catalog = StoryCatalog;
    let examples = catalog.examples();
    let panel_report = StorybookPanel::interaction_report(&examples);
    let visual_report = StorybookVisual.coverage_report();
    let mut steps: [Box<dyn FnMut() -> Result<(), String> + '_>; HEADLESS_SCENARIO_STEP_COUNT] = [
        Box::new(|| {
            write_json(
                Path::new("target/storybook-panel-interaction-report.json"),
                &panel_report,
                "failed to write panel interaction report",
            )
        }),
        Box::new(|| {
            write_json(
                Path::new("target/storybook-visual-coverage.json"),
                &visual_report,
                "failed to write visual coverage report",
            )
        }),
        Box::new(|| {
            save_scenario_png(
                "target/storybook-panel-light.png",
                "light",
                DEFAULT_STORYBOOK_PAGE,
                false,
            )
        }),
        Box::new(|| {
            save_scenario_png(
                "target/storybook-panel-dark.png",
                "dark",
                DEFAULT_STORYBOOK_PAGE,
                false,
            )
        }),
        Box::new(|| {
            save_scenario_png(
                "target/storybook-panel-after-operation.png",
                "dark",
                DEFAULT_STORYBOOK_PAGE,
                true,
            )
        }),
        Box::new(|| save_modal_png("target/storybook-panel-modal-window.png")),
    ];
    run_steps(&mut steps)?;
    println!(
        "katana-ui-core-storybook-headless: {} {} {}",
        StorybookSummary.render(),
        panel_report.summary(),
        visual_report.summary()
    );
    Ok(())
}

pub(super) fn run_headless_interaction_audit() -> Result<(), String> {
    let report = StorybookVisual.live_interaction_audit_report();
    let mut steps: [Box<dyn FnMut() -> Result<(), String> + '_>; 1] = [Box::new(|| {
        write_json(
            Path::new("target/storybook-live-interaction-audit.json"),
            &report,
            "failed to write live interaction audit report",
        )
    })];
    run_steps(&mut steps)?;
    println!(
        "katana-ui-core-storybook-live-interaction: {}",
        report.summary()
    );
    Ok(())
}

pub(super) fn run_steps(
    steps: &mut [Box<dyn FnMut() -> Result<(), String> + '_>],
) -> Result<(), String> {
    for step in steps {
        step()?;
    }
    Ok(())
}

pub(super) fn write_json<T: serde::Serialize>(
    path: &Path,
    value: &T,
    failure: &str,
) -> Result<(), String> {
    let Some(parent) = path.parent() else {
        return Err(format!("{failure}: missing parent directory"));
    };
    fs::create_dir_all(parent).map_err(|error| format!("{failure}: {error}"))?;
    let json =
        serde_json::to_string_pretty(value).map_err(|error| format!("{failure}: {error}"))?;
    fs::write(path, json).map_err(|error| format!("{failure}: {error}"))
}

pub(super) fn modal_snapshot_error(error: image::ImageError) -> String {
    format!("failed to write modal snapshot: {error}")
}

pub(super) fn scenario_snapshot_error(error: image::ImageError) -> String {
    format!("failed to write scenario snapshot: {error}")
}

fn save_modal_png(path: &str) -> Result<(), String> {
    let output_path = Path::new(path);
    SnapshotCommand::prepare(output_path, "failed to prepare modal snapshot")?;
    StorybookVisual
        .save_modal_png(output_path)
        .map_err(modal_snapshot_error)
}

fn save_scenario_png(
    path: &str,
    theme_id: &str,
    selected_page: &str,
    operation: bool,
) -> Result<(), String> {
    let output_path = Path::new(path);
    SnapshotCommand::prepare(output_path, "failed to prepare scenario snapshot")?;
    StorybookVisual
        .save_scenario_png(output_path, theme_id, selected_page, operation)
        .map_err(scenario_snapshot_error)
}
