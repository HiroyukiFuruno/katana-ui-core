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
- [x] 2.7 `README.md` を現在スコープが atoms / molecules + Storybook である説明へ更新する。

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

## 6. Storybook

- [x] 6.1 左ペインを KUC TreeView で実装し、atoms / molecules / Storybook internal をカテゴリ別のネスト構造で表示し、カテゴリ開閉と選択中表示を同じ状態から描画する。
- [x] 6.2 各部品ページに preview、settings、state 表示、event 履歴、action 履歴を配置する。
- [x] 6.3 settings から option 値を変更し、preview に即時反映する操作経路を定義する。
- [x] 6.4 各部品ページの preset 切替を KUC Tabs で実装する。
- [x] 6.5 Storybook 内部に必要な shell / navigation / inspector は internal organisms として実装し、公開 widget API にはしない。
- [x] 6.6 Storybook や画像証跡を完了根拠にせず、自動テストで検証することを docs と gate に反映する。

## 7. 自動テストと品質ゲート

- [x] 7.1 core 契約テストで theme、font、text、input、event、state、layout を検証する。
- [x] 7.2 atoms 契約テストで UI ごとの option / action / event / state を検証する。
- [x] 7.3 molecules 契約テストで合成部品の状態遷移と子部品 state の独立性を検証する。
- [x] 7.4 rendering contract で主要 preset の非空描画コマンド、layout bounds、theme 適用、操作後差分を検証する。
- [x] 7.5 input regression でキー入力、日本語入力（IME）確定文字、OS 絵文字入力を検証する。
- [x] 7.6 guard で framework 混入、state 外部化、placeholder Storybook、未網羅 option / action / event、日本語・絵文字未検証を失敗扱いにする。

## 8. 実装フェーズ引き渡し

- [x] 8.1 `openspec validate establish-kuc-atoms-molecules-catalog --strict` を通す。
- [x] 8.2 `openspec validate ui-core-root-plan --strict` を通す。
- [x] 8.3 `git diff --check` を通す。
- [x] 8.4 01〜24 が tasks に漏れなく移管されていることを確認する。
- [x] 8.5 Storybook はフィードバック用の実画面であり、検証の主役ではないことを自動 gate で固定する。
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
- [x] 10.3 navigation、selected preview、inspector、selected component detail が重ならないことをレイアウト自動テストで検証する。
- [x] 10.4 theme 切替、preset 切替、navigation 選択のクリック操作を state 変化と描画差分で自動検証する。
- [x] 10.5 画像再生成を完了根拠にせず、rendering contract とレイアウト契約を品質ゲートに含める。
- [x] 10.6 固定画像に依存する完了判定を廃止し、古い画像参照を readiness guard で失敗扱いにする。
- [x] 10.7 preset tab をボタン列ではなく、Katana app に寄せた隙間なし・同一高さ・下辺アクセントの連結タブとして表現し、rendering contract test で検証する。
- [x] 10.8 preset tab の 3 個目・4 個目も選択できるように、preset 状態を bool ではなく index として管理する。
- [x] 10.9 Storybook 全体を縦スクロール可能にし、スクロール後の viewport 差分を自動テストで検証する。
- [x] 10.10 Storybook の実行は feedback 用に限定し、品質ゲートは release binary の自動 contract test に寄せる。
- [x] 10.11 TreeView のカテゴリ行を開閉可能にし、クリック範囲、カレント表示、アイコンと文字の上下中央揃えを自動テストで検証する。
- [x] 10.12 スクロールバーの表示・非表示を Storybook state、画面操作、スナップショット引数で制御できるようにする。
- [x] 10.13 各部品ページに option / action / event / state / preset / preview / settings / test / visual の実情報を表示し、見せかけだけのページを guard で失敗扱いにする。
- [x] 10.14 Storybook のスクロール領域に選択中 UI の契約表と状態表を描画し、上部だけで内容が終わらない構成にする。

## 11. 01〜24 実装フェーズ

