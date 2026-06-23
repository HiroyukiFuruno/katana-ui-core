# Design — Banner molecule (inline alert)

## 目的

「画面内に常駐するセクション告知」を Banner molecule として確立する。toast（一過性）/ status bar（永続フッタ）/ banner（永続インライン）の責務を分ける。

## 採用方針

### 1. severity と icon の対応

| severity | default icon | tone |
| --- | --- | --- |
| Info | info | accent |
| Success | check | success |
| Warning | alert-triangle | warning |
| Danger | alert-octagon | danger |
| Neutral | none | muted |

icon は `leading_icon` で上書き可能。

### 2. 構造

- leading icon
- title（heading）+ message（body）
- expanded_details（小さな展開ボタン → 詳細）
- actions（primary / secondary）
- trailing dismiss button（dismissible=true）

### 3. action

- `BannerAction { id, label, tone, disabled, destructive }`
- ボタンは内部で `Button` atom を子に持つ
- accelerator なし（banner は accelerator を持たない）

### 4. details 展開

- 折りたたみ部分は `Accordion` semantics と共通だが、Banner 専用の細部（max-height, scroll, animate）
- `details_open` を state に持つ

### 5. dismiss

- dismiss は visible=false にし、`BannerDismissed` を発火
- consumer はこの event を受けて永続化（次回も dismissed のままにするか）を判断
- KUC は dismiss state を永続化しない

### 6. 配置 / 並び

- `placement_hint = Inline`（標準）または `Sticky`（scroll しても上部固定）
- 実際の固定 layout は consumer 側責務（Sticky は scroll container と組合せ）
- 複数 banner が並ぶ場合は consumer 側で `Column` / list に並べる

### 7. accessibility

- role = "status"（Info / Success / Neutral）または "alert"（Warning / Danger）
- live region announce

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `NotificationToast` を persistent モードで使う | toast は一過性 + 位置がスタック (overlay)。永続インラインは別の責務であり contract が混ざる。 |
| `StatusBar` を画面上部にも使う | StatusBar は永続フッタ用の severity message 1 件で、actions / details / expansion を持たない。 |
| `Card` + `Text` + `Button` を組み合わせる | severity 色 / icon / dismiss / accessibility の揺れが consumer ごとに発生する。 |

## Out of scope

- グローバル banner manager（連続表示 queue）：v2 以降
- in-app messaging（marketing 系）：別 layer
- アニメーション（slide-down / slide-up）：`add-animation-primitives-18`

## 影響範囲

- `NotificationToast` Storybook ページに「persistent は Banner」リンク
- `StatusBar` Storybook ページに「画面上部の告知は Banner」リンク
- consumer 側の persistent 告知を KUC で統一
