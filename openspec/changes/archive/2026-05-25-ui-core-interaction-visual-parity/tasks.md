# Tasks — ui-core-interaction-visual-parity

> Superseded: Storybook 操作面と visual gate は `openspec/changes/establish-kuc-atoms-molecules-catalog/` の自動品質ゲートへ移管する。このファイルの `[x]` は旧基準の証跡であり、現在の部品完了根拠にしない。

## 1. 棚卸しと境界固定

- [x] 1.1 `docs/architecture/ui-separation/ui-core-parity-gap.md` の「後続 UI 詳細」を、この change の対象表へ更新する。
- [x] 1.2 archive 01〜24 と active 18 / 23 / 24 から、旧 props、操作、Storybook 条件を UI ごとに抽出する。
- [x] 1.3 抽出結果を `docs/architecture/ui-separation/owned-ui-task-map.md` に KUC task として追記する。
- [x] 1.4 `kal` 側へ追記しないことを `scripts/` の検査方針に明記する。
- [x] 1.5 `git diff --check -- openspec/changes/ui-core-interaction-visual-parity docs/architecture/ui-separation` を通す。

## 2. typed interaction model 基盤

- [x] 2.1 `UiInteractionState` を横断 summary として残す方針を docs と spec に固定する。
- [x] 2.2 UI ごとの typed props / state / action を置く module 境界を決める。
- [x] 2.3 `UiAction` と `UiActionResult` を定義し、操作後 state と callback log を返せるようにする。
- [x] 2.4 `Component` が内部 state を保持したまま action を受けられる API を追加する。
- [x] 2.5 同種同 label UI を複数置いたとき、typed state も `UiStateId` ごとに分離される test を追加する。
- [x] 2.6 typed state を `serde` で snapshot 化できる test を追加する。
- [x] 2.7 外部 store を必須にする API が core に入っていないことを script で検査する。
- [x] 2.8 `UiCoreFacade` で theme、font role、style sheet、global state を明示的に渡せるようにする。
- [x] 2.9 font は core で OS 固有 path を持たず、`Proportional` / `Monospace` の抽象 family と role だけを持つ。
- [x] 2.10 global state は theme / focus / overlay など横断状態に限定し、UI ごとの内部 state を奪わない test を追加する。
- [x] 2.11 Storybook の文字描画を日本語・絵文字・font fallback 対応の経路へ切り替え、snapshot に表示する。
- [x] 2.12 英語、日本語、英日混在、絵文字混在の上下中央揃えを同一行ボックスで検査する。

## 3. atoms / simple molecules

- [x] 3.1 Text / Icon / KeyCap に accessibility props と visual role を追加する。
- [x] 3.2 Button / SvgButton / TextButton / IconTextButton に variant、tone、size、disabled、loading、callback log を追加する。
- [x] 3.3 Input / TextInput に value、placeholder、readonly、invalid、leading / trailing slot、clear action を追加する。
- [x] 3.4 Checkbox / Radio / Toggle に checked、disabled、on_change 相当 action を追加する。
- [x] 3.5 Badge / StatusBar / NotificationToast に severity、variant、dismiss action を追加する。
- [x] 3.6 ProgressBar / Spinner / LoadingDots に determinate / indeterminate、label、animation state を追加する。
- [x] 3.7 Storybook panel 上で 3.1〜3.6 の操作結果と callback log を確認できるようにする。

## 4. selector / overlay / navigation 系

- [x] 4.1 SelectBox に open、options、selected、disabled、long list、close action を追加する。
- [x] 4.2 ComboBox に input value、filter result、free input、selected、keyboard navigation を追加する。
- [x] 4.3 Menu / MenuButton に open、items、framed、trigger、select action を追加する。
- [x] 4.4 Popover / Tooltip に placement、offset、outside click、Esc、anchor summary を追加する。
- [x] 4.5 ModalOverlay に backdrop、Esc、focus return、dismiss policy を追加する。
- [x] 4.6 Accordion に controlled / uncontrolled、disabled、multiple、indicator position、tree mode props を追加する。
- [x] 4.7 Tabs / Breadcrumb / SideMenu に selected、crumb action、hover expansion、icon action を追加する。
- [x] 4.8 SelectionList / SlideControl に section、marker、more row、min / max / step / binding を追加する。
- [x] 4.9 Storybook panel 上で 4.1〜4.8 の開閉、選択、キーボード操作、閉じる条件を確認できるようにする。

