## Why

`katana` の `views/top_bar/title_bar.rs` は OS のウィンドウ装飾（macOS の traffic lights、Windows の system buttons、Linux の CSD など）をエミュレートしながらアプリ独自の title + breadcrumbs + tab を表示する。`katana-chat-ui` も chat ウィンドウの title bar に provider icon + 新規チャット + 履歴 button を埋め込む。

KUC は `runtime / window / surface` API で `WindowConfig.decorations` 等を持つが、「ウィンドウ chrome の中に置く widget」（draggable region、traffic lights、min/max/close、center title、leading / trailing slot）を表現する molecule がない。consumer ごとに OS 別の chrome レイアウトが手書きされており、揃わない。

## What Changes

- `widget::molecules` に `TitleBar` molecule を追加する:
  - option:
    - `style: Native | EmbeddedNative | Custom`
    - `position: WindowControlsPosition = Leading | Trailing | Auto`
    - `title: String`
    - `subtitle: Option<String>`
    - `leading_slot: Option<UiTree>`
    - `center_slot: Option<UiTree>`
    - `trailing_slot: Option<UiTree>`
    - `height: TitleBarHeight = Compact | Default | Tall`
    - `draggable_regions: Vec<Rect>`（adapter に伝える drag-to-move 領域）
    - `controls: WindowControls = Standard | CustomList(Vec<Control>)`
  - action: Minimize / Maximize / Restore / Close / EnterFullscreen / ExitFullscreen / OpenWindowMenu
  - event: ControlPressed { which } / TitleClicked / DoubleClicked

## Capabilities

### New Capabilities

- `kuc-title-bar`: TitleBar molecule の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `runtime-window-surface`: TitleBar molecule と Window decorations の関係を明記する（KUC は title bar 描画 model を持つが、OS native chrome を取り扱うのは adapter の責務）。

## Impact

- `crates/katana-ui-core/src/molecule/structured/title_bar.rs` 新設。
- adapter（floem / egui / gpui）に「draggable region」「window controls dispatch」を伝える API を整備。
- consumer (`katana` title_bar、`katana-chat-ui` chrome) は KUC molecule に統一可能。
