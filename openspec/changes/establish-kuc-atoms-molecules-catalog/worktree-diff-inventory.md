# Worktree diff inventory

作成日: 2026-05-18
対象 change: `establish-kuc-atoms-molecules-catalog`

## 結論

現時点の未コミット差分は、docs / OpenSpec 正本化差分と、既に存在している core / Storybook / guard 実装差分が混在している。
次の実装作業では、この分類を守り、対象外差分を巻き戻さない。

## 分類

### A. docs / OpenSpec 正本化

今回の change の正本化に直接関係する。
優先して別コミット境界に分ける候補。

- `README.md`
- `docs/architecture/ui-separation/owned-ui-task-map.md`
- `docs/architecture/ui-separation/ui-core-parity-gap.md`
- `docs/directory-structure.md`
- `docs/ui-separation-plan.md`
- `docs/widget-extraction-policy.md`
- `openspec/changes/README.md`
- `openspec/changes/establish-kuc-atoms-molecules-catalog/**`
- `openspec/changes/archive/2026-05-25-katana-widget-parity-backlog/{proposal.md,tasks.md}`
- `openspec/changes/archive/2026-05-25-ui-core-interaction-visual-parity/**`
- `openspec/changes/archive/2026-05-25-18-accordion/proposal.md`
- `openspec/changes/archive/2026-05-25-23-color-picker-complete-parity/proposal.md`
- `openspec/changes/archive/2026-05-25-24-code-diff/proposal.md`
- `openspec/changes/ui-core-root-plan/{design.md,tasks.md}`

### B. root plan / dependency docs の既存更新

root plan 完了状態や dependency policy の更新として残っている。
正本化コミットに含める場合は、A と同じ docs 関心事として review する。

- `docs/adr/0002-katana-ui-core-rename.md`
- `docs/architecture/ui-separation/implementation-notes.md`
- `docs/architecture/ui-separation/root-plan-source.md`
- `docs/dependency-policy.md`

### C. core foundation / widget model 実装差分

KUC core 基盤の実装候補。
docs 正本化とは混ぜず、次の実装フェーズで契約テストと合わせて review する。

- `Cargo.toml`
- `Cargo.lock`
- `crates/katana-ui-core/src/atom/**`
- `crates/katana-ui-core/src/component.rs`
- `crates/katana-ui-core/src/facade.rs`
- `crates/katana-ui-core/src/interaction/**`
- `crates/katana-ui-core/src/lib.rs`
- `crates/katana-ui-core/src/molecule/**`
- `crates/katana-ui-core/src/render_model/**`
- `crates/katana-ui-core/src/theme/**`
- `crates/katana-ui-core/tests/**`

### D. Storybook / rendering surface 実装差分

Storybook の実装候補。
Storybook は品質ゲートの代替ではないため、C と guard / regression の整合を確認してから扱う。

- `crates/katana-ui-core-storybook/Cargo.toml`
- `crates/katana-ui-core-storybook/src/catalog/**`
- `crates/katana-ui-core-storybook/src/panel.rs`
- `crates/katana-ui-core-storybook/src/panel/**`
- `crates/katana-ui-core-storybook/src/visual/**`
- `crates/katana-ui-core-storybook/src/lib.rs`
- `crates/katana-ui-core-storybook/src/main.rs`

### E. guard / regression 実装差分

品質ゲート実装候補。
KUC 固有 rule は repo-local `scripts/` に閉じ、`kal` 側へ追記しない。

- `scripts/assert-kuc-state-ownership.py`
- `scripts/assert-storybook-page-layout.py`
- `scripts/kuc_guardrails.py`
- `scripts/storybook-requirement-gate.sh`
- `scripts/test_kuc_guardrails.py`

### F. agent / skill / local instruction 差分

実装本体ではない。
正本化や runtime 実装に混ぜる前に、必要性を別途確認する。

- `AGENTS.md`
- `.agents/skills/kml-harness-engineering/SKILL.md`
- `.agents/skills/kuw-workflow-guide/SKILL.md`

## 保護方針

- `git reset --hard`、`git checkout .`、`git clean -fd` は使わない。
- 対象外カテゴリの差分は巻き戻さない。
- 一括 formatter や広域 rewrite は使わない。
- docs / OpenSpec 正本化と core / Storybook / guard 実装は commit 境界を分ける。
- commit はユーザー承認後にだけ行う。
- 次にコード実装へ入る場合は、C、D、E の順に契約・テスト・Storybook 表示を突き合わせる。

## 次の実装入口

1. A と B を review し、正本化差分として先に固定する。
2. C の core foundation を `kuc-core-foundation` spec に照らして確認する。
3. E の guard が C / D の完了条件を検査できるか確認する。
4. D の Storybook catalog が KUC TreeView / Tabs / preview / settings の要件を満たすか確認する。
