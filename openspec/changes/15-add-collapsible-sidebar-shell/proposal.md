## Why

`katana` の explorer / preview side panel、`katana-chat-ui` の history panel、KDV の TOC / export panel、KLE の find / diagnostics panel は、いずれも「折りたたみ、hover 一時展開、幅変更、固定」を持つパネルを必要とする。

KUC は `SideMenu` molecule と `SplitPane` layout で部分的にカバーできるが、「collapse / expand / icon-only mode / persistence の起点 / ホバーで一時展開 / drag による幅変更 / pin / floating overlay」を domain-free に表す panel molecule がない。

KUC は app shell や画面テンプレートを提供しない。
この change は、利用側が shell を組むための collapsible panel molecule だけを扱う。

## What Changes

- `widget::molecules` に `CollapsiblePanel` molecule を追加する:
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
- `SplitPane`、`SideMenu`、`TreeView`、`Toolbar` を child として受け取れる slot contract を定義する。
- `AppShell` は追加しない。top / bottom bar / main / leading / trailing の組み合わせは consumer が実装する。

## Capabilities

### New Capabilities

- `kuc-collapsible-panel`: CollapsiblePanel molecule の完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `SideMenu`（メニュー）と `CollapsiblePanel`（折りたたみ可能な panel）と `SplitPane`（汎用分割）の責務境界を明記する。

## Impact

- `crates/katana-ui-core/src/molecule/structured/collapsible_panel.rs` 新設。
- consumer (`katana` sidebar / TOC、`katana-chat-ui` history panel、KDV / KLE side panel) は KUC molecule を組み合わせて shell を構築できる。
- `SplitPane`、`SideMenu` を内部で再利用。
