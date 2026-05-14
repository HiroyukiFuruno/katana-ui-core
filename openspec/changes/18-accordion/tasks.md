# Tasks — 18-accordion

## 1. 実装

- [x] 1.1 `layout/accordion/types.rs` に `AccordionProps` / `IndicatorPosition` を定義
- [x] 1.2 `layout/accordion/view.rs` に展開・折り畳みアニメーションを実装
- [x] 1.3 `layout/accordion/mod.rs` で公開 API を整理
- [x] 1.4 `tree_mode` 相当の情報を受け取るプロパティを追加し、選択行/深さ/ライン表示を扱える状態を用意

## 2. 追加要件対応

- [x] 2.1 `default_open / default_closed` を `expanded` で選択可能にした
- [x] 2.2 `controlled / uncontrolled` の差分を API と挙動で用意した
- [x] 2.3 `disabled` 時はトグルアクションを止める
- [x] 2.4 `multiple` 展開許可を group API で切り替え可能にした
- [x] 2.5 トリガー領域をアイコン＋文字、アイコンのみ、文字のみ、行全体で切り替え可能にした
- [x] 2.6 tree mode 相当のデータ（depth/selected/show_lines）を扱い、ネスト見た目を描画
- [x] 2.7 展開アニメーションと reduced motion の無効化を実装

## 3. Storybook

- [x] 3.1 `storybook/src/pages/accordion.rs` を追加し `pages/mod.rs` に登録済み
- [x] 3.2 ページ内表示
  - [x] 既定 / 展開済み
  - [x] indicator 位置違い 3 種
  - [x] disabled
  - [x] light / dark 追従
  - [x] controlled / uncontrolled 例
  - [x] クリック領域 4 種
  - [x] 同時展開制御（single / multiple）
  - [x] tree mode の表示
  - [x] reduced motion の比較
  - [x] 操作結果 / callback log を同じ画面内に表示

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で accordion ページが想定通り表示
  - 2026-05-11: `KATANA_UI_WIDGET_STORYBOOK_PAGE=accordion` で起動し、katana方式の `screencapture -l <window_id>` でAccordionページのウィンドウ単体表示を確認。
- [x] 4.3 ast-lint 通過
  - 2026-05-11: `just ast-lint` 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [/] アコーディオンがプルダウンに見えないよう、ヘッダー行を横幅いっぱいの開閉行として表示する。ヘッダーは任意表示（Node）を受け取れるようにし、開いた本文領域の枠線はオプションにする。
  - 2026-05-12: `Accordion::header(...)` / `AccordionTriggerArea::FullRow` / `body_border(false)` を Storybook sidebar と accordion page で利用し、横幅いっぱいの開閉行として検証できる状態にした。
- [/] Storybook の表示パネル幅を右端まで伸ばし、縦スクロールバーがパネル右端に出るようにする。
  - 2026-05-12: Storybook root / content / Accordion page の scroll container を `width_full` + `height_full` に揃え、全ページ `just storybook-smoke` を通過。
