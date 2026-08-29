use katana_ui_core::interaction::placement::{Rect, Size};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeToolbar, FloatingCommandToolbar,
    FloatingCommandToolbarPresentation, FloatingCommandToolbarVisibility,
};
use std::error::Error;

#[test]
fn adapter_measured_floating_toolbar_requires_no_consumer_panel_size() -> Result<(), Box<dyn Error>>
{
    let mut floating = FloatingCommandToolbar::new_adapter_measured(
        CommandChromeToolbar::new().action(CommandChromeAction::new("one", "One")),
        Rect::new(10, 10, 1, 1),
        Rect::new(0, 0, 120, 80),
    )
    .initial_visibility(FloatingCommandToolbarVisibility::Visible);
    assert_eq!(floating.layout_model().panel_size, Size::new(0, 0));
    assert!(floating.synchronize_measured_panel(Size::new(64, 24)));
    let bounds = floating
        .bounds_model()
        .ok_or_else(|| std::io::Error::other("placement"))?;
    assert_eq!(bounds.width, 64);
    Ok(())
}

#[test]
fn controlled_floating_presentation_uses_frame_facts_without_panel_size_or_events() {
    let mut floating = FloatingCommandToolbar::new_adapter_measured(
        CommandChromeToolbar::new().action(CommandChromeAction::new("one", "One")),
        Rect::new(10, 10, 1, 1),
        Rect::new(0, 0, 120, 80),
    );
    let _ = floating.synchronize_measured_panel(Size::new(64, 24));
    assert!(
        floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
            Rect::new(90, 60, 1, 1),
            Rect::new(0, 0, 100, 70),
            FloatingCommandToolbarVisibility::Visible,
        ))
    );
    assert_eq!(
        floating.visibility_model(),
        FloatingCommandToolbarVisibility::Visible
    );
    assert_eq!(floating.layout_model().panel_size, Size::new(64, 24));
    assert!(
        !floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
            Rect::new(90, 60, 1, 1),
            Rect::new(0, 0, 100, 70),
            FloatingCommandToolbarVisibility::Visible,
        ))
    );
}
