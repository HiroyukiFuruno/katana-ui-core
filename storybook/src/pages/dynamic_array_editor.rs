use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, label, scroll, v_stack};
use katana_ui_widget::composite::dynamic_array_editor::{
    DynamicArrayEditor, DynamicArrayEditorItem,
};
use katana_ui_widget::theme::Theme;

const MAX_ITEMS: usize = 4;

fn item_renderer(item: &DynamicArrayEditorItem<String>, index: usize) -> Box<dyn floem::View> {
    let value = item.value.clone();

    label(move || format!("{index}: {}", value.as_str()))
        .style(|style| style.font_size(14.0))
        .into_any()
}

fn sample_editor(theme: Theme, log: floem::reactive::RwSignal<String>) -> impl IntoView {
    let add_seed = create_rw_signal(0_usize);

    let editor = DynamicArrayEditor::new(
        vec![
            DynamicArrayEditorItem::new("A 画像 (編集可)".to_string()),
            DynamicArrayEditorItem::new("B 画像 (編集不可)".to_string()).deletable(false),
            DynamicArrayEditorItem::new("C 画像 (移動不可)".to_string()).reorderable(false),
        ],
        move || {
            let next = add_seed.get();
            add_seed.set(next + 1);
            DynamicArrayEditorItem::new(format!("New Item {}", next))
        },
        item_renderer,
    )
    .max_items(MAX_ITEMS)
    .empty_state("アイテムがありません")
    .on_change({
        let log = log;
        move |items| {
            log.set(format!("on_change: {} 件", items.len()));
        }
    })
    .on_add({
        let log = log;
        move |index| {
            log.set(format!("追加: index={index}"));
        }
    })
    .on_edit({
        let log = log;
        move |index| {
            log.set(format!("編集: index={index}"));
        }
    })
    .on_delete({
        let log = log;
        move |index| {
            log.set(format!("削除: index={index}"));
        }
    })
    .on_move({
        let log = log;
        move |from, to| {
            log.set(format!("並び替え: from={from}, to={to}"));
        }
    })
    .view(theme.clone());

    editor
}

fn empty_editor(theme: Theme) -> impl IntoView {
    DynamicArrayEditor::new(
        Vec::new(),
        || DynamicArrayEditorItem::new("empty".to_string()),
        |item, _| {
            let value = item.value.clone();
            label(move || value.clone()).into_any()
        },
    )
    .empty_state("空です。追加ボタンで項目を作れます。")
    .max_items(2)
    .view(theme)
}

fn disable_editor(theme: Theme, log: floem::reactive::RwSignal<String>) -> impl IntoView {
    DynamicArrayEditor::new(
        vec![DynamicArrayEditorItem::new("locked 1".to_string())],
        || DynamicArrayEditorItem::new("disabled".to_string()),
        |item, _| {
            let value = item.value.clone();
            label(move || value.clone()).into_any()
        },
    )
    .disabled(true)
    .max_items(MAX_ITEMS)
    .on_add({
        let log = log;
        move |_| {
            log.set("disabled: add not executed".to_string());
        }
    })
    .view(theme)
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let action_log = create_rw_signal("未操作".to_string());

    crate::interaction::replay("add-item", "dynamic-array-editor", "added-index-3", {
        let action_log = action_log;
        move || {
            action_log.set("追加: index=3".to_string());
        }
    });

    scroll(
        v_stack((
            label(|| "DynamicArrayEditor Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            container(sample_editor(theme.clone(), action_log)).style(|s| s.width_full()),
            label(|| "空状態の確認")
                .style(|s| s.margin_top(12.0).margin_bottom(8.0).font_size(14.0)),
            container(empty_editor(theme.clone())).style(|s| s.width_full()),
            label(|| "disabled 状態")
                .style(|s| s.margin_top(12.0).margin_bottom(8.0).font_size(14.0)),
            container(disable_editor(theme.clone(), action_log)).style(|s| s.width_full()),
            label(|| "入力ヒントと callback log").style(|s| s.margin_top(12.0).font_size(13.0)),
            container(label(move || format!("log: {}", action_log.get())))
                .style(move |s| s.padding_top(6.0).color(text)),
        ))
        .style(move |s| {
            s.gap(10.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .min_width_full()
        }),
    )
}

pub fn dynamic_array_editor_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
