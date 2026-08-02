#[cfg(target_os = "linux")]
use katana_ui_core_storybook::{StorybookVisual, StorybookVisualError};

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires a Linux Xvfb display"]
fn native_xvfb_integration_covers_storybook_dependency_adapters() -> Result<(), StorybookVisualError>
{
    StorybookVisual.open_window(1)?;
    Ok(())
}
