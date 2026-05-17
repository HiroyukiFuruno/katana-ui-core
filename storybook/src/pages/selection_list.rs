use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_core::composite::selection_list::{
    SelectionList, SelectionListItem, SelectionListSection,
};
use katana_ui_core::theme::Theme;

fn set_log(log: RwSignal<String>, value: &'static str) -> impl Fn() + 'static {
    move || {
        log.set(value.to_string());
    }
}

fn sections(theme: &Theme, log: RwSignal<String>) -> Vec<SelectionListSection> {
    vec![
        SelectionListSection::new(
            "Light theme presets",
            vec![
                SelectionListItem::new("Light accents", theme.color.accent)
                    .selected(true)
                    .on_select(set_log(log, "Light accents を選択"))
                    .content(label(|| "選択中は背景色が変わります。").style({
                        let muted = PenikoColor::rgb8(
                            theme.color.text_muted.r,
                            theme.color.text_muted.g,
                            theme.color.text_muted.b,
                        );
                        move |s| s.font_size(11.0).color(muted)
                    })),
                SelectionListItem::new("Warning accents", theme.color.warning)
                    .on_select(set_log(log, "Warning accents を選択")),
                SelectionListItem::new("Success accents", theme.color.success)
                    .on_select(set_log(log, "Success accents を選択")),
                SelectionListItem::new("Danger accents", theme.color.danger)
                    .hidden(true)
                    .on_select(set_log(log, "Danger accents を選択")),
            ],
        ),
        SelectionListSection::new(
            "Dark theme presets",
            vec![
                SelectionListItem::new("Main background", theme.color.surface).disabled(true),
                SelectionListItem::new("Surface background", theme.color.surface)
                    .on_select(set_log(log, "Surface background を選択")),
                SelectionListItem::new("Muted text", theme.color.text_muted)
                    .hidden(true)
                    .on_select(set_log(log, "Muted text を選択")),
            ],
        ),
    ]
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let action_log = create_rw_signal("未操作".to_string());
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_color = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let list = SelectionList::new(sections(theme, action_log))
        .show_more("もっと表示", set_log(action_log, "隠し項目を表示"))
        .view(theme.clone());

    scroll(
        v_stack((
            label(|| "SelectionList Samples")
                .style(|style| style.font_size(16.0).margin_bottom(8.0)),
            list,
            label(move || format!("操作結果: {}", action_log.get()))
                .style(|style| style.font_size(12.0)),
        ))
        .style(move |style| {
            style
                .gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text_color)
                .min_width_full()
        }),
    )
}

pub fn selection_list_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
