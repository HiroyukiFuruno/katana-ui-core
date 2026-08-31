use super::*;

#[test]
fn available_capability_reports_no_disabled_reason() {
    let capability = CommandChromeCapability::available();
    assert!(capability.is_available());
    assert_eq!(capability.disabled_reason(), None);
}

#[test]
fn unavailable_capability_exposes_disabled_reason() {
    let capability = CommandChromeCapability::unavailable("disabled for test");
    assert!(!capability.is_available());
    assert_eq!(capability.disabled_reason(), Some("disabled for test"));
}
