# Quality gates contract

作成日: 2026-05-18
対象: `kuc-quality-gates`

## 結論

KUC の完了判定は Storybook 目視ではなく、自動テスト、配置回帰（layout regression）、画像回帰（visual regression）、入力回帰（input regression）、静的検査（guard）で行う。
Storybook は操作確認と目視確認の場に限定する。

## Gate ownership

| gate | 主な責務 | 実行入口 |
| --- | --- | --- |
| core contract | theme、font、text、input、event、state、layout | `just check` 内の Rust tests |
| atoms contract | atom ごとの option / action / event / state | `just check` 内の Rust tests |
| molecules contract | 親子 state 分離、合成部品の状態遷移 | `just check` 内の Rust tests |
| visual regression | 非空描画、theme 差分、操作後差分、bounds | `just storybook-regression` |
| input regression | key、日本語入力（IME）、OS 絵文字 | `just check` または dedicated input gate |
| guard | 依存混入、state 外部化、placeholder、網羅漏れ | `just ast-lint` / `just kuc-guardrails` |

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
- preset ごとの snapshot
- Storybook page 登録

表示専用 atom も、theme、font、layout、state、visual regression を必須にする。

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

## 7.4 Visual regression

画像回帰（visual regression）は、単に window が開いたことでは通さない。
次をすべて検査する。

| 検査 | 契約 |
| --- | --- |
| non-empty rendering | 背景以外の pixel が十分にある。 |
| dedicated renderer | required UI が generic fallback に落ちない。 |
| layout bounds | preview、settings、overlay の bounds が期待範囲にある。 |
| theme application | light / dark / Katana accent の差分 pixel が出る。 |
| operation diff | click / input / open / select 後に意味のある差分が出る。 |
| text samples | 日本語、英日混在、絵文字が読める。 |

しきい値を下げて通す変更は禁止する。
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

## 7.6 Static guards

静的検査（guard）は次を失敗扱いにする。

| guard | 失敗条件 |
| --- | --- |
| framework leak | core crate に Floem / egui / GPUI / application domain が混入する。 |
| state ownership | component state が外部 store / global mutable state に逃げる。 |
| placeholder story | Storybook page が文字列や generic `node` だけで終わる。 |
| coverage | option / action / event / state / preset / test が未定義。 |
| Japanese / emoji | 文字描画または入力回帰に日本語・英日混在・絵文字がない。 |
| Storybook role | Storybook screenshot を完了根拠の主役にする。 |

guard の追加は KUC repo 内で行う。
共通 lint に移す必要が出た場合は、KUC 側で失敗条件を固定してから別 change で扱う。
