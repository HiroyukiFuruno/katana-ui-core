---
name: storybook-ui-harness
description: katana-ui-core の Storybook UI を、theme / preset / Inspector option / state-event-action / 実操作確認まで網羅して構築するときに使う。見せかけ preset、Inspector option 不在、利用側で初めて不備に気づく状態を防ぐ。ユーザーが「次をすすめて」「continue」「次」など抽象的に Storybook 実装の継続を依頼した場合も必ず使い、優先順表から次の storybook-page-* leaf change を解決する。
---

# Storybook UI Harness

KUC の Storybook は静的な部品一覧ではなく、利用側へ組み込む前に UI の不備へ気づくための playground として作る。

## 次をすすめての解決

ユーザーが `次をすすめて`、`continue`、`次` のように抽象的に Storybook 実装の継続を依頼した場合、full の OpenSpec change ディレクトリ名を聞き返さない。

1. `rtk proxy python3 scripts/next-storybook-page-change.py --json` を実行する。
2. JSON の `change` を現在の OpenSpec leaf change として扱う。
3. `openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md` の `実装状況`、`DoD 状況`、`次アクション` を読んで着手内容を決める。
4. `NN-add-*` と archive 済み 01〜24 は入力元として扱い、現在の着手キューにはしない。
5. 完了後は leaf change の `tasks.md` と優先順表の状態が食い違わないように更新する。

## 必須構成

新しい UI page、または既存 UI page を直すときは、次を同時に満たす。

### Gate の定義

Storybook / KUC の gate は、UI の仕様・要件・core public API contract を満たしていることを担保し、破壊を検知するための仕組みである。

- gate は「操作ログが増えた」「pixel diff が出た」「native smoke が通った」こと自体を合格理由にしない。
- UI の見た目、hit target、状態遷移、event/action/callback、keyboard/focus/hover/scroll/drag/context menu、layout bounds、text clipping、overlap、disabled/readonly block を仕様として固定する。
- 既にユーザーが指摘した破綻は、同じ壊れ方が再発した時に落ちる自動 test / guard / manifest contract へ落とし込む。
- gate が検出できなかった UI 破綻を見つけた場合は、gate の不備として扱い、実装修正と同時に検出契約を追加する。
- release 判定では、gate の対象外または未検証の領域を `OK` と言わず、明示的に未検証として残す。

### 監査台帳差分の評価ゲート

`docs/storybook-77ui-deep-audit-ledger.md` や machine-readable manifest を更新する前に、対象 UI ごとに次を実施する。

1. 既存台帳の「不足」と「あるべき姿」を、core public API、Storybook harness、既存テスト、直近の失敗ログへ照合する。
2. 既存台帳が過大評価、過小評価、または正しい不足のどれかを明記してから編集する。
3. 「不足」が別 UI の責務だった場合でも、対象 UI の public props / state / action / event / callback / 必須操作に残る真の不足を分離して書く。
4. 直近で通ったテスト、live audit、preset 差分だけを根拠に `verified` へ変更しない。台帳差分の正誤評価と core public API 経由の証跡が揃った場合だけ verified にする。
5. ユーザーから「差分を評価したか」「なぜ確認しないのか」と指摘された場合は、実装を進めず、まず評価根拠、未確認箇所、修正する運用を返答する。

1. `requirements.rs` の required page に対応する専用描画を `visual/dedicated.rs` から辿れる場所に置く。
2. `catalog/preset_labels.rs` に 4 つ以上の preset/tab を定義し、それぞれ見た目と state が変わるようにする。
3. `visual/storybook_ui_option_contract.rs` に page ごとの option contract を 4 つ以上登録する。
4. Inspector は option contract を表示し、代表 option はクリック操作で state/event/action と preview 差分へ反映する。
5. light / dark theme の両方で背景、枠線、文字色、入力面が theme token から描かれることを確認する。
6. 操作可能な UI は `window_interaction` 経由の click / wheel / drag テストを持つ。
7. text-input / text-area / search-box などの入力 UI は、見た目や click action だけで完了にしない。入力欄 focus 後の keyboard input、Backspace、Enter commit が live window の state/action/event/preview へ反映される自動テストを持つ。
8. 入力 atom の runtime state は atom instance ごとに Storybook 内部へ閉じる。page や atom 種別ごとの単一 state で複数 instance、preset、tab の値・focus・caret が同期する状態を禁止する。
9. readonly 入力は focus 可否と値 mutation を分けて扱い、keyboard input、Backspace、Clear などの書き込み経路が block されることを state/action/event の自動テストで固定する。
10. preset/tab label は描画前に文字幅を測り、縮小または clip して隣接 tab へ描画が漏れない数値化テストを持つ。
11. UI の見た目を直す場合は、pixel diff だけでなく、component frame、hit rect、label/text rect、control body、scrollbar、resize handle、status/debug surface の bounds / overlap / spacing guard を追加する。
12. Storybook 専用の debug 表示、click count、state/status chip は、core 部品の viewport/control body に重ねない。必要な場合も部品本体とは別の専用領域へ分離し、矩形テストで固定する。

## 禁止

- preset/tab の label だけを変え、preview の描画差分がない状態。
- Inspector に option が表示されない状態。
- `interaction_spec` の 1 option だけで UI の option 網羅を完了扱いすること。
- live audit の required operation が通っただけで、監査台帳や manifest の `audit_status` を `verified` にすること。台帳の「不足」に structural / slot / aria / public props / design hierarchy の未検証項目が残る場合は、operation 証跡を `evidence` に追加しても `partial` のまま残す。
- 台帳行を更新するときに、既存の「不足」と「あるべき姿」のどちらが正しいか評価せず、直近で通ったテスト結果だけに合わせて完了扱いすること。
- ユーザーが台帳差分、監査差分、manifest 差分について「どちらが正しいか」「評価したか」と問うた場合に、差分の根拠、既存台帳の不足、実装証跡を照合せず、実装継続や完了扱いへ進むこと。
- Storybook 外側の scroll や global toggle で、部品自身の scroll / visibility を確認したことにすること。
- light theme で入力面や panel surface が dark 固定色のままになること。
- text-input を `input_commit` の固定表示や preset 差分だけで完了扱いし、実際のキーボード入力経路を持たない状態。
- 入力 atom の runtime state を単一フィールドで持ち、tab 切り替えや複数 instance で値・focus・caret を共有してしまう状態。
- readonly preset を表示だけ readonly にし、Storybook の keyboard 経路から値を書き換えられる状態。
- 長い preset/tab label を隣の tab へ描画して、文字がめり込む状態。
- UI として表示した text/status/debug label が、control body、scroll viewport、scrollbar、resize handle、隣接 control へめり込む状態。
- component action hit rect と実際の描画フレームがずれ、操作対象と見えている UI が別物になる状態。
- pixel diff、action/event/state 更新、native smoke だけを根拠に、bounds / overlap / spacing の破綻を見逃して release 判定へ進むこと。

## 検証

最低限、次を実行する。

```bash
rtk just ast-lint
rtk just storybook-check
rtk cargo test -p katana-ui-core-storybook --locked
rtk just storybook-interaction-smoke
rtk just storybook-requirement-gate
```

画面系変更では、最後に PNG snapshot または native window で実際の見え方と操作も確認する。
ただし目視確認は補助証跡に限る。見た目の指摘を受けた場合は、同じ壊れ方を再発検知する bounds / overlap / spacing / hit rect の自動 guard を必ず追加する。
完了済み page でも、native smoke / requirement gate が落ちる場合は「Storybook を開いたら何も出来ていない」状態の再発として扱い、leaf change を完了にしない。
