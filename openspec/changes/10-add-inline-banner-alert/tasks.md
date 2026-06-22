# Tasks — add-inline-banner-alert-10

## 1. 設計確定

- [x] 1.1 severity × icon × tone マッピングを確定する。
- [x] 1.2 expanded_details の挙動（折りたたみ / max-height / scroll）を確定する。
- [x] 1.3 actions / dismiss / accessibility role を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/disclosure/banner.rs` を新設する。
- [x] 2.2 option / action / event / state を実装する。
- [x] 2.3 `widget::molecules` の re-export に `Banner` を追加する。

## 3. 自動テスト

- [x] 3.1 severity 切替えで icon / tone / role が連動して変わることを検証する。
- [x] 3.2 dismiss action で visible=false かつ `BannerDismissed` 発火を検証する。
- [x] 3.3 expanded_details の toggle が `details_open` を切替えることを検証する。
- [x] 3.4 actions（primary / secondary）の typed event 発火と disabled 抑止を検証する。
- [x] 3.5 accessibility role（status / alert）が severity から正しく導出されることを検証する。

## 4. 数値化された描画契約

- [x] 4.1 severity 5 種 × density 2 種 × actions 0/1/2 の主要 subset を描画契約で検証する。
- [x] 4.2 expanded_details open / closed を render tree と state contract で検証する。
- [x] 4.3 dismissible 表示と長文 message の折返しを layout contract で検証する。
- [x] 4.4 light / dark theme を theme token contract で検証する。

## 5. Storybook ページ

- [x] 5.1 `Disclosure > Banner` ノードを追加する。
- [x] 5.2 preset「保存失敗」「adapter 未接続」「添付サイズ超過」「成功通知」「details 展開」を実装する。
- [x] 5.3 settings で severity / density / actions / details / dismissible を切替えできるようにする。

## 6. ドキュメント

- [x] 6.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に Banner 行を追加する。
- [x] 6.2 `NotificationToast` と `StatusBar` のドキュメントから Banner との責務境界を相互参照する。

## 7. 品質ゲート / DoD

- [x] 7.1 `cargo test -p katana-ui-core` をパスする。
- [x] 7.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 7.3 `openspec validate 10-add-inline-banner-alert --strict` をパスする。
- [x] 7.4 数値化された描画契約と Storybook requirement gate をパスする。
