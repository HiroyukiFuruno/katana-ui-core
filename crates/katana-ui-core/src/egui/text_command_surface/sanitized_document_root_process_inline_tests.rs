use super::super::sanitized_document_root_input::{
    SanitizedDocumentRootIdentity, SanitizedDocumentRootInput,
};
use super::super::sanitized_document_root_record::SanitizedDocumentRootRecord;
use super::super::sanitized_document_root_style::SanitizedDocumentRootStyleKey;
use super::super::{
    SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
    SanitizedCommandTarget, SanitizedSearchControlPresentation,
    SanitizedSearchLocalizedPresentation, SanitizedSearchOperationPresentation,
    SanitizedSearchProjection, SanitizedSearchProjectionBuilder,
    SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget,
    SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation, SanitizedTab,
    SanitizedTabCapabilities, SanitizedTabGroup, SanitizedTabProjection, SanitizedTabTarget,
};
use super::{SanitizedDocumentRootProcess, SanitizedDocumentRootProcessError};
use crate::molecule::structured::CloseableTabStripEvent;
use crate::render_model::UiIconProps;

const ROOT_VIEWPORT_SIZE: egui::Vec2 = egui::vec2(640.0, 480.0);

fn input(revision: u64, identity: &[u8], snapshot: &str) -> SanitizedDocumentRootInput {
    SanitizedDocumentRootInput::new(
        revision,
        SanitizedDocumentRootIdentity::from_opaque_bytes(identity.to_vec()),
        snapshot,
        SanitizedDocumentRootStyleKey::Default,
    )
}

fn input_with_projection(
    revision: u64,
    identity: &[u8],
    snapshot: &str,
    command_projection: SanitizedCommandProjection,
) -> SanitizedDocumentRootInput {
    input(revision, identity, snapshot).with_command_projection(command_projection)
}

fn input_with_search_projection(
    revision: u64,
    identity: &[u8],
    snapshot: &str,
    search_projection: SanitizedSearchProjection,
) -> SanitizedDocumentRootInput {
    input(revision, identity, snapshot).with_search_projection(search_projection)
}

fn context_projection(label: &str, target: u8) -> super::super::SanitizedContextMenuProjection {
    super::super::SanitizedContextMenuProjectionBuilder::new()
        .item(super::super::SanitizedContextMenuItem::new(
            super::super::SanitizedContextMenuTarget::from_opaque_bytes([target]),
            1,
            label,
        ))
        .build()
}

fn input_with_context_projection(
    revision: u64,
    identity: &[u8],
    snapshot: &str,
    context_projection: super::super::SanitizedContextMenuProjection,
) -> SanitizedDocumentRootInput {
    input(revision, identity, snapshot).with_context_projection(context_projection)
}

fn input_with_tab_projection(
    revision: u64,
    identity: &[u8],
    snapshot: &str,
    tab_projection: SanitizedTabProjection,
) -> SanitizedDocumentRootInput {
    input(revision, identity, snapshot).with_tab_projection(tab_projection)
}

fn projection(label: &str) -> SanitizedCommandProjection {
    SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "g").item(
        SanitizedCommandItem::new(
            SanitizedCommandTarget::from_opaque_bytes(label.as_bytes()),
            1,
            label,
        )
        .with_icon(UiIconProps::new("<svg/>")),
    )])
}

fn search_text(value: &str) -> SanitizedSearchTextPresentation {
    SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
}

fn localized_search(next: &str) -> SanitizedSearchLocalizedPresentation {
    SanitizedSearchLocalizedPresentation::new(
        SanitizedSearchControlPresentation::new(
            search_text("検索"),
            search_text("検索語"),
            search_text("置換"),
            search_text("大文字小文字"),
            search_text("単語"),
            search_text("正規表現"),
        ),
        SanitizedSearchOperationPresentation::new(
            search_text("前へ"),
            search_text(next),
            search_text("置換"),
            search_text("すべて置換"),
            search_text("閉じる"),
        ),
        SanitizedSearchResultSummaryPresentation::new(
            "検索待機 ⭐️",
            "一致なし ⭐️",
            "一件 ⭐️",
            "位置 ⭐️",
            "件数 ⭐️",
        ),
        SanitizedSearchUnavailablePresentation::new(
            "正規表現は利用不可 ⭐️",
            "置換は利用不可 ⭐️",
            "移動は利用不可 ⭐️",
            "閉じる操作は利用不可 ⭐️",
        ),
    )
}

fn search_projection(label: &str, target: u8) -> Result<SanitizedSearchProjection, String> {
    SanitizedSearchProjectionBuilder::new()
        .localized_presentation(localized_search(label))
        .next_enabled(true)
        .next_target(SanitizedSearchTarget::from_opaque_bytes([target]))
        .build()
        .map_err(|error| format!("{error:?}"))
}

fn tab_projection(second_label: &str) -> SanitizedTabProjection {
    SanitizedTabProjection::new([SanitizedTabGroup::new(
        crate::egui::text_command_surface::sanitized_document_root::sanitized_tab_projection::SanitizedTabGroupTarget::from_opaque_bytes([0]),
        0,
        "ドキュメント",
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "最初")
            .with_capabilities(
                SanitizedTabCapabilities::new()
                    .active_state(true)
                    .close_state(true),
            ),
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([2]), 1, second_label)
            .with_capabilities(SanitizedTabCapabilities::new().close_state(true)),
    )])
}

fn render_record(
    process: &mut SanitizedDocumentRootProcess,
    context: &egui::Context,
) -> Result<SanitizedDocumentRootRecord, String> {
    let mut output = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                ROOT_VIEWPORT_SIZE,
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                output = Some(process.show(ui).map_err(|error| error.to_string()));
            });
        },
    );
    platform_output.textures_delta.clear();
    let output = output.ok_or_else(|| "frame output was not produced".to_owned())??;
    Ok(SanitizedDocumentRootRecord::from_output(
        process.input.revision,
        &output,
    ))
}

include!("sanitized_document_root_process_inline_tests/synchronization.rs");
include!("sanitized_document_root_process_inline_tests/rendering.rs");
