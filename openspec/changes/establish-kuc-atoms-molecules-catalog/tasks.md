# Tasks — establish-kuc-atoms-molecules-catalog

## 1. 棚卸しと旧change整理

- [x] 1.1 既存差分を docs / OpenSpec / core / Storybook / guard に分類し、この change の対象外差分へ触れない方針を確認する。
- [x] 1.2 archive 済み 01〜17、19〜22 と active 18 / 23 / 24 から、KUC に移す option / action / event / state / preset / test 要件を抽出する。
- [x] 1.3 `katana-widget-parity-backlog` の 01〜24 要件をこの change の対象表へ移し、同 change は superseded と明記する。
- [x] 1.4 `ui-core-interaction-visual-parity` の Storybook / visual gate 要件をこの change の品質ゲートへ移し、同 change は superseded と明記する。
- [x] 1.5 `18-accordion`、`23-color-picker-complete-parity`、`24-code-diff` の要件を移したうえで archive 候補として記録する。
- [x] 1.6 旧 01 theme-tokens を Theme / Panel theme の core foundation 要件へ移管する。
- [x] 1.7 旧 02 text-primitive を Text atom 要件へ移管する。
- [x] 1.8 旧 03 icon-primitive を Icon atom 要件へ移管する。
- [x] 1.9 旧 04 spinner-primitive を Spinner / LoadingDots atom 要件へ移管する。
- [x] 1.10 旧 05 svg-button を SvgButton 要件へ移管する。
- [x] 1.11 旧 06 text-button を TextButton 要件へ移管する。
- [x] 1.12 旧 07 icon-text-button を IconTextButton 要件へ移管する。
- [x] 1.13 旧 08 toggle を Toggle 要件へ移管する。
- [x] 1.14 旧 09 segmented-toggle を SegmentedToggle 要件へ移管する。
- [x] 1.15 旧 10 select-box を SelectBox 要件へ移管する。
- [x] 1.16 旧 11 color-swatch を ColorSwatch 要件へ移管する。
- [x] 1.17 旧 12 text-input を Input / TextInput 要件へ移管する。
- [x] 1.18 旧 13 search-box を SearchBox 要件へ移管する。
- [x] 1.19 旧 14 tooltip を Tooltip 要件へ移管する。
- [x] 1.20 旧 15 badge を Badge 要件へ移管する。
- [x] 1.21 旧 16 key-cap を KeyCap 要件へ移管する。
- [x] 1.22 旧 17 card を Card 要件へ移管する。
- [x] 1.23 旧 18 accordion を Accordion 要件へ移管する。
- [x] 1.24 旧 19 split-pane を SplitPane 要件へ移管する。
- [x] 1.25 旧 20 modal-overlay を ModalOverlay / Modal 要件へ移管する。
- [x] 1.26 旧 21 popover を Popover 要件へ移管する。
- [x] 1.27 旧 22 rgba-color-picker を ColorPicker 要件へ移管する。
- [x] 1.28 旧 23 color-picker-complete-parity を ColorPicker parity 要件へ移管する。
- [x] 1.29 旧 24 code-diff を CodeDiff 要件へ移管する。

## 2. docs / OpenSpec 正本化

- [x] 2.1 `openspec/changes/README.md` にこの change を次フェーズの正本として追加し、外部変換層を使う Storybook の古い記述を削除する。
- [x] 2.2 `docs/ui-separation-plan.md` を root architecture の説明へ寄せ、実装詳細はこの change へ誘導する。
- [x] 2.3 `docs/architecture/ui-separation/owned-ui-task-map.md` を atoms / molecules / Storybook internal の再分類表へ更新する。
- [x] 2.4 `docs/architecture/ui-separation/ui-core-parity-gap.md` の既存証跡を旧基準として整理し、新基準では未完了扱いに直す。
- [x] 2.5 `docs/directory-structure.md` に `widget::atoms` / `widget::molecules` と Storybook internal organisms の扱いを反映する。
- [x] 2.6 `docs/widget-extraction-policy.md` に organisms / templates / pages は今は公開対象外だが将来拡張を妨げない方針を追記する。
- [x] 2.7 `README.md` を現在スコープが atoms / molecules + 部品カタログである説明へ更新する。

## 3. Core 基盤契約

- [x] 3.1 theme と font を外側から設定できる KUC facade 契約を確定する。
- [x] 3.2 Katana accent color を既定 theme として固定する。
- [x] 3.3 英語、日本語、英日混在、絵文字の上下中央揃えを text regression 契約にする。
- [x] 3.4 キー入力、日本語入力（IME）、OS 絵文字入力を input event 契約にする。
- [x] 3.5 component ごとの内部 state と `UiStateId` 一意性を契約にする。
- [x] 3.6 layout regression が寸法、余白、中央揃え、スクロール、重なり順を検査できる契約を定義する。

## 4. Atoms 契約と実装対象

- [x] 4.1 Text の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 4.2 Icon の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 4.3 Button / SvgButton / TextButton / IconTextButton の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 4.4 Input / TextInput の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 4.5 Checkbox / Radio / Toggle の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 4.6 Badge / Divider / Spacer / KeyCap の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 4.7 Spinner / LoadingDots / ProgressBar / ColorSwatch の option / action / event / state / preset / test / Storybook ページ要件を定義する。

## 5. Molecules 契約と実装対象