- [x] 11.1 利用側の入口として `widget::atoms` / `widget::molecules` を公開し、01〜24 の対象部品を pages/templates なしで組み合わせられることをテストする。
- [x] 11.2 01 Theme / Panel theme の利用者向け facade を Storybook settings と同じ操作モデルで検証する。
- [x] 11.3.1 02〜12 の atom 系部品について、button 系の既定表示契約、SlideControl 専用 action、Storybook action/event 履歴を自動テストで検証する。
- [x] 11.3 02〜12 の atom 系部品について、option / action / event / state / preset / visual を部品ごとに実装・検証する。
- [x] 11.4.1 13 / 17 / 19 / 20 / 09 の優先 molecule として SearchBox、Card、SplitPane、ModalOverlay、SegmentedToggle を専用 model / action / event / Storybook 履歴つきで検証する。
- [x] 11.4 13〜24 の molecule 系部品について、option / action / event / state / preset / visual を部品ごとに実装・検証する。
- [x] 11.5 Storybook の settings 操作を部品ごとの typed action に接続し、単なる表示切替ではなく component state を更新する。
- [x] 11.6 01〜24 の完了判定を `widget` 公開 API、contract test、rendering contract、Storybook page の全てで追跡する。
- [x] 11.6.1 旧 01〜24 と現行 Storybook page の対応、interactive page の action log、panel の操作履歴を integration test で固定する。

## 12. 残 Task 再棚卸しと完成度引き上げ

- [x] 12.1 固定画像に依存する下部 viewport 判定を廃止し、古い画像参照を readiness guard で失敗扱いにする。
- [x] 12.1.1 固定画像の更新有無ではなく、Storybook requirement gate が typed option / action / event / state / layout contract を検査するようにする。
- [x] 12.2 Storybook の contract / settings / state / event / action 表示が横にはみ出さないことを自動テストで検証する。
- [x] 12.3 各部品ページの settings が、表示文言ではなく component の typed option / typed action を実際に変更することを UI ごとに検証する。
- [x] 12.3.1 各 Storybook page で preview と details の 6 セクションが実体として生成されることを integration test で固定する。
- [x] 12.3.2 `StorybookPanelInteractionReport` に全 53 page の settings mutation を追加し、typed option の before / after を gate 対象にする。
- [x] 12.4 TreeView の垂直線、directory / file icon、context menu、既定開閉、開閉 trigger 領域を Storybook 上で切り替え可能にし、クリック結果を自動検証する。
- [x] 12.4.1 Storybook coverage report に TreeView の選択、settings 表示、action log、左ナビ開閉差分を追加し、requirement gate で必須化する。
- [x] 12.4.2 TreeView の line / icon / font-theme / context menu / default open / toggle trigger / click toggle を interaction report と core contract test で固定する。
- [x] 12.5 旧 01〜24 について、ページ存在と action log だけでなく、UI ごとの option / action / event / state / preset / visual の最低 1 ケースを個別テストで固定する。
- [x] 12.5.1 旧 01〜24 の全対応 Storybook page を実描画し、default と interactive preset の rendering 差分を検証する visual case を追加する。
- [x] 12.5.2 旧 01〜24 の各 page で typed option settings、action/event、state id、preset 差分、非空 visual をまとめて検査する。
- [x] 12.6 Storybook の tab、TreeView、scrollbar、typography、余白を Katana app の見た目に寄せ、数値で検査できる visual rule に落とす。
- [x] 12.6.1 選択中 preset tab の下辺と preview surface の上辺を接続し、隙間が戻ったら落ちる visual rule を追加する。
- [x] 12.6.2 日本語・英日混在・絵文字の font sample と、tab gap / nav row height / nav row step / inspector 境界を数値ルールで固定する。
- [x] 12.7 theme / font / global state facade を multi-platform 前提の public API として再確認し、OS 固有 path や単一 backend 前提を guard で禁止する。
- [x] 12.7.1 `TextInput` alias、`CommandPalette`、`DynamicArrayEditor`、`NotificationToast`、`widget::molecules::SlideControl` を公開 API contract に追加する。
- [x] 12.7.2 IME 確定文字、日本語・英語混在、絵文字混在が input action 後に component state へ保持されることを回帰テストに追加する。
- [x] 12.7.3 `assert-core-public-api-neutral.sh` で OS 固有 font path と backend 固有 symbol の混入を拒否する。
- [x] 12.8 200 行を超えている Storybook 実行系ファイルを責務単位に分割し、今後触る基盤ファイルを規約内に戻す。
- [x] 12.9 `just storybook` で開く実ウィンドウを release binary 前提で確認し、スクロール、tab 3 / 4、TreeView 開閉、modal window を同一 QA 手順に固定する。

