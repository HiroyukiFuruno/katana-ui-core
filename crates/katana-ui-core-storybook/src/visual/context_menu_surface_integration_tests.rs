use super::context_menu_surface_storybook_runner::run_storybook_context_menu;

#[test]
fn actual_egui_context_menu_storybook_integration_is_repeatable()
-> Result<(), Box<dyn std::error::Error>> {
    let first = run_storybook_context_menu()?;
    let second = run_storybook_context_menu()?;
    assert_eq!(first, second);
    assert!(first.pointer_clamped);
    assert!(first.colored_star_texture);
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("整形 ⭐️"))
    );
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("opaque code kind"))
    );
    Ok(())
}

#[test]
fn actual_egui_context_menu_storybook_integration_preserves_contract_evidence_hashes()
-> Result<(), Box<dyn std::error::Error>> {
    let first = run_storybook_context_menu()?;
    let second = run_storybook_context_menu()?;
    assert_eq!(first, second);
    assert!(!first.composite_hash.is_empty());
    assert!(!first.plan_hash.is_empty());
    assert!(!first.frame_hash.is_empty());
    assert!(first.colored_star_texture);
    assert!(first.pointer_clamped);
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("整形 ⭐️"))
    );
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("opaque code kind"))
    );
    assert_eq!(first.plan_hash, second.plan_hash);
    assert_eq!(first.composite_hash, second.composite_hash);
    assert_eq!(first.frame_hash, second.frame_hash);
    Ok(())
}
