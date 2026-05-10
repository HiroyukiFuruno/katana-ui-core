use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::indicator::key_cap::{KeyCap, KeyCombo, KeyLabel, NamedKey};
use katana_ui_widget::theme::Theme;

fn cap_view(display: &'static str, bg_r: u8, bg_g: u8, bg_b: u8, text_r: u8, text_g: u8, text_b: u8, font_sz: f32) -> impl IntoView {
    let bg = PenikoColor::rgb8(bg_r, bg_g, bg_b);
    let text_color = PenikoColor::rgb8(text_r, text_g, text_b);
    label(move || display).style(move |s| {
        s.background(bg)
            .color(text_color)
            .font_size(font_sz)
            .padding(4.0)
            .border(1.0)
            .border_radius(2.0)
    })
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_cmd = KeyCap::new(KeyLabel::Cmd).resolve(theme);
    let r_shift = KeyCap::new(KeyLabel::Shift).resolve(theme);
    let r_char_p = KeyCap::new(KeyLabel::Char('p')).resolve(theme);
    let r_f1 = KeyCap::new(KeyLabel::Named(NamedKey::F1)).resolve(theme);
    let r_enter = KeyCap::new(KeyLabel::Named(NamedKey::Enter)).resolve(theme);
    let r_esc = KeyCap::new(KeyLabel::Named(NamedKey::Escape)).resolve(theme);

    let combo = KeyCombo::new(vec![KeyLabel::Cmd, KeyLabel::Shift, KeyLabel::Char('p')]).resolve(theme);
    let combo2 = KeyCombo::new(vec![KeyLabel::Ctrl, KeyLabel::Char('c')]).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let cap = |r: &katana_ui_widget::composite::indicator::key_cap::ResolvedKeyCap| {
        let d: &'static str = Box::leak(r.display.clone().into_boxed_str());
        cap_view(d, r.bg_color.r, r.bg_color.g, r.bg_color.b, r.text_color.r, r.text_color.g, r.text_color.b, r.font_size)
    };

    let combo_row = |caps: Vec<katana_ui_widget::composite::indicator::key_cap::ResolvedKeyCap>| {
        let views: Vec<_> = caps.iter().map(cap).collect();
        h_stack(views).style(|s| s.gap(2.0).items_center())
    };

    scroll(
        v_stack((
            label(|| "KeyCap Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            v_stack((
                label(|| "Single keys").style(|s| s.font_size(12.0)),
                h_stack((
                    cap(&r_cmd),
                    cap(&r_shift),
                    cap(&r_char_p),
                    cap(&r_f1),
                    cap(&r_enter),
                    cap(&r_esc),
                )).style(|s| s.gap(4.0)),
            )).style(|s| s.gap(4.0)),
            v_stack((
                label(|| "Combos").style(|s| s.font_size(12.0)),
                combo_row(combo.caps),
                combo_row(combo2.caps),
            )).style(|s| s.gap(4.0)),
            label(|| "Note: Cmd shows ⌘ on macOS, Ctrl elsewhere").style(|s| s.font_size(11.0).margin_top(8.0)),
        ))
        .style(move |s| {
            s.gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn key_cap_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "KeyCap").style(|s| s.font_size(20.0)),
            label(move || if is_dark.get() { "Dark" } else { "Light" }),
            toggle_button(move || is_dark.get()).on_toggle(move |v| is_dark.set(v)),
        ))
        .style(|s| s.gap(12.0).items_center().padding(12.0)),
        dyn_container(
            move || is_dark.get(),
            move |dark| {
                let theme = if dark {
                    Theme::default_dark()
                } else {
                    Theme::default_light()
                };
                page_content(&theme)
            },
        ),
    ))
}