## 13. 01〜24 DoD 正本化と Storybook 完全版

11〜12 の完了チェックは、構造と仮 gate の通過記録である。
`legacy-01-24-dod.md` の DoD を満たすまで、01〜24 の Storybook 完全版は未完了として扱う。

- [x] 13.1 `legacy-01-24-dod.md` に、01〜24 の実現内容、操作、Storybook DoD、自動検査 DoD を正本化する。
- [x] 13.2 Storybook preview が全 UI で同じ骨格に見えないよう、01〜24 の専用 preview renderer を実装する。
- [x] 13.2.0 01〜24 対応ページを page id ごとの専用 renderer に接続し、選択中 preview 領域の画像署名が同一化しないことを自動テストで固定する。
- [x] 13.2.1 01 Theme / Panel theme の専用 preview と settings / theme action / visual marker を実装する。
- [x] 13.2.2 02 Text の role grid、日本語、英日混在、絵文字、code / muted preview を実装する。
- [x] 13.2.3 03 Icon の SVG grid、size、color token、custom SVG preview を実装する。
- [x] 13.2.4 04 Spinner / LoadingDots の motion / reduced motion / label preview を実装する。
- [x] 13.2.5 05 SvgButton の icon only、disabled、loading、focus preview を実装する。
- [x] 13.2.6 06 TextButton の variant / tone / size matrix preview を実装する。
- [x] 13.2.7 07 IconTextButton の leading / trailing icon spacing preview を実装する。
- [x] 13.2.8 08 Toggle の on / off / disabled preview と state action を実装する。
- [x] 13.2.9 09 SegmentedToggle の selected marker、icon segment、disabled segment preview を実装する。
- [x] 13.2.10 10 SelectBox の trigger + floating options、placeholder、long list preview を実装する。
- [x] 13.2.11 11 ColorSwatch の palette grid、selected ring、disabled preview を実装する。
- [x] 13.2.12 12 Input / TextInput の value、placeholder、slot、invalid、IME / emoji preview を実装する。
- [x] 13.2.13 13 SearchBox の search icon、clear button、regex / word / case option preview を実装する。
- [x] 13.2.14 14 Tooltip の anchor、hover / focus、placement overlay preview を実装する。
- [x] 13.2.15 15 Badge の tone / variant / size / leading icon grid preview を実装する。
- [x] 13.2.16 16 KeyCap の modifier combo、macOS / non-macOS platform preview を実装する。
- [x] 13.2.17 17 Card の header / body / footer / actions slot と nested controls preview を実装する。
- [x] 13.2.18 18 Accordion の header / body、indicator、trigger area、tree mode preview を実装する。
- [x] 13.2.19 19 SplitPane の horizontal / vertical、handle、ratio、min clamp preview を実装する。
- [x] 13.2.20 20 ModalOverlay / Modal の overlay dialog、native modal、focus trap、footer preview を実装する。
- [x] 13.2.21 21 Popover の anchor、placement、offset、outside close preview を実装する。
- [x] 13.2.22 22 ColorPicker の RGB / RGBA panel、color plane、hue、alpha、preview reflection を実装する。
- [x] 13.2.23 23 ColorPicker parity の color-only trigger、trigger size、border、floating panel、seamless hue preview を実装する。
- [x] 13.2.24 24 CodeDiff の split、inline、added / removed / unchanged、collapsed、long line、Japanese diff preview を実装する。
- [x] 13.3 各 UI の settings が typed option を実際に変え、preview / state / action / event に反映されることを UI 別に検査する。
- [x] 13.4 各 UI の部品別 preset が dummy ではなく UI 固有の意味を持つことを検査する。
- [x] 13.5 generic fallback で 01〜24 が完了扱いにならないよう、UI 固有 rendering marker を rendering contract に追加する。
- [x] 13.6 `storybook-requirement-gate.sh` で 01〜24 の UI 固有 marker、settings mutation、preset 差分を必須化する。
- [x] 13.7 全メニューが同じ画面に戻らないことを、page ごとの typed contract と rendering marker で検査する。

