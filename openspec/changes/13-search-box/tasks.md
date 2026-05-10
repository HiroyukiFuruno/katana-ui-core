# Tasks — 13-search-box

## 1. 実装

- [x] 1.1 `composite/input/search/types.rs` に `SearchBoxProps` を定義
- [x] 1.2 `composite/input/search/view.rs` を実装。`TextInput` を内部利用、Esc / Enter のキー処理
- [x] 1.3 `composite/input/search/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 Esc で value が空文字に戻ることのテスト
- [x] 2.2 Enter で `on_submit` が呼ばれることのテスト
- [x] 2.3 value 非空時のみ clear ボタンが trailing に出ることのテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/search_box.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 空 / 入力済み
  - [x] disabled
  - [x] サイズトークン違い
  - [x] Esc / Enter のライブ確認セル
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で search_box ページが想定通り表示
- [x] 4.3 ast-lint 通過（`composite/input/search` から `composite/input/text` のみを参照）

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [/] Storybook の SearchBox ページから resolved 値をラベルで再現した疑似サンプルを除去し、実際の `SearchBox::view` で live / readonly / size を確認できる構成にする。
- [ ] マテリアルUI（Material UI）のように検索アイコン、clear、submit、検索オプションを input の枠内に内包する。input の外に記号やボタンが並ぶ構成は不可。
- [ ] 正規表現利用、単語単位検索、大文字小文字区別の 3 種コントロールを実装する。各コントロールは default false とし、表示 / 非表示 / 領域だけ確保を利用側から制御できるようにする。
- [ ] dark mode で SVG の配色が theme token に追従するようにする。preset SVG と custom SVG の両方で `currentColor` 相当の配色反映を確認する。
- [ ] Storybook に Material UI 風の input 内アイコン配置、3 種コントロール、dark mode SVG 配色変化の live sample を追加する。
