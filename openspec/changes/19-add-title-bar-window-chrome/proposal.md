## Why

`katana` の title area と `katana-chat-ui` の header は、close / minimize / maximize などの window control と、provider icon / 新規 chat / 履歴 button のような header action を並べる。

ただし title bar、window chrome、drag region、native decoration は app / adapter の責務であり、KUC の atoms / molecules 公開範囲を超える。
この change は title bar 全体ではなく、window control button group と header action group を domain-free molecule として扱う。

## What Changes

- `widget::molecules` に `WindowControlButtonGroup` molecule を追加する:
  - option:
    - `position: WindowControlsPosition = Leading | Trailing | Auto`
    - `controls: Vec<WindowControlKind>`（Minimize / Maximize / Restore / Close）
    - `size: Small | Medium`
    - `tone: Neutral | Danger`
    - `visibility: Always | Hover | FullscreenHover`
  - action: PressControl / SetHover
  - event: ControlPressed { which } / VisibilityChanged
- Header title、breadcrumbs、tab、drag region、native window menu は consumer / adapter が持つ。

## Capabilities

### New Capabilities

- `kuc-window-control-button-group`: WindowControlButtonGroup molecule の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `runtime-window-surface`: KUC は draggable region / OS native chrome を扱わず、window command intent だけを consumer / adapter へ返すことを明記する。

## Impact

- `crates/katana-ui-core/src/molecule/selection/window_control_button_group.rs` 新設。
- adapter（floem / egui / gpui）には window command intent の変換だけを求める。
- consumer (`katana` title area、`katana-chat-ui` header) は KUC molecule と自前 layout を組み合わせる。
