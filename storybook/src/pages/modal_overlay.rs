use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, RwSignal, SignalGet, SignalUpdate};
use floem::views::{button, container, dyn_container, empty, h_stack, label, scroll, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::layout::modal::{Modal, ModalSize};
use katana_ui_widget::theme::Theme;

#[derive(Clone, Copy)]
enum FooterSample {
    Confirm,
    Form,
    Detail,
}

impl FooterSample {
    fn label(self) -> &'static str {
        match self {
            Self::Confirm => "confirm",
            Self::Form => "form",
            Self::Detail => "detail",
        }
    }

    fn body(self) -> &'static str {
        match self {
            Self::Confirm => "保存前に内容を確認します。",
            Self::Form => "必須項目を入力してから実行してください。",
            Self::Detail => "詳細確認の完了結果を表示します。",
        }
    }
}

#[derive(Clone)]
struct OpenPreset {
    label: &'static str,
    size: ModalSize,
    title: &'static str,
    body: &'static str,
    footer: FooterSample,
    dismiss_on_backdrop: bool,
    dismiss_on_esc: bool,
}

fn size_label(size: &ModalSize) -> &'static str {
    match size {
        ModalSize::Sm => "Sm",
        ModalSize::Md => "Md",
        ModalSize::Lg => "Lg",
        ModalSize::Custom(_) => "Custom",
    }
}

fn open_button(
    preset: OpenPreset,
    is_open: RwSignal<bool>,
    selected_size: RwSignal<ModalSize>,
    selected_title: RwSignal<String>,
    selected_body: RwSignal<String>,
    selected_footer: RwSignal<FooterSample>,
    dismiss_on_backdrop: RwSignal<bool>,
    dismiss_on_esc: RwSignal<bool>,
) -> impl IntoView {
    let label_text = preset.label;
    let size = preset.size;
    let title = preset.title;
    let body = preset.body;
    let footer = preset.footer;
    let dismiss_backdrop = preset.dismiss_on_backdrop;
    let dismiss_escape = preset.dismiss_on_esc;
    button(label(move || label_text)).action(move || {
        is_open.set(true);
        selected_size.set(size.clone());
        selected_title.set(title.to_string());
        selected_body.set(body.to_string());
        selected_footer.set(footer);
        dismiss_on_backdrop.set(dismiss_backdrop);
        dismiss_on_esc.set(dismiss_escape);
    })
}

