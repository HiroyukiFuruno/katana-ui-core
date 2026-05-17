use floem::IntoView;
use floem::View;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, button, container, empty, h_stack, label, scroll, v_stack, v_stack_from_iter,
};
use katana_ui_core::layout::accordion::{
    Accordion, AccordionGroup, AccordionGroupItem, AccordionTreeMode, AccordionTriggerArea,
    IndicatorPosition,
};
use katana_ui_core::theme::Theme;

fn body_row(title: &'static str, body: &'static str) -> impl IntoView {
    v_stack((
        label(|| title.to_string()).style(|style| style.font_size(12.0)),
        label(|| body.to_string()).style(|style| style.font_size(11.0)),
    ))
    .style(|style| {
        style
            .gap(3.0)
            .padding_left(8.0)
            .padding_right(8.0)
            .padding_vert(8.0)
    })
}

fn rich_header(theme: Theme) -> impl IntoView {
    let accent = PenikoColor::rgb8(
        theme.color.accent.r,
        theme.color.accent.g,
        theme.color.accent.b,
    );
    h_stack((
        container(empty()).style(move |style| {
            style
                .width(8.0)
                .height(20.0)
                .border_radius(4.0)
                .background(accent)
        }),
        v_stack((
            label(|| "任意表示（Node）".to_string()).style(|style| style.font_size(13.0)),
            label(|| "開いた領域にも別の表示部品を配置".to_string())
                .style(|style| style.font_size(11.0)),
        ))
        .style(|style| style.gap(2.0)),
    ))
    .style(|style| style.gap(8.0).items_center())
}

fn rich_body(theme: Theme) -> impl IntoView {
    let accent = PenikoColor::rgb8(
        theme.color.accent.r,
        theme.color.accent.g,
        theme.color.accent.b,
    );
    h_stack((
        container(empty()).style(move |style| {
            style
                .width(48.0)
                .height(32.0)
                .border_radius(4.0)
                .background(accent)
        }),
        body_row("本文領域", "枠線あり・任意表示"),
    ))
    .style(|style| style.gap(8.0).items_center())
}

