#[cfg(test)]
use crate::layout::accordion::{Accordion, IndicatorPosition};
#[cfg(test)]
use crate::theme::Theme;
#[cfg(test)]
use floem::views::label;

#[test]
fn collapsed_shows_down_chevron() {
    let theme = Theme::default_light();
    let r = Accordion::new("Section").resolve(&theme);
    assert!(!r.expanded);
    assert_eq!(r.chevron, Some("▼"));
}

#[test]
fn expanded_shows_up_chevron() {
    let theme = Theme::default_light();
    let r = Accordion::new("Section").expanded(true).resolve(&theme);
    assert!(r.expanded);
    assert_eq!(r.chevron, Some("▲"));
}

#[test]
fn indicator_none_hides_chevron() {
    let theme = Theme::default_light();
    let r = Accordion::new("Section")
        .indicator(IndicatorPosition::None)
        .resolve(&theme);
    assert!(r.chevron.is_none());
}

#[test]
fn disabled_uses_muted_text() {
    let theme = Theme::default_light();
    let r = Accordion::new("Section").disabled(true).resolve(&theme);
    assert_eq!(r.header_text, theme.color.text_disabled);
}

#[test]
fn toggle_calls_on_toggle_with_next_state() {
    let called = std::rc::Rc::new(std::cell::RefCell::new(None));
    let called_ref = std::rc::Rc::clone(&called);
    let theme = Theme::default_light();
    let r = Accordion::new("Section")
        .expanded(false)
        .on_toggle(move |expanded| {
            *called_ref.borrow_mut() = Some(expanded);
        })
        .resolve(&theme);

    assert_eq!(r.toggle(), Some(true));
    assert_eq!(*called.borrow(), Some(true));
}

#[test]
fn disabled_toggle_does_not_call_on_toggle() {
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let called_ref = std::rc::Rc::clone(&called);
    let theme = Theme::default_light();
    let r = Accordion::new("Section")
        .disabled(true)
        .on_toggle(move |_| {
            *called_ref.borrow_mut() = true;
        })
        .resolve(&theme);

    assert_eq!(r.toggle(), None);
    assert!(!*called.borrow());
}

#[test]
fn node_header_builder_is_accepted() {
    let theme = Theme::default_light();
    let r = Accordion::new("Section")
        .header(|| label(|| "Rich header".to_string()))
        .resolve(&theme);

    assert_eq!(r.header, "Section");
}

#[test]
fn body_border_is_optional() {
    let theme = Theme::default_light();
    let default = Accordion::new("Section").resolve(&theme);
    let bordered = Accordion::new("Section").body_border(true).resolve(&theme);

    assert!(!default.body_border);
    assert!(bordered.body_border);
}
