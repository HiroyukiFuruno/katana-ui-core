# Design — TitleBar molecule

## 目的

OS 別の窓装飾（chrome）の上にアプリ独自の title / slot を被せ、cross-platform で揃う TitleBar を提供する。

## 採用方針

### 1. style

```text
TitleBarStyle =
  | Native           // OS の chrome を尊重。content の上には何も描かない（only metadata）
  | EmbeddedNative   // OS chrome の代わりに描画。traffic lights / system buttons を含む
  | Custom           // OS chrome なし、アプリ専用
```

### 2. WindowControlsPosition

- macOS は Leading、Windows / Linux は Trailing が標準
- `Auto` は OS から自動判定
- アプリが意図的に override 可能

### 3. WindowControls

- `Standard`: Minimize / Maximize-Restore / Close
- `CustomList(Vec<Control>)`: 任意の controls（settings / theme / login 状態など）も並べられる

### 4. draggable_regions

- TitleBar 内の特定 Rect を adapter にドラッグ可能領域として渡す
- center_slot / leading_slot に置かれた interactive elements は自動的に drag 不可
- adapter は OS の drag-to-move API を呼ぶ

### 5. height

- `Compact`（28px）/ `Default`（32px）/ `Tall`（44px）
- macOS では 28px が一般的、Windows 10/11 では 32px

### 6. accessibility

- title は role=banner / heading
- controls は role=button
- screen reader 用に「Window controls: close, minimize, maximize」を読み上げ可能

### 7. fullscreen handling

- EnterFullscreen / ExitFullscreen action は KUC の `WindowCommand` を発火
- fullscreen 中は TitleBar を hide するか、auto-hide trigger を提供（hover で再表示）

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| TitleBar を完全に adapter 任せ | アプリ独自の title / slot / controls を持てない、揃わない。 |
| OS chrome を一切使わない（Custom only） | macOS の標準動作（fullscreen トランジション等）に従えず、UX 品質が落ちる。 |
| 個別 widget（traffic lights / breadcrumb / title）を別々に並べる | drag region / window menu / OS 差異の handling が散らかる。 |

## Out of scope

- macOS の vibrancy / Windows mica など OS native visual effects：adapter / consumer 責務
- Top menu bar（File / Edit / View）の native 描画：別 widget（adapter escape hatch）
- ウィンドウ resize handle：adapter / window manager 責務

## 影響範囲

- adapter の window controls dispatch / draggable region transfer
- consumer の title_bar 実装を統一
- KUC の `WindowCommand` enum を Minimize / Maximize / Restore / Close / EnterFullscreen / ExitFullscreen で揃える