## 14. Storybook 体験の責務修正

13 までの完了チェックは、01〜24 の存在、専用 preview、settings / state / event / action / preset の最低限の証跡である。
ただし、Storybook の本来の役割は「カテゴリーから対象部品を選び、その部品専用の UI/UX、設定、操作、状態変化、品質状態を把握できること」である。
中央本文で毎回全 component 一覧を表示する構成はこの役割に反するため、追加修正の対象にする。

- [x] 14.1 左 TreeView のカテゴリー分けは探索と選択のためだけに使い、中央本文から `All components` の全件カード一覧を削除する。
- [x] 14.2 中央本文を、選択中 UI の専用 preview、部品別 preset、settings、state、event、action、quality を扱う画面へ変更する。
- [x] 14.3 preset tab を `Default / Interactive / Edge / Theme` の汎用名ではなく、01〜24 各 UI の確認観点を示す部品別 preset 名で表示する。
- [x] 14.4 旧 01〜24 の全ページで、preset 名、contract 表、status 表が選択中 UI 固有になっていることを自動テストで固定する。
- [x] 14.5 Storybook の品質ゲートと docs から、中央本文に全件 component card を出す前提を外し、選択中 UI 詳細の検証へ置き換える。

## 15. Storybook を操作可能な実画面へ戻す

14 までの修正でも、preview に描いた Button が画面操作の hit target になっていなければ Storybook ではない。
Storybook は見た目だけを描く場所ではなく、部品ごとの layout、option、action、event、state、rendering 差分を実画面で触ってフィードバックする場所である。
この指摘を口頭で終わらせないため、まず Button 系を P0 として画面操作から state と履歴が動くことを固定し、同じ経路を他 UI へ展開する。

- [x] 15.1 Button / TextButton / SvgButton / IconTextButton の preview 上に実クリック hit target を持たせ、押下で action / event / state / rendering が変わることを自動テストで固定する。
- [x] 15.2 Storybook の settings 行を画面上の設定操作として扱い、クリックで UI option が変わり preview と Inspector に反映されることを自動テストで固定する。
- [x] 15.3 Inspector に、静的な callback log ではなく画面操作後の action、event、state、settings revision を表示する。
- [x] 15.4 Storybook guard に、preview button hit test と settings mutation の実装 marker を追加し、見た目だけの Button へ戻る変更を失敗扱いにする。
- [x] 15.5 15.1〜15.4 を起点に、残り UI の操作可能性を page ごとの DoD へ展開する。

## 16. TreeView の実画面要件補強

TreeView は Storybook 自身の左ペインにも関わるため、単なる structured placeholder では不十分である。
垂直線、directory / file icon、開閉状態、context menu、trigger 領域が画面上で見え、入力経路から状態履歴へ反映されることを補強対象にする。

- [x] 16.1 `tree-view` の選択中 preview を専用 renderer に接続し、垂直線、folder / file icon、開閉表示、context menu 表示を描画する。
- [x] 16.2 TreeView preview が汎用 structured 表示に戻らないよう、選択行、folder icon、file icon、垂直線の rendering contract test を追加する。
- [x] 16.3 実ウィンドウ入力モデルに右クリック相当の context click を追加し、TreeView preview 上で action / event / state が変わることを固定する。
- [x] 16.4 TreeView 専用 preview が generic fallback に戻らないことを rendering contract で検査する。

## 17. Panel ごとの縦スクロールと MDN 型 Storybook 操作

Storybook のタブは見た目を切り替える飾りではなく、部品ごとの設定プリセットである。
設定操作後は MDN のサンプルのように上の view が更新され、Panel の縦スクロールは画面全体ではなく Panel ごとに独立して管理される必要がある。

