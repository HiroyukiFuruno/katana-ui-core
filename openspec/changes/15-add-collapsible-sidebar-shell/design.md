# Design — CollapsibleSidebar + AppShell

## 目的

「サイドバー + メインコンテンツ + 永続バー」を持つアプリ shell を統一する。

## 採用方針

### 1. mode

```text
SidebarMode = Expanded | IconOnly | Collapsed | FloatingOverlay
```

- `Expanded`: 通常の幅で表示
- `IconOnly`: アイコンだけのスリム表示（content 側は context に応じてアイコン表示を切替えるよう consumer に通知）
- `Collapsed`: 完全に隠す
- `FloatingOverlay`: メイン content の上に floating で重ねて表示（pin=false で hover や trigger で表示）

### 2. persistence

- `width.persist_id: Option<String>` を渡すと、consumer 側ストアにキーとして使われる
- KUC 自身はストレージを持たない（adapter / consumer 責務）
- `persist_id` がない場合、width は session-only

### 3. expand_on_hover

- `pinned=false` のとき trigger（小さなハンドル）にホバーで一時展開
- ホバーが離れたら自動的に元の mode に戻る
- `pinned=true` のときは hover は無視

### 4. resize

- `resize_handle=true` で右端 (Leading) または左端 (Trailing) にドラッグハンドル
- ドラッグで width 変更、min / max に clamp
- ダブルクリックで default に戻す

### 5. AppShell の構造

```text
AppShell {
  top_bar: Option<UiTree>,
  leading_sidebar: Option<CollapsibleSidebar>,
  main: UiTree,
  trailing_sidebar: Option<CollapsibleSidebar>,
  bottom_bar: Option<UiTree>,
}
```

- top / bottom bar は高さ固定 or auto
- 内部レイアウトは Grid model（layout primitive `Grid` を使う）
- sidebar の mode 変化で main の available width が動的に変わる
- FloatingOverlay の sidebar は z-index 上層に絶対配置

### 6. accessibility

- sidebar header に role=banner 相当
- ToggleExpand action は keyboard shortcut（Cmd/Ctrl + B 等）を accelerator で受けられる（accelerator は consumer の listener に任せる）
- screen reader: 「Sidebar expanded」「Sidebar collapsed」announce

### 7. mobile-like FloatingOverlay

- screen 幅が狭い場合、consumer 判断で FloatingOverlay モードへ切替え
- KUC 自身は breakpoint を判定しない（responsive 判断は consumer）

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `SplitPane` を sidebar 用に拡張 | SplitPane は対等な 2 ペイン分割。sidebar の collapse / icon-only / float / persistence が SplitPane の責務と乖離。 |
| `SideMenu` molecule に collapse を入れる | SideMenu は「メニュー項目」の表示が責務。shell 境界 / resize / pin / float は別 layer。 |
| consumer 側で実装する | KatanA / chat-ui / Storybook で shell ロジックが重複し、揃わない。 |

## Out of scope

- 縦に分割した sidebar 内の更なる分割 widget：consumer 側 SplitPane 等で組む
- ドラッグでの detach（別ウィンドウ化）：adapter escape hatch
- 「右クリックで隠す」のような細かな UX：v2 以降

## 影響範囲

- consumer 側 shell 実装を KUC で統一
- 内部で `SplitPane`、`SideMenu`、layout `Grid` を使う
