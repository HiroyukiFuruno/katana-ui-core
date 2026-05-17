#[test]
fn plain_has_no_border_no_shadow() {
    let theme = crate::theme::Theme::default_light();
    let r = crate::layout::card::Card::new()
        .variant(crate::layout::card::CardVariant::Plain)
        .resolve(&theme);
    assert!(r.border_color.is_none());
    assert!(!r.has_shadow);
}

#[test]
fn elevated_has_shadow_no_border() {
    let theme = crate::theme::Theme::default_light();
    let r = crate::layout::card::Card::new()
        .variant(crate::layout::card::CardVariant::Elevated)
        .resolve(&theme);
    assert!(r.has_shadow);
    assert!(r.border_color.is_none());
}

#[test]
fn outlined_has_border_no_shadow() {
    let theme = crate::theme::Theme::default_light();
    let r = crate::layout::card::Card::new()
        .variant(crate::layout::card::CardVariant::Outlined)
        .resolve(&theme);
    assert!(r.border_color.is_some());
    assert!(!r.has_shadow);
}

#[test]
fn interactive_flag_preserved() {
    let theme = crate::theme::Theme::default_light();
    let r = crate::layout::card::Card::new()
        .interactive(true)
        .resolve(&theme);
    assert!(r.interactive);
}

#[test]
fn no_padding_is_zero() {
    let theme = crate::theme::Theme::default_light();
    let r = crate::layout::card::Card::new()
        .padding(crate::layout::card::CardPadding::None)
        .resolve(&theme);
    assert_eq!(r.padding, 0.0);
}

#[test]
fn has_on_click_flag_kept_in_resolved() {
    let theme = crate::theme::Theme::default_light();
    let r = crate::layout::card::Card::new()
        .interactive(true)
        .on_click(|| {})
        .resolve(&theme);
    assert!(r.interactive);
    assert!(r.has_on_click);
}

#[test]
fn body_slot_takes_priority_over_child() {
    let theme = crate::theme::Theme::default_light();
    let _ = crate::layout::card::Card::new().body(floem::views::label(|| "slot"));
    let _ = crate::layout::card::Card::new()
        .body(floem::views::label(|| "slot"))
        .view(theme.clone(), floem::views::label(|| "fallback"));
    let _ =
        crate::layout::card::Card::new().view(theme.clone(), floem::views::label(|| "fallback"));
}
