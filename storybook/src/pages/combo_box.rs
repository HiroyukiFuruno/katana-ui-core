use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, h_stack, label, scroll, v_stack};
use katana_ui_widget::composite::combo_box::{ComboBox, ComboBoxOption, ComboBoxPlacement};
use katana_ui_widget::theme::Theme;

fn font_options() -> Vec<ComboBoxOption<String>> {
    vec![
        ComboBoxOption::new("Inter", "Inter".to_string()),
        ComboBoxOption::new("Noto Sans JP", "Noto Sans JP".to_string()),
        ComboBoxOption::new("Roboto", "Roboto".to_string()),
        ComboBoxOption::new("Source Han Sans", "Source Han Sans".to_string()),
    ]
}

fn file_options() -> Vec<ComboBoxOption<String>> {
    vec![
        ComboBoxOption::new("main.rs", "main.rs".to_string()),
        ComboBoxOption::new("lib.rs", "lib.rs".to_string()),
        ComboBoxOption::new("theme.rs", "theme.rs".to_string()),
    ]
}

fn should_start_open() -> bool {
    crate::interaction::open_requested("combo-box", "initial-open")
}

fn page_content(theme: Theme) -> impl IntoView {
    let bg = floem::peniko::Color::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text =
        floem::peniko::Color::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let font_selected = create_rw_signal("Inter".to_string());
    let font_input = create_rw_signal("".to_string());
    let file_selected = create_rw_signal("".to_string());
    let file_input = create_rw_signal("".to_string());
    let free_selected = create_rw_signal("".to_string());
    let free_input = create_rw_signal("".to_string());
    let placement_selected = create_rw_signal("未選択".to_string());

    let header =
        label(|| "ComboBox Samples").style(|style| style.font_size(18.0).margin_bottom(8.0));

    let font_label = label(move || format!("selected: {}", font_selected.get()));
    let font_input_label = label(move || format!("input: {}", font_input.get()));
    let file_label = label(move || {
        format!(
            "selected: {}, input: {}",
            file_selected.get(),
            file_input.get()
        )
    });
    let free_label = label(move || format!("input: {}", free_input.get()));
    let placement_label =
        label(move || format!("placement selected: {}", placement_selected.get()));

    let fonts = ComboBox::new(font_options(), "Font")
        .value("Inter".to_string())
        .strict(true)
        .open(should_start_open())
        .on_input_change({
            let font_input = font_input;
            move |text| {
                font_input.set(format!("type: {text}"));
            }
        })
        .on_select({
            let font_selected = font_selected;
            move |value| {
                font_selected.set(value);
            }
        });

    let files = ComboBox::new(file_options(), "File")
        .placeholder("ファイル名を入力")
        .on_input_change({
            let file_input = file_input;
            move |text| {
                file_input.set(text);
            }
        })
        .on_select({
            let file_selected = file_selected;
            move |value| {
                file_selected.set(value);
            }
        });

    let free = ComboBox::new(
        vec![ComboBoxOption::new(
            "tmp/log.txt",
            "tmp/log.txt".to_string(),
        )],
        "Free",
    )
    .placeholder("新規文字列を入力")
    .on_input_change({
        let free_input = free_input;
        move |text| {
            free_input.set(text);
        }
    })
    .on_select({
        let free_selected = free_selected;
        move |value| {
            free_selected.set(value);
        }
    });

    let state_section = v_stack((
        label(|| "状態差分")
            .style(|style| style.font_size(14.0).margin_top(12.0).margin_bottom(4.0)),
        ComboBox::new(font_options(), "Disabled")
            .placeholder("disabled")
            .disabled(true)
            .view(theme.clone()),
        ComboBox::new(font_options(), "Open")
            .placeholder("open on mount")
            .open(true)
            .view(theme.clone()),
    ))
    .style(|style| style.gap(8.0));

    let placement_section = v_stack((
        label(|| "配置差分（クリックで候補リストの開く位置を確認）")
            .style(|style| style.font_size(14.0).margin_top(12.0)),
        h_stack((
            ComboBox::new(font_options(), "BottomStart")
                .placeholder("BottomStart")
                .placement(ComboBoxPlacement::BottomStart)
                .on_select({
                    let placement_selected = placement_selected;
                    move |value| placement_selected.set(format!("BottomStart: {value}"))
                })
                .view(theme.clone()),
            ComboBox::new(font_options(), "TopEnd")
                .placeholder("TopEnd")
                .placement(ComboBoxPlacement::TopEnd)
                .on_select({
                    let placement_selected = placement_selected;
                    move |value| placement_selected.set(format!("TopEnd: {value}"))
                })
                .view(theme.clone()),
            ComboBox::new(font_options(), "End")
                .placeholder("End")
                .placement(ComboBoxPlacement::End)
                .on_select({
                    let placement_selected = placement_selected;
                    move |value| placement_selected.set(format!("End: {value}"))
                })
                .view(theme.clone()),
        ))
        .style(|style| style.gap(12.0).items_center()),
        placement_label,
    ))
    .style(|style| style.gap(8.0));

    scroll(
        v_stack((
            header,
            label(|| "フォント選択（strict）").style(|style| style.font_size(14.0)),
            fonts.view(theme.clone()),
            font_label,
            font_input_label,
            label(|| "ファイル名入力（選択肢あり）").style(|style| style.font_size(14.0)),
            files.view(theme.clone()),
            file_label,
            label(|| "自由入力").style(|style| style.font_size(14.0)),
            free.view(theme.clone()),
            free_label,
            label(move || format!("自由入力確定: {}", free_selected.get())),
            state_section,
            placement_section,
        ))
        .style(move |style| {
            style
                .gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .min_width_full()
        }),
    )
}

pub fn combo_box_page(theme: Theme) -> impl IntoView {
    page_content(theme)
}