- [x] 17.1 core の Panel model に Panel ごとの vertical scroll state を追加し、親 Panel と子 Panel が別々の scroll state を持てる契約テストを追加する。
- [x] 17.2 Storybook の Navigation / Preview / Details Panel に独立した vertical scroll state を設定し、Panel report と gate で検出する。
- [x] 17.3 TreeView preview 自体に独立した縦スクロールバーと scroll thumb を描画し、親スクロールとは別の rendering contract test を追加する。
- [x] 17.4 preset tab を「設定プリセット」として扱い、切替時に選択中 UI の上部 view 本体が変わることを rendering contract test で固定する。
- [x] 17.5 settings 操作後に、chip や枠線だけでなく preview 本体の表示が変わることを Button と TreeView で固定する。

## 18. Button P0 再実装

Button は Storybook の操作可能性の基準になるため、表示だけ・押せそうな絵だけ・ログ文言だけでは未完了として扱う。
画面上でボタンに見え、押下・keyboard activation・disabled / loading・settings 変更・action / event 発火を同じ page 内で確認できる必要がある。

- [x] 18.1 Button / TextButton / SvgButton / IconTextButton の preview を、押せる大きさ、明確な枠、hover / focus / pressed / disabled / loading の状態表示を持つ専用 renderer に差し替える。
- [x] 18.2 Button label をボタン矩形の上下中央へ描画し、上下中央からずれた表示へ戻る変更を rendering contract test で検出する。
- [x] 18.3 Button 押下時に preview 本体、上部状態列、Inspector の action / event / state が同時に変わることを自動テストで固定する。
- [x] 18.4 Button settings 操作時に typed option が変わり、preview 本体の variant / tone / loading / disabled 表示へ反映されることを自動テストで固定する。
- [x] 18.5 Storybook ヘッダーの無意味な font sample 列を、選択中 UI の状態・操作・最後の action / event を示す summary controls に置き換える。
- [x] 18.6 Button の見た目 layout preset として modern / classic / basic / dense を core option と Storybook preset に持たせる。ただし preset は `ButtonLayoutDto` の初期値生成に限定し、利用者が preset から一部上書き、または DTO 直接指定で padding / min size / border / radius / label align / icon gap を完全上書きできる契約テストを追加する。

## 19. Toggle / 入力系 P0 再実装

Button と同じく、Toggle / Input / SearchBox は見た目だけでは未完了として扱う。
画面上で操作対象が分かり、操作後に preview 本体、summary controls、Inspector の action / event / state が変わる必要がある。

- [x] 19.1 Toggle preview を、off / on / disabled / keyboard の状態が見えるスイッチ表示へ差し替え、押下後につまみ位置と action / event が変わることを自動テストで固定する。
- [x] 19.2 Input / TextInput preview を、value / placeholder / IME / emoji / invalid / disabled が読める入力欄へ差し替え、入力 action 後に value と event が変わることを自動テストで固定する。
- [x] 19.3 SearchBox preview を、検索 icon / clear button / submit / option toggle が見える入力欄へ差し替え、submit 後に action / event が変わることを自動テストで固定する。

## 20. 選択系 UI P0 再実装

SelectBox / SegmentedToggle は、選択前後の表示差分と action / event が見えない限り未完了として扱う。

- [x] 20.1 SelectBox preview を trigger / floating options / selected row が見える表示へ差し替え、選択後に trigger value と action / event が変わることを自動テストで固定する。
- [x] 20.2 SegmentedToggle preview を selected marker / disabled segment / icon segment が見える表示へ差し替え、選択後に marker と action / event が変わることを自動テストで固定する。

## 21. Core render model 反映の補強

Storybook 側の固定文字列や見た目専用 renderer だけで満たすのではなく、core の `UiNode` に typed option / item が落ちていることを完了条件にする。
TreeView は Storybook navigation の基盤でもあるため、線、icon、開閉、trigger、context menu、item list を render model へ渡せない状態を P0 未完了として扱う。

