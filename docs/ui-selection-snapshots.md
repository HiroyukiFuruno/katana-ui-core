# UI選択状態の保存と復元

hostは本文と編集履歴を管理し、KUCは選択方向、折返し位置のaffinity、縦移動の希望列を保持する。
`ui_selection_before_input()` とframeの `ui_selection_after_input()` から、同じlogical frameの
入力前後を識別できる。`KucUiSelectionSnapshot` はClone/Eqを持ち、座標やrendererを公開しない。
同値性にはUI状態だけを使い、frame番号、revision、root識別、本文は含めない。

snapshotはfocusedかつ編集可能・表示中の本文だけで有効となる。
入力passが終わるまではProvisionalで、discardされたpassから再利用可能なsnapshotを返さない。
確定後のframeを保持し、`Context::run_ui` が完了してから観測を取得する。
unfocused、readonly、disabled、hidden、IME未確定は別のavailabilityで識別する。

```rust,ignore
let saved = frame.ui_selection_after_input().snapshot().cloned();
// hostのUndo/Redoで本文transactionを先に復元し、その本文をrootへ同期する。
if let Some(saved) = saved {
    root.restore_ui_selection_once(&saved.restore_request())?;
}
```

同じsnapshotから新しいrestore requestを何度でも作れる。
同じrequestの二重適用は `DuplicateRequest` で拒否する。
別root、別本文、別document identity、利用不能な状態もtyped errorで拒否し、本文やfocusを変更しない。
hostが同じrootで文書を切り替える場合、`text_state_id` に文書ごとの安定した異なるidentityを投影する。
IME未確定状態は保存・取消・復元せず、明示拒否する。

公開rootの実入力回帰は `crates/katana-ui-core/tests/egui_ui_selection_snapshot_contract.rs` にある。
