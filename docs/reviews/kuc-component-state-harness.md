# KUC component state harness

## 結論

KUC の状態は、通常は部品内部状態として閉じる。
利用側が通信中や保存中のような app 側状態で UI を制御したい場合だけ、`UiStateHandle` を使って短命に読み取り、`set/update` で部品内部状態へ反映する。

## 契約

| 対象 | 方針 |
| --- | --- |
| 部品内部状態 | `UiStateId` と `UiComponentState` で部品ごとに一意に持つ。 |
| 読み取り | `get` または `with` で snapshot として読む。長く参照を保持しない。 |
| 更新 | `set/update` で handle を更新し、部品は `sync_state` で取り込む。 |
| global state | focus、overlay、modal など全体状態だけを扱う。button の loading や input value は置かない。 |
| 回帰防止 | `app_global_state_updates_component_owned_state_via_handle` と guard で検出する。 |

## 例

```rust
let mut button = Button::new("Next page").loading(true);
let button_state = button.state_handle();

button_state.update(|state| {
    state.loading = false;
});

button = button.sync_state(&button_state);
```