- [x] 21.1 Button layout の入口を `preset`、完全 DTO、`preset + patch DTO` の 3 系統に整理し、preset から一部上書きできる公開契約を core test で固定する。
- [x] 21.2 TreeView の line / icon / font / theme / context menu / default open / toggle / item list を `UiNode.props.tree` に反映し、core test で固定する。

## 22. ColorSwatch P0 再実装

ColorSwatch は色の四角を並べるだけでは未完了として扱う。
palette、selected ring、disabled token、クリック後の selected state、action / event が同じ page 内で見える必要がある。

- [x] 22.1 ColorSwatch preview を palette grid / selected ring / disabled token が見える専用 renderer に差し替え、クリック後に selected ring と action / event / state が変わることを rendering contract test で固定する。

## 23. Tooltip / Popover P0 再実装

Tooltip / Popover は浮いた箱を置くだけでは未完了として扱う。
anchor、placement、open / close、action / event / state が同じ page 内で分かり、操作後に overlay 本体の描画が変わる必要がある。

- [x] 23.1 Tooltip preview を anchor / placement / hover-focus 状態 / action-event-state 表示へ更新し、操作後に overlay 本体が変わることを rendering contract test で固定する。
- [x] 23.2 Popover preview を anchor / placement / offset / outside close / action-event-state 表示へ更新し、操作後に panel 本体が変わることを rendering contract test で固定する。

## 24. Disclosure / Layout / Modal P0 再実装

Accordion / SplitPane / Modal は、静的な説明図だけでは未完了として扱う。
開閉、drag 後の handle、閉じる操作後の dialog state が preview 本体と action / event / state に反映される必要がある。

- [x] 24.1 Accordion preview を header / body / indicator / trigger area が見える表示へ更新し、操作後に body 本体と action / event / state が変わることを rendering contract test で固定する。
- [x] 24.2 SplitPane preview を horizontal / vertical / handle / ratio が見える表示へ更新し、操作後に handle 位置と action / event / state が変わることを rendering contract test で固定する。
- [x] 24.3 Modal / ModalOverlay preview を backdrop / dialog / native modal / footer が見える表示へ更新し、操作後に dialog 本体と action / event / state が変わることを rendering contract test で固定する。

## 25. ColorPicker / CodeDiff P0 再実装

ColorPicker / CodeDiff は見た目の説明図だけでは未完了として扱う。
色変更、RGBA preview、diff mode 変更、collapsed / inline 表示が操作後の preview 本体と action / event / state に反映される必要がある。

- [x] 25.1 ColorPicker preview を RGBA panel / hue / alpha / trigger size / floating panel が見える表示へ更新し、操作後に preview color と action / event / state が変わることを rendering contract test で固定する。
- [x] 25.2 CodeDiff preview を split / inline / added / removed / collapsed / Japanese diff が見える表示へ更新し、操作後に mode 表示と action / event / state が変わることを rendering contract test で固定する。

## 26. Badge / Card P0 再実装

Badge / Card は、静的な tone 表や slot 図だけでは未完了として扱う。
Badge dismiss、Card click、子要素との分離が preview 本体と action / event / state に反映される必要がある。

- [x] 26.1 Badge preview を tone / variant / size / leading icon が見える表示へ更新し、dismiss 操作後に badge 本体と action / event / state が変わることを rendering contract test で固定する。
- [x] 26.2 Card preview を header / body / footer / actions slot が見える表示へ更新し、card 操作後に surface 本体と action / event / state が変わることを rendering contract test で固定する。

## 27. 受動 atom の settings 反映補強

Text / Icon / Theme / Loading / Spinner / Progress は受動 UI でも、settings 変更後に preview 本体が変わらないなら Storybook として不十分である。
設定 chip だけでなく、部品の本体色、role、icon 色、motion phase、progress 幅が変わることを固定する。

- [x] 27.1 Theme / Text / Icon / LoadingDots / Spinner / ProgressBar の settings 変更を preview 本体に反映し、rendering interaction contract test で固定する。

## 28. Storybook 完了判定の実体化

