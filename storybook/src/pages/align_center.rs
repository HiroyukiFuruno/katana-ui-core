use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, button, h_stack, label, scroll, v_stack};
use katana_ui_widget::composite::selector::color_picker::LabeledColorPicker;
use katana_ui_widget::layout::align_center::AlignCenterWrapper;
use katana_ui_widget::primitive::icon::{Icon, IconSize, IconSource};
use katana_ui_widget::theme::Theme;
use katana_ui_widget::theme::color::Color;

const ICON_CHECK: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' fill='currentColor'><path d='M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.75.75 0 1 1 1.06-1.06L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0z'/></svg>";

fn sample_view(
    title: &'static str,
    wrapper: AlignCenterWrapper,
    child: impl IntoView + 'static,
    theme: Theme,
) -> impl IntoView {
    let border = PenikoColor::rgb8(
        theme.color.border.r,
        theme.color.border.g,
        theme.color.border.b,
    );
    let text_color = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    v_stack((
        label(move || title).style(|s| s.font_size(12.0)),
        wrapper
            .view(theme.clone(), child)
            .style(move |s| s.border(1.0).border_color(border).padding(6.0)),
    ))
    .style(move |s| s.gap(6.0).color(text_color))
}

fn sample_area(theme: Theme) -> impl IntoView {
    let live = sample_view(
        "Live widget: horizontal + vertical + text + button",
        AlignCenterWrapper::new()
            .width(320.0)
            .height(140.0)
            .padding(12.0)
            .gap(6.0)
            .background(theme.color.surface),
        v_stack((
            label(|| "中央寄せラッパー"),
            h_stack((label(|| "text"), button(label(|| "button"))))
                .style(|s| s.gap(8.0).items_center()),
        ))
        .style(|s| s.gap(6.0).items_center()),
        theme.clone(),
    );

    let text_only_small = sample_view(
        "Small 140x70（幅/高さ固定）",
        AlignCenterWrapper::new()
            .width(140.0)
            .height(70.0)
            .padding(8.0)
            .background(theme.color.surface),
        label(|| "centered text only"),
        theme.clone(),
    );

    let button_only_medium = sample_view(
        "Medium 180x70（横中央のみ）",
        AlignCenterWrapper::new()
            .width(180.0)
            .height(70.0)
            .padding(8.0)
            .vertical(false)
            .background(theme.color.surface),
        button(label(|| "button only")),
        theme.clone(),
    );

    let disabled_button = sample_view(
        "Disabled visual（無効状態）",
        AlignCenterWrapper::new()
            .width(320.0)
            .height(72.0)
            .padding(8.0)
            .gap(8.0)
            .disabled(true),
        h_stack((label(|| "disabled"), button(label(|| "action")))).style(|s| s.gap(8.0)),
        theme.clone(),
    );

    v_stack((live, text_only_small, button_only_medium, disabled_button))
}

fn page_content(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_color = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "AlignCenterWrapper Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            sample_area(theme.clone()),
            label(|| "サイズ違いを比較")
                .style(|s| s.font_size(14.0).margin_top(12.0).margin_bottom(8.0)),
            sample_view(
                "button（260x100）",
                AlignCenterWrapper::new()
                    .width(260.0)
                    .height(100.0)
                    .padding(10.0)
                    .background(theme.color.surface),
                button(label(|| "中央のボタン")),
                theme.clone(),
            ),
            sample_view(
                "color picker（320x96）",
                AlignCenterWrapper::new()
                    .width(320.0)
                    .height(96.0)
                    .padding(6.0)
                    .background(theme.color.surface),
                LabeledColorPicker::new(
                    "Accent",
                    Color {
                        r: theme.color.accent.r,
                        g: theme.color.accent.g,
                        b: theme.color.accent.b,
                        a: 255,
                    },
                )
                .rgba(true)
                .view(theme.clone()),
                theme.clone(),
            ),
            sample_view(
                "icon（160x80）",
                AlignCenterWrapper::new()
                    .width(160.0)
                    .height(80.0)
                    .padding(8.0)
                    .background(theme.color.surface),
                Icon::new(IconSource::SvgBytes(ICON_CHECK))
                    .size(IconSize::Lg)
                    .color_override(theme.color.accent)
                    .view(theme.clone()),
                theme.clone(),
            ),
        ))
        .style(move |s| {
            s.gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text_color)
                .min_width_full()
        }),
    )
    .style(|style| style.width_full().height_full().flex_grow(1.0))
}

pub fn align_center_page(theme: Theme) -> impl IntoView {
    page_content(theme)
}
