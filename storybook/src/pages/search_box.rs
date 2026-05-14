use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::input::search::{
    SearchBox, SearchBoxControl, SearchBoxControlMode, SearchBoxIconMode, SearchBoxIconPreset,
    SearchBoxIconSlot,
};
use katana_ui_widget::composite::input::text::InputSize;
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;

const CUSTOM_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><path d=\"M8 2l2 4 4 .5-3 3 .8 4.5L8 12l-3.8 2 .8-4.5-3-3L6 6z\" fill=\"currentColor\"/></svg>";

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let submit_log = create_rw_signal("未送信".to_string());
    let option_log = create_rw_signal("regex=false, word=false, case=false".to_string());

    crate::interaction::replay("toggle-search-options", "search-box", "options-all-true", {
        let option_log = option_log;
        move || {
            option_log.set("regex=true, word=true, case=true".to_string());
        }
    });

    let readonly_section = v_stack((
        label(|| "Readonly display")
            .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
        label(|| "Default hidden icons").style(|style| style.font_size(12.0)),
        SearchBox::new("Empty search")
            .placeholder("Search...")
            .disabled(true)
            .view(theme.clone()),
        label(|| "Reserved icon space").style(|style| style.font_size(12.0)),
        SearchBox::new("With query")
            .value("floem widgets")
            .search_icon(SearchBoxIconMode::Reserved)
            .clear_icon(SearchBoxIconMode::Reserved)
            .submit_icon(SearchBoxIconMode::Reserved)
            .control_mode(SearchBoxControl::Regex, SearchBoxControlMode::Reserved)
            .control_mode(SearchBoxControl::WholeWord, SearchBoxControlMode::Reserved)
            .control_mode(
                SearchBoxControl::CaseSensitive,
                SearchBoxControlMode::Reserved,
            )
            .disabled(true)
            .view(theme.clone()),
        label(|| "Custom leading SVG").style(|style| style.font_size(12.0)),
        SearchBox::new("Custom search")
            .value("custom icon")
            .search_icon(SearchBoxIconMode::Visible)
            .icon_source(
                SearchBoxIconSlot::Leading,
                IconSource::SvgBytes(CUSTOM_ICON),
            )
            .clear_icon(SearchBoxIconMode::Visible)
            .submit_icon(SearchBoxIconMode::Visible)
            .show_all_controls()
            .regex(true)
            .whole_word(true)
            .case_sensitive(true)
            .disabled(true)
            .view(theme.clone()),
    ))
    .style(|style| style.gap(8.0));

    let preset_section = v_stack((
        label(|| "Icon preset comparison").style(|style| style.font_size(12.0)),
        SearchBox::new("Preset search")
            .value("preset: search")
            .search_icon(SearchBoxIconMode::Visible)
            .icon_preset(SearchBoxIconSlot::Leading, SearchBoxIconPreset::Search)
            .disabled(true)
            .view(theme.clone()),
        SearchBox::new("Preset clear")
            .value("preset: clear")
            .search_icon(SearchBoxIconMode::Visible)
            .icon_preset(SearchBoxIconSlot::Leading, SearchBoxIconPreset::Clear)
            .disabled(true)
            .view(theme.clone()),
        SearchBox::new("Preset submit")
            .value("preset: submit")
            .search_icon(SearchBoxIconMode::Visible)
            .icon_preset(SearchBoxIconSlot::Leading, SearchBoxIconPreset::Submit)
            .disabled(true)
            .view(theme.clone()),
    ))
    .style(|style| style.gap(8.0));

    let control_section = v_stack((
        label(|| "Control mode comparison").style(|style| style.font_size(12.0)),
        SearchBox::new("Control hidden")
            .value("hidden controls")
            .control_mode(SearchBoxControl::Regex, SearchBoxControlMode::Hidden)
            .control_mode(SearchBoxControl::WholeWord, SearchBoxControlMode::Hidden)
            .control_mode(
                SearchBoxControl::CaseSensitive,
                SearchBoxControlMode::Hidden,
            )
            .disabled(true)
            .view(theme.clone()),
        SearchBox::new("Control reserved")
            .value("reserved controls")
            .control_mode(SearchBoxControl::Regex, SearchBoxControlMode::Reserved)
            .control_mode(SearchBoxControl::WholeWord, SearchBoxControlMode::Reserved)
            .control_mode(
                SearchBoxControl::CaseSensitive,
                SearchBoxControlMode::Reserved,
            )
            .disabled(true)
            .view(theme.clone()),
        SearchBox::new("Control visible")
            .value("visible controls")
            .show_all_controls()
            .regex(true)
            .whole_word(true)
            .case_sensitive(true)
            .disabled(true)
            .view(theme.clone()),
    ))
    .style(|style| style.gap(8.0));

    let size_section = v_stack((
        label(|| "Size display")
            .style(|style| style.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
        SearchBox::new("Small search")
            .size(InputSize::Sm)
            .value("small")
            .search_icon(SearchBoxIconMode::Visible)
            .clear_icon(SearchBoxIconMode::Visible)
            .submit_icon(SearchBoxIconMode::Visible)
            .disabled(true)
            .view(theme.clone()),
        SearchBox::new("Large search")
            .size(InputSize::Lg)
            .placeholder("Large")
            .search_icon(SearchBoxIconMode::Visible)
            .clear_icon(SearchBoxIconMode::Visible)
            .submit_icon(SearchBoxIconMode::Visible)
            .disabled(true)
            .view(theme.clone()),
    ))
    .style(|style| style.gap(8.0));

    scroll(
        v_stack((
            label(|| "SearchBox Samples").style(|style| style.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|style| style.font_size(13.0)),
            SearchBox::new("Live search")
                .search_icon(SearchBoxIconMode::Visible)
                .clear_icon(SearchBoxIconMode::Visible)
                .submit_icon(SearchBoxIconMode::Visible)
                .show_all_controls()
                .on_submit({
                    let submit_log = submit_log;
                    move |value| submit_log.set(format!("submit: {value}"))
                })
                .on_options_change({
                    let option_log = option_log;
                    move |options| {
                        option_log.set(format!(
                            "regex={}, word={}, case={}",
                            options.regex, options.whole_word, options.case_sensitive
                        ));
                    }
                })
                .view(theme.clone()),
            label(move || format!("callback log: {}", submit_log.get()))
                .style(|style| style.font_size(12.0)),
            label(move || format!("option log: {}", option_log.get()))
                .style(|style| style.font_size(12.0)),
            readonly_section,
            preset_section,
            control_section,
            size_section,
        ))
        .style(move |style| {
            style
                .gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
    .style(|style| style.min_width_full().flex_grow(1.0))
}

pub fn search_box_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
