use crate::StoryCatalog;
use crate::catalog::StoryPresetLabels;

use super::StorybookVisual;
use std::fs;

#[test]
fn every_declared_preset_renders_before_and_after_interaction() {
    let examples = StoryCatalog.examples();
    let mut rendered = 0;
    let mut pixel_signature = 0_u64;

    for example in &examples {
        let presets = StoryPresetLabels::for_page(example.page);
        let preset_count = presets.len().max(1);

        for preset_index in 0..preset_count {
            let scroll_y = if preset_index % 2 == 0 { 0 } else { 10_000 };
            let scrollbar_visible = preset_index % 3 != 0;
            let initial = StorybookVisual.render_preset_with_scrollbar(
                "dark",
                example.page,
                preset_index,
                scroll_y,
                scrollbar_visible,
            );
            let interacted = StorybookVisual.render_clicked_preset_with_scrollbar(
                "light",
                example.page,
                preset_index,
                scroll_y,
                !scrollbar_visible,
            );

            assert_eq!(initial.width(), interacted.width());
            assert_eq!(initial.height(), interacted.height());
            assert!(!initial.pixels().is_empty());
            assert!(!interacted.pixels().is_empty());

            pixel_signature = pixel_signature
                .wrapping_add(u64::from(initial.pixels()[0]))
                .wrapping_add(u64::from(
                    interacted.pixels()[interacted.pixels().len() - 1],
                ));
            rendered += 2;
        }
    }

    assert!(rendered > examples.len() * 2);
    assert_ne!(pixel_signature, 0);
}

#[test]
fn every_public_png_entrypoint_writes_a_non_empty_png() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = std::env::temp_dir().join(format!(
        "katana-ui-core-storybook-png-contract-{}",
        std::process::id()
    ));
    fs::create_dir_all(&output_dir)?;
    let outputs = [
        output_dir.join("default.png"),
        output_dir.join("scenario.png"),
        output_dir.join("preset.png"),
        output_dir.join("preset-scrolled.png"),
        output_dir.join("preset-scrollbar.png"),
        output_dir.join("preset-clicked.png"),
        output_dir.join("modal.png"),
    ];

    StorybookVisual.save_png(&outputs[0])?;
    StorybookVisual.save_scenario_png(&outputs[1], "light", "button", true)?;
    StorybookVisual.save_preset_png(&outputs[2], "dark", "checkbox", 1)?;
    StorybookVisual.save_preset_scrolled_png(&outputs[3], "dark", "panel", 1, 10_000)?;
    StorybookVisual.save_preset_scrolled_png_with_scrollbar(
        &outputs[4],
        "light",
        "text-area",
        5,
        10_000,
        false,
    )?;
    StorybookVisual.save_clicked_preset_scrolled_png_with_scrollbar(
        &outputs[5],
        "dark",
        "button",
        0,
        0,
        true,
    )?;
    StorybookVisual.save_modal_png(&outputs[6])?;

    for output in &outputs {
        let bytes = fs::read(output)?;
        assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
        assert!(bytes.len() > 8);
    }
    fs::remove_dir_all(output_dir)?;
    assert_eq!(StorybookVisual.coverage_report().required_ui, 77);
    assert_eq!(
        StorybookVisual
            .live_interaction_audit_report()
            .scenarios
            .len(),
        403
    );
    Ok(())
}
