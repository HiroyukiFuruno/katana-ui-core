# Design — 20-modal-overlay

## 目的

`Modal` は親画面の中に重ねる表示ではなく、別のネイティブウィンドウを開く部品として扱う。

同一ウィンドウ内に重ねる表示は `OverlayDialog` として分離する。

## 画面上の見え方

- `Modal`: Storybook の親ウィンドウとは別に、小さな独立ウィンドウが前面に開く。
- `OverlayDialog`: Storybook のページ内に、背景の上へ重ねて表示される。

## 操作

- `Modal` は起動ボタンで別ウィンドウを開く。
- `Modal` は `Close` または Esc で閉じる。
- `Modal` を閉じると `on_close` と `on_focus_return` が呼ばれる。
- `OverlayDialog` は同一ページ内で開閉し、backdrop と Esc の挙動確認に使う。

## API 境界

`ModalParentInteraction` を公開し、親画面との相互作用方針を明示する。

- `Block`: 既定値。別ウィンドウを前面固定（AlwaysOnTop）で開き、閉じると focus return を呼ぶ。
- `Allow`: 前面固定を使わず、補助ウィンドウとして開く。

Adapter 0.2 の公開 API では、親ウィンドウを OS レベルで無効化する親子 modal 制御は提供されていない。
そのため `Block` は「親より前面に維持する」「閉じるまで modal 側の操作を優先する」「閉じたら呼び出し元へ戻す」というアプリケーションレベルの抑制として定義する。

## 互換性

既存の同一ウィンドウ内 overlay 実装は `OverlayDialog` として残す。

`Modal::view` は同一ウィンドウ内に描画せず、`open=true` の場合に native window を開く導線へ寄せる。
同一ウィンドウでの重ね表示が必要な利用側は `as_overlay_dialog()` または `OverlayDialog` を使う。