## 5. heavy UI 詳細

- [x] 5.1 CodeDiff に `CodeDiffSource`、表示 mode、行 model、ハイライト範囲、省略ブロックを追加する。
- [x] 5.2 CodeDiff の split / inline、左右 / 上下、長い行、空白記号、末尾改行差分を snapshot test にする。
- [x] 5.3 ColorPicker に RGBA model、色面、色相、透明度、readonly、disabled、trigger size、title を追加する。
- [x] 5.4 ColorPicker のドラッグ相当 action と連続更新 report を Storybook regression に追加する。
- [x] 5.5 TreeView に node id、depth、expanded、selected、active、line display を追加する。
- [x] 5.6 CommandPalette に query、filtered actions、selected index、Enter / Esc / Arrow key action を追加する。
- [x] 5.7 DynamicArrayEditor に add、delete、reorder、edit、empty state を追加する。
- [x] 5.8 5.1〜5.7 は汎用 `value` / `item_count` だけで完了扱いにしない guard を追加する。

## 6. Storybook panel 操作面

- [x] 6.1 左ナビで story を選択し、右プレビューが選択 story 詳細へ切り替わる model を追加する。
- [x] 6.2 light / dark theme 切替 control を panel 内に追加し、両 panel と story root に theme id を反映する。
- [x] 6.3 story controls を KUC component の action API に接続する。
- [x] 6.4 callback log panel を追加し、操作名、target state id、before / after summary を表示する。
- [x] 6.5 `--headless-scenario` に story selection、theme switch、操作 sequence の report を追加する。
- [x] 6.6 `just storybook` の可視 window で同じ操作面が見えることを確認する。

## 7. visual renderer coverage

- [x] 7.1 visual renderer を `UiNodeKind` ごとの責務に分割する。
- [x] 7.2 required UI で `node` fallback が出たら gate を失敗させる。
- [x] 7.3 Button / Input / Select / Toggle / Badge / Progress / Toast の見た目を専用描画にする。
- [x] 7.4 Popover / Tooltip / ModalOverlay / Modal の重ね表示を専用描画にする。
- [x] 7.5 CodeDiff の行、追加、削除、省略、ハイライトを専用描画にする。
- [x] 7.6 ColorPicker の色面、色相、透明度、trigger preview を専用描画にする。
- [x] 7.7 TreeView / CommandPalette / DynamicArrayEditor を専用描画にする。
- [x] 7.8 screenshot は light / dark、操作前 / 操作後、modal window を保存する。
- [x] 7.9 非空 pixel、UI coverage、theme 差分、操作後差分を script で検査する。

## 8. KUC 専用 guard

- [x] 8.1 `scripts/assert-kuc-state-ownership.py` を typed state / action まで検査する。
- [x] 8.2 `scripts/assert-storybook-page-layout.py` を操作面と callback log まで検査する。
- [x] 8.3 visual fallback 禁止 guard を追加する。
- [x] 8.4 archive 01〜24 の旧 checkbox を完了根拠にしない guard を追加する。
- [x] 8.5 `kal` 側へルール追加していないことを repo-local guard で確認する。

## 9. 検証

- [x] 9.1 `rtk just ast-lint` を通す。
- [x] 9.2 `rtk just storybook-check` を通す。
- [x] 9.3 `rtk just storybook-regression` を通す。
- [x] 9.4 `rtk just release-verify` を通す。
- [x] 9.5 `rtk ./scripts/openspec validate ui-core-interaction-visual-parity --strict` を通す。
- [x] 9.6 `target/storybook-panel.png` と追加 screenshot を確認し、証跡 path を docs に記録する。
- [x] 9.7 `docs/architecture/ui-separation/ui-core-parity-gap.md` を完了証跡表へ更新する。
