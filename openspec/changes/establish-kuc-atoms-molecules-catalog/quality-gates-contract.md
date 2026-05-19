# Quality gates contract

作成日: 2026-05-18
対象: `kuc-quality-gates`

## 結論

KUC の完了判定は Storybook、ユーザー目視、画像差し替えではなく、自動テスト、配置回帰（layout regression）、入力回帰（input regression）、静的検査（guard）で行う。
Storybook は利用者や開発者が実画面で触ってフィードバックするための場であり、検証責務を持たない。
v0.1.0 の DoD は、`katana` と `katana-chat-ui` が `katana-ui-core` だけで app UI を構築できることを、公開 API 契約、自動テスト、数値化された layout / rendering contract、interaction / input / state / event tests、guard で検証できる状態にする。
画像検証、画像証跡、目視補助、ユーザー検証は release readiness の完了根拠にしない。

## Gate ownership

| gate | 主な責務 | 実行入口 |
| --- | --- | --- |
| core contract | theme、font、text、input、event、state、layout | `just check` 内の Rust tests |
| atoms contract | atom ごとの option / action / event / state | `just check` 内の Rust tests |
| molecules contract | 親子 state 分離、合成部品の状態遷移 | `just check` 内の Rust tests |
| rendering contract | 非空描画 command、theme 差分、操作後差分、bounds、panel 独立 scroll を数値で検証 | `just storybook-regression` |
| interaction / state / event | theme、preset、navigation、settings、hit-target、scroll の操作後状態と履歴 | `just check` / `just storybook-regression` |
| input regression | key、日本語入力（IME）、OS 絵文字 | `just check` または dedicated input gate |
| guard | 依存混入、state 外部化、placeholder、網羅漏れ | `just ast-lint` / `just kuc-guardrails` |
| consumer readiness | `katana` と `katana-chat-ui` が必要とする atoms / molecules / panel / event / state / layout 契約 | `just check` 内の公開 API / adapter 非依存 contract |
| requirement coverage | 要件行ごとの対応テスト有無 | `just release-readiness-check` |

KUC 固有 guard はこの repository の `scripts/` 配下に置く。
`kal` 側には KUC 固有ルールを追記しない。

## 7.1 Core contract tests

core contract test は次を検証する。

| 対象 | 必須検証 |
| --- | --- |
| theme | default dark、light / dark diff、Katana accent token、panel token |
| font | `body` proportional、`code` monospace、fallback role、OS path 非依存 |
| text | 英語、日本語、英日混在、絵文字の line metrics と上下中央揃え |
| input | key、text commit、日本語入力（IME）、OS 絵文字 |
| event | pointer、keyboard、focus、command、input event routing |
| state | duplicate UI の `UiStateId` 一意性、global state への混入禁止 |
| layout | size、spacing、alignment、scroll、overlay、overlap |

## 7.2 Atoms contract tests

atoms contract test は、`atoms-contract.md` の各 UI 行をテスト対象にする。
各 atom は最低限、次を持つ。

- option の既定値と上書き
- action がある場合の handled / ignored
- event がある場合の target と payload
- state id 一意性
- disabled / readonly / loading の抑止
- preset ごとの数値化された layout / rendering contract
- Storybook page 登録

表示専用 atom も、theme、font、layout、state、数値化された rendering contract を必須にする。

## 7.3 Molecules contract tests

molecules contract test は、`molecules-contract.md` の各 UI 行をテスト対象にする。
各 molecule は最低限、次を持つ。

- parent state と child state の分離
- open / close / select / input / drag / dismiss / mode switch
- child event の routing
- disabled / readonly の抑止
- keyboard 操作
- long list、empty state、edge placement
- Storybook page 登録

子 atom の state を parent molecule の global store に吸収する実装は失敗扱いにする。

## 7.4 Rendering contract

描画契約（rendering contract）は、単に window が開いたことや PNG を差し替えたことでは通さない。
次をすべて検査する。

| 検査 | 契約 |
| --- | --- |
| non-empty rendering | 背景以外の描画 command と bounds が十分にある。 |
| dedicated renderer | required UI が generic fallback に落ちない。 |
| layout bounds | preview、settings、overlay の bounds が期待範囲にある。 |
| theme application | light / dark / Katana accent の render token と描画 command 差分が出る。 |
| operation diff | click / input / open / select / settings 変更後に preview 本体へ意味のある render-model 数値差分が出る。 |
| panel scroll | Navigation / Preview / Details の縦スクロール state が独立している。 |
| scrollbar model | 表示方式、thumb bounds、track bounds、drag 後 offset が検査できる。 |
| text samples | 日本語、英日混在、絵文字の line metrics と bounds を検査できる。 |

しきい値を下げて通す変更や、画像ファイル差し替えだけで通す変更は禁止する。
失敗時は描画側または契約側の原因を修正する。

## 7.5 Input regression

入力回帰（input regression）は次を検査する。

| 入力 | 必須確認 |
| --- | --- |
| key | key、modifier、target が保持される。 |
| text | 確定文字列が component state に反映される。 |
| 日本語入力（IME） | composition と commit を区別し、commit 後の文字を保持する。 |
| emoji | OS 絵文字を失わず text commit として扱う。 |
| mixed text | 英日混在と emoji が同じ input で扱える。 |

固定待機や手動確認だけで通すことは禁止する。

## 7.5.1 Requirement coverage

要件に書いたことが自動テストで検証できていない場合、それは実装完了ではなくテストシナリオ漏れとして扱う。
各 UI の option / action / event / state / preset / layout / rendering は、Storybook で触れるだけでは完了にしない。
完了判定時は、要件行から対応する contract test、interaction regression、input regression、state/event test、rendering contract、guard のいずれかへ追跡できる必要がある。

## 7.6 Static guards

静的検査（guard）は次を失敗扱いにする。

| guard | 失敗条件 |
| --- | --- |
| framework leak | core crate に Floem / egui / GPUI / application domain が混入する。 |
| state ownership | component state が外部 store / global mutable state に逃げる。 |
| placeholder story | Storybook page が文字列や generic `node` だけで終わる。 |
| static sample gallery | 中央本文が全件カード一覧だけで、選択中 UI の option / action / event / state / rendering を扱えない。 |
| panel scroll | Navigation / Preview / Details の scroll state が独立していない。 |
| coverage | option / action / event / state / preset / test が未定義。 |
| Japanese / emoji | 文字描画または入力回帰に日本語・英日混在・絵文字がない。 |
| Storybook role | Storybook の画面操作やユーザー確認を完了根拠にする。 |
| no-image policy | 画像回帰、screenshot、目視補助、ユーザー検証を release readiness の完了根拠にする表現を拒否する。 |
| uncovered requirement | 要件行に対応する自動テスト、interaction regression、input regression、state/event test、rendering contract、guard が存在しない。 |

guard の追加は KUC repo 内で行う。
共通 lint に移す必要が出た場合は、KUC 側で失敗条件を固定してから別 change で扱う。
