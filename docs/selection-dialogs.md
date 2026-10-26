# 汎用選択ダイアログ

`EguiTextCommandSurfaceHostRoot::synchronize_selection_dialog` は、保持中のrootへ選択画面を接続する。
hostは対象と表示情報を投影し、KUCは描画、入力、スクロール、フォーカス、AccessKitを管理する。
対象の意味、検索処理、ファイル読み書きはhostが担当する。

## ファイル選択

```rust
use katana_ui_core::egui::text_command_surface::{
    FileSelectionItem, FileSelectionProjection, KucDialogControls, KucDialogTarget,
    SelectionDialogProjection, SelectionDialogProjectionLease,
};

let projection = SelectionDialogProjection::File(FileSelectionProjection {
    items: vec![FileSelectionItem {
        target: KucDialogTarget::from_opaque_bytes(b"entry-1".to_vec()),
        label: "日本語の資料.txt".into(),
        depth: 0,
        navigable: false,
        selectable: true,
        disabled: false,
    }],
    failure: None,
});
```

表示名は識別子として解釈されない。ディレクトリなどの移動可能な項目には
`navigable: true` を指定し、`Navigated` の対象を受けたhostが新しいrevisionを投影する。
`FileSelected`、`Cancelled`、`Failed` は別の操作として通知される。

## 検索結果

`SearchResultProjection` は複数の `SearchResultMode` と選択中のmodeを持つ。
modeごとにquery、任意のinclude/exclude、options、history、resultsを投影する。
結果行のhighlightはsnippet（なければlabel）のUTF-8 byte範囲で、書記素境界以外は拒否される。
`searched: false` は未検索、`true` で空のresultsは検索結果なしを表す。

## 接続と通知

画面内の全操作名は `KucDialogControls` でローカライズする。
`SelectionDialogProjectionLease::new(revision, title, controls, projection)` をrootへ渡し、
通常の `show` と `forward_events_once` を使う。
同じrevisionまたは古いrevisionは拒否される。
`readonly(true)` / `disabled(true)` は選択や変更を抑止し、取消は利用できる。

`KucRootEventBatchDispatcher::dispatch_selection_dialog_events` またはhost routerの
`KucRootEventBatchContext::selection_dialog_events` からtyped operationとopaque targetを受け取る。
転送は他のrootイベントと同じ一回限りの契約に従う。

閉じた後は元の入力先へフォーカスを戻す。eguiがmodal layerを解放するframeに届いた
文字・IME・キー入力は次の再描画へ保持し、一度だけ元の入力先へ渡す。
この遷移ではKUCが再描画を要求するため、hostは通常の再描画要求を処理する。

実行可能な使用例と入力回帰は
`crates/katana-ui-core/tests/egui_selection_dialog_contract.rs` にある。
