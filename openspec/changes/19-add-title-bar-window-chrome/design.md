# Design — WindowControlButtonGroup molecule

## 目的

title bar / window chrome 全体ではなく、window control button group を domain-free molecule として提供する。
アプリの header、drag region、native decoration は consumer / adapter が持つ。

## 採用方針

### 1. scope

```text
WindowControlButtonGroup =
  controls: Vec<WindowControlKind>
  position: Leading | Trailing | Auto
  size: Small | Medium
  visibility: Always | Hover | FullscreenHover
```

### 2. WindowControlsPosition

- macOS は Leading、Windows / Linux は Trailing が標準
- `Auto` は OS から自動判定
- アプリが意図的に override 可能

### 3. WindowControls

- `Close`
- `Minimize`
- `Maximize`
- `Restore`

### 4. size

- `Compact`（28px）/ `Default`（32px）/ `Tall`（44px）
- macOS では 28px が一般的、Windows 10/11 では 32px

### 5. accessibility

- controls は role=button
- screen reader 用に「Window controls: close, minimize, maximize」を読み上げ可能

### 6. fullscreen handling

- fullscreen 中の表示 / 非表示は `visibility` と hover state で表す
- 実際の fullscreen 操作は event を受けた consumer / adapter が行う

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| TitleBar 全体を KUC に入れる | organisms / templates に近く、KUC の atoms / molecules 境界を超える。 |
| OS chrome を KUC が扱う | adapter / window manager の責務と衝突する。 |
| Window controls を各 consumer に任せる | close / minimize / maximize の見た目と event が揃わない。 |

## Out of scope

- macOS の vibrancy / Windows mica など OS native visual effects：adapter / consumer 責務
- Top menu bar（File / Edit / View）の native 描画：別 widget（adapter escape hatch）
- ウィンドウ resize handle：adapter / window manager 責務
- draggable region：adapter / consumer 責務
- title、breadcrumb、tab、header layout：consumer 責務

## 影響範囲

- adapter の window controls dispatch
- consumer の header / title area 内で使う小部品を統一
- KUC の window control intent を Minimize / Maximize / Restore / Close で揃える
