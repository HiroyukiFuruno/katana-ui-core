use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::selector::select::SelectBox;
use katana_ui_widget::theme::Theme;

fn select_row(
    heading: &'static str,
    trigger: &str,
    tr: u8,
    tg: u8,
    tb: u8,
    font_sz: f32,
    options: Vec<(String, bool)>,
) -> impl IntoView {
    let trigger_color = PenikoColor::rgb8(tr, tg, tb);
    let trigger_lbl: &'static str = Box::leak(trigger.to_string().into_boxed_str());

    let opt_rows: Vec<_> = options
        .into_iter()
        .map(|(lbl, selected)| {
            let indicator = if selected { " ◀" } else { "" };
            let lbl: &'static str = Box::leak(lbl.into_boxed_str());
            label(move || format!("  {lbl}{indicator}")).style(|s| s.font_size(11.0).padding(2.0))
        })
        .collect();

    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        label(move || trigger_lbl)
            .style(move |s| s.font_size(font_sz).color(trigger_color).padding(4.0).border(1.0)),
        v_stack(opt_rows).style(|s| s.padding_left(8.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let short_opts: Vec<(u8, String)> =
        vec![(1, "Apple".into()), (2, "Banana".into()), (3, "Cherry".into())];
    let long_opts: Vec<(u8, String)> = (1u8..=10)
        .map(|i| (i, format!("Option {i}")))
        .collect();

    let r_placeholder = SelectBox::new(short_opts.clone(), "Fruit").resolve(theme);
    let r_selected = SelectBox::new(short_opts.clone(), "Fruit").value(2u8).resolve(theme);
    let r_open = SelectBox::new(short_opts.clone(), "Fruit open").open(true).resolve(theme);
    let r_long = SelectBox::new(long_opts, "Long list").value(5u8).resolve(theme);
    let r_disabled = SelectBox::new(short_opts, "Fruit disabled").disabled(true).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let to_row = |heading: &'static str,
                  r: katana_ui_widget::composite::selector::select::ResolvedSelectBox| {
        let opts = r
            .options
            .iter()
            .map(|o| (o.label.clone(), o.selected))
            .collect();
        select_row(
            heading,
            &r.trigger_label,
            r.trigger_text.r,
            r.trigger_text.g,
            r.trigger_text.b,
            r.font_size,
            opts,
        )
    };

    scroll(
        v_stack((
            label(|| "SelectBox Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            to_row("Placeholder (no value)", r_placeholder),
            to_row("Value selected (Banana)", r_selected),
            to_row("Open state (border = accent)", r_open),
            to_row("Long list (Option 5 selected)", r_long),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            to_row("Disabled", r_disabled),
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

pub fn select_box_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "SelectBox").style(|s| s.font_size(20.0)),
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
