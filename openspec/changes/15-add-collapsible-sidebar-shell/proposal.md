## Why

`katana` の workspace shell（app_frame/sidebar/explorer + central_content + status_bar）、`katana-chat-ui` の history panel + chat content、Storybook 自身の navigation + preview など、いずれも「左サイドバー（または右サイドバー）+ メインコンテンツ + オプションのサブパネル」のシェルレイアウトを取る。

KUC は `SideMenu` molecule、`SplitPane` molecule、`SideMenu` で部分的にカバーするが、「collapse / expand / icon-only mode / persistence の起点 / ホバーで一時展開 / drag による幅変更 / pin / floating overlay」を統合的に持つ shell molecule がない。

## What Changes

- `widget::molecules` に `CollapsibleSidebar` molecule を追加する:
  - option:
    - `side: Leading | Trailing`
    - `mode: Expanded | IconOnly | Collapsed | FloatingOverlay`
    - `width: ResizableWidth { min, max, default, persist_id }`
    - `pinned: bool`
    - `expand_on_hover: bool`
    - `header_slot: Option<UiTree>`
    - `content: UiTree`
    - `footer_slot: Option<UiTree>`
    - `resize_handle: bool`
  - action: SetMode / ToggleExpand / Pin / Unpin / ResizeWidth / FloatingOpen / FloatingClose
  - event: ModeChanged / WidthChanged / PinChanged / FloatingShown / FloatingHidden
  - state: mode, width, pinned, hover_open, callback_log
- 横並びの shell layout を支える `AppShell` molecule（leading sidebar + main + trailing sidebar + top bar + bottom bar）を追加する。

## Capabilities

### New Capabilities

- `kuc-collapsible-sidebar`: CollapsibleSidebar molecule の完了条件を定義する。
- `kuc-app-shell`: AppShell molecule の完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `SideMenu`（メニュー）と `CollapsibleSidebar`（シェル境界の panel）と `SplitPane`（汎用分割）の責務境界を明記する。

## Impact

- `crates/katana-ui-core/src/molecule/structured/sidebar.rs` 新設。
- `crates/katana-ui-core/src/molecule/structured/app_shell.rs` 新設。
- consumer (`katana` workspace shell、`katana-chat-ui` history panel、Storybook navigation) は KUC molecule に統一可能になる。
- `SplitPane`、`SideMenu` を内部で再利用。
