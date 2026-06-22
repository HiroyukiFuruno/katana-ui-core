## Why

`katana` 編集画面のセーブ失敗、`katana` settings 画面の警告、`katana-chat-ui` の adapter 接続失敗 / モデル切替注意 / 添付サイズ超過 / プロバイダ未認証など、画面内（インライン）に常駐するアラート（banner）が必要な箇所がある。これらは toast（一過性）でも modal（モーダル中断）でもなく、当該 view 上部や form 上部に persistent に並ぶ告知である。

KUC は `NotificationToast`（一過性）と `StatusBar`（severity message 1 件）を持つが、persistent な「インライン banner / alert」widget がない。consumer ごとに `Card` + 色 + アイコンで ad hoc に組まれており、デザイン揺れが出ている。

## What Changes

- `widget::molecules` に `Banner` molecule を追加する:
  - option:
    - `severity: Info | Success | Warning | Danger | Neutral`
    - `title: Option<String>`
    - `message: String`
    - `leading_icon: Option<SvgIcon>`（default は severity から導出）
    - `actions: Vec<BannerAction>`（primary / secondary）
    - `dismissible: bool`
    - `expanded_details: Option<String>`（折りたたみで詳細表示）
    - `density: Compact | Default`
    - `placement_hint: Inline | Sticky`（描画位置のヒント、layout は consumer 責務）
  - action: `PressAction` / `Dismiss` / `ToggleDetails`
  - event: `BannerActioned` / `BannerDismissed` / `BannerDetailsToggled`
  - state: visible, details_open, callback_log

## Capabilities

### New Capabilities

- `kuc-banner`: Banner molecule の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `Banner`（persistent インライン）と `NotificationToast`（一過性）と `StatusBar`（永続的フッタ）の責務境界を明記する。

## Impact

- `crates/katana-ui-core/src/molecule/disclosure/banner.rs` を新設する。
- consumer は KUC banner で persistent な告知を統一可能になる。
- toast / status bar との混在を避けるための contract test を追加する。