Storybook の page contract は固定値で完了扱いにしてはならない。
preview の node 数、state id、action / event log、UI 固有 preset、必須 page 対応から算出し、受動 UI と操作 UI を分けて判定する。

- [x] 28.1 `StoryPageContract::complete()` の固定完了を廃止し、materialized tree と callback log から完了判定を導出する。
- [x] 28.2 Text / Icon / KeyCap は受動 atom として、操作ログではなく設定・state・preset・visual の証跡で完了判定する。
- [x] 28.3 Badge は受動扱いにせず、core の `dismiss` action と Storybook の action / event log を必須証跡にする。

## 29. Storybook 要件の正本化

Storybook は静的見本帳ではない。
左 TreeView で選択した UI について、layout / option / action / event / state / rendering / panel 独立 scroll を実画面で触ってフィードバックできる場として扱う。
ユーザーは Storybook で検証しない。実装者が自動テスト、数値化された layout / rendering contract、入力回帰、guard で動作担保を済ませる。

- [x] 29.1 `design.md` と `storybook-catalog-contract.md` に、静的見本帳ではなくフィードバック用の実画面であることを明記する。
- [x] 29.2 `specs/kuc-storybook-catalog/spec.md` に、中央本文は全件カード一覧ではなく選択中 UI 詳細であることを必須条件として追加する。
- [x] 29.3 `specs/kuc-storybook-catalog/spec.md` に、layout / option / action / event / state / rendering を同じ実画面で扱えることを必須条件として追加する。
- [x] 29.4 `specs/kuc-storybook-catalog/spec.md` と `specs/kuc-quality-gates/spec.md` に、Navigation / Preview / Details の panel 独立 scroll を検証対象として追加する。
- [x] 29.5 関連 docs に同じ方針を反映し、Storybook を画像証跡置き場や全件一覧ページとして扱わないことを明記する。

## 30. 作業プロセス固定と再発防止

ユーザー指摘を会話履歴だけに残さず、作業プロセスと自動ゲートに固定する。
Storybook は「見た目の描画」ではなく、選択中 UI の layout / option / action / event / state / rendering を実画面で触ってフィードバックする場として扱う。
検証は実装者が自動テストで済ませる。

- [x] 30.1 完了済み subagent 4 件を状態確認後に閉じ、以後の実装・修正は未解決指摘だけを main で引き取る。
- [x] 30.2 settings mutation を手書き文字列差分ではなく、`UiProps` の typed option を実際に変更した結果から生成する。
- [x] 30.3 settings mutation が `-settings` suffix の合成 after に戻らないよう、`assert-storybook-page-layout.py` と `panel_interaction` test で固定する。
- [x] 30.4 Storybook の Navigation / Preview / Inspector の panel 別 scroll を `PanelScrollOffsets` と契約テストで固定する。
- [x] 30.5 `just storybook-regression`、`openspec validate establish-kuc-atoms-molecules-catalog --strict`、`openspec validate ui-core-root-plan --strict`、`git diff --check` を通す。
- [x] 30.6 旧方式の完了証跡を rendering contract と requirement gate に移す。

## 31. v0.1.0 DoD と最新 P0/P1 要求の正本化

この節は、最新要求を会話履歴だけに残さないための正本である。
過去節の完了済み記録は戻さず、v0.1.0 完了判定と追加 P0/P1 要求をここで追跡する。

