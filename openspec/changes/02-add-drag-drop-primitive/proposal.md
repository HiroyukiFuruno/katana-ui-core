## Why

`katana` の explorer（ファイルツリー）、tab bar、editor toolbar、chat composer の添付（attachment）といった多くの画面が drag & drop（DnD）を前提とした操作面を持つ。しかし `katana-ui-core` の `event` model には pointer / keyboard / focus / command 系のみが定義されており、drag source / drop target / drop indicator / drag preview を表す共通 model が存在しない。

この結果、explorer や tab bar の DnD ロジックは KUC の外で再実装されており、ホットスポット判定（reorder vs insert）や drop indicator の描画、escape での cancel、autoscroll 開始しきい値などが consumer ごとに別実装になっている。これらは入力回帰 / 画像回帰の対象から漏れ、結果として ad-hoc な挙動の差が積み上がる。

## What Changes

- `event` module に `DragEvent`（`DragStart` / `DragMove` / `DragEnter` / `DragLeave` / `DragOver` / `Drop` / `DragCancel` / `DragEnd`）を追加し、`UiEvent` enum に組み込む。
- `interaction` module に `DragSource` / `DropTarget` / `DropZone` / `DropEffect`（`Move` / `Copy` / `Link` / `None`）/ `DragData`（typed payload + MIME-like tag）を追加する。
- molecule layer に `DragHandle` atom（明示的なドラッグつまみ）、`DropIndicator` atom（before / after / inside / none）、`DragPreview` molecule（半透明プレビュー描画 model）を追加する。
- autoscroll しきい値（edge zone size、加速度カーブ）、reorder vs insert の判定 model、Esc でのキャンセル、drop accept 判定 callback を契約に含める。
- keyboard accessibility として「Space で pick up、矢印で移動、Space で drop、Esc で cancel」のキーボードドラッグを契約に含める。
- adapter contract に native OS DnD（OS ファイル / 外部 URL）の escape hatch を定義する（OS payload は core model に持ち込まない）。

## Capabilities

### New Capabilities

- `kuc-drag-drop`: drag source / drop target / drop indicator / drag preview / keyboard drag / autoscroll / cancel の契約を定義する。

### Modified Capabilities

- `kuc-core-foundation`: `UiEvent` enum に `DragEvent` を追加し、event bubbling / capture policy に drag 系を含めることを明記する。

## Impact

- `crates/katana-ui-core/src/event/` に `drag.rs` を追加し、`mod.rs` を更新する。
- `crates/katana-ui-core/src/interaction/` に `drag_source.rs` / `drop_target.rs` / `drag_data.rs` を追加する。
- `crates/katana-ui-core/src/atom/` に `drag_handle.rs` / `drop_indicator.rs` を追加する。
- `crates/katana-ui-core/src/molecule/` に `drag_preview.rs` を追加する。
- external runtime は native DnD を escape hatch に変換する責務を持つ（core contract で確認）。
- Storybook に DnD playground（reorder list / file drop / tab reorder / attachment drop）を追加する。