fn modal_layer(
    theme: Theme,
    is_open: RwSignal<bool>,
    selected_size: RwSignal<ModalSize>,
    selected_title: RwSignal<String>,
    selected_body: RwSignal<String>,
    selected_footer: RwSignal<FooterSample>,
    dismiss_on_backdrop: RwSignal<bool>,
    dismiss_on_esc: RwSignal<bool>,
    close_log: RwSignal<String>,
) -> impl IntoView {
    let overlay_bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_color = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let theme = theme.clone();

    dyn_container(
        move || {
            (
                is_open.get(),
                selected_size.get(),
                selected_title.get(),
                selected_body.get(),
                selected_footer.get(),
                dismiss_on_backdrop.get(),
                dismiss_on_esc.get(),
            )
        },
        move |(open_now, selected_size_now, selected_title_now, selected_body_now, selected_footer_now, dismiss_backdrop, dismiss_esc)| {
            if !open_now {
                return container(empty())
                    .style(move |s| s.background(overlay_bg).padding(12.0).min_width_full());
            }

            let close = {
                let is_open = is_open;
                let close_log = close_log;
                move || {
                    is_open.set(false);
                    close_log.set("on_close() が呼ばれた".to_string());
                }
            };

            let modal = Modal::new()
                .open(true)
                .size(selected_size_now.clone())
                .title(selected_title_now.clone())
                .children(selected_body_now.clone())
                .footer(selected_footer_now.body())
                .dismiss_on_backdrop(dismiss_backdrop)
                .dismiss_on_esc(dismiss_esc)
                .on_close(close)
                .resolve(&theme);
            let selected_title_text = selected_title_now.clone();
            let selected_body_text = selected_body_now.clone();
            let footer_label = selected_footer_now.label();
            let footer_body = selected_footer_now.body();

            let overlay_color = PenikoColor::rgba8(
                modal.overlay_color.r,
                modal.overlay_color.g,
                modal.overlay_color.b,
                modal.overlay_color.a,
            );

            let close_btn = {
                let on_close = modal.on_close.clone();
                button(label(|| "close [x]".to_string())).action(move || {
                    on_close();
                })
            };

            let backdrop_btn = {
                let modal = modal.clone();
                let close_log = close_log;
                button(label(|| "close_by_backdrop()".to_string())).action(move || {
                    if modal.close_with_backdrop() {
                        close_log.set("backdrop: closed".to_string());
                    } else {
                        close_log.set("backdrop: ignored".to_string());
                    }
                })
            };

            let esc_btn = {
                let modal = modal.clone();
                let close_log = close_log;
                button(label(|| "close_by_esc()".to_string())).action(move || {
                    if modal.close_with_esc() {
                        close_log.set("Esc: closed".to_string());
                    } else {
                        close_log.set("Esc: ignored".to_string());
                    }
                })
            };

            let status = h_stack((
                label(move || format!("size={}", size_label(&selected_size_now))),
                label(move || format!("footer={footer_label}")),
            ))
            .style(|s| s.gap(8.0));

            let title_font_size = modal.title_font_size;
            let content_gap = modal.content_gap;
            let footer_gap = modal.footer_gap;
            let dialog_width = modal.dialog_width;
            let dialog_padding = modal.padding;
            let dialog_border_radius = modal.corner_radius;
            let dialog_bg = PenikoColor::rgb8(modal.dialog_bg.r, modal.dialog_bg.g, modal.dialog_bg.b);
            let dialog_border = PenikoColor::rgb8(
                modal.dialog_border.r,
                modal.dialog_border.g,
                modal.dialog_border.b,
            );
            let dialog_text = PenikoColor::rgb8(
                modal.title_color.r,
                modal.title_color.g,
                modal.title_color.b,
            );

            let dialog = v_stack((
                label(move || selected_title_text.clone()).style(move |s| s.font_size(title_font_size)),
                status,
                label(move || format!("body: {}", selected_body_text))
                    .style(move |s| s.font_size(11.0).margin_top(content_gap)),
                label(move || format!("foot: {footer_body}"))
                    .style(move |s| s.font_size(11.0).margin_top(footer_gap)),
                h_stack((close_btn, backdrop_btn, esc_btn)).style(|s| s.gap(8.0)),
            ))
            .style(move |s| {
                s.width(dialog_width)
                    .background(dialog_bg)
                    .border(1.0)
                    .border_color(dialog_border)
                    .border_radius(dialog_border_radius)
                    .padding(dialog_padding)
                    .gap(content_gap)
                    .color(dialog_text)
            });

            container(dialog)
                .style(move |s| {
                    s.background(overlay_color)
                        .padding(24.0)
                        .min_width_full()
                        .color(text_color)
                })
        },
    )
}