- [x] 31.1 v0.1.0 DoD を、`katana` と `katana-chat-ui` が `katana-ui-core` だけで app UI を構築できることとして、design、Storybook contract、quality gate、spec に明文化する。
- [x] 31.2 Storybook を静的表示ではなく、各 UI ごとに option / action / event / state / preset / preview / settings を実画面で触ってフィードバックできる場として tasks、contract、spec に明文化する。
- [x] 31.3 Storybook やユーザー操作だけを完了根拠にせず、自動テスト、数値化された layout / rendering contract、入力回帰、guard を品質ゲートにすることを quality gate と spec に固定する。
- [x] 31.12 要件に対応する自動テストがない場合はテストシナリオ漏れとして扱い、01〜24 の各要件行を contract test / interaction regression / rendering contract / guard へ追跡できるようにする。
- [x] 31.4 P0: SVG icon atom を `SvgIcon` 相当の typed props として定義し、SVG source、viewBox、path summary、stroke / fill policy、theme token、accessibility label を core 契約に含める。
- [x] 31.5 P0: Button 専用ではない汎用クリック event を定義し、Button、Card、Badge dismiss、TreeView row などクリック可能 UI が同じ event 契約を使えるようにする。
- [x] 31.6 P0: Text と `AlignNode` 相当の配置 node を Storybook 専用ではなく core 構成部品として定義し、中央揃え、左右寄せ、上下中央揃えを共通検証する。
- [x] 31.7 P1: Accordion と TreeView の開閉制御を shared disclosure foundation として整理し、表示 option は部品ごと、開閉 state / event routing は共通契約にする。
- [x] 31.8 P1: Button 幅 DTO を `auto` / `px` / `percent` / `fill` で定義し、preset、preset + patch DTO、DTO 直接指定の 3 経路を公開契約にする。
- [x] 31.9 P1: Tabs に close、pin、group、dirty、icon を追加し、Storybook preset tabs だけでなく app UI の tab bar として再利用できる契約にする。
- [x] 31.10 P1: Panel ごとの独立 scrollbar と drag を core foundation に定義し、Navigation / Preview / Details / TreeView preview の scroll state が混ざらないことを検証対象にする。
- [x] 31.11 P1: scrollbar 拡張モデルを visibility、track bounds、thumb bounds、offset、overlay / reserved、always / auto / hidden、drag state として定義する。

## 32. Material UI ベースラインの正本化

Material UI は、見た目、押せる範囲、状態表現、option 分割の初期基準として扱う。
ただし React / Material UI 互換ではなく、KUC の Rust 型付き DTO、preset、partial override、internal state、action-event-state 自動テストで実現する。
Storybook は触ってフィードバックするための実画面であり、検証の主根拠や画像回帰 DoD にはしない。

- [x] 32.1 `design.md` に Material UI を v0.1.0 の UI 設計ベースラインとして追加し、React / Material UI 互換 API と画像回帰 DoD を Non-Goals に明記する。
- [x] 32.2 `atoms-contract.md` に共通 props DTO、preset + partial override、DTO 完全上書き、rendering contract 主体の検証方針を追加する。
- [x] 32.3 `atoms-contract.md` で Button / TextButton / SvgButton / IconTextButton を別契約として扱い、SvgButton は見た目上 icon only と明記する。
- [x] 32.4 `atoms-contract.md` で Switch を label + switch の行コンポーネントとして定義し、行全体クリックを action / event / state 自動テスト対象にする。
- [x] 32.5 `specs/kuc-widget-layer/spec.md` に Material UI baseline、共通 props DTO、Button 系分離、Switch 行クリックの必須要件を追加する。

## 33. 01〜24 Panel / Button 粒度の自動検証

ここから先は、01〜24 を「完了済み」とは扱わない。
直近の Panel / Button と同じ粒度で、UI ごとの option、action、event、state、preset、preview、settings、描画差分、state 分離を個別に検査する。
横断 required page テストだけでは不足とし、01〜24 の各行を明示的な case として自動テストへ落とす。

- [x] 33.1 01〜24 の正本行を `legacy_01_24_contract` としてコード上の明示 case に変換する。
- [x] 33.2 各 case で action / event / state / option / after / preset が期待値と一致することを自動テストで固定する。
- [x] 33.3 各 case で preview クリック後に action / event / state と preview 本体の描画差分が出ることを自動テストで固定する。
- [x] 33.4 各 case で settings 変更後に typed option と preview 本体の描画差分が出ることを自動テストで固定する。
- [x] 33.5 各 case で page 間、preset 間の state が共有されないことを自動テストで固定する。
- [x] 33.6 `assert-storybook-page-layout.py` に 33.1〜33.5 の guard marker を追加し、横断 required page テストだけへ戻る変更を失敗扱いにする。
- [x] 33.7 33.1〜33.6 を含めて `just check` を通す。