- [x] 5.1 SelectBox / ComboBox / MenuButton / SegmentedToggle の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 5.2 SearchBox / FormField / SlideControl / DynamicArrayEditor の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 5.3 Tooltip / Popover / ModalOverlay / NotificationToast の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 5.4 Card / Accordion / SplitPane / Tabs / Breadcrumb / SideMenu / Toolbar / StatusBar の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 5.5 TreeView / SelectionList / CommandPalette の option / action / event / state / preset / test / Storybook ページ要件を定義する。
- [x] 5.6 ColorPicker / CodeDiff の option / action / event / state / preset / test / Storybook ページ要件を定義する。

## 6. Storybook 部品カタログ

- [x] 6.1 左ペインを KUC TreeView で実装し、atoms / molecules / Storybook internal をカテゴリ別のネスト構造で表示し、カテゴリ開閉と選択中表示を同じ状態から描画する。
- [x] 6.2 各部品ページに preview、settings、state 表示、event 履歴、action 履歴を配置する。
- [x] 6.3 settings から option 値を変更し、preview に即時反映する操作経路を定義する。
- [x] 6.4 各部品ページの preset 切替を KUC Tabs で実装する。
- [x] 6.5 Storybook 内部に必要な shell / navigation / inspector は internal organisms として実装し、公開 widget API にはしない。
- [x] 6.6 Storybook のスクリーンショット確認を目視補助として扱い、自動テストの代替にしないことを docs と gate に反映する。

## 7. 自動テストと品質ゲート

- [x] 7.1 core 契約テストで theme、font、text、input、event、state、layout を検証する。
- [x] 7.2 atoms 契約テストで UI ごとの option / action / event / state を検証する。
- [x] 7.3 molecules 契約テストで合成部品の状態遷移と子部品 state の独立性を検証する。
- [x] 7.4 visual regression で主要 preset の非空描画、layout bounds、theme 適用、操作後差分を検証する。
- [x] 7.5 input regression でキー入力、日本語入力（IME）確定文字、OS 絵文字入力を検証する。
- [x] 7.6 guard で framework 混入、state 外部化、placeholder Storybook、未網羅 option / action / event、日本語・絵文字未検証を失敗扱いにする。

## 8. 実装フェーズ引き渡し

- [x] 8.1 `openspec validate establish-kuc-atoms-molecules-catalog --strict` を通す。
- [x] 8.2 `openspec validate ui-core-root-plan --strict` を通す。
- [x] 8.3 `git diff --check` を通す。
- [x] 8.4 01〜24 が tasks に漏れなく移管されていることを確認する。
- [x] 8.5 Storybook が検証の主役ではなく、操作確認と目視確認の場として定義されていることを確認する。
- [x] 8.6 品質ゲートが自動テスト中心で定義されていることを確認する。

## 9. 構成不能な基盤欠落の是正

- [x] 9.1 P0: SVG アイコン atom を、画像そのものではなく `SvgIcon` 相当の typed props として core に持たせる。
- [x] 9.2 P0: Button 専用ではない汎用クリック event を core action と callback log に持たせる。
- [x] 9.3 P0: TreeView が directory / file icon、開閉 icon、font role、theme id を option として持てるようにする。
- [x] 9.4 P0: TreeView が空領域右クリック時の context menu 表示可否と action を持てるようにする。
- [x] 9.5 P0: TreeView が既定の開閉状態、開閉対象領域、垂直線の有無・種類・太さを持てるようにする。
- [x] 9.6 P1: Text 表示と AlignCenter などの汎用配置 node を、Storybook 専用ではなく core の構成部品として検証する。
- [x] 9.7 P1: Accordion の開閉制御を TreeView と共有できる disclosure foundation として整理する。
- [x] 9.8 P1: Storybook 左ペインを上記 TreeView option で操作できる確認画面に更新する。

## 10. Storybook 品質検証の堅牢化

- [x] 10.1 macOS の最大化ボタンが効くように、Storybook のメインウィンドウをサイズ変更可能にする。
- [x] 10.2 preset tab の高さ、幅、間隔、選択状態を数値で固定し、表示崩れを自動テストで検出する。
- [x] 10.3 navigation、preview、inspector、component card が重ならないことをレイアウト自動テストで検証する。
- [x] 10.4 theme 切替、preset 切替、navigation 選択のクリック操作を state 変化と描画差分で自動検証する。
- [x] 10.5 Storybook のスクリーンショットを再生成し、目視だけでなく pixel 差分とレイアウト契約を品質ゲートに含める。
- [x] 10.6 固定パスのスクリーンショット生成前に旧ファイルを削除し、生成ログにサイズと更新時刻を出して古い画像参照を検出できるようにする。
- [x] 10.7 preset tab をボタン列ではなく、Katana app に寄せた隙間なし・同一高さ・下辺アクセントの連結タブとして表現し、pixel test で検証する。
- [x] 10.8 preset tab の 3 個目・4 個目も選択できるように、preset 状態を bool ではなく index として管理する。
- [x] 10.9 Storybook 全体を縦スクロール可能にし、スクロール後の viewport 差分を自動テストで検証する。
- [x] 10.10 操作確認用の Storybook 起動と固定SS生成を release binary に切り替え、debug 実行による重さを避ける。
- [x] 10.11 TreeView のカテゴリ行を開閉可能にし、クリック範囲、カレント表示、アイコンと文字の上下中央揃えを自動テストで検証する。
- [x] 10.12 スクロールバーの表示・非表示を Storybook state、画面操作、スナップショット引数で制御できるようにする。
- [x] 10.13 各部品ページに option / action / event / state / preset / preview / settings / test / visual の実情報を表示し、見せかけだけのページを guard で失敗扱いにする。
