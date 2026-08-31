use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdownCloseReason, CommandChromeToolbarEvent,
};
use katana_ui_core::molecule::toolbar::ToolbarActionId;
use katana_ui_core_svg_raster::UiSvgRasterConfig;
use katana_ui_core_text_raster::{
    PlatformFontCatalog, PlatformTextMetricsFrame, PlatformTextRasterConfig,
};

use super::{EguiCommandChromeAdapter, EguiCommandChromeError, dropdown_focus_return_target};

#[test]
fn dropdown_focus_return_target_prefers_last_matching_close_reason() {
    let events = vec![
        CommandChromeToolbarEvent::DropdownClosed {
            action_id: ToolbarActionId::new("first"),
            reason: CommandChromeDropdownCloseReason::Explicit,
        },
        CommandChromeToolbarEvent::DropdownClosed {
            action_id: ToolbarActionId::new("second"),
            reason: CommandChromeDropdownCloseReason::Escape,
        },
    ];

    assert_eq!(
        Some("second"),
        dropdown_focus_return_target(&events),
        "most recent matching close reason should be preferred"
    );
}

#[test]
fn dropdown_focus_return_target_prefers_last_matching_close_reason_for_outside_click_and_item_activation()
 {
    let events = vec![
        CommandChromeToolbarEvent::DropdownClosed {
            action_id: ToolbarActionId::new("escape"),
            reason: CommandChromeDropdownCloseReason::Escape,
        },
        CommandChromeToolbarEvent::DropdownClosed {
            action_id: ToolbarActionId::new("outside"),
            reason: CommandChromeDropdownCloseReason::OutsideClick,
        },
        CommandChromeToolbarEvent::CommandActivated {
            action_id: ToolbarActionId::new("activated"),
        },
        CommandChromeToolbarEvent::DropdownClosed {
            action_id: ToolbarActionId::new("item"),
            reason: CommandChromeDropdownCloseReason::ItemActivated,
        },
    ];

    assert_eq!(Some("item"), dropdown_focus_return_target(&events));
}

#[test]
fn dropdown_focus_return_target_ignores_non_focus_relevant_events_and_empty() {
    let non_matching = vec![
        CommandChromeToolbarEvent::DropdownClosed {
            action_id: ToolbarActionId::new("first"),
            reason: CommandChromeDropdownCloseReason::Explicit,
        },
        CommandChromeToolbarEvent::CommandActivated {
            action_id: ToolbarActionId::new("second"),
        },
    ];

    assert_eq!(None, dropdown_focus_return_target(&non_matching));
    assert_eq!(None, dropdown_focus_return_target(&[]));
}

#[test]
fn command_chrome_adapter_catalog_is_accessible() {
    let adapter = EguiCommandChromeAdapter::new(
        PlatformTextRasterConfig::default(),
        UiSvgRasterConfig::default(),
    );
    let catalog = adapter.catalog();

    assert!(Arc::strong_count(&catalog) >= 1);
}

#[test]
fn command_chrome_adapter_with_catalog_and_metrics_configures_dependency_links() {
    let config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let metrics = Rc::new(RefCell::new(PlatformTextMetricsFrame::new()));

    let adapter = EguiCommandChromeAdapter::with_catalog_and_metrics(
        Arc::clone(&catalog),
        config,
        UiSvgRasterConfig::default(),
        Rc::clone(&metrics),
    )
    .expect("adapter should be created from provided catalog/metrics");

    assert!(Rc::ptr_eq(&adapter.metrics, &metrics));
    assert!(Arc::ptr_eq(&adapter.text_rasterizer.catalog(), &catalog));
}

#[test]
fn command_chrome_adapter_rejects_a_catalog_from_a_different_font_policy() {
    let catalog_config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(catalog_config.catalog_policy()));
    let mut incompatible_config = catalog_config;
    incompatible_config
        .proportional_candidates
        .push("/fonts/not-in-catalog.ttf".into());

    let error = match EguiCommandChromeAdapter::with_catalog_and_metrics(
        catalog,
        incompatible_config,
        UiSvgRasterConfig::default(),
        Rc::new(RefCell::new(PlatformTextMetricsFrame::new())),
    ) {
        Ok(_) => panic!("a catalog/config policy mismatch must fail closed"),
        Err(error) => error,
    };

    assert!(matches!(
        error,
        EguiCommandChromeError::Text(
            katana_ui_core_text_raster::PlatformTextRasterError::CatalogConfigurationMismatch
        )
    ));
}