pub fn accordion_page(theme: Theme) -> impl IntoView {
    let theme = theme.clone();
    let accordion_log = create_rw_signal("待機中".to_string());
    let controlled_state = create_rw_signal(false);
    let uncontrolled_state = create_rw_signal(false);

    let default_closed = Accordion::new("default closed").on_toggle({
        let accordion_log = accordion_log;
        move |is_open| {
            accordion_log.set(format!("default closed toggled: {is_open}"));
        }
    });
    let expanded = {
        let header_theme = theme.clone();
        Accordion::new("expanded by default")
            .header(move || rich_header(header_theme.clone()))
            .expanded(true)
            .body_border(true)
            .on_toggle({
                let accordion_log = accordion_log;
                move |is_open| {
                    accordion_log.set(format!("expanded toggled: {is_open}"));
                }
            })
    };
    let leading = Accordion::new("leading indicator").indicator(IndicatorPosition::Leading);
    let none = Accordion::new("no indicator").indicator(IndicatorPosition::None);
    let disabled = Accordion::new("disabled").disabled(true).on_toggle({
        let accordion_log = accordion_log;
        move |is_open| {
            accordion_log.set(format!("disabled toggled unexpectedly: {is_open}"));
        }
    });

    let controlled = {
        let signal_for_toggle = controlled_state;
        let accordion_log = accordion_log;
        Accordion::new("controlled")
            .controlled(controlled_state)
            .on_toggle(move |is_open| {
                signal_for_toggle.set(is_open);
                accordion_log.set(format!("controlled callback: {is_open}"));
            })
    };

    let uncontrolled = {
        let signal_for_toggle = uncontrolled_state;
        let accordion_log = accordion_log;
        Accordion::new("uncontrolled")
            .expanded(false)
            .uncontrolled()
            .on_toggle(move |is_open| {
                signal_for_toggle.set(is_open);
                accordion_log.set(format!("uncontrolled callback: {is_open}"));
            })
    };

    let controlled_controls = h_stack((
        button(label(|| "controlled open")).action({
            let controlled_state = controlled_state;
            let accordion_log = accordion_log;
            move || {
                controlled_state.set(true);
                accordion_log.set("controlled external open".to_string());
            }
        }),
        button(label(|| "controlled close")).action({
            let controlled_state = controlled_state;
            let accordion_log = accordion_log;
            move || {
                controlled_state.set(false);
                accordion_log.set("controlled external close".to_string());
            }
        }),
    ))
    .style(|style| style.gap(4.0));

    let trigger_areas = v_stack((
        Accordion::new("Icon + Label")
            .trigger_area(AccordionTriggerArea::IconAndLabel)
            .view(theme.clone(), || {
                body_row("icon + label", "領域: アイコンと文字")
            }),
        Accordion::new("Icon Only")
            .indicator(IndicatorPosition::Leading)
            .trigger_area(AccordionTriggerArea::IconOnly)
            .view(theme.clone(), || {
                body_row("icon only", "領域: アイコンだけ")
            }),
        Accordion::new("Label Only")
            .indicator(IndicatorPosition::Trailing)
            .trigger_area(AccordionTriggerArea::LabelOnly)
            .view(theme.clone(), || body_row("label only", "領域: 文字だけ")),
        Accordion::new("Full Row")
            .trigger_area(AccordionTriggerArea::FullRow)
            .view(theme.clone(), || body_row("full row", "領域: 行全体")),
    ));

    let exclusive = AccordionGroup::new()
        .allow_multiple(false)
        .push(
            AccordionGroupItem::new("Exclusive A", || body_row("A", "同時展開: 1件目"))
                .tree_depth(0),
        )
        .push(
            AccordionGroupItem::new("Exclusive B", || body_row("B", "同時展開: 2件目"))
                .tree_depth(0),
        )
        .push(
            AccordionGroupItem::new("Exclusive C", || body_row("C", "同時展開: 3件目"))
                .tree_depth(0),
        );

    let multiple = AccordionGroup::new()
        .allow_multiple(true)
        .push(
            AccordionGroupItem::new("Multiple A", || body_row("A", "同時展開: true")).tree_depth(0),
        )
        .push(
            AccordionGroupItem::new("Multiple B", || body_row("B", "同時展開: true")).tree_depth(0),
        );

    let tree_mode = AccordionGroup::new()
        .allow_multiple(false)
        .push(
            AccordionGroupItem::new("Tree Parent", || body_row("tree", "親ノード（子供あり）"))
                .tree_mode(AccordionTreeMode::Enabled)
                .tree_depth(0)
                .tree_has_children(true)
                .tree_selected(true),
        )
        .push(
            AccordionGroupItem::new("Tree Child", || body_row("child", "深さ1"))
                .tree_mode(AccordionTreeMode::Enabled)
                .tree_depth(1),
        )
        .push(
            AccordionGroupItem::new("Tree Child 2", || body_row("child", "深さ1"))
                .tree_mode(AccordionTreeMode::Enabled)
                .tree_depth(1)
                .tree_show_lines(false),
        )
        .push(
            AccordionGroupItem::new("Tree Sibling", || body_row("tree", "別支線"))
                .tree_mode(AccordionTreeMode::Enabled)
                .tree_depth(0),
        );

    let mut content = Vec::<Box<dyn View>>::new();

    content.push(
        label(|| "Accordion Samples".to_string())
            .style(|style| style.font_size(18.0).margin_bottom(8.0))
            .into_any(),
    );
    content.push(
        label(|| "表示状態".to_string())
            .style(|style| style.font_size(14.0))
            .into_any(),
    );
    content.push(
        default_closed
            .view(theme.clone(), || body_row("collapsed", "body"))
            .into_any(),
    );
    content.push(
        expanded
            .view(theme.clone(), {
                let body_theme = theme.clone();
                move || rich_body(body_theme.clone())
            })
            .into_any(),
    );
    content.push(
        leading
            .view(theme.clone(), || body_row("leading", "body"))
            .into_any(),
    );
    content.push(
        none.view(theme.clone(), || body_row("none", "body"))
            .into_any(),
    );
    content.push(
        disabled
            .view(theme.clone(), || body_row("disabled", "body"))
            .into_any(),
    );
    content.push(
        label(|| "制御モード".to_string())
            .style(|style| style.font_size(14.0).margin_top(8.0))
            .into_any(),
    );
    content.push(
        label(move || {
            format!(
                "操作結果 controlled={} uncontrolled={}",
                controlled_state.get(),
                uncontrolled_state.get()
            )
        })
        .style(|style| style.font_size(11.0))
        .into_any(),
    );
    content.push(
        label(move || format!("callback log: {}", accordion_log.get()))
            .style(|style| style.font_size(11.0))
            .into_any(),
    );
    content.push(controlled_controls.into_any());
    content.push(
        controlled
            .view(theme.clone(), || body_row("controlled", "open signal"))
            .into_any(),
    );
    content.push(
        uncontrolled
            .view(theme.clone(), || body_row("uncontrolled", "内部state"))
            .into_any(),
    );
    content.push(
        label(|| "クリック領域".to_string())
            .style(|style| style.font_size(14.0).margin_top(8.0))
            .into_any(),
    );
    content.push(trigger_areas.into_any());
    content.push(
        label(|| "同時展開制御".to_string())
            .style(|style| style.font_size(14.0).margin_top(8.0))
            .into_any(),
    );

    let mut expand_group = Vec::<Box<dyn View>>::new();
    expand_group.push(exclusive.view(theme.clone()).into_any());
    expand_group.push(multiple.view(theme.clone()).into_any());
    content.push(
        v_stack_from_iter(expand_group)
            .style(|style| style.gap(6.0))
            .into_any(),
    );

    content.push(
        label(|| "Tree mode".to_string())
            .style(|style| style.font_size(14.0).margin_top(8.0))
            .into_any(),
    );
    content.push(tree_mode.view(theme.clone()).into_any());
    content.push(
        label(|| "Reduced motion".to_string())
            .style(|style| style.font_size(14.0).margin_top(8.0))
            .into_any(),
    );
    content.push(
        Accordion::new("motion on")
            .reduced_motion(false)
            .view(theme.clone(), || body_row("normal", "アニメーションあり"))
            .into_any(),
    );
    content.push(
        Accordion::new("motion off")
            .reduced_motion(true)
            .view(theme.clone(), || body_row("reduced", "アニメーション無効"))
            .into_any(),
    );

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let content = v_stack_from_iter(content).style(move |style| {
        style
            .padding(16.0)
            .gap(8.0)
            .background(bg)
            .color(text)
            .width_full()
    });

    scroll(content).style(|style| {
        style
            .width_full()
            .height_full()
            .flex_grow(1.0)
            .min_width(0.0)
    })
}