fn sample_buttons(
    is_open: RwSignal<bool>,
    selected_size: RwSignal<ModalSize>,
    selected_title: RwSignal<String>,
    selected_body: RwSignal<String>,
    selected_footer: RwSignal<FooterSample>,
    dismiss_on_backdrop: RwSignal<bool>,
    dismiss_on_esc: RwSignal<bool>,
) -> (impl IntoView, impl IntoView) {
    let size_buttons = h_stack((
        open_button(
            OpenPreset {
                label: "Open Sm",
                size: ModalSize::Sm,
                title: "Small modal",
                body: "Smallサイズで開くサンプル",
                footer: FooterSample::Confirm,
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
        open_button(
            OpenPreset {
                label: "Open Md",
                size: ModalSize::Md,
                title: "Medium modal",
                body: "Mediumサイズで開くサンプル",
                footer: FooterSample::Form,
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
        open_button(
            OpenPreset {
                label: "Open Lg",
                size: ModalSize::Lg,
                title: "Large modal",
                body: "Largeサイズで開くサンプル",
                footer: FooterSample::Detail,
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
        open_button(
            OpenPreset {
                label: "Open Custom",
                size: ModalSize::Custom(360.0),
                title: "Custom modal",
                body: "Customサイズ(360)で開くサンプル",
                footer: FooterSample::Confirm,
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
    ))
    .style(|s| s.gap(8.0));

    let footer_buttons = h_stack((
        open_button(
            OpenPreset {
                label: "Footer confirm",
                size: ModalSize::Md,
                title: "Footer Confirm",
                body: "confirm用フッター",
                footer: FooterSample::Confirm,
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
        open_button(
            OpenPreset {
                label: "Footer form",
                size: ModalSize::Md,
                title: "Footer Form",
                body: "form用フッター",
                footer: FooterSample::Form,
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
        open_button(
            OpenPreset {
                label: "Footer detail",
                size: ModalSize::Md,
                title: "Footer Detail",
                body: "detail用フッター",
                footer: FooterSample::Detail,
                dismiss_on_backdrop: true,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
        open_button(
            OpenPreset {
                label: "dismiss_on_backdrop=false",
                size: ModalSize::Md,
                title: "Backdrop 無効",
                body: "Backdrop で閉じないケース",
                footer: FooterSample::Confirm,
                dismiss_on_backdrop: false,
                dismiss_on_esc: true,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
        open_button(
            OpenPreset {
                label: "dismiss_on_esc=false",
                size: ModalSize::Md,
                title: "Esc 無効",
                body: "Esc で閉じないケース",
                footer: FooterSample::Confirm,
                dismiss_on_backdrop: true,
                dismiss_on_esc: false,
            },
            is_open,
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
        ),
    ))
    .style(|s| s.gap(8.0));

    (size_buttons, footer_buttons)
}

fn page_content(theme: Theme) -> impl IntoView {
    let is_open = create_rw_signal(false);
    let selected_size = create_rw_signal(ModalSize::Md);
    let selected_title = create_rw_signal("確認ダイアログ".to_string());
    let selected_body = create_rw_signal("実行してもよいですか？".to_string());
    let selected_footer = create_rw_signal(FooterSample::Confirm);
    let dismiss_on_backdrop = create_rw_signal(true);
    let dismiss_on_esc = create_rw_signal(true);
    let close_log = create_rw_signal("closed".to_string());
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);

    let modal_area = modal_layer(
        theme.clone(),
        is_open,
        selected_size,
        selected_title,
        selected_body,
        selected_footer,
        dismiss_on_backdrop,
        dismiss_on_esc,
        close_log,
    );

    let (size_buttons, footer_buttons) = sample_buttons(
        is_open,
        selected_size,
        selected_title,
        selected_body,
        selected_footer,
        dismiss_on_backdrop,
        dismiss_on_esc,
    );

    let status = v_stack((
        label(move || {
            let state = if is_open.get() { "Open" } else { "Closed" };
            format!(
                "open={state} size={} footer={}",
                size_label(&selected_size.get()),
                selected_footer.get().label(),
            )
        }),
        label(move || {
            let backdrop_flag = if dismiss_on_backdrop.get() { "true" } else { "false" };
            let esc_flag = if dismiss_on_esc.get() { "true" } else { "false" };
            format!("close setting: backdrop={backdrop_flag} esc={esc_flag}")
        }),
        label(move || format!("close log: {}", close_log.get())),
    ));

    scroll(
        v_stack((
            label(|| "Modal Overlay Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            dyn_container(
                move || {
                    (
                        is_open.get(),
                        selected_size.get(),
                        selected_title.get(),
                        selected_body.get(),
                        selected_footer.get(),
                        dismiss_on_backdrop.get(),
                        dismiss_on_esc.get(),
                    )
                },
                {
                    let theme = theme.clone();
                    move |(
                        open,
                        size,
                        title,
                        body,
                        footer,
                        dismiss_backdrop,
                        dismiss_escape,
                    ): (
                        bool,
                        ModalSize,
                        String,
                        String,
                        FooterSample,
                        bool,
                        bool,
                    )| {
                        Modal::new()
                            .open(open)
                            .size(size)
                            .title(title)
                            .children(body)
                            .footer(footer.body())
                            .dismiss_on_backdrop(dismiss_backdrop)
                            .dismiss_on_esc(dismiss_escape)
                            .on_close({
                                let close_log = close_log;
                                let is_open = is_open;
                                move || {
                                    is_open.set(false);
                                    close_log.set("Modal::view on_close()".to_string());
                                }
                            })
                            .view(theme.clone())
                    }
                },
            ),
            label(|| "開閉トリガー + 各 size のサンプル").style(|s| s.font_size(13.0)),
            size_buttons,
            label(|| "title/footer slot（confirm / form / detail）").style(|s| s.font_size(13.0)),
            footer_buttons,
            label(|| "状態").style(|s| s.font_size(13.0)),
            status,
            label(|| "close ボタンは直接 on_close / close_with_backdrop / close_with_esc をそれぞれ実行します")
                .style(|s| s.font_size(11.0)),
            modal_area,
        ))
        .style(move |s| {
            s.gap(8.0)
                .padding(16.0)
                .background(bg)
                .min_width_full()
        }),
    )
}

pub fn modal_overlay_page(theme: Theme) -> impl IntoView {
    page_content(theme)
}
