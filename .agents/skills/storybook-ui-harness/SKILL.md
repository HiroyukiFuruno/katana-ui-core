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

1. `requirements.rs` の required page に対応する専用描画を `visual/dedicated.rs` から辿れる場所に置く。
2. `catalog/preset_labels.rs` に 4 つ以上の preset/tab を定義し、それぞれ見た目と state が変わるようにする。
3. `visual/storybook_ui_option_contract.rs` に page ごとの option contract を 4 つ以上登録する。
4. Inspector は option contract を表示し、代表 option はクリック操作で state/event/action と preview 差分へ反映する。
5. light / dark theme の両方で背景、枠線、文字色、入力面が theme token から描かれることを確認する。
6. 操作可能な UI は `window_interaction` 経由の click / wheel / drag テストを持つ。
7. text-input / text-area / search-box などの入力 UI は、見た目や click action だけで完了にしない。入力欄 focus 後の keyboard input、Backspace、Enter commit が live window の state/action/event/preview へ反映される自動テストを持つ。

## 禁止

- preset/tab の label だけを変え、preview の描画差分がない状態。
- Inspector に option が表示されない状態。
- `interaction_spec` の 1 option だけで UI の option 網羅を完了扱いすること。
- Storybook 外側の scroll や global toggle で、部品自身の scroll / visibility を確認したことにすること。
- light theme で入力面や panel surface が dark 固定色のままになること。
- text-input を `input_commit` の固定表示や preset 差分だけで完了扱いし、実際のキーボード入力経路を持たない状態。

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
完了済み page でも、native smoke / requirement gate が落ちる場合は「Storybook を開いたら何も出来ていない」状態の再発として扱い、leaf change を完了にしない。
