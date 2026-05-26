## Why

> Superseded: interaction / visual の要件は `openspec/changes/establish-kuc-atoms-molecules-catalog/` の品質ゲートへ移管する。この change の完了記録は旧基準の証跡として扱う。

`ui-core-root-plan` と `katana-widget-parity-backlog` は完了しているが、完了範囲は KUC の中立 model、内部 state、Storybook panel、theme gate の最低線である。
旧 Floem 実装と同等+αの UI として扱うには、UI ごとの操作意味、専用 props、表示密度、可視検証がまだ不足している。

この change は、寝ている間に進めても終わるか終わらない規模の残作業として、KUC 独自 UI の「実操作できる画面部品」化を対象にする。

## What Changes

- 汎用 `UiInteractionState` だけで表している UI を、UI ごとの専用 props / state / action へ分解する。
- Storybook panel を、一覧表示だけでなく、選択、theme 切替、操作、操作結果、callback log を同じ画面で確認できる確認面にする。
- 可視描画（visual renderer）を `node` / `input value` の簡易 hint から、UI 種別ごとの構造が分かる描画へ拡張する。
- theme、font、style、global state を `UiCoreFacade` から差し替えられる一括窓口として定義する。
- 旧 archive 01〜24 の細かい完了条件を、旧 checkbox ではなく KUC 独自 UI task として再構成する。
- 内製検査は KUC repo 内に追加し、`kal` 側には追記しない。
- Storybook のスクリーンショット（screenshot）と runtime regression を、全 UI の実操作・表示密度・非空描画の根拠にする。

## Capabilities

### New Capabilities

- `ui-core-interaction-model`: UI ごとの props / state / action / callback log を KUC model として定義する。
- `ui-core-facade`: theme、font role、style、global state を multi-platform な core contract として束ねる。
- `storybook-panel-operation`: Storybook panel 上で UI を選択し、操作し、結果を確認できるようにする。
- `visual-parity-gate`: UI 種別ごとの描画、非空領域、theme 適用、操作後差分を検査する。

### Modified Capabilities

- `ui-core-architecture`: `UiInteractionState` の最低線を、UI ごとの専用 state へ拡張する。
- `migration-quality-gates`: Storybook gate を marker ではなく可視 UI と操作の根拠へ拡張する。

## Impact

- `crates/katana-ui-core/src/atom`、`molecule`、`render_model`、`event`、`style` に UI ごとの model が増える。
- `crates/katana-ui-core-storybook/src` に操作可能な panel state、story controls、callback log、visual renderer が増える。
- `scripts/` に KUC 専用の検査が増える。`kal` には追記しない。
- `docs/architecture/ui-separation/ui-core-parity-gap.md` は未完了表から完了証跡表へ更新する。

## Non-Goals

- KatanA 本体や sibling repository は変更しない。
- Floem 旧実装を復活させない。
- egui / GPUI の本実装はこの change の完了条件にしない。
- JSX / TSX 互換は目指さない。
